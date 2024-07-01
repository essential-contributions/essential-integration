use std::{collections::HashMap, path::PathBuf};

use app_utils::{
    addresses::get_addresses, compile::compile_pint_project, local_server::setup_server,
};
use defi_app::{
    inputs::{SignedSwapArgs, SwapArgs, Trade, Transfer},
    Swap,
};
use essential_deploy_contract::sign_and_deploy;
use essential_rest_client::EssentialClient;
use essential_types::{
    contract::Contract, convert::word_4_from_u8_32, ContentAddress, PredicateAddress, Word,
};
use essential_wallet::{Scheme, Wallet};

const PRIV_KEY: &str = "128A3D2146A69581FD8FC4C0A9B7A96A5755D85255D4E47F814AFA69D7726C8D";

#[tokio::test]
async fn test_signed_swap() {
    let (server_address, _child) = setup_server().await.unwrap();
    let pint_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut contracts = Contracts::default();
    let i = compile_pint_project(pint_directory.join("../../token/pint"), "signed.pnt")
        .await
        .unwrap();
    add_contract(
        &mut contracts,
        "signed",
        &["burn", "cancel", "mint", "transfer", "transfer_with"],
        i,
    );
    let i = compile_pint_project(pint_directory.join("../../token/pint"), "token.pnt")
        .await
        .unwrap();
    add_contract(
        &mut contracts,
        "tokenA",
        &["burn", "cancel", "mint", "transfer"],
        i,
    );
    let i = compile_pint_project(pint_directory.join("../../token/pint"), "token_2.pnt")
        .await
        .unwrap();
    add_contract(
        &mut contracts,
        "tokenB",
        &["burn", "cancel", "mint", "transfer"],
        i,
    );
    let i = compile_pint_project(pint_directory.join("../pint"), "swap.pnt")
        .await
        .unwrap();
    add_contract(&mut contracts, "swap", &["signed_swap", "swap"], i);

    let mut wallet = essential_wallet::Wallet::temp().unwrap();

    let alice = "alice";
    setup_key(&mut wallet, alice);

    let bob = "bob";
    wallet.new_key_pair(bob, Scheme::Secp256k1).unwrap();

    for i in contracts.contract.iter() {
        sign_and_deploy(server_address.clone(), alice, &mut wallet, i.clone())
            .await
            .unwrap();
    }

    let client = EssentialClient::new(server_address).unwrap();

    let mint = token::inputs::SignedMint {
        auth_address: contracts.addrs["signed"]["mint"].clone(),
        mint_address: contracts.addrs["tokenA"]["mint"].clone(),
        account_name: alice.to_string(),
        new_nonce: 1,
        amount: 100_000_000,
        decimals: 3,
        name: [0; 4],
        symbol: [0; 4],
    };
    let mint = mint.build(&mut wallet).unwrap();
    // let outcome = client.check_solution(mint.clone()).await.unwrap();
    client.submit_solution(mint).await.unwrap();

    let mint = token::inputs::SignedMint {
        auth_address: contracts.addrs["signed"]["mint"].clone(),
        mint_address: contracts.addrs["tokenB"]["mint"].clone(),
        account_name: alice.to_string(),
        new_nonce: 1,
        amount: 100_000_000,
        decimals: 3,
        name: [1, 0, 0, 0],
        symbol: [0; 4],
    };
    let mint = mint.build(&mut wallet).unwrap();
    // let outcome = client.check_solution(mint.clone()).await.unwrap();
    client.submit_solution(mint).await.unwrap();

    wait_balance(
        &client,
        &mut wallet,
        &contracts.contracts["tokenA"],
        alice,
        100_000_000,
    )
    .await;

    wait_balance(
        &client,
        &mut wallet,
        &contracts.contracts["tokenB"],
        alice,
        100_000_000,
    )
    .await;

    let transfer = token::inputs::SignedTransfer {
        auth_address: contracts.addrs["signed"]["transfer"].clone(),
        token_address: contracts.addrs["tokenB"]["transfer"].clone(),
        from_account_name: alice.to_string(),
        to_account_name: bob.to_string(),
        new_nonce: 2,
        amount: 100_000,
        new_from_balance: 100_000_000 - 100_000,
        new_to_balance: 100_000,
    };
    let transfer = transfer.build(&mut wallet).unwrap();
    // let outcome = client.check_solution(transfer.clone()).await.unwrap();
    client.submit_solution(transfer).await.unwrap();

    wait_balance(
        &client,
        &mut wallet,
        &contracts.contracts["tokenB"],
        bob,
        100_000,
    )
    .await;

    let swap = SwapArgs {
        key: get_hashed_key(&mut wallet, alice).into(),
        account_b: get_hashed_key(&mut wallet, alice).into(),
        token_a: content_addr(&contracts.contracts["tokenA"]).into(),
        token_b: content_addr(&contracts.contracts["tokenB"]).into(),
        amount_a_max: 100_000.into(),
        amount_b_min: 10_000.into(),
    };
    let swap_args = SignedSwapArgs {
        account_name: alice.to_string(),
        swap: swap.clone(),
        nonce: 2.into(),
        token_addr: contracts.addrs["tokenA"]["transfer"].clone(),
        signed_swap_addr: contracts.addrs["swap"]["signed_swap"].clone(),
    };
    let signed_swap = swap_args.build(&mut wallet).unwrap();
    let alice_swap = Swap {
        swap,
        signed: signed_swap,
    };

    let swap = SwapArgs {
        key: get_hashed_key(&mut wallet, bob).into(),
        account_b: get_hashed_key(&mut wallet, bob).into(),
        token_a: content_addr(&contracts.contracts["tokenB"]).into(),
        token_b: content_addr(&contracts.contracts["tokenA"]).into(),
        amount_a_max: 10_000.into(),
        amount_b_min: 100_000.into(),
    };
    let swap_args = SignedSwapArgs {
        account_name: bob.to_string(),
        swap: swap.clone(),
        nonce: 1.into(),
        token_addr: contracts.addrs["tokenB"]["transfer"].clone(),
        signed_swap_addr: contracts.addrs["swap"]["signed_swap"].clone(),
    };
    let signed_swap = swap_args.build(&mut wallet).unwrap();
    let bob_swap = Swap {
        swap,
        signed: signed_swap,
    };

    let trade = Trade {
        swap_a: alice_swap,
        swap_b: bob_swap,
        swap_addr: contracts.addrs["swap"]["swap"].clone(),
        signed_swap_addr: contracts.addrs["swap"]["signed_swap"].clone(),
        auth_intent: contracts.addrs["signed"]["transfer_with"].clone(),
        transfer_a: Transfer {
            from: get_hashed_key(&mut wallet, alice).into(),
            to: get_hashed_key(&mut wallet, bob).into(),
            amount: 100_000.into(),
            new_from_balance: (100_000_000 - 100_000).into(),
            new_to_balance: 100_000.into(),
            nonce: 2.into(),
            token_addr: contracts.addrs["tokenA"]["transfer"].clone(),
        },
        transfer_b: Transfer {
            from: get_hashed_key(&mut wallet, bob).into(),
            to: get_hashed_key(&mut wallet, alice).into(),
            amount: 10_000.into(),
            new_from_balance: (100_000 - 10_000).into(),
            new_to_balance: (100_000_000 - 100_000 + 10_000).into(),
            nonce: 1.into(),
            token_addr: contracts.addrs["tokenB"]["transfer"].clone(),
        },
    };
    let trade = trade.build().unwrap();
    let outcome = client.check_solution(trade.clone()).await.unwrap();
}

#[tokio::test]
async fn test_swap() {}

fn setup_key(wallet: &mut Wallet, account_name: &str) {
    let key = hex::decode(PRIV_KEY).unwrap();
    wallet
        .insert_key(
            account_name,
            essential_signer::Key::Secp256k1(
                essential_signer::secp256k1::SecretKey::from_slice(&key).unwrap(),
            ),
        )
        .unwrap();
}

#[derive(Default)]
struct Contracts {
    contracts: HashMap<String, ContentAddress>,
    addrs: HashMap<String, HashMap<String, PredicateAddress>>,
    contract: Vec<Contract>,
}

fn add_contract(contracts: &mut Contracts, contract: &str, names: &[&str], i: Contract) {
    let (s, addrs) = get_addresses(&i);
    contracts.contracts.insert(contract.to_string(), s);
    contracts.addrs.insert(
        contract.to_string(),
        names.iter().map(|n| n.to_string()).zip(addrs).collect(),
    );
    contracts.contract.push(i);
}

async fn wait_balance(
    client: &EssentialClient,
    wallet: &mut Wallet,
    addr: &ContentAddress,
    account_name: &str,
    target: Word,
) {
    let mut bal = 0;
    while bal != target {
        bal = check_balance(client, wallet, addr, account_name).await;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("{} balance {}, target {}", account_name, bal, target);
    }
}

async fn check_balance(
    client: &EssentialClient,
    wallet: &mut Wallet,
    addr: &ContentAddress,
    account_name: &str,
) -> Word {
    let key = get_hashed_key(wallet, account_name);
    let mut k = vec![0];
    k.extend_from_slice(&key);
    let state = client.query_state(addr, &k).await.unwrap();
    state.first().copied().unwrap_or_default()
}

fn get_hashed_key(wallet: &mut Wallet, account_name: &str) -> [Word; 4] {
    let public_key = wallet.get_public_key(account_name).unwrap();
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        panic!("Invalid public key")
    };
    let encoded = essential_sign::encode::public_key(&public_key);
    word_4_from_u8_32(essential_hash::hash_words(&encoded))
}

fn content_addr(addr: &ContentAddress) -> [Word; 4] {
    word_4_from_u8_32(addr.0)
}
