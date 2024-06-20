//! Dry run of checking a solution on a Essential server through an Essential REST client.
//!
//! This crate can be used as a library and a binary CLI tool.

#![deny(missing_docs)]
#![deny(unsafe_code)]

use essential_rest_client::EssentialClient;
use essential_server_types::CheckSolutionOutput;
use essential_types::{intent::Intent, solution::Solution};
use std::path::Path;
use tokio::io::{AsyncReadExt, BufReader};

/// Dry run a solution check.
/// The intents should be deployed to the server before calling this function.
pub async fn dry_run(server: String, solution: Solution) -> anyhow::Result<CheckSolutionOutput> {
    let client = EssentialClient::new(server)?;
    let output = client.check_solution(solution).await?;
    Ok(output)
}

/// Dry run a solution check.
/// Deserializes the solution from a string, then checks the solution.
/// The intents should be deployed to the server before calling this function.
pub async fn dry_run_from_string(
    server: String,
    solution: String,
) -> anyhow::Result<CheckSolutionOutput> {
    let solution: Solution = serde_json::from_str(&solution)?;
    let output = dry_run(server, solution).await?;
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

/// Dry run a solution check with given intents.
/// Reads intents from a directory and deserializes the solution from a string, then checks the solution.
pub async fn dry_run_with_intents_from_path(
    server: String,
    intents: &Path,
    solution: String,
) -> anyhow::Result<CheckSolutionOutput> {
    let intents = read_intent_sets(intents)
        .await?
        .into_iter()
        .flatten()
        .collect();
    let solution: Solution = serde_json::from_str(&solution)?;
    let output = dry_run_with_intents(server, intents, solution).await?;
    Ok(output)
}

/// Read and deserialize intent sets in a directory.
pub async fn read_intent_sets(path: &Path) -> anyhow::Result<Vec<Vec<Intent>>> {
    let mut intents: Vec<Vec<Intent>> = vec![];
    for intent in path.read_dir()? {
        let name = intent?.file_name();
        let name = name
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("invalid file name"))?;
        let path = path.join(name);
        let intent_set = read_intents(&path).await?;
        intents.push(intent_set);
    }
    Ok(intents)
}

/// Read and deserialize intents from a file.
pub async fn read_intents(path: &Path) -> anyhow::Result<Vec<Intent>> {
    let file = tokio::fs::File::open(path).await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;
    Ok(serde_json::from_slice::<Vec<Intent>>(&bytes)?)
}
