use essential_app_utils::{self as utils, compile::compile_pint_project};
use essential_node_types::BigBang;
use essential_signer::Signature;
use essential_types::{convert::word_4_from_u8_32, solution::SolutionSet, Word};
use essential_wallet::Wallet;
use token::Query;

// Constants for the test

/// The private key for the test account.
const PRIV_KEY: &str = "128A3D2146A69581FD8FC4C0A9B7A96A5755D85255D4E47F814AFA69D7726C8D";
/// The name of the token.
const TOKEN_NAME: &str = "alice coin";
/// The symbol of the token.
const TOKEN_SYMBOL: &str = "ALC";

#[tokio::test]
async fn mint_and_transfer() {
    // Initialize tracing for better debugging
    tracing_subscriber::fmt::init();

    // Compile the token contract
    // This requires `pint` be available on PATH
    let (transfer, programs) =
        compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint/token").into())
            .await
            .unwrap();

    // Create a temporary wallet for testing
    let mut wallet = essential_wallet::Wallet::temp().unwrap();

    // Set up Alice's account
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

    // Set the initial mint amount and get Alice's hashed key
    let first_mint_amount = 1000000;
    let alice_hashed_key = hash_key(&mut wallet, alice);

    // Create new databases for testing
    let dbs = utils::db::new_dbs().await;
    let big_bang = BigBang::default();

    // Deploy the token contract
    let contract_registry = big_bang.contract_registry;
    let program_registry = big_bang.program_registry;
    essential_app_utils::deploy::register_contract_and_programs(
        &dbs.builder,
        &contract_registry,
        &program_registry,
        &transfer,
        programs,
    )
    .await
    .unwrap();

    // Get Alice's nonce key
    let alice_nonce_key = token::nonce_key(alice_hashed_key);
    let nonce = utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_nonce_key)
        .await
        .unwrap();

    // Prepare the mint
    let init = token::mint::Init {
        hashed_key: alice_hashed_key,
        amount: first_mint_amount,
        decimals: 18,
        nonce: Query(nonce),
    };
    let to_sign = token::mint::data_to_sign(init).unwrap();
    let sig = wallet.sign_words(&to_sign.to_words(), alice).unwrap();
    let Signature::Secp256k1(sig) = sig else {
        panic!("Invalid signature")
    };

    // Get Alice's balance key
    let alice_balance_key = token::balance_key(alice_hashed_key);
    let balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_balance_key)
            .await
            .unwrap();

    // Build the mint solution
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

    let solution_set = SolutionSet {
        solutions: vec![solution],
    };

    // Submit the mint solution
    utils::builder::submit(&dbs.builder, solution_set.clone())
        .await
        .unwrap();

    // Validate the mint solution
    utils::node::validate_solution(&dbs.node, solution_set.clone())
        .await
        .unwrap();

    // Build a block
    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert!(o.failed.is_empty(), "{:?}", o.failed);

    // Verify Alice's balance after minting
    let balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_balance_key)
            .await
            .unwrap();
    assert_eq!(token::balance(Query(balance)).unwrap(), first_mint_amount);

    // Set up Bob's account
    let bob = "bob";
    wallet
        .new_key_pair(bob, essential_wallet::Scheme::Secp256k1)
        .unwrap();

    // Get Bob's hashed key
    let bob_hashed_key = hash_key(&mut wallet, bob);

    // Prepare the transfer solution
    let nonce = utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_nonce_key)
        .await
        .unwrap();
    let init = token::transfer::Init {
        hashed_from_key: alice_hashed_key,
        hashed_to_key: bob_hashed_key,
        amount: 500,
        nonce: Query(nonce),
    };

    // Sign the transfer solution
    let to_sign = token::transfer::data_to_sign(init).unwrap();
    let sig = wallet.sign_words(&to_sign.to_words(), alice).unwrap();
    let Signature::Secp256k1(sig) = sig else {
        panic!("Invalid signature")
    };

    // Get current balances for Alice and Bob
    let from_balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_balance_key)
            .await
            .unwrap();

    let bob_balance_key = token::balance_key(bob_hashed_key);
    let to_balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &bob_balance_key)
            .await
            .unwrap();

    // Build the transfer solution
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
    let solution_set = SolutionSet {
        solutions: vec![solution],
    };

    // Submit the transfer solution
    utils::builder::submit(&dbs.builder, solution_set.clone())
        .await
        .unwrap();

    // Validate the transfer solution
    utils::node::validate_solution(&dbs.node, solution_set.clone())
        .await
        .unwrap();
    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert!(o.failed.is_empty(), "{:?}", o.failed);

    // Verify Alice's balance after transfer
    let balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &alice_balance_key)
            .await
            .unwrap();
    assert_eq!(
        token::balance(Query(balance)).unwrap(),
        first_mint_amount - 500
    );

    // Verify Bob's balance after transfer
    let balance =
        utils::node::query_state_head(&dbs.node, &token::token::ADDRESS, &bob_balance_key)
            .await
            .unwrap();

    assert_eq!(token::balance(Query(balance)).unwrap(), 500);
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
