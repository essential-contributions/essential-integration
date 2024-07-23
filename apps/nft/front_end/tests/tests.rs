use essential_app_utils::local_server::setup_server;
use nft::{deploy_app, print_addresses, Nft};
use std::path::PathBuf;

#[tokio::test]
async fn mint_and_transfer() {
    let (server_address, _child) = setup_server().await.unwrap();
    let mut wallet = essential_wallet::Wallet::temp().unwrap();

    wallet
        .new_key_pair("deployer", essential_wallet::Scheme::Secp256k1)
        .ok();

    let pint_directory = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint"));

    deploy_app(
        server_address.clone(),
        &mut wallet,
        "deployer",
        &pint_directory,
    )
    .await
    .unwrap();
    print_addresses();

    let account_name = "alice";

    let token = 0;

    let mut nft = Nft::new(server_address.clone(), wallet).unwrap();

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

    nft.mint_for_contract(&nft::swap_any::Swap::ADDRESS, art_token)
        .await
        .unwrap();

    while !nft
        .do_i_own_contract(&nft::swap_any::Swap::ADDRESS, art_token)
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
        .do_i_own_contract(&nft::swap_any::Swap::ADDRESS, token)
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
