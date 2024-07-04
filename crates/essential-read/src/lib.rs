//! Library to read predicates and solutions.
//!
//! Provides functions to read and optionally deserialize predicates and solutions in JSON format.

#![deny(missing_docs)]
#![deny(unsafe_code)]

use anyhow::{anyhow, Result};
use essential_types::{contract::Contract, solution::Solution};
use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncReadExt, BufReader};

/// Read and deserialize predicates from a file.
///
/// Calls `check_path_json` via call to `read_bytes`.
pub async fn read_contract(path: PathBuf) -> Result<Contract> {
    let bytes = read_bytes(path).await?;
    let contract = deserialize_contract(bytes).await?;
    Ok(contract)
}

/// Read and deserialize contracts in a directory.
///
/// Calls `check_path_json` for every entry in directory with a valid UTF-8 file name.
pub async fn read_contracts(path: PathBuf) -> Result<Vec<Contract>> {
    let mut contracts: Vec<Contract> = vec![];
    for entry in path.read_dir()? {
        let file_path = dir_entry_to_path(&path, entry?)
            .inspect_err(|err| println!("skipping file: {}", err))?;
        check_path_json(&path)?;
        let bytes = read_bytes(file_path).await?;
        let contract = deserialize_contract(bytes).await?;
        contracts.push(contract);
    }
    Ok(contracts)
}

/// Read and deserialize a solution from a file.
///
/// Calls `check_path_json` via call to `read_bytes`.
pub async fn read_solution(path: PathBuf) -> Result<Solution> {
    let bytes = read_bytes(path).await?;
    let solution = deserialize_solution(bytes).await?;
    Ok(solution)
}

/// Read and deserialize solutions in a directory.
///
/// Calls `check_path_json` for every entry in directory with a valid UTF-8 file name.
pub async fn read_solutions(path: PathBuf) -> Result<Vec<Solution>> {
    let mut solutions: Vec<Solution> = vec![];
    for entry in path.read_dir()? {
        #[cfg(feature = "tracing")]
        let file_path = dir_entry_to_path(&path, entry?)
            .inspect_err(|err| tracing::warn!("skipping file: {}", err));

        #[cfg(not(feature = "tracing"))]
        let file_path = dir_entry_to_path(&path, entry?);

        let Ok(file_path) = file_path else {
            continue;
        };

        #[cfg(not(feature = "tracing"))]
        if check_path_json(&path).is_err() {
            continue;
        }

        #[cfg(feature = "tracing")]
        if let Err(e) = check_path_json(&path) {
            tracing::warn!("skipping file because it's not json: {}", e);
            continue;
        }

        let bytes = read_bytes_inner(file_path).await?;
        let solution = deserialize_solution(bytes).await?;
        solutions.push(solution);
    }
    Ok(solutions)
}

/// Read the contents of a file as bytes.
///
/// Calls `check_path_json`.
pub async fn read_bytes(path: PathBuf) -> Result<Vec<u8>> {
    check_path_json(&path)?;
    read_bytes_inner(path).await
}

async fn read_bytes_inner(path: PathBuf) -> Result<Vec<u8>> {
    let file = tokio::fs::File::open(path).await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;
    Ok(bytes)
}

/// Read the contents of files in a directory as a vector of bytes.
///
/// Calls `check_path_json` for every entry in directory with a valid UTF-8 file name.
pub async fn read_bytes_dir(path: PathBuf) -> Result<Vec<Vec<u8>>> {
    let mut all_bytes: Vec<Vec<u8>> = vec![];
    for entry in path.read_dir()? {
        #[cfg(feature = "tracing")]
        let file_path = dir_entry_to_path(&path, entry?)
            .inspect_err(|err| tracing::warn!("skipping file: {}", err));

        #[cfg(not(feature = "tracing"))]
        let file_path = dir_entry_to_path(&path, entry?);

        let Ok(file_path) = file_path else {
            continue;
        };

        #[cfg(not(feature = "tracing"))]
        if check_path_json(&path).is_err() {
            continue;
        }

        #[cfg(feature = "tracing")]
        if let Err(e) = check_path_json(&path) {
            tracing::warn!("skipping file because it's not json: {}", e);
            continue;
        }

        let bytes = read_bytes_inner(file_path).await?;
        all_bytes.push(bytes);
    }
    Ok(all_bytes)
}

/// Deserialize a contract from bytes.
pub async fn deserialize_contract(bytes: Vec<u8>) -> Result<Contract> {
    let contract = serde_json::from_slice::<Contract>(&bytes)?;
    Ok(contract)
}

/// Deserialize a solution from bytes.
pub async fn deserialize_solution(bytes: Vec<u8>) -> Result<Solution> {
    let solution = serde_json::from_slice::<Solution>(&bytes)?;
    Ok(solution)
}

/// Convert a `DirEntry` in directory with given path to a `PathBuf`.
///
/// Returns an error if the file name is not valid UTF-8
fn dir_entry_to_path(path: &Path, entry: DirEntry) -> Result<PathBuf> {
    let name = entry.file_name();
    let name = name
        .to_str()
        .ok_or_else(|| anyhow!("file name is invalid UTF-8"))?;
    let path = path.join(name);
    Ok(path)
}

/// Check if a path is a JSON file.
///
/// Returns an error if:
/// - The path is not a file.
/// - The path does not have a `.json` extension.
fn check_path_json(path: &Path) -> Result<()> {
    if !path.is_file() {
        return Err(anyhow!("path is not a file: {:?}", path));
    }
    if !path.extension().map_or(false, |ext| ext == "json") {
        return Err(anyhow!("path is not a JSON file: {:?}", path));
    }
    Ok(())
}
