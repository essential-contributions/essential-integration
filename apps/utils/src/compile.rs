use essential_types::contract::Contract;
use std::path::PathBuf;
use tokio::{
    io::{AsyncReadExt, BufReader},
    process::Command,
};

pub async fn compile_pint_project(path: PathBuf, name: &str) -> anyhow::Result<Contract> {
    let pint_manifest_path = path.join(name).join("pint.toml");
    assert!(pint_manifest_path.exists());

    let output = Command::new("pint")
        .arg("--manifest-path")
        .arg(pint_manifest_path.display().to_string())
        .output()
        .await?;

    assert!(
        output.status.success(),
        "pintc failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let file = tokio::fs::File::open(path.join("out").join(format!("{}.json", name))).await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;

    let contract: Contract = serde_json::from_slice(&bytes)?;
    Ok(contract)
}
