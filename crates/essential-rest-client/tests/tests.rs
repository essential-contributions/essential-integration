use essential_rest_client::node_client::EssentialNodeClient;
use essential_types::{ContentAddress, Key};
use utils::setup_node_as_server;

mod utils;

#[tokio::test]
async fn test_api() {
    let addr = setup_node_as_server().await.unwrap();
    let client = EssentialNodeClient::new(addr).unwrap();

    let contract_ca = ContentAddress([42u8; 32]);
    let key: Key = vec![0];

    let r = client.query_state(contract_ca, key).await.unwrap();

    assert_eq!(r, None);
}
