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
        Some(ca) => essential_node_db::get_block_header(&tx, &ca)?
            .map(|h| h.number)
            .unwrap_or_default(),
        None => 0,
    };
    let r = essential_node_db::finalized::query_state_inclusive_block(&tx, address, key, num)?;
    Ok(r)
}

pub async fn validate_solution(
    conn: &essential_node::db::ConnectionPool,
    solution_set: essential_types::solution::SolutionSet,
) -> anyhow::Result<()> {
    let contract_registry_predicate = essential_node_types::BigBang::default().contract_registry;
    let program_registry_predicate = essential_node_types::BigBang::default().program_registry;
    essential_node::validate_solution_set_dry_run(
        conn,
        &contract_registry_predicate.contract,
        &program_registry_predicate.contract,
        solution_set,
    )
    .await?;
    Ok(())
}
