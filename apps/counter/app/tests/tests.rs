use counter_app::*;
use essential_app_utils::compile::compile_pint_project;
use essential_builder as builder;
use essential_builder_db as builder_db;
use essential_node as node;
use essential_types::{ContentAddress, PredicateAddress, Word};

#[tokio::test]
async fn number_go_up() {
    let counter = compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint").into())
        .await
        .unwrap();

    let contract_address = essential_hash::contract_addr::from_contract(&counter);
    let predicate_address = essential_hash::content_addr(&counter.predicates[0]);
    let predicate_address = PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };

    let node_conn = node::db(&Default::default()).unwrap();

    // TODO: Deploy the contract

    let key = counter_key();
    let count = read_count(&node_conn, predicate_address.contract.clone(), key.clone()).await;
    assert_eq!(count, 0);

    // TODO: Demonstrate validating solution on node.

    // TODO: Demonstrate validating block on node.

    let builder_conn = builder_db::ConnectionPool::with_tables(&Default::default()).unwrap();

    let new_count = increment(&node_conn, &builder_conn, predicate_address.clone()).await;

    builder::build_block_fifo(&builder_conn, &node_conn, &Default::default())
        .await
        .unwrap();

    let count = read_count(&node_conn, predicate_address.contract.clone(), key.clone()).await;
    assert_eq!(count, new_count);

    let _ = increment(&node_conn, &builder_conn, predicate_address.clone()).await;
    let expected_new_count = increment(&node_conn, &builder_conn, predicate_address.clone()).await;

    let count = read_count(&node_conn, predicate_address.contract.clone(), key.clone()).await;
    assert_eq!(count, new_count);

    builder::build_block_fifo(&builder_conn, &node_conn, &Default::default())
        .await
        .unwrap();

    let count = read_count(&node_conn, predicate_address.contract.clone(), key.clone()).await;
    assert_eq!(count, expected_new_count);

    // Demonstrate syncing node with deployed node and reading count.
}

async fn read_count(
    conn: &node::db::ConnectionPool,
    address: ContentAddress,
    key: CounterKey,
) -> Word {
    let r = conn.query_state(address, key.0).await.unwrap();
    extract_count(QueryCount(r)).unwrap()
}

async fn increment(
    node_conn: &node::db::ConnectionPool,
    builder_conn: &builder_db::ConnectionPool,
    predicate_address: PredicateAddress,
) -> Word {
    let key = counter_key();
    let current_count = node_conn
        .query_state(predicate_address.contract.clone(), key.0)
        .await
        .unwrap();
    let (solution, new_count) =
        incremented_solution(predicate_address, QueryCount(current_count)).unwrap();

    builder_conn
        .insert_solution_submission(
            std::sync::Arc::new(solution),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap(),
        )
        .await
        .unwrap();
    new_count
}
