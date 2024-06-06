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
            "[run_loop]=trace,[check_intent]=trace,[constraint]=trace",
        )
        // .env("RUST_LOG", "trace")
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

    // child.stdout = Some(lines.into_inner().into_inner());
    assert_ne!(port, 0);

    let server_address = format!("http://localhost:{}", port);

    // Compile Pint files
    let pint_dir_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../pint");
    let pint_path = PathBuf::from(pint_dir_path).join("nft.pnt");
    assert!(pint_path.exists());
    let pint_target_path = PathBuf::from(pint_dir_path).join("target");
    std::fs::create_dir(pint_dir_path).ok();

    let output = Command::new("pintc")
        .arg(pint_path.display().to_string())
        .arg("--output")
        .arg(pint_target_path.join("nft.json"))
        .output()
        .await
        .unwrap();

    assert!(
        output.status.success(),
        "pintc failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Deploy intents
    let client = EssentialClient::new(server_address.clone()).unwrap();

    let file = File::open(pint_target_path.join("nft.json")).await.unwrap();
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await.unwrap();

    let intents: Vec<Intent> =
        serde_json::from_slice(&bytes).expect("failed to deserialize intent set");
    let intent_addresses = ["mint".to_string()]
        .into_iter()
        .zip(intents.iter().map(essential_hash::content_addr))
        .collect::<HashMap<String, _>>();

    essential_wallet::new_key_pair("deployer".to_string(), essential_wallet::Scheme::Secp256k1)
        .ok();

    let intents = essential_wallet::sign_intent_set(intents, "deployer").unwrap();
    let set_address = client.deploy_intent_set(intents).await.unwrap();

    let addresses = intent_addresses
        .into_iter()
        .map(|(n, i)| {
            (
                n,
                IntentAddress {
                    set: set_address.clone(),
                    intent: i,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let account_name = "alice";
    let art = "this_is_art";
    let hash = essential_signer::hash_bytes(art.as_bytes()).unwrap();

    let nft = Nft::new(server_address, addresses).unwrap();

    nft.create_account(account_name).ok();

    nft.mint(account_name, hash).await.unwrap();

    while !nft.do_i_own(account_name, hash).await.unwrap() {
        println!("I don't own the nft");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("I own the nft!!!");
}
