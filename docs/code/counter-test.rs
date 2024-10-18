
// ANCHOR: full
// ANCHOR: use
use counter_app::*;
use essential_app_utils as utils;
use essential_app_utils::{
    compile::compile_pint_project,
    db::{new_dbs, Dbs},
};
use essential_node as node;
use essential_types::{ContentAddress, PredicateAddress, Word};
// ANCHOR_END: use

// ANCHOR: test-start
#[tokio::test]
async fn test() {
// ANCHOR_END: test-start
    // ANCHOR: addr
    let counter = compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint").into())
        .await
        .unwrap();

    let contract_address = essential_hash::contract_addr::from_contract(&counter);
    let predicate_address = essential_hash::content_addr(&counter.predicates[0]);
    let predicate_address = PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };
    // ANCHOR_END: addr

    // ANCHOR: dep
    let dbs = new_dbs().await;

    // Deploy the contract
    essential_app_utils::deploy::deploy_contract(&dbs.builder, &counter)
        .await
        .unwrap();
    // ANCHOR_END: dep

    // ANCHOR: read
    let key = counter_key();
    let count = read_count(&dbs.node, &predicate_address.contract, &key).await;
    assert_eq!(count, 0);
    // ANCHOR_END: read

    // ANCHOR: incr
    let new_count = increment(&dbs, predicate_address.clone()).await;
    // ANCHOR_END: incr

    // ANCHOR: build
    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert_eq!(o.succeeded.len(), 3);
    assert!(o.failed.is_empty());

    let count = read_count(&dbs.node, &predicate_address.contract, &key).await;
    assert_eq!(count, new_count);
    // ANCHOR_END: build

    // ANCHOR: comp
    let _ = increment(&dbs, predicate_address.clone()).await;
    let expected_new_count = increment(&dbs, predicate_address.clone()).await;

    let count = read_count(&dbs.node, &predicate_address.contract, &key).await;
    assert_eq!(count, new_count);

    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert_eq!(o.succeeded.len(), 1);

    let count = read_count(&dbs.node, &predicate_address.contract, &key).await;
    assert_eq!(count, expected_new_count);
    // ANCHOR_END: comp
// ANCHOR: test-end
}
// ANCHOR_END: test-end

// ANCHOR: read-count
async fn read_count(
    conn: &node::db::ConnectionPool,
    address: &ContentAddress,
    key: &CounterKey,
) -> Word {
    let r = utils::node::query_state_head(conn, address, &key.0)
        .await
        .unwrap();
    extract_count(QueryCount(r)).unwrap()
}
// ANCHOR_END: read-count

// ANCHOR: inc
async fn increment(dbs: &Dbs, predicate_address: PredicateAddress) -> Word {
    let key = counter_key();
    let current_count = dbs
        .node
        .query_state(predicate_address.contract.clone(), key.0)
        .await
        .unwrap();
    let (solution, new_count) =
        incremented_solution(predicate_address, QueryCount(current_count)).unwrap();

    utils::builder::submit(&dbs.builder, solution)
        .await
        .unwrap();
    new_count
}
// ANCHOR_END: inc
// ANCHOR_END: full