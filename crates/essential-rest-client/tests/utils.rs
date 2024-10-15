const LOCALHOST: &str = "127.0.0.1";

async fn test_listener() -> tokio::net::TcpListener {
    tokio::net::TcpListener::bind(format!("{LOCALHOST}:0"))
        .await
        .unwrap()
}

pub async fn setup_node_as_server() -> anyhow::Result<String> {
    let conf = essential_node::db::Config {
        source: essential_node::db::Source::Memory(uuid::Uuid::new_v4().into()),
        ..Default::default()
    };
    let db = essential_node::db(&conf).unwrap();
    let state = essential_node_api::State {
        conn_pool: db,
        new_block: None,
    };
    let router = essential_node_api::router(state);
    let listener = test_listener().await;
    let port = listener.local_addr().unwrap().port();
    let _jh = tokio::spawn(async move {
        essential_node_api::serve(
            &router,
            &listener,
            essential_node_api::DEFAULT_CONNECTION_LIMIT,
        )
        .await
    });
    let address = format!("http://{LOCALHOST}:{port}/");
    Ok(address)
}
