//! Library to read predicates and solutions.
//!
//! Provides functions to read and optionally deserialize predicates and solutions in JSON format.

#![deny(missing_docs)]
#![deny(unsafe_code)]

use anyhow::{anyhow, Result};
use essential_types::{predicate::Predicate, solution::Solution};
use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncReadExt, BufReader};

/// Read and deserialize contracts in a directory.
pub async fn read_contracts(path: PathBuf) -> Result<Vec<Vec<Predicate>>> {
    let mut contracts: Vec<Vec<Predicate>> = vec![];
    for entry in path.read_dir()? {
        let file_path = dir_entry_to_path(&path, entry?)
            .inspect_err(|err| println!("skipping file: {}", err))?;
        let bytes = read_bytes(file_path).await?;
        let predicate = deserialize_contract(bytes).await?;
        contracts.push(predicate);
    }
    Ok(contracts)
}

/// Read and deserialize predicates from a file.
pub async fn read_contract(path: PathBuf) -> Result<Vec<Predicate>> {
    let bytes = read_bytes(path).await?;
    let contract = deserialize_contract(bytes).await?;
    Ok(contract)
}

/// Read and deserialize solutions in a directory.
pub async fn read_solutions(path: PathBuf) -> Result<Vec<Solution>> {
    let mut solutions: Vec<Solution> = vec![];
    for entry in path.read_dir()? {
        let file_path = dir_entry_to_path(&path, entry?)
            .inspect_err(|err| println!("skipping file: {}", err))?;

        let bytes = read_bytes(file_path).await?;
        let solution = deserialize_solution(bytes).await?;
        solutions.push(solution);
    }
    Ok(solutions)
}

/// Read and deserialize a solution from a file.
pub async fn read_solution(path: PathBuf) -> Result<Solution> {
    let bytes = read_bytes(path).await?;
    let solution = deserialize_solution(bytes).await?;
    Ok(solution)
}

/// Read the contents of a file as bytes.
pub async fn read_bytes(path: PathBuf) -> Result<Vec<u8>> {
    let file = tokio::fs::File::open(path).await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;
    Ok(bytes)
}

/// Read the contents of files in a directory as a vector of bytes.
pub async fn read_bytes_dir(path: PathBuf) -> Result<Vec<Vec<u8>>> {
    let mut all_bytes: Vec<Vec<u8>> = vec![];
    for entry in path.read_dir()? {
        let file_path = dir_entry_to_path(&path, entry?)
            .inspect_err(|err| println!("skipping file: {}", err))?;
        let bytes = read_bytes(file_path).await?;
        all_bytes.push(bytes);
    }
    Ok(all_bytes)
}

/// Deserialize contract from bytes.
pub async fn deserialize_contract(bytes: Vec<u8>) -> Result<Vec<Predicate>> {
    let contract = serde_json::from_slice::<Vec<Predicate>>(&bytes)?;
    Ok(contract)
}

/// Deserialize a solution from bytes.
pub async fn deserialize_solution(bytes: Vec<u8>) -> Result<Solution> {
    let solution = serde_json::from_slice::<Solution>(&bytes)?;
    Ok(solution)
}

/// Convert a `DirEntry` in directory with given path to a `PathBuf`.
fn dir_entry_to_path(path: &Path, entry: DirEntry) -> Result<PathBuf> {
    if entry
        .file_type()
        .inspect_err(|err| println!("skipping file: {}", err))?
        .is_dir()
    {
        return Err(anyhow!("skipping directory: {:?}", entry.path()));
    }

    let name = entry.file_name();
    let name = name
        .to_str()
        .ok_or_else(|| anyhow!("file name is invalid UTF-8"))?;
    if !name.ends_with(".json") {
        return Err(anyhow!("skipping non-JSON file: {:?}", name));
    }
    let path = path.join(name);
    Ok(path)
}
