use essential_app_utils::local_server::setup_server;
use std::path::PathBuf;
use token::{actions::deploy_app, token::Token};

const PRIV_KEY: &str = "128A3D2146A69581FD8FC4C0A9B7A96A5755D85255D4E47F814AFA69D7726C8D";

#[tokio::test]
#[ignore = "Will break CI because it requires the essential-rest-server to be on the path"]
async fn mint_and_transfer_local() {
    let (server_address, _child) = setup_server().await.unwrap();
    mint_and_transfer(server_address).await;
}

#[tokio::test]
#[ignore = "Will break CI because it runs on the deployed server."]
async fn mint_and_transfer_remote() {
    let server_address = std::env::var("ESSENTIAL_SERVER_ADDR").unwrap();
    mint_and_transfer(server_address).await;
}

async fn mint_and_transfer(server_address: String) {
    // setup essential wallet
    let mut wallet = essential_wallet::Wallet::temp().unwrap();

    // setup deployer account
    let deployer_name = "deployer".to_string();
    wallet
        .new_key_pair(&deployer_name, essential_wallet::Scheme::Secp256k1)
        .ok();

    let alice = "alice";
    let key = hex::decode(PRIV_KEY).unwrap();
    wallet
        .insert_key(
            alice,
            essential_signer::Key::Secp256k1(
                essential_signer::secp256k1::SecretKey::from_slice(&key).unwrap(),
            ),
        )
        .unwrap();

    let alice_pub_key = wallet.get_public_key(alice).unwrap();

    let alice_pub_key = to_hex(&alice_pub_key);
    println!("alice public key: {}", alice_pub_key);

    let pint_directory = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint"));

    let predicate_addresses = deploy_app(
        server_address.clone(),
        &mut wallet,
        &deployer_name,
        pint_directory.clone(),
    )
    .await
    .unwrap();

    let mut token = Token::new(server_address.clone(), predicate_addresses, wallet).unwrap();

    // alice mint 800 tokens
    let first_mint_amount = 1000000;

    let _mint_solution_address = token.mint(alice, first_mint_amount).await.unwrap();
    let mut balance = None;
    while balance.is_none() {
        println!("{} balance {}", alice, 0);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        balance = token.balance(alice).await.unwrap();
    }
    println!("{} balance {}", alice, balance.unwrap());
    assert_eq!(balance.unwrap(), first_mint_amount);

    // alice transfer 500 tokens to bob
    let bob = "bob";
    token.create_account(bob).unwrap();
    let transfer_amount = 500;
    let _transfer_solution_address = token.transfer(alice, bob, transfer_amount).await.unwrap();
    let mut alice_balance = balance;
    while alice_balance == balance {
        println!("{} balance {}", alice, balance.unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        alice_balance = token.balance(alice).await.unwrap();
    }
    println!("{} balance {}", alice, alice_balance.unwrap());
    let mut bob_balance = None;
    while bob_balance.is_none() {
        println!("{} balance {}", bob, 0);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        bob_balance = token.balance(bob).await.unwrap();
    }
    println!("{} balance {}", bob, bob_balance.unwrap());
    assert_eq!(alice_balance.unwrap(), first_mint_amount - transfer_amount);
    assert_eq!(bob_balance.unwrap(), transfer_amount);

    // alice burn 100 tokens
    let burn_amount = 100;
    let _burn_solution_address = token.burn(alice, burn_amount).await.unwrap();
    let mut alice_new_balance = alice_balance;
    while alice_new_balance == alice_balance {
        println!("{} balance {}", alice, alice_balance.unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        alice_new_balance = token.balance(alice).await.unwrap();
    }
    println!("{} balance {}", alice, alice_new_balance.unwrap());
    assert_eq!(
        alice_new_balance.unwrap(),
        first_mint_amount - transfer_amount - burn_amount
    );
}

fn to_hex(k: &essential_signer::PublicKey) -> String {
    let k = match k {
        essential_signer::PublicKey::Secp256k1(k) => k,
        essential_signer::PublicKey::Ed25519(_) => unreachable!(),
    };
    let encoded = essential_sign::encode::public_key(k);
    hex::encode_upper(essential_hash::hash_words(&encoded))
}

pub fn find_address(predicate: &str, num: usize) -> Option<&str> {
    predicate
        .split("0x")
        .nth(num)
        .and_then(|s| s.split(&[' ', ')', ',', ']', ';']).next())
        .map(|s| s.trim())
}
