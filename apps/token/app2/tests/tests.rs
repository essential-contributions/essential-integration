use essential_app_utils::{self as utils, compile::compile_pint_project};
use essential_signer::Signature;
use essential_types::{convert::word_4_from_u8_32, Word};
use essential_wallet::Wallet;
use token::Query;

const PRIV_KEY: &str = "128A3D2146A69581FD8FC4C0A9B7A96A5755D85255D4E47F814AFA69D7726C8D";
const TOKEN_NAME: &str = "alice coin";
const TOKEN_SYMBOL: &str = "ALC";

#[tokio::test]
async fn mint_and_transfer() {
    tracing_subscriber::fmt::init();
    let transfer =
        compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint/token").into())
            .await
            .unwrap();
    let mut wallet = essential_wallet::Wallet::temp().unwrap();
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

    let first_mint_amount = 1000000;
    let alice_hashed_key = hash_key(&mut wallet, alice);

    let dbs = utils::db::new_dbs().await;

    // Deploy the contract
    essential_app_utils::deploy::deploy_contract(&dbs.builder, &transfer)
        .await
        .unwrap();

    let nonce: Vec<_> = token::token::storage::keys::keys()
        .nonce(|e| e.entry(alice_hashed_key))
        .into();
    let alice_nonce_key = nonce[0].clone();
    let nonce = utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_nonce_key)
        .await
        .unwrap();

    let account = token::mint::Init {
        hashed_key: alice_hashed_key,
        amount: first_mint_amount,
        decimals: 18,
        nonce: Query(nonce),
    };
    let to_sign = token::mint::data_to_sign(account).unwrap();
    let sig = wallet.sign_words(&to_sign.to_words(), alice).unwrap();
    let Signature::Secp256k1(sig) = sig else {
        panic!("Invalid signature")
    };

    let balance: Vec<_> = token::token::storage::keys::keys()
        .balances(|e| e.entry(alice_hashed_key))
        .into();
    let alice_balance_key = balance[0].clone();
    let balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_balance_key)
            .await
            .unwrap();
    let build_solution = token::mint::BuildSolution {
        new_nonce: to_sign.new_nonce,
        current_balance: Query(balance),
        hashed_key: alice_hashed_key,
        amount: first_mint_amount,
        decimals: 18,
        signature: sig,
        token_name: TOKEN_NAME.to_string(),
        token_symbol: TOKEN_SYMBOL.to_string(),
    };
    let solution = token::mint::build_solution(build_solution).unwrap();

    utils::builder::submit(&dbs.builder, solution.clone())
        .await
        .unwrap();

    eprintln!("solution: {:?}", solution);
    utils::node::validate_solution(&dbs.node, solution.clone())
        .await
        .unwrap();
    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert!(o.failed.is_empty(), "{:?}", o.failed);

    let balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_balance_key)
            .await
            .unwrap();
    assert_eq!(token::balance(Query(balance)).unwrap(), first_mint_amount);

    let bob = "bob";
    wallet
        .new_key_pair(bob, essential_wallet::Scheme::Secp256k1)
        .unwrap();

    let bob_hashed_key = hash_key(&mut wallet, bob);
    let nonce = utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_nonce_key)
        .await
        .unwrap();
    let init = token::transfer::Init {
        hashed_from_key: alice_hashed_key,
        hashed_to_key: bob_hashed_key,
        amount: 500,
        nonce: Query(nonce),
    };

    let to_sign = token::transfer::data_to_sign(init).unwrap();
    let sig = wallet.sign_words(&to_sign.to_words(), alice).unwrap();
    let Signature::Secp256k1(sig) = sig else {
        panic!("Invalid signature")
    };

    let from_balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_balance_key)
            .await
            .unwrap();

    let balance: Vec<_> = token::token::storage::keys::keys()
        .balances(|e| e.entry(bob_hashed_key))
        .into();
    let bob_balance_key = balance[0].clone();
    let to_balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &bob_balance_key)
            .await
            .unwrap();
    let solution = token::transfer::BuildSolution {
        hashed_from_key: alice_hashed_key,
        hashed_to_key: bob_hashed_key,
        new_nonce: to_sign.new_nonce,
        amount: 500,
        current_from_balance: Query(from_balance),
        current_to_balance: Query(to_balance),
        signature: sig,
    };
    let solution = token::transfer::build_solution(solution).unwrap();

    utils::builder::submit(&dbs.builder, solution.clone())
        .await
        .unwrap();

    eprintln!("solution: {:?}", solution);
    utils::node::validate_solution(&dbs.node, solution.clone())
        .await
        .unwrap();
    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert!(o.failed.is_empty(), "{:?}", o.failed);

    let balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_balance_key)
            .await
            .unwrap();
    assert_eq!(
        token::balance(Query(balance)).unwrap(),
        first_mint_amount - 500
    );

    let balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &bob_balance_key)
            .await
            .unwrap();

    assert_eq!(token::balance(Query(balance)).unwrap(), 500);
}

fn hash_key(wallet: &mut Wallet, account_name: &str) -> [Word; 4] {
    let public_key = wallet.get_public_key(account_name).unwrap();
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        panic!("Invalid public key")
    };
    let encoded = essential_sign::encode::public_key(&public_key);
    word_4_from_u8_32(essential_hash::hash_words(&encoded))
}
