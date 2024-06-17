use essential_types::intent::Intent;
use std::path::PathBuf;
use tokio::{
    io::{AsyncReadExt, BufReader},
    process::Command,
};

pub async fn compile_pint_file(path: PathBuf, name: &str) -> anyhow::Result<Vec<Intent>> {
    let pint_path = path.join(name);
    assert!(pint_path.exists());
    let pint_target_path = path.join("target");
    std::fs::create_dir(path).ok();

    let output = Command::new("pintc")
        .arg(pint_path.display().to_string())
        .arg("--output")
        .arg(pint_target_path.join(name))
        .output()
        .await?;

    assert!(
        output.status.success(),
        "pintc failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let file = tokio::fs::File::open(pint_target_path.join(name)).await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;

    let intents: Vec<Intent> = serde_json::from_slice(&bytes)?;
    Ok(intents)
}
