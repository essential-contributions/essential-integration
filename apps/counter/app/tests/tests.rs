use counter_app::*;
use essential_app_utils as utils;
use essential_app_utils::{
    compile::compile_pint_project,
    db::{new_dbs, Dbs},
};
use essential_node as node;
use essential_node_types::BigBang;
use essential_types::{
    contract::Contract, ContentAddress, PredicateAddress, Program, SolutionSet, Word,
};

#[tokio::test]
async fn number_go_up() {
    tracing_subscriber::fmt::init();
    let contract_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../pint").into();
    let (counter, programs): (Contract, Vec<Program>) =
        compile_pint_project(contract_path).await.unwrap();

    let contract_address = essential_hash::contract_addr::from_contract(&counter);
    let predicate_address = essential_hash::content_addr(&counter.predicates[0]);
    let predicate_address = PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };

    let dbs = new_dbs().await;
    let big_bang = BigBang::default();

    // Deploy the contract
    let contract_registry = big_bang.contract_registry;
    let program_registry = big_bang.program_registry;
    essential_app_utils::deploy::register_contract_and_programs(
        &dbs.builder,
        &contract_registry,
        &program_registry,
        &counter,
        programs,
    )
    .await
    .unwrap();

    let key = counter_key();
    let count = read_count(&dbs.node, &predicate_address.contract, &key).await;
    assert_eq!(count, 0);

    // TODO: Demonstrate validating solution on node.

    // TODO: Demonstrate validating block on node.

    increment(&dbs, predicate_address.clone()).await;

    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert_eq!(o.succeeded.len(), 3);
    assert!(o.failed.is_empty());

    let count = read_count(&dbs.node, &predicate_address.contract, &key).await;
    assert_eq!(count, 1);

    let _ = increment(&dbs, predicate_address.clone()).await;
    increment(&dbs, predicate_address.clone()).await;

    let count = read_count(&dbs.node, &predicate_address.contract, &key).await;
    assert_eq!(count, 1);

    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert_eq!(o.succeeded.len(), 2);

    assert_eq!(o.failed.len(), 1);

    let count = read_count(&dbs.node, &predicate_address.contract, &key).await;
    assert_eq!(count, 2);
}

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

async fn increment(dbs: &Dbs, predicate_address: PredicateAddress) -> Word {
    let key = counter_key();
    let current_count =
        utils::node::query_state_head(&dbs.node, &predicate_address.contract, &key.0)
            .await
            .unwrap();
    let (solution, new_count) =
        incremented_solution(predicate_address, QueryCount(current_count)).unwrap();

    let solution_set = SolutionSet {
        solutions: vec![solution],
    };

    utils::builder::submit(&dbs.builder, solution_set)
        .await
        .unwrap();
    new_count
}
