use std::{collections::HashMap, path::PathBuf, process::Stdio};

use essential_rest_client::EssentialClient;
use essential_types::{intent::Intent, IntentAddress};
use nft_front_end::Nft;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    process::Command,
};

#[tokio::test]
#[ignore = "Will break CI because it requires the essential-rest-server to be on the path"]
async fn mint_and_transfer() {
    let mut child = Command::new("essential-rest-server")
        .env(
            "RUST_LOG",
            "[run_loop]=trace,[check_intent]=trace,[constraint]=trace,[recover_secp256k1]=trace",
        )
        .arg("--db")
        .arg("memory")
        .arg("0.0.0.0:0")
        .arg("--loop-freq")
        .arg("1")
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = child.stdout.take().unwrap();

    let buf = BufReader::new(stdout);
    let mut lines = buf.lines();

    let port;
    loop {
        if let Some(line) = lines.next_line().await.unwrap() {
            if line.contains("Listening") {
                port = line
                    .split(':')
                    .next_back()
                    .unwrap()
                    .trim()
                    .parse::<u16>()
                    .unwrap();
                break;
            }
        }
    }

    tokio::spawn(async move {
        loop {
            if let Some(line) = lines.next_line().await.unwrap() {
                println!("{}", line);
            }
        }
    });

    assert_ne!(port, 0);

    let server_address = format!("http://localhost:{}", port);

    let auth_intents = compile_pint_file("auth.pnt").await;

    let auth_addresses = named_addresses(&auth_intents, &["init", "key"]);
    for (name, address) in &auth_addresses {
        println!(
            "{}: set: {}, intent: {}",
            name,
            hex::encode_upper(address.set.0),
            hex::encode_upper(address.intent.0),
        );
    }

    let nft_intents = compile_pint_file("nft.pnt").await;

    // Deploy intents
    let client = EssentialClient::new(server_address.clone()).unwrap();

    let nft_addresses = named_addresses(&nft_intents, &["mint", "transfer"]);

    let mut wallet = essential_wallet::Wallet::temp().unwrap();

    wallet
        .new_key_pair("deployer", essential_wallet::Scheme::Secp256k1)
        .ok();

    let intents = wallet.sign_intent_set(nft_intents, "deployer").unwrap();
    client.deploy_intent_set(intents).await.unwrap();
    let intents = wallet.sign_intent_set(auth_intents, "deployer").unwrap();
    client.deploy_intent_set(intents).await.unwrap();

    let account_name = "alice";

    let art = "this_is_art";
    let hash = essential_signer::hash_bytes(art.as_bytes()).unwrap();

    let mut nft = Nft::new(
        server_address,
        nft_addresses
            .into_iter()
            .chain(auth_addresses.into_iter())
            .collect(),
        wallet,
    )
    .unwrap();

    nft.create_account(account_name).ok();

    nft.mint(account_name, hash).await.unwrap();

    while !nft.do_i_own(account_name, hash).await.unwrap() {
        println!("I don't own the nft");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("I own the nft!!!");

    let to = "bob";
    nft.create_account(to).ok();
    nft.transfer(account_name, to, hash).await.unwrap();

    while nft.do_i_own(account_name, hash).await.unwrap() {
        println!("I own the nft");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("I don't the nft!!!");

    while !nft.do_i_own(to, hash).await.unwrap() {
        println!("{} doesn't own the nft", to);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("{} owns the nft!", to);
}

async fn compile_pint_file(name: &str) -> Vec<Intent> {
    // Compile Pint files
    let pint_dir_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../pint");
    let pint_path = PathBuf::from(pint_dir_path).join(name);
    assert!(pint_path.exists());
    let pint_target_path = PathBuf::from(pint_dir_path).join("target");
    std::fs::create_dir(pint_dir_path).ok();

    let output = Command::new("pintc")
        .arg(pint_path.display().to_string())
        .arg("--output")
        .arg(pint_target_path.join(name))
        .output()
        .await
        .unwrap();

    assert!(
        output.status.success(),
        "pintc failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let file = File::open(pint_target_path.join(name)).await.unwrap();
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await.unwrap();

    let intents: Vec<Intent> =
        serde_json::from_slice(&bytes).expect("failed to deserialize intent set");
    intents
}

fn named_addresses(intents: &[Intent], names: &[&str]) -> HashMap<String, IntentAddress> {
    assert_eq!(intents.len(), names.len());
    let set = essential_hash::intent_set_addr::from_intents(intents);
    intents
        .iter()
        .zip(names.iter())
        .map(|(intent, name)| {
            (
                name.to_string(),
                IntentAddress {
                    set: set.clone(),
                    intent: essential_hash::content_addr(intent),
                },
            )
        })
        .collect()
}
