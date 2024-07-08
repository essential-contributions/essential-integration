use clap::{Args, Parser, Subcommand};
use counter_app::App;
use essential_app_utils::compile::compile_pint_project;
use essential_types::PredicateAddress;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    ReadCount {
        #[command(flatten)]
        server: Shared,
    },
    IncrementCount {
        #[command(flatten)]
        server: Shared,
    },
}

#[derive(Args)]
pub struct Shared {
    /// The address of the server to connect to.
    pub server: String,
    /// The directory containing the pint files.
    pub pint_directory: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if let Err(err) = run(args).await {
        eprintln!("Command failed because: {}", err);
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let Cli { command } = cli;
    match command {
        Command::ReadCount {
            server: Shared {
                server,
                pint_directory,
            },
        } => {
            let app = create_app(pint_directory, server).await?;
            let count = app.read_count().await?;
            println!("Current count is: {}", count);
        }
        Command::IncrementCount {
            server: Shared {
                server,
                pint_directory,
            },
        } => {
            let app = create_app(pint_directory, server).await?;
            let new_count = app.increment().await?;
            println!("Incremented count to: {}", new_count);
        }
    }
    Ok(())
}

async fn create_app(pint_directory: PathBuf, server: String) -> Result<App, anyhow::Error> {
    let counter = compile_pint_project(pint_directory).await?;
    let contract_address = essential_hash::contract_addr::from_contract(&counter);
    let predicate_address = essential_hash::content_addr(&counter.predicates[0]);
    let predicate_address = PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };
    let app = App::new(server, predicate_address)?;
    Ok(app)
}
