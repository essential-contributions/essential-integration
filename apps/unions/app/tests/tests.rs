use essential_app_utils::{compile::compile_pint_project, local_server::setup_server};
use essential_types::{PredicateAddress, Word};
use unions::App;

#[tokio::test]
async fn test_unions() {
    let (addr, _server) = setup_server().await.unwrap();
    let unions = compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/..").into())
        .await
        .unwrap();
    let contract_address = essential_hash::contract_addr::from_contract(&unions);
    let predicate_address = essential_hash::content_addr(&unions.predicates[0]);
    let predicate_address = PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };

    let mut wallet = essential_wallet::Wallet::temp().unwrap();
    wallet
        .new_key_pair("alice", essential_wallet::Scheme::Secp256k1)
        .unwrap();

    essential_deploy_contract::sign_and_deploy(addr.clone(), "alice", &mut wallet, unions)
        .await
        .unwrap();

    let app = App::new(addr, predicate_address).unwrap();
    assert_eq!(app.read_nonce().await.unwrap(), 0);
    app.increment().await.unwrap();
    wait_for_change(&app, 1).await;
    app.increment().await.unwrap();
    wait_for_change(&app, 2).await;
}

async fn wait_for_change(app: &App, expected: Word) {
    loop {
        if app.read_nonce().await.unwrap() == expected {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
