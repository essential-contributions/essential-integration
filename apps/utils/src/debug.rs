use std::path::Path;

use essential_debugger::Source;
use essential_server_types::{QueryStateReads, StateReadRequestType};
use essential_types::solution::Solution;

use crate::compile::{get_contracts, NamedContract};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct Target {
    pub contract: String,
    pub predicate: String,
    pub data_index: usize,
    pub constraint: usize,
}

pub async fn debug(
    pint_directory: &Path,
    server_address: &str,
    solution: &Solution,
    target: Target,
) {
    let contracts = get_contracts(pint_directory.to_owned(), &[&target.contract])
        .await
        .unwrap();
    let predicate = contracts
        .get_contract(&target.contract)
        .unwrap()
        .get_predicate(&target.predicate)
        .unwrap()
        .clone();
    let query = QueryStateReads::from_solution(
        solution.clone(),
        target.data_index as u16,
        &predicate,
        StateReadRequestType::Reads,
    );

    let r = essential_rest_client::EssentialClient::new(server_address.to_string())
        .unwrap()
        .query_state_reads(query)
        .await
        .unwrap();
    let state = match r {
        essential_server_types::QueryStateReadsOutput::Reads(r) => r,
        _ => unreachable!(),
    };
    let state = state.into_iter().collect();
    let contract = contracts.get_contract(&target.contract).unwrap();
    let source = get_source(contract, &target.predicate, target.constraint);

    essential_debugger::run_with_source(
        solution.clone(),
        target.data_index as u16,
        predicate,
        target.constraint,
        state,
        source,
    )
    .await
    .unwrap();
}

impl Target {
    pub fn new(contract: &str, predicate: &str, data_index: usize, constraint: usize) -> Self {
        Self {
            contract: contract.to_string(),
            predicate: predicate.to_string(),
            data_index,
            constraint,
        }
    }

    pub async fn debug(self, pint_directory: &Path, server_address: &str, solution: &Solution) {
        debug(pint_directory, server_address, solution, self).await
    }
}

fn get_source(contract: &NamedContract, predicate_name: &str, constraint_num: usize) -> Source {
    let other: String = contract
        .source
        .lines()
        .take_while(|l| !l.starts_with("predicate "))
        .fold(String::new(), |acc, l| acc + l + "\n");
    let predicate_name = predicate_name
        .trim()
        .trim_start_matches("::")
        .to_lowercase();

    let mut count = 0;
    let predicate: String = contract
        .source
        .lines()
        .skip_while(|l| {
            if l.starts_with("predicate ") {
                let Some(name) = l.trim().split(' ').nth(1) else {
                    return true;
                };
                name.trim().trim_start_matches("::").to_lowercase() != predicate_name
            } else {
                true
            }
        })
        .take_while(|l| {
            if l.starts_with("predicate ") {
                count += 1;
            }
            count < 2
        })
        .fold(String::new(), |acc, l| acc + l + "\n");

    Source::default()
        .with_other_code(other)
        .with_predicate_find_line(predicate, constraint_num)
}
