// ANCHOR: full
// ANCHOR: use
use counter_app::*;
use essential_app_utils as utils;
// ANCHOR_END: use

// ANCHOR: test-start
#[tokio::test]
async fn test() {
// ANCHOR_END: test-start
    // ANCHOR: counter 
    let path: std::path::PathBuf = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../contract/out/debug/counter.json"
    )
    .into();

    let counter =
        serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(path).unwrap()))
            .unwrap();
    // ANCHOR_END: counter

    // ANCHOR: dep
    let dbs = utils::db::new_dbs().await;

    // Deploy the contract
    essential_app_utils::deploy::deploy_contract(&dbs.builder, &counter)
        .await
        .unwrap();
    // ANCHOR_END: dep

    // ANCHOR: read
    let count = read_count(&dbs).await;
    assert_eq!(count, 0);
    // ANCHOR_END: read

    // ANCHOR: incr
    increment(&dbs).await;
    // ANCHOR_END: incr

    // ANCHOR: build
    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert_eq!(o.succeeded.len(), 3);
    assert!(o.failed.is_empty());

    let count = read_count(&dbs).await;
    assert_eq!(count, 1);
    // ANCHOR_END: build

    // ANCHOR: comp
    increment(&dbs).await;
    increment(&dbs).await;

    let o = utils::builder::build_default(&dbs).await.unwrap();
    assert_eq!(o.succeeded.len(), 2);

    let count = read_count(&dbs).await;
    assert_eq!(count, 2);
    // ANCHOR_END: comp
// ANCHOR: test-end
}
// ANCHOR_END: test-end

// ANCHOR: read-count
async fn read_count(dbs: &utils::db::Dbs) -> essential_types::Word {
    let r = utils::node::query_state_head(&dbs.node, &ADDRESS, &counter_key().0)
        .await
        .unwrap();
    extract_count(r).unwrap()
}
// ANCHOR_END: read-count

// ANCHOR: inc
async fn increment(dbs: &utils::db::Dbs) {
    let current_count = extract_count(
        utils::node::query_state_head(&dbs.node, &ADDRESS, &counter_key().0)
            .await
            .unwrap(),
    )
    .unwrap();

    utils::builder::submit(&dbs.builder, create_solution(current_count + 1))
        .await
        .unwrap();
}
// ANCHOR_END: inc
// ANCHOR_END: full
