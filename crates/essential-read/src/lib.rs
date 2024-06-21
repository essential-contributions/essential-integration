//! Library to read intents and solutions.
//!
//! Provides functions to read and optionally deserialize intents and solutions in JSON format.

#![deny(missing_docs)]
#![deny(unsafe_code)]

use anyhow::{anyhow, Result};
use essential_types::{intent::Intent, solution::Solution};
use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncReadExt, BufReader};

/// Read and deserialize intent sets in a directory.
pub async fn read_intent_sets(path: PathBuf) -> Result<Vec<Vec<Intent>>> {
    let mut intent_sets: Vec<Vec<Intent>> = vec![];
    for entry in path.read_dir()? {
        let file_path = dir_entry_to_path(&path, entry?)
            .inspect_err(|err| println!("skipping file: {}", err))?;
        let bytes = read_bytes(file_path).await?;
        let intents = deserialize_intents(bytes).await?;
        intent_sets.push(intents);
    }
    Ok(intent_sets)
}

/// Read and deserialize intents from a file.
pub async fn read_intents(path: PathBuf) -> Result<Vec<Intent>> {
    let bytes = read_bytes(path).await?;
    let intents = deserialize_intents(bytes).await?;
    Ok(intents)
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

/// Deserialize intents from bytes.
pub async fn deserialize_intents(bytes: Vec<u8>) -> Result<Vec<Intent>> {
    let intents = serde_json::from_slice::<Vec<Intent>>(&bytes)?;
    Ok(intents)
}

/// Deserialize a solution from bytes.
pub async fn deserialize_solution(bytes: Vec<u8>) -> Result<Solution> {
    let solution = serde_json::from_slice::<Solution>(&bytes)?;
    Ok(solution)
}

/// Convert a `DirEntry` in directory with given path to a `PathBuf`.
fn dir_entry_to_path(path: &Path, entry: DirEntry) -> Result<PathBuf> {
    let name = entry.file_name();
    let name = name
        .to_str()
        .ok_or_else(|| anyhow!("file name is invalid UTF-8"))?;
    let path = path.join(name);
    Ok(path)
}
