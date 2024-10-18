use essential_builder_db as builder_db;
use essential_node as node;

pub struct Dbs {
    pub builder: builder_db::ConnectionPool,
    pub node: node::db::ConnectionPool,
}

pub async fn new_dbs() -> Dbs {
    let config = node::db::Config {
        source: node::db::Source::Memory(uuid::Uuid::new_v4().to_string()),
        ..Default::default()
    };
    // let config = node::db::Config {
    //     source: node::db::Source::Path(concat!(env!("CARGO_MANIFEST_DIR"), "/db.db").into()),
    //     ..Default::default()
    // };
    let node = node::db(&config).unwrap();
    init_node_db(&node).await.unwrap();
    let config = builder_db::pool::Config {
        source: builder_db::pool::Source::Memory(uuid::Uuid::new_v4().to_string()),
        ..Default::default()
    };
    let builder = builder_db::ConnectionPool::with_tables(&config).unwrap();
    Dbs { builder, node }
}

pub async fn init_node_db(db: &node::db::ConnectionPool) -> anyhow::Result<()> {
    let big_bang = essential_node_types::BigBang::default();

    essential_node::ensure_big_bang_block(db, &big_bang).await?;
    Ok(())
}
