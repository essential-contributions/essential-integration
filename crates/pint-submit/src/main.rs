use clap::Parser;
use pint_submit::{submit_solution, SolutionInputType};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Tool to submit a solution to an Essential builder endpoint.
struct Args {
    /// The endpoint of builder to bind to.
    #[arg(long)]
    builder_address: String,
    /// Path to the solution file as a json `Solution`.
    #[arg(long)]
    solution: PathBuf,
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
        solution,
    } = args;

    let solution_input = SolutionInputType::Path(solution);
    submit_solution(builder_address, solution_input).await?;
    Ok(())
}
