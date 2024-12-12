use essential_node_types::{register_contract_solution, register_program_solution};
use essential_types::{
    contract::Contract,
    solution::{Solution, SolutionSet},
    ContentAddress, PredicateAddress, Program,
};

pub async fn register_contract_and_programs(
    builder_conn: &essential_builder_db::ConnectionPool,
    contract_registry: &PredicateAddress,
    program_registry: &PredicateAddress,
    contract: &Contract,
    programs: Vec<Program>,
) -> anyhow::Result<ContentAddress> {
    let solutions = register_contract_solution(contract_registry.clone(), contract)
        .into_iter()
        .chain(
            programs
                .iter()
                .map(|p| register_program_solution(program_registry.clone(), p)),
        )
        .collect();
    let solution_set = std::sync::Arc::new(SolutionSet { solutions });
    let r = builder_conn
        .insert_solution_set_submission(
            solution_set,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap(),
        )
        .await?;
    Ok(r)
}

pub fn registered_contract_solution(
    contract: &Contract,
    contract_registry: &PredicateAddress,
) -> Solution {
    register_contract_solution(contract_registry.clone(), contract).unwrap()
}

pub fn registered_program_solution(
    program: &Program,
    predicate_registry: &PredicateAddress,
) -> Solution {
    register_program_solution(predicate_registry.clone(), program)
}
