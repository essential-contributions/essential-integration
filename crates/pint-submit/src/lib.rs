use essential_types::solution::Solution;
use std::path::PathBuf;

pub enum SolutionInputType {
    Path(PathBuf),
    Json(String),
}

pub async fn solution_from_input(solution: SolutionInputType) -> Result<Solution, anyhow::Error> {
    let solution = match solution {
        SolutionInputType::Path(path) => from_file(path).await?,
        SolutionInputType::Json(json) => json,
    };
    Ok(serde_json::from_str(&solution)?)
}

async fn from_file(path: PathBuf) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}