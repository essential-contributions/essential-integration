use essential_builder as builder;
use essential_builder_db as builder_db;
use essential_node as node;
use essential_signer::Signature;
use essential_types::{convert::word_4_from_u8_32, solution::Solution, Word};
use essential_wallet::Wallet;

const PRIV_KEY: &str = "128A3D2146A69581FD8FC4C0A9B7A96A5755D85255D4E47F814AFA69D7726C8D";
const TOKEN_NAME: &str = "alice coin";
const TOKEN_SYMBOL: &str = "ALC";

#[tokio::test]
async fn mint_and_transfer() {
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
    let hashed_key = hash_key(&mut wallet, alice);

    let node_conn = node::db(&Default::default()).unwrap();
    let nonce: Vec<_> = token::token::storage::keys::keys()
        .nonce(|e| e.entry(hashed_key))
        .into();
    let nonce = node_conn
        .query_state(token::token::ADDRESS, nonce[0].clone())
        .await
        .unwrap();

    let account = token::mint::Account {
        hashed_key,
        amount: first_mint_amount,
        decimals: 18,
        nonce: token::mint::Query(nonce),
    };
    let to_sign = token::mint::data_to_sign(account).unwrap();
    let sig = wallet.sign_words(&to_sign.to_words(), alice).unwrap();
    let Signature::Secp256k1(sig) = sig else {
        panic!("Invalid signature")
    };

    let balance: Vec<_> = token::token::storage::keys::keys()
        .balances(|e| e.entry(hashed_key))
        .into();
    let balance = node_conn
        .query_state(token::token::ADDRESS, balance[0].clone())
        .await
        .unwrap();
    let build_solution = token::mint::BuildSolution {
        new_nonce: to_sign.new_nonce,
        current_balance: token::mint::Query(balance),
        hashed_key,
        amount: first_mint_amount,
        decimals: 18,
        signature: sig,
        token_name: TOKEN_NAME.to_string(),
        token_symbol: TOKEN_SYMBOL.to_string(),
    };
    let solution = token::mint::build_solution(build_solution).unwrap();

    let builder_conn = builder_db::ConnectionPool::with_tables(&Default::default()).unwrap();

    submit_solution(&builder_conn, solution).await;

    builder::build_block_fifo(&builder_conn, &node_conn, &Default::default())
        .await
        .unwrap();

    let account = token::transfer::Account {
        hashed_from_key: hashed_key,
        hashed_to_key: todo!(),
        amount: todo!(),
        nonce: todo!(),
    };
}

fn hash_key(wallet: &mut Wallet, account_name: &str) -> [Word; 4] {
    let public_key = wallet.get_public_key(account_name).unwrap();
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        panic!("Invalid public key")
    };
    let encoded = essential_sign::encode::public_key(&public_key);
    word_4_from_u8_32(essential_hash::hash_words(&encoded))
}

async fn submit_solution(builder_conn: &builder_db::ConnectionPool, solution: Solution) {
    builder_conn
        .insert_solution_submission(
            std::sync::Arc::new(solution),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap(),
        )
        .await
        .unwrap();
}
