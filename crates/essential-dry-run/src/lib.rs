//! Dry run of checking a solution on a Essential server through an Essential REST client.
//!
//! This crate can be used as a library and a binary CLI tool.

#![deny(missing_docs)]
#![deny(unsafe_code)]

use essential_read::{read_intent_sets, read_solution};
use essential_rest_client::EssentialClient;
use essential_server_types::CheckSolutionOutput;
use essential_types::{intent::Intent, solution::Solution};
use std::path::PathBuf;

/// Dry run a solution check.
/// The intents should be deployed to the server before calling this function.
pub async fn dry_run(server: String, solution: Solution) -> anyhow::Result<CheckSolutionOutput> {
    let client = EssentialClient::new(server)?;
    let output = client.check_solution(solution).await?;
    Ok(output)
}

/// Dry run a solution check with given intents.
pub async fn dry_run_with_intents(
    server: String,
    intents: Vec<Intent>,
    solution: Solution,
) -> anyhow::Result<CheckSolutionOutput> {
    let client = EssentialClient::new(server)?;
    let output = client.check_solution_with_data(solution, intents).await?;
    Ok(output)
}

/// Dry run a solution check.
/// Reads a solution from file, then checks it.
/// The intents should be deployed to the server before calling this function.
pub async fn dry_run_from_path(
    server: String,
    solution: PathBuf,
) -> anyhow::Result<CheckSolutionOutput> {
    let solution = read_solution(solution).await?;
    let output = dry_run(server, solution).await?;
    Ok(output)
}

/// Dry run a solution check with given intents.
/// Reads intents from a directory and a solution from a file, then checks the solution.
pub async fn dry_run_with_intents_from_path(
    server: String,
    intents: PathBuf,
    solution: PathBuf,
) -> anyhow::Result<CheckSolutionOutput> {
    let intents = read_intent_sets(intents)
        .await?
        .into_iter()
        .flatten()
        .collect();
    let solution = read_solution(solution).await?;
    let output = dry_run_with_intents(server, intents, solution).await?;
    Ok(output)
}
