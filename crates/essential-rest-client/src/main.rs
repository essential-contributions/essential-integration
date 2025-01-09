use clap::{Parser, Subcommand};
use essential_node_types::BigBang;
use essential_rest_client::{
    builder_client::EssentialBuilderClient, contract_from_path, node_client::EssentialNodeClient,
};
use essential_types::{
    convert::words_from_hex_str,
    solution::{Solution, SolutionSet},
    ContentAddress, Key, Word,
};
use std::{path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Essential REST Client
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Commands for calling functions.
#[derive(Subcommand, Debug)]
enum Command {
    /// List blocks in the given block number range.
    ListBlocks {
        /// The endpoint of node to bind to.
        node_address: String,
        /// Range of block number of blocks to list, end of range exclusive.
        range: BlockRange,
    },
    /// Query the state of a contract.
    QueryState {
        /// The endpoint of node to bind to.
        node_address: String,
        /// Address of the contract to query, encoded as hex.
        #[arg(short, long)]
        content_address: ContentAddress,
        /// Key to query, encoded as hex.
        #[arg(value_parser = words_from_hex_str)]
        key: Key,
    },
    /// Deploy a contract.
    RegisterContract {
        /// The endpoint of builder to bind to.
        builder_address: String,
        /// Path to the contract file as a json `Contract`.
        contract: PathBuf,
    },
    /// Submit a solution.
    SubmitSolutionSet {
        /// The endpoint of builder to bind to.
        builder_address: String,
        /// Path to the solution set file as a json `SolutionSet`.
        solution_set: PathBuf,
    },
    /// Get the latest failures for solution.
    LatestSolutionFailures {
        /// The endpoint of builder to bind to.
        builder_address: String,
        /// The content address of the solution.
        #[arg(short, long)]
        content_address: ContentAddress,
        /// The number of failures to get.
        limit: u32,
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
        Command::ListBlocks {
            node_address,
            range,
        } => {
            let node_client = EssentialNodeClient::new(node_address)?;
            let output = node_client.list_blocks(range.start..range.end).await?;
            println!("{}", serde_json::to_string(&output)?);
        }
        Command::QueryState {
            node_address,
            content_address,
            key,
        } => {
            let node_client = EssentialNodeClient::new(node_address)?;
            let output = node_client
                .query_state(content_address.to_owned(), key)
                .await?;
            println!("{}", serde_json::to_string(&output)?);
        }
        Command::RegisterContract {
            builder_address,
            contract,
        } => {
            // FIXME: Allow for specifying big bang config via CLI argument.
            let big_bang = BigBang::default();
            let builder_client = EssentialBuilderClient::new(builder_address)?;
            let (contract, programs) = contract_from_path(&contract).await?;
            let output = builder_client
                .register_contract(&big_bang, &contract, &programs)
                .await?;
            println!("{}", output);
        }
        Command::SubmitSolutionSet {
            builder_address,
            solution_set,
        } => {
            let builder_client = EssentialBuilderClient::new(builder_address)?;
            let solution_set =
                serde_json::from_str::<SolutionSet>(&from_file(solution_set).await?)?;
            let output = builder_client.submit_solution_set(&solution_set).await?;
            println!("{}", output);
        }
        Command::LatestSolutionFailures {
            builder_address,
            content_address,
            limit,
        } => {
            let builder_client = EssentialBuilderClient::new(builder_address)?;
            let output = builder_client
                .latest_solution_failures(&content_address, limit)
                .await?;
            println!("{}", serde_json::to_string(&output)?);
        }
    }
    Ok(())
}

async fn from_file(path: PathBuf) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}

#[derive(Clone, Debug)]
pub struct BlockRange {
    start: Word,
    end: Word,
}

impl FromStr for BlockRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("..");
        let start = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("No start block"))?;
        let end = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("No end block"))?;
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}
