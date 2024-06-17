use std::path::PathBuf;

pub async fn read_pint_file(path: PathBuf, name: &str) -> anyhow::Result<String> {
    let file = tokio::fs::read_to_string(path.join(name)).await?;
    Ok(file)
}
