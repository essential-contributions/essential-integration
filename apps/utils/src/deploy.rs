use essential_node_types::{register_contract_solution, register_program_solution};
use essential_types::{
    contract::Contract,
    solution::{Solution, SolutionSet},
    ContentAddress, Program,
};

pub async fn deploy_contract(
    builder_conn: &essential_builder_db::ConnectionPool,
    contract: &Contract,
    programs: &[Program],
) -> anyhow::Result<ContentAddress> {
    let mut solutions = vec![];
    solutions.push(registered_contract_solution(contract));
    solutions.extend(programs.iter().map(|p| registered_program_solution(p)));
    let solutions = SolutionSet { solutions };

    let r = builder_conn
        .insert_solution_set_submission(
            std::sync::Arc::new(solutions),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap(),
        )
        .await?;
    Ok(r)
}

pub fn registered_contract_solution(contract: &Contract) -> Solution {
    let registry_predicate = essential_node_types::BigBang::default().contract_registry;
    register_contract_solution(registry_predicate.clone(), contract).unwrap()
}

pub fn registered_program_solution(program: &Program) -> Solution {
    let registry_predicate = essential_node_types::BigBang::default().program_registry;
    register_program_solution(registry_predicate.clone(), program)
}
