use essential_types::{ContentAddress, Key, Value};

pub async fn query_state_head(
    conn: &essential_node::db::ConnectionPool,
    address: &ContentAddress,
    key: &Key,
) -> anyhow::Result<Option<Value>> {
    let mut c = conn.acquire().await?;
    let tx = c.transaction()?;
    let ca = essential_node_db::get_latest_finalized_block_address(&tx)?;
    let num = match ca {
        Some(ca) => essential_node_db::get_block_number(&tx, &ca)?.unwrap_or_default(),
        None => 0,
    };
    let r = essential_node_db::finalized::query_state_inclusive_block(&tx, address, key, num)?;
    Ok(r)
}

pub async fn validate_solution(
    conn: &essential_node::db::ConnectionPool,
    solution: essential_types::solution::Solution,
) -> anyhow::Result<()> {
    let registry_predicate = essential_node_types::BigBang::default().contract_registry;
    essential_node::validate_solution(conn, &registry_predicate.contract, solution).await?;
    Ok(())
}
