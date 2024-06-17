use app_utils::local_server::setup_server;
use std::path::PathBuf;
use token::{actions::deploy_app, token::Token};

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

    let pint_directory = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint"));

    let intent_addresses = deploy_app(
        server_address.clone(),
        &mut wallet,
        &deployer_name,
        pint_directory,
    )
    .await
    .unwrap();

    let mut token = Token::new(server_address, intent_addresses, wallet).unwrap();

    // initialize token
    let _init_solution_address = token.init("art").await.unwrap();
    let mut name = token.name().await;
    while name.is_err() {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        name = token.name().await;
    }
    println!("token initialized");

    // alice mint 800 tokens
    let alice = "alice";
    token.create_account(&alice).unwrap();
    let first_mint_amount = 800;
    let _mint_solution_address = token.mint(&alice, first_mint_amount).await.unwrap();
    let mut balance = None;
    while balance == None {
        println!("{} balance {}", alice, 0);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        balance = token.balance(&alice).await.unwrap();
    }
    println!("{} balance {}", alice, balance.unwrap());
    assert_eq!(balance.unwrap(), first_mint_amount);

    // alice mint 200 tokens
    let second_mint_amount = 200;
    let mint_amount = first_mint_amount + second_mint_amount;
    let _mint_solution_address = token.mint(&alice, second_mint_amount).await.unwrap();
    let mut new_balance = balance;
    while new_balance == balance {
        println!("{} balance {}", alice, balance.unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        new_balance = token.balance(&alice).await.unwrap();
    }
    println!("{} balance {}", alice, new_balance.unwrap());
    assert_eq!(new_balance.unwrap(), mint_amount);

    // alice transfer 500 tokens to bob
    let bob = "bob";
    token.create_account(&bob).unwrap();
    let transfer_amount = 500;
    let _transfer_solution_address = token.transfer(&alice, &bob, transfer_amount).await.unwrap();
    let mut alice_balance = balance;
    while alice_balance == balance {
        println!("{} balance {}", alice, new_balance.unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        alice_balance = token.balance(&alice).await.unwrap();
    }
    println!("{} balance {}", alice, alice_balance.unwrap());
    let mut bob_balance = None;
    while bob_balance == None {
        println!("{} balance {}", bob, 0);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        bob_balance = token.balance(&bob).await.unwrap();
    }
    println!("{} balance {}", bob, bob_balance.unwrap());
    assert_eq!(alice_balance.unwrap(), mint_amount - transfer_amount);
    assert_eq!(bob_balance.unwrap(), transfer_amount);

    // alice burn 100 tokens
    let burn_amount = 100;
    let _burn_solution_address = token.burn(&alice, burn_amount).await.unwrap();
    let mut alice_new_balance = alice_balance;
    while alice_new_balance == alice_balance {
        println!("{} balance {}", alice, alice_balance.unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        alice_new_balance = token.balance(&alice).await.unwrap();
    }
    println!("{} balance {}", alice, alice_new_balance.unwrap());
    assert_eq!(
        alice_new_balance.unwrap(),
        mint_amount - transfer_amount - burn_amount
    );
}
