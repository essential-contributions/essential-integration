use app_utils::local_server::setup_server;
use nft_front_end::{deploy_app, print_addresses, Nft};
use std::path::PathBuf;

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
    let mut wallet = essential_wallet::Wallet::temp().unwrap();

    wallet
        .new_key_pair("deployer", essential_wallet::Scheme::Secp256k1)
        .ok();

    let pint_directory = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint"));

    let addresses = deploy_app(
        server_address.clone(),
        &mut wallet,
        "deployer",
        pint_directory,
    )
    .await
    .unwrap();
    print_addresses(&addresses);

    let account_name = "alice";

    let token = 0;

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

    let art_token = 1;

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
