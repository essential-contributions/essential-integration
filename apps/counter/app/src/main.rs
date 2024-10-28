use clap::{Parser, Subcommand};
use counter_app::{counter_key, extract_count, incremented_solution, CounterKey, QueryCount};
use essential_app_utils::compile::compile_pint_project;
use essential_rest_client::node_client::EssentialNodeClient;
use essential_types::{ContentAddress, PredicateAddress};
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
        /// The address of the node to connect to.
        node_api: String,
        /// The directory containing the pint files.
        pint_directory: PathBuf,
    },
    IncrementCount {
        /// The address of the node to connect to.
        node_api: String,
        /// The address of the builder to connect to.
        builder_api: String,
        /// The directory containing the pint files.
        pint_directory: PathBuf,
    },
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
            node_api,
            pint_directory,
        } => {
            let address = compile_address(pint_directory).await?;
            let node = essential_rest_client::node_client::EssentialNodeClient::new(node_api)?;
            let key = counter_key();
            let count = query_count(node, address.contract, key).await?;
            let count_value = extract_count(count)?;
            println!("Current count is: {}", count_value);
        }
        Command::IncrementCount {
            builder_api,
            node_api,
            pint_directory,
        } => {
            let address = compile_address(pint_directory).await?;
            let node = essential_rest_client::node_client::EssentialNodeClient::new(node_api)?;
            let key = counter_key();
            let count = query_count(node, address.contract.clone(), key).await?;
            let (solution, new_count) = incremented_solution(address, count)?;
            let builder =
                essential_rest_client::builder_client::EssentialBuilderClient::new(builder_api)?;
            let ca = builder.submit_solution(&solution).await?;
            println!("Submitted solution: {}", ca);
            println!("Incremented count to: {}", new_count);
        }
    }
    Ok(())
}

async fn query_count(
    node: EssentialNodeClient,
    address: ContentAddress,
    key: CounterKey,
) -> anyhow::Result<QueryCount> {
    Ok(QueryCount(node.query_state(address, key.0).await?))
}

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
