use clap::{Parser, Subcommand};
use essential_rest_client::{
    builder_client::EssentialBuilderClient, node_client::EssentialNodeClient,
};
use essential_types::{convert::word_from_bytes, solution::Solution, ContentAddress, Word};
use std::{path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Essential REST Client
struct Cli {
    /// The endpoint of node to bind to.
    #[arg(long)]
    node_address: Option<String>,
    /// The endpoint of builder to bind to.
    #[arg(long)]
    builder_address: Option<String>,
    #[command(subcommand)]
    commands: Commands,
}

/// Commands for calling functions.
#[derive(Subcommand, Debug)]
enum Commands {
    #[command(flatten)]
    Node(NodeCommands),
    #[command(flatten)]
    Builder(BuilderCommands),
}

/// Commands for calling node functions.
#[derive(Subcommand, Debug)]
enum NodeCommands {
    /// List blocks in the given block number range.
    ListBlocks {
        /// Range of block number of blocks to list, end of range exclusive.
        range: BlockRange,
    },
    /// Query the state of a contract.
    QueryState {
        /// Address of the contract to query, encoded as hex.
        address: ContentAddress,
        /// Key to query, encoded as hex.
        key: Key,
    },
}

/// Commands for calling builder functions.
#[derive(Parser, Debug)]
enum BuilderCommands {
    /// Submit a solution.
    SubmitSolution {
        /// Path to the solution file as a json `Solution`.
        solution: PathBuf,
    },
    /// Get the latest failures for solution.
    LatestSolutionFailures {
        /// The content address of the solution.
        address: ContentAddress,
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
    let Cli {
        node_address,
        builder_address,
        commands,
    } = cli;
    node_address
        .as_ref()
        .or(builder_address.as_ref())
        .ok_or_else(|| {
            anyhow::anyhow!("No address provided. Please provide either a node or builder address.")
        })?;
    if let Some(addr) = node_address {
        let node_client = EssentialNodeClient::new(addr)?;
        match commands {
            Commands::Node(ref node_commands) => match node_commands {
                NodeCommands::ListBlocks { range } => {
                    let output = node_client.list_blocks(range.start..range.end).await?;
                    print!("{}", serde_json::to_string(&output)?);
                }
                NodeCommands::QueryState { address, key } => {
                    let output = node_client
                        .query_state(address.to_owned(), key.0.to_owned())
                        .await?;
                    print!("{}", serde_json::to_string(&output)?);
                }
            },
            Commands::Builder(_) => {}
        }
    }
    if let Some(addr) = builder_address {
        let builder_client = EssentialBuilderClient::new(addr)?;
        match commands {
            Commands::Builder(builder_commands) => match builder_commands {
                BuilderCommands::SubmitSolution { solution } => {
                    let solution = serde_json::from_str::<Solution>(&from_file(solution).await?)?;
                    let output = builder_client.submit_solution(&solution).await?;
                    print!("{}", output);
                }
                BuilderCommands::LatestSolutionFailures { address, limit } => {
                    let output = builder_client
                        .latest_solution_failures(&address, limit)
                        .await?;
                    print!("{}", serde_json::to_string(&output)?);
                }
            },
            Commands::Node(_) => {}
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
