use essential_app_utils::{self as utils, compile::compile_pint_project};
use essential_wallet::Wallet;
use essential_rest_client::node_client::EssentialNodeClient;
use essential_types::{convert::word_4_from_u8_32, Word, ContentAddress};
use regex::Regex;
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command as TokioCommand},
};
use token::Query;

// Constants for the test

/// The private key for the test account.
const PRIV_KEY: &str = "128A3D2146A69581FD8FC4C0A9B7A96A5755D85255D4E47F814AFA69D7726C8D";
/// The name of the token.
const TOKEN_NAME: &str = "alice coin";
/// The symbol of the token.
const TOKEN_SYMBOL: &str = "ALC";
/// The path to the PINT project directory.
const PINT_DIRECTORY: &str = "../pint";
/// The name of Alice's test account.
const ALICE: &str = "alice";

#[tokio::test]
async fn mint_and_transfer_integration() {
    // Initialize tracing for better debugging
    tracing_subscriber::fmt::init();

    let (_builder_process, node_address, builder_address) = start_essential_builder().await;

    // Ensure the token contract is compiled
    let _ =
        compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint/token").into())
            .await
            .unwrap();

    // Create a temporary wallet for testing, with password "password"
    // what does this do?
    // - creates a new wallet with password "password", using a tempdir.

    // @todo add a function to create a new unlocked test wallet using the cli.
    // let mut wallet = essential_wallet::Wallet::temp().unwrap();


    // Set up Alice's account
    // let alice = "alice";
    let key = hex::decode(PRIV_KEY).unwrap();
    create_test_wallet(ALICE, PRIV_KEY).await;
    // wallet
    //     .insert_key(
    //         alice,
    //         essential_signer::Key::Secp256k1(
    //             essential_signer::secp256k1::SecretKey::from_slice(&key).unwrap(),
    //         ),
    //     )
    //     .unwrap();

    // deploy the token contract
    deploy_contract(builder_address.clone()).await;

    // Set the initial mint amount and get Alice's hashed key
    let first_mint_amount = 1000000;
    // let alice_hashed_key = hash_key(&mut wallet, ALICE);

    // // Get Alice's nonce key
    // let alice_nonce_key = token::nonce_key(alice_hashed_key);
    // let nonce = nonce(&node_address, &token::token::ADDRESS, alice_nonce_key).await;

    // // Prepare the mint
    // let init = token::mint::Init {
    //     hashed_key: alice_hashed_key,
    //     amount: first_mint_amount,
    //     decimals: 18,
    //     nonce: Query(nonce),
    // };

    // let alice_balance_before = balance(ALICE, &node_address, PINT_DIRECTORY).await;

    // mint(&node_address, PINT_DIRECTORY, first_mint_amount, ALICE, TOKEN_NAME, TOKEN_SYMBOL, &mut wallet).await;

    // let alice_balance_after = balance(ALICE, &node_address, PINT_DIRECTORY).await;

}

async fn create_test_wallet(name: &str, key: &str) {
    // let key_string = key.iter()
    //     .map(|num| num.to_string())
    //     .collect::<Vec<String>>()
    //     .join(",");

    let wallet_output = TokioCommand::new("essential-wallet")
        .args([
            "--password",
            "password",
            "temp",
            name,
            "--private-key",
            key,
        ])
        .output()
        .await
        .expect("Failed to execute command");
    dbg!(&wallet_output);

}

async fn mint(node_address: &str, pint_directory: &str, amount: i64, account: &str, name: &str, symbol: &str, wallet: &mut Wallet) {
    let mut mint_process = TokioCommand::new("cargo")
        .args([
            "run",
            "--",
            "mint",
            &format!("http://{}", node_address),
            pint_directory,
            &amount.to_string(),
            account,
            name,
            symbol,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    // Capture stdout
    let stdout = mint_process.stdout.take().expect("Failed to capture stdout");
    let mut reader = BufReader::new(stdout).lines();

    // Provide the password via stdin
    let mut password_provided = false;
    if let Some(mut stdin) = mint_process.stdin.take() {
        while let Some(line) = reader.next_line().await.expect("Failed to read line") {
            println!("stdout: {}", line);
            if line.contains("Enter password to unlock wallet:") {
                stdin.write_all(b"password\n").await.expect("Failed to write to stdin");
                password_provided = true;
                break;
            }
        }
    }

    if !password_provided {
        panic!("Password prompt not found in stdout");
    }

    let mint_output = mint_process
        .wait_with_output()
        .await
        .expect("Failed to wait on child");

    dbg!(&mint_output);
    assert!(mint_output.status.success(), "Command failed to run");
}

async fn burn(node_address: String, pint_directory: &str) {}

async fn transfer(node_address: String, pint_directory: &str) {}

async fn balance(account: &str, node_address: &str, pint_directory: &str) -> i64 {
    let balance_output = TokioCommand::new("cargo")
        .args([
            "run",
            "--",
            "balance",
            account,
            &format!("http://{}", node_address),
            pint_directory,
        ])
        .output()
        .await
        .expect("Failed to execute command");

    dbg!(&balance_output);
    assert!(balance_output.status.success(), "Command failed to run");
    let balance_str = String::from_utf8(balance_output.stdout).expect("Failed to parse output as UTF-8");
    balance_str
        .trim()
        .parse::<i64>()
        .expect("Failed to parse balance")

}

async fn external_balance(node_address: String, pint_directory: &str) {}

async fn deploy_contract(builder_address: String) {
    let deploy_output = TokioCommand::new("essential-rest-client")
        .args([
            "deploy-contract",
            &format!("http://{}", builder_address.as_str()),
            concat!(env!("CARGO_MANIFEST_DIR"), "/../pint/token/out/debug/token.json").into(),
        ])
        .output()
        .await
        .expect("Failed to execute command");

    assert!(deploy_output.status.success(), "Command failed to run");
}

async fn start_essential_builder() -> (Child, String, String) {
    let mut builder_process = TokioCommand::new("essential-builder")
        .args([
            "--block-interval-ms",
            "100",
            // @todo remove once new Node & Builder published.
            "--state-derivation",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to start essential-builder");

    // Ensure the process is running
    let stdout = builder_process
        .stdout
        .take()
        .expect("Failed to capture stdout");
    let mut reader = BufReader::new(stdout).lines();

    // Regular expression to capture the address
    let regx_node = Regex::new(r"Starting node API server at (.+)").unwrap();
    let regx_builder = Regex::new(r"Starting builder API server at (.+)").unwrap();
    let mut node_address = String::new();
    let mut builder_address = String::new();

    // Wait for the specific line in the builder output
    while let Some(line) = reader.next_line().await.unwrap() {
        println!("Builder output: {}", line);
        if let Some(captures) = regx_node.captures(&line) {
            node_address = captures[1].to_string();
        }
        if let Some(captures) = regx_builder.captures(&line) {
            builder_address = captures[1].to_string();
        }
        if line.contains("Running the block builder") {
            break;
        }
    }

    (builder_process, node_address, builder_address)
}

// Helper function to hash a public key
fn hash_key(wallet: &mut Wallet, account_name: &str) -> [Word; 4] {
    let public_key = wallet.get_public_key(account_name).unwrap();
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        panic!("Invalid public key")
    };
    let encoded = essential_sign::encode::public_key(&public_key);
    word_4_from_u8_32(essential_hash::hash_words(&encoded))
}

// Helper function to get the current nonce
async fn nonce(node_address: &str, content_address: &ContentAddress, key: Vec<i64>) -> Option<Vec<i64>> {
    let hex_key = key.iter()
        .map(|num| format!("{:016x}", num))
        .collect::<Vec<String>>()
        .join("");

    let nonce_output = TokioCommand::new("essential-rest-client")
        .args([
            "query-state",
            "--content-address",
            &content_address.to_string(),
            &format!("http://{}", node_address),
            &hex_key,
        ])
        .output()
        .await
        .expect("Failed to execute command");

    assert!(nonce_output.status.success(), "Command failed to run");

    let output_str = String::from_utf8(nonce_output.stdout).expect("Failed to parse output as UTF-8");
    let parsed_result: Result<Vec<i64>, _> = output_str
        .trim()
        .split(',')
        .map(|s| s.trim().parse::<i64>())
        .collect();

    match parsed_result {
        Ok(vec) => Some(vec),
        Err(_) => None,
    }
}