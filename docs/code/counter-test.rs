
// ANCHOR: full
// ANCHOR: use
use app_utils::{compile::compile_pint_project, local_server::setup_server};
use counter_app::App;
use essential_types::{PredicateAddress, Word};
// ANCHOR_END: use

// ANCHOR: test-start
#[tokio::test]
async fn test_counter() {
// ANCHOR_END: test-start
    // ANCHOR: p1
    // ANCHOR: setup
    let (addr, _server) = setup_server().await.unwrap();
    // ANCHOR_END: setup

    // ANCHOR: compile
    let counter = compile_pint_project(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../contract").into(),
        "counter",
    )
    .await
    .unwrap();
    // ANCHOR_END: compile

    // ANCHOR: address
    let contract_address = essential_hash::contract_addr::from_contract(&counter);
    let predicate_address = essential_hash::content_addr(&counter.predicates[0]);
    let predicate_address = PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };
    // ANCHOR_END: address

    // ANCHOR: key
    let mut wallet = essential_wallet::Wallet::temp().unwrap();
    wallet
        .new_key_pair("alice", essential_wallet::Scheme::Secp256k1)
        .unwrap();
    // ANCHOR_END: key

    // ANCHOR: deploy
    essential_deploy_contract::sign_and_deploy(addr.clone(), "alice", &mut wallet, counter)
        .await
        .unwrap();
    // ANCHOR_END: deploy
    // ANCHOR_END: p1

    // ANCHOR: app
    let app = App::new(addr, predicate_address).unwrap();
    // ANCHOR_END: app

    // ANCHOR: read
    assert_eq!(app.read_count().await.unwrap(), 0);
    // ANCHOR_END: read

    // ANCHOR: inc
    app.increment().await.unwrap();
    // ANCHOR_END: inc

    // ANCHOR: wait
    wait_for_change(&app, 1).await;
    // ANCHOR_END: wait
    
    // ANCHOR: inc-again
    app.increment().await.unwrap();
    // ANCHOR_END: inc-again

    // ANCHOR: wait-again
    wait_for_change(&app, 2).await;
    // ANCHOR_END: wait-again
// ANCHOR: test-end
}
// ANCHOR_END: test-end

// ANCHOR: wait-fn
async fn wait_for_change(app: &App, expected: Word) {
    loop {
        if app.read_count().await.unwrap() == expected {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
// ANCHOR_END: wait-fn
// ANCHOR_END: full