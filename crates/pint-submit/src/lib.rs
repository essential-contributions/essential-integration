use clap::builder::styling::Style;
use essential_rest_client::builder_client::EssentialBuilderClient;
use essential_types::{solution::Solution, ContentAddress};
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

pub async fn submit_solution(
    builder_address: String,
    solution_input: SolutionInputType,
) -> anyhow::Result<ContentAddress> {
    let builder_client = EssentialBuilderClient::new(builder_address)?;
    let solution = solution_from_input(solution_input).await?;
    let solution_ca = essential_hash::content_addr(&solution);
    print_submitting(&solution_ca);
    let output = builder_client.submit_solution(&solution).await?;
    if solution_ca != output {
        anyhow::bail!("The content address of the submitted solution differs from expected. May be a serialization error.");
    }
    print_submitted();
    Ok(output)
}

async fn from_file(path: PathBuf) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}

/// Print the "Submitting ..." output.
pub fn print_submitting(ca: &ContentAddress) {
    let bold = Style::new().bold();
    println!(
        "  {}Submitting{} solution {}",
        bold.render(),
        bold.render_reset(),
        ca,
    );
}

/// Print the "Submitted" output.
pub fn print_submitted() {
    let bold = Style::new().bold();
    println!(
        "   {}Submitted{} successfully",
        bold.render(),
        bold.render_reset(),
    );
}
