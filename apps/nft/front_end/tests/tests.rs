use std::{collections::HashMap, path::PathBuf, process::Stdio};

use essential_rest_client::EssentialClient;
use essential_types::{intent::Intent, ContentAddress, IntentAddress};
use nft_front_end::{Addresses, Nft};
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

    let key_intents = compile_pint_file("key.pnt").await;
    let key_addresses = named_addresses(&key_intents, &["init", "key"]);

    let nft_intents = compile_pint_file("nft.pnt").await;
    let nft_addresses = named_addresses(&nft_intents, &["mint", "transfer"]);

    let auth_intents = compile_pint_file("auth.pnt").await;
    let auth_addresses = named_addresses(&auth_intents, &["auth"]);

    let swap_any_intents = compile_pint_file("swap_any.pnt").await;
    let swap_any_addresses = named_addresses(&swap_any_intents, &["init", "swap"]);

    let addresses = Addresses {
        nft: nft_addresses["mint"].set.clone(),
        nft_mint: nft_addresses["mint"].clone(),
        nft_transfer: nft_addresses["transfer"].clone(),
        auth: auth_addresses["auth"].set.clone(),
        auth_auth: auth_addresses["auth"].clone(),
        key: key_addresses["key"].set.clone(),
        key_init: key_addresses["init"].clone(),
        key_key: key_addresses["key"].clone(),
        swap_any: swap_any_addresses["swap"].set.clone(),
        swap_any_init: swap_any_addresses["init"].clone(),
        swap_any_swap: swap_any_addresses["swap"].clone(),
    };

    print_addresses(&addresses);

    // Deploy intents
    let client = EssentialClient::new(server_address.clone()).unwrap();

    let mut wallet = essential_wallet::Wallet::temp().unwrap();

    wallet
        .new_key_pair("deployer", essential_wallet::Scheme::Secp256k1)
        .ok();

    let intents = wallet.sign_intent_set(nft_intents, "deployer").unwrap();
    client.deploy_intent_set(intents).await.unwrap();
    let intents = wallet.sign_intent_set(key_intents, "deployer").unwrap();
    client.deploy_intent_set(intents).await.unwrap();
    let intents = wallet.sign_intent_set(auth_intents, "deployer").unwrap();
    client.deploy_intent_set(intents).await.unwrap();
    let intents = wallet
        .sign_intent_set(swap_any_intents, "deployer")
        .unwrap();
    client.deploy_intent_set(intents).await.unwrap();

    let account_name = "alice";

    let art = "this_is_art";
    let token = essential_signer::hash_bytes(art.as_bytes()).unwrap();

    let mut nft = Nft::new(server_address, addresses.clone(), wallet).unwrap();

    nft.create_account(account_name).ok();

    nft.mint(account_name, token).await.unwrap();

    while !nft.do_i_own(account_name, token).await.unwrap() {
        println!("I don't own the nft");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("I own the nft!!!");

    let to = "bob";
    nft.create_account(to).ok();
    nft.transfer(account_name, to, token).await.unwrap();

    while nft.do_i_own(account_name, token).await.unwrap() {
        println!("I own the nft");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("I don't the nft!!!");

    while !nft.do_i_own(to, token).await.unwrap() {
        println!("{} doesn't own the nft", to);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("{} owns the nft!", to);

    let diff_art = "different_art";
    let art_token = essential_signer::hash_bytes(diff_art.as_bytes()).unwrap();

    nft.mint_for_contract(&addresses.swap_any_swap, art_token)
        .await
        .unwrap();

    while !nft
        .do_i_own_contract(&addresses.swap_any_swap, art_token)
        .await
        .unwrap()
    {
        println!("Contract doesn't own the nft");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("Contract owns the nft!!!");

    nft.init_swap_any(art_token).await.unwrap();

    loop {
        if let Some(t) = nft.swap_any_owns().await.unwrap() {
            if t == art_token {
                println!("Contract initialized");
                break;
            }
        }
        println!("Contract not initialized");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    nft.swap_with_contract("bob", token).await.unwrap();

    while nft.do_i_own("bob", art_token).await.unwrap() {
        println!("Bob doesn't own different nft");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("Bob owns different nft");

    while !nft
        .do_i_own_contract(&addresses.swap_any_swap, token)
        .await
        .unwrap()
    {
        println!("Contract doesn't own the nft");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("Contract owns the original nft!!!");

    loop {
        if let Some(t) = nft.swap_any_owns().await.unwrap() {
            if t == token {
                println!("Contract state synced");
                break;
            }
        }
        println!("Contract state out of sync");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
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

fn print_addresses(addresses: &Addresses) {
    let Addresses {
        nft,
        nft_mint,
        nft_transfer,
        auth,
        auth_auth,
        key,
        key_init,
        key_key,
        swap_any,
        swap_any_init,
        swap_any_swap,
    } = addresses;
    print_set_address("nft", nft);
    print_address("nft_mint", nft_mint);
    print_address("nft_transfer", nft_transfer);
    print_set_address("auth", auth);
    print_address("auth_auth", auth_auth);
    print_set_address("key", key);
    print_address("key_init", key_init);
    print_address("key_key", key_key);
    print_set_address("swap_any", swap_any);
    print_address("swap_any_init", swap_any_init);
    print_address("swap_any_swap", swap_any_swap);
}

fn print_address(name: &str, address: &IntentAddress) {
    println!(
        "{}: set: {}, intent: {}",
        name,
        hex::encode_upper(address.set.0),
        hex::encode_upper(address.intent.0),
    );
}

fn print_set_address(name: &str, address: &ContentAddress) {
    println!("{}: set: {}", name, hex::encode_upper(address.0),);
}
