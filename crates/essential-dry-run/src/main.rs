use clap::{Parser, Subcommand};
use essential_dry_run::{dry_run_from_path, dry_run_with_intents_from_path};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Server address to bind to. Default: "http://0.0.0.0:0"
    #[arg(default_value_t = String::from("http://0.0.0.0:0"))]
    address: String,
    /// Select a subcommand to run
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    CheckWithIntents {
        /// The address of the server to connect to.
        #[arg(long)]
        server: String,
        /// Path to compiled intents.
        #[arg(long)]
        intents: PathBuf,
        /// Path to solution.
        #[arg(long)]
        solution: PathBuf,
    },
    Check {
        /// The address of the server to connect to.
        #[arg(long)]
        server: String,
        /// Path to solution.
        #[arg(long)]
        solution: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if let Err(e) = run(args).await {
        eprintln!("Command failed because: {}", e);
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::CheckWithIntents {
            server,
            intents,
            solution,
        } => {
            let output = dry_run_with_intents_from_path(server, intents, solution).await?;
            println!("{}", serde_json::to_string(&output)?);
        }
        Command::Check { solution, server } => {
            let output = dry_run_from_path(server, solution).await?;
            println!("{}", serde_json::to_string(&output)?);
        }
    }
    Ok(())
}
