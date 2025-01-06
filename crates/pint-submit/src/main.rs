use clap::Parser;
use essential_rest_client::builder_client::EssentialBuilderClient;
use essential_types::solution::SolutionSet;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Tool to submit a solution to an Essential builder endpoint.
struct Args {
    /// The endpoint of builder to bind to.
    #[arg(long)]
    builder_address: String,
    /// Path to the solutions file in the form of a JSON-serialized `SolutionSet`.
    #[arg(long)]
    solutions: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(err) = run(args).await {
        eprintln!("Command failed because: {}", err);
    }
}

async fn run(args: Args) -> anyhow::Result<()> {
    let Args {
        builder_address,
        solutions,
    } = args;

    let builder_client = EssentialBuilderClient::new(builder_address)?;
    let solution_set = serde_json::from_str::<SolutionSet>(&from_file(solutions).await?)?;
    let _ = builder_client.submit_solution_set(&solution_set).await?;
    Ok(())
}

async fn from_file(path: PathBuf) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}
