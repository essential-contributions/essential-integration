use clap::Parser;
use pint_submit::submit_solution_set;
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

    submit_solution_set(Some(solutions), builder_address, None).await?;
    Ok(())
}
