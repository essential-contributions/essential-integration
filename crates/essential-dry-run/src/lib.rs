//! Dry run of checking a solution on a Essential server through an Essential REST client.
//!
//! This crate can be used as a library and a binary CLI tool.

#![deny(missing_docs)]
#![deny(unsafe_code)]

use essential_read::{read_contracts, read_solution};
use essential_rest_client::EssentialClient;
use essential_server_types::CheckSolutionOutput;
use essential_types::{contract::Contract, solution::Solution};
use std::path::PathBuf;

/// Dry run a solution check.
/// The contracts should be deployed to the server before calling this function.
pub async fn dry_run(server: String, solution: Solution) -> anyhow::Result<CheckSolutionOutput> {
    let client = EssentialClient::new(server)?;
    let output = client.check_solution(solution).await?;
    Ok(output)
}

/// Dry run a solution check with given contracts.
pub async fn dry_run_with_contracts(
    server: String,
    contracts: Vec<Contract>,
    solution: Solution,
) -> anyhow::Result<CheckSolutionOutput> {
    let client = EssentialClient::new(server)?;
    let output = client
        .check_solution_with_contracts(solution, contracts)
        .await?;
    Ok(output)
}

/// Dry run a solution check.
/// Reads a solution from file, then checks it.
/// The contracts should be deployed to the server before calling this function.
pub async fn dry_run_from_path(
    server: String,
    solution: PathBuf,
) -> anyhow::Result<CheckSolutionOutput> {
    let solution = read_solution(solution).await?;
    let output = dry_run(server, solution).await?;
    Ok(output)
}

/// Dry run a solution check with given contracts.
/// Reads contracts from a directory and a solution from a file, then checks the solution.
pub async fn dry_run_with_contracts_from_path(
    server: String,
    contracts: PathBuf,
    solution: PathBuf,
) -> anyhow::Result<CheckSolutionOutput> {
    let contracts = read_contracts(contracts).await?.into_iter().collect();
    let solution = read_solution(solution).await?;
    let output = dry_run_with_contracts(server, contracts, solution).await?;
    Ok(output)
}
