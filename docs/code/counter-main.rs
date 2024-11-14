// ANCHOR: full
// ANCHOR: use
use clap::{Args, Parser, Subcommand};
use counter_app::{counter_key, extract_count, incremented_solution, CounterKey};
use essential_app_utils::compile::compile_pint_project;
use essential_rest_client::node_client::EssentialNodeClient;
use essential_types::{ContentAddress, PredicateAddress, Value};
use std::path::PathBuf;
// ANCHOR_END: use

// ANCHOR: cli
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
        /// The address of the builder to connect to.
        builder_api: String,
        #[command(flatten)]
        server: Shared,
    },
}

#[derive(Args)]
pub struct Shared {
    /// The address of the node to connect to.
    pub node_api: String,
    /// The directory containing the pint files.
    pub pint_directory: PathBuf,
}
// ANCHOR_END: cli

// ANCHOR: main
#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if let Err(err) = run(args).await {
        eprintln!("Command failed because: {}", err);
    }
}
// ANCHOR_END: main

// ANCHOR: run
async fn run(cli: Cli) -> anyhow::Result<()> {
    let Cli { command } = cli;
    match command {
        Command::ReadCount {
            server: Shared {
                node_api,
                pint_directory,
            },
        } => {
            let address = compile_address(pint_directory).await?;
            let node = EssentialNodeClient::new(node_api)?;
            let key = counter_key();
            let count = query_count(node, address.contract, key).await?;
            let count_value = extract_count(count)?;
            println!("Current count is: {}", count_value);
        }
        Command::IncrementCount {
            builder_api,
            server: Shared {
                node_api,
                pint_directory,
            },
        } => {
            let address = compile_address(pint_directory).await?;
            let node = EssentialNodeClient::new(node_api)?;
            let key = counter_key();
            let count = query_count(node, address.contract.clone(), key).await?;
            let (solution, new_count) = incremented_solution(count)?; // Pass only count
            let builder =
                essential_rest_client::builder_client::EssentialBuilderClient::new(builder_api)?;
            let ca = builder.submit_solution(&solution).await?;
            println!("Submitted solution: {}", ca);
            println!("Incremented count to: {}", new_count);
        }
    }
    Ok(())
}
// ANCHOR_END: run

// ANCHOR: qry
async fn query_count(
    node: EssentialNodeClient,
    address: ContentAddress,
    key: CounterKey,
) -> anyhow::Result<Option<Value>> {
    Ok(node.query_state(address, key.0).await?)
}
// ANCHOR_END: qry

// ANCHOR: comp
async fn compile_address(pint_directory: PathBuf) -> Result<PredicateAddress, anyhow::Error> {
    let counter = compile_pint_project(pint_directory).await?;
    let contract_address = essential_hash::contract_addr::from_contract(&counter);
    let predicate_address = essential_hash::content_addr(&counter.predicates[0]);
    let predicate_address = PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };
    Ok(predicate_address)
}
// ANCHOR_END: comp
// ANCHOR_END: full
