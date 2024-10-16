use anyhow::{bail, ensure};
use essential_types::{contract::Contract, predicate::Predicate};
use std::path::PathBuf;
use tokio::{
    io::{AsyncReadExt, BufReader},
    process::Command,
};

#[derive(Debug)]
pub struct NamedContracts {
    pub contracts: Vec<NamedContract>,
}

#[derive(Debug)]
pub struct NamedContract {
    pub name: String,
    pub contract: Contract,
    pub predicates: Vec<String>,
    pub source: String,
}

pub async fn compile_pint_project(path: PathBuf) -> anyhow::Result<Contract> {
    let (bytes, _, _) = compile_pint_project_inner(path, false).await?;
    let contract: Contract = serde_json::from_slice(&bytes)?;
    Ok(contract)
}

pub async fn compile_pint_project_and_abi(
    path: PathBuf,
) -> anyhow::Result<(Contract, serde_json::Value)> {
    let (bytes, abi, _) = compile_pint_project_inner(path, false).await?;
    let contract: Contract = serde_json::from_slice(&bytes)?;
    let abi: serde_json::Value = serde_json::from_slice(&abi)?;
    Ok((contract, abi))
}

pub async fn compile_pint_project_and_abi_with_source(
    path: PathBuf,
) -> anyhow::Result<(Contract, serde_json::Value, String)> {
    let (bytes, abi, source) = compile_pint_project_inner(path, true).await?;
    let contract: Contract = serde_json::from_slice(&bytes)?;
    let abi: serde_json::Value = serde_json::from_slice(&abi)?;
    Ok((contract, abi, source))
}

pub async fn compile_pint_project_inner(
    path: PathBuf,
    include_source: bool,
) -> anyhow::Result<(Vec<u8>, Vec<u8>, String)> {
    let pint_manifest_path = path.join("pint.toml");
    assert!(
        pint_manifest_path.exists(),
        "pint.toml not found: {:?}",
        pint_manifest_path
    );
    dbg!(&pint_manifest_path);

    let pint_toml = tokio::fs::read_to_string(&pint_manifest_path).await?;
    let pint_toml = pint_toml.parse::<toml::Table>()?;
    let Some(name) = pint_toml
        .get("package")
        .and_then(|p| p.as_table()?.get("name"))
        .and_then(|name| name.as_str())
    else {
        bail!("name not found in pint.toml")
    };

    let output = if include_source {
        Command::new("pint")
            .arg("build")
            .arg("--manifest-path")
            .arg(pint_manifest_path.display().to_string())
            .arg("--silent")
            .arg("--print-flat")
            .output()
            .await?
    } else {
        Command::new("pint")
            .arg("build")
            .arg("--manifest-path")
            .arg(pint_manifest_path.display().to_string())
            .output()
            .await?
    };

    ensure!(
        output.status.success(),
        "pint failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let source = if include_source {
        let s = String::from_utf8_lossy(&output.stdout);
        s.lines()
            .skip_while(|line| !line.trim().starts_with(&(format!("\u{1b}[1m{}", name))))
            .skip(1)
            .fold(String::new(), |acc, line| acc + line + "\n")
    } else {
        String::new()
    };

    let file = tokio::fs::File::open(
        path.join("out")
            .join("debug")
            .join(format!("{}.json", name)),
    )
    .await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;

    let abi_file = tokio::fs::File::open(
        path.join("out")
            .join("debug")
            .join(format!("{}-abi.json", name)),
    )
    .await?;
    let mut abi_bytes = Vec::new();
    let mut reader = BufReader::new(abi_file);
    reader.read_to_end(&mut abi_bytes).await?;

    Ok((bytes, abi_bytes, source))
}

pub async fn get_contracts(
    pint_directory: PathBuf,
    contracts: &[&str],
) -> anyhow::Result<NamedContracts> {
    let mut out = Vec::with_capacity(contracts.len());

    for name in contracts {
        let (contract, abi, source) =
            compile_pint_project_and_abi_with_source(pint_directory.clone().join(name)).await?;
        let predicate_names = abi["predicates"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|predicate| !predicate["name"].as_str().unwrap().is_empty())
            .map(|predicate| predicate["name"].as_str().unwrap().to_string())
            .collect();
        let contract = NamedContract {
            name: name.to_string(),
            contract,
            predicates: predicate_names,
            source,
        };
        out.push(contract);
    }
    Ok(NamedContracts { contracts: out })
}

impl NamedContracts {
    pub fn get_contract(&self, name: &str) -> Option<&NamedContract> {
        self.contracts.iter().find(|contract| contract.name == name)
    }
}

impl NamedContract {
    pub fn get_predicate(&self, name: &str) -> Option<&Predicate> {
        self.predicates
            .iter()
            .position(|predicate| {
                predicate.trim().trim_start_matches("::").to_lowercase()
                    == name.trim().trim_start_matches("::").to_lowercase()
            })
            .and_then(|pos| self.contract.predicates.get(pos))
    }
}
