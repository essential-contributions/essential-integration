use essential_node_types::register_contract_solution;
use essential_types::{contract::Contract, solution::Solution, ContentAddress};

pub async fn deploy_contract(
    builder_conn: &essential_builder_db::ConnectionPool,
    contract: &Contract,
) -> anyhow::Result<ContentAddress> {
    let solution = deploy_contract_solution(contract)?;
    let r = builder_conn
        .insert_solution_submission(
            std::sync::Arc::new(solution),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap(),
        )
        .await?;
    Ok(r)
}

pub fn deploy_contract_solution(contract: &Contract) -> anyhow::Result<Solution> {
    let registry_predicate = essential_node_types::BigBang::default().contract_registry;
    let solution = register_contract_solution(registry_predicate, contract)?;
    Ok(Solution {
        data: vec![solution],
    })
}
