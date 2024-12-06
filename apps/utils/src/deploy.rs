use essential_node_types::register_contract_solution;
use essential_types::{contract::Contract, solution::SolutionSet, ContentAddress};

pub async fn deploy_contract(
    builder_conn: &essential_builder_db::ConnectionPool,
    contract: &Contract,
) -> anyhow::Result<ContentAddress> {
    let solution_set = deploy_contract_solution(contract)?;
    let r = builder_conn
        .insert_solution_set_submission(
            std::sync::Arc::new(solution_set),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap(),
        )
        .await?;
    Ok(r)
}

pub fn deploy_contract_solution(contract: &Contract) -> anyhow::Result<SolutionSet> {
    let registry_predicate = essential_node_types::BigBang::default().contract_registry;
    let solution = register_contract_solution(registry_predicate.clone(), contract)?;
    Ok(SolutionSet {
        solutions: vec![solution],
    })
}
