use clap::Parser;
use essential_rest_client::builder_client::EssentialBuilderClient;
use essential_types::{solution::SolutionSet, ContentAddress};
use pint_submit::{print_submitted, print_submitting, solution_from_input, SolutionInputType};
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
    let solution_ca = essential_hash::content_addr(&solution_set);
    let solution_input = SolutionInputType::Path(solution);
    print_submitting(&solution_ca);
    let output = builder_client.submit_solution(&solution_set).await?;
    if solution_ca != output {
        anyhow::bail!("The content address of the submitted solution set differs from expected. May be a serialization error.");
    }
    print_submitted();
    Ok(())
}
