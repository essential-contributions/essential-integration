use clap::{Parser, Subcommand};
use essential_rest_client::{
    builder_client::EssentialBuilderClient, node_client::EssentialNodeClient,
};
use essential_types::{
    contract::Contract, convert::word_from_bytes, solution::Solution, ContentAddress, Word,
};
use std::{path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Essential REST Client
struct Cli {
    #[command(subcommand)]
    command: Command,
    address: String,
}

/// Commands for calling functions.
#[derive(Subcommand, Debug)]
enum Command {
    /// List blocks in the given block number range.
    ListBlocks {
        /// Range of block number of blocks to list, end of range exclusive.
        range: BlockRange,
    },
    /// Query the state of a contract.
    QueryState {
        /// Address of the contract to query, encoded as hex.
        #[arg(short, long)]
        content_address: ContentAddress,
        /// Key to query, encoded as hex.
        key: Key,
    },
    /// Deploy a contract.
    DeployContract {
        /// Path to the contract file as a json `Contract`.
        contract: PathBuf,
    },
    /// Submit a solution.
    SubmitSolution {
        /// Path to the solution file as a json `Solution`.
        solution: PathBuf,
    },
    /// Get the latest failures for solution.
    LatestSolutionFailures {
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
    let Cli { command, address } = cli;
    match command {
        Command::ListBlocks { range } => {
            let node_client = EssentialNodeClient::new(address)?;
            let output = node_client.list_blocks(range.start..range.end).await?;
            println!("{}", serde_json::to_string(&output)?);
        }
        Command::QueryState {
            content_address,
            key,
        } => {
            let node_client = EssentialNodeClient::new(address)?;
            let output = node_client
                .query_state(content_address.to_owned(), key.0.to_owned())
                .await?;
            println!("{}", serde_json::to_string(&output)?);
        }
        Command::DeployContract { contract } => {
            let builder_client = EssentialBuilderClient::new(address)?;
            let contract = serde_json::from_str::<Contract>(&from_file(contract).await?)?;
            let output = builder_client.deploy_contract(&contract).await?;
            println!("{}", output);
        }
        Command::SubmitSolution { solution } => {
            let builder_client = EssentialBuilderClient::new(address)?;
            let solution = serde_json::from_str::<Solution>(&from_file(solution).await?)?;
            let output = builder_client.submit_solution(&solution).await?;
            println!("{}", output);
        }
        Command::LatestSolutionFailures {
            content_address,
            limit,
        } => {
            let builder_client = EssentialBuilderClient::new(address)?;
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

#[derive(Clone, Debug)]
struct Key(essential_types::Key);

impl FromStr for Key {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            hex::decode(s)?
                .chunks_exact(8)
                .map(|chunk| word_from_bytes(chunk.try_into().expect("Always 8 bytes")))
                .collect(),
        ))
    }
}
