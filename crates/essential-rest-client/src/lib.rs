//! Client libraries for interacting with the Essential builder and the Essential node.

#![deny(missing_docs)]

use anyhow::Context;
use std::path::Path;

use essential_types::{Contract, Program};

/// Client library for sending requests to the Essential builder.
pub mod builder_client;
/// Client library for sending requests to the Essential node.
pub mod node_client;

/// A helper for reading a [`Contract`] and its [`Program`]s from a JSON file at the given path.
///
/// Specifically, expects the JSON to be the serialized form of `(Contract, Vec<Program>)`.
pub async fn contract_from_path(contract_path: &Path) -> anyhow::Result<(Contract, Vec<Program>)> {
    let contract_string = tokio::fs::read_to_string(&contract_path)
        .await
        .with_context(|| format!("failed to read contract from file {contract_path:?}"))?;
    let (contract, programs): (Contract, Vec<Program>) = serde_json::from_str(&contract_string)
        .with_context(|| {
            format!("failed to parse contract and/or its programs from JSON at {contract_path:?}")
        })?;
    Ok((contract, programs))
}

/// Map `reqwest::Response` into `anyhow::Result`.
async fn handle_response(response: reqwest::Response) -> anyhow::Result<reqwest::Response> {
    let status = response.status();
    if status.is_success() {
        Ok(response)
    } else {
        let text = response.text().await?;
        Err(anyhow::anyhow!("{}: {}", status, text))
    }
}
