use clap::{Parser, Subcommand};
use essential_rest_client::EssentialClient;
use essential_types::{
    contract::{Contract, SignedContract},
    convert::{bytes_from_word, word_from_bytes},
    solution::Solution,
    ContentAddress, PredicateAddress,
};
use std::{path::PathBuf, str::FromStr, time::Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Essential REST Client
struct Cli {
    #[arg(default_value_t = String::from("http://0.0.0.0:0"))]
    /// Server address to bind to
    address: String,
    #[command(subcommand)]
    commands: Option<Commands>,
}

/// Commands for calling server functions.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Deploy a contract to the server.
    DeployContract {
        /// Path to the contract file as a json `SignedContract`.
        contract: PathBuf,
    },
    /// Check a solution against the server.
    CheckSolution {
        /// Path to the solution file as a json `Solution`.
        solution: PathBuf,
    },
    /// Check a solution against the server with data.
    CheckSolutionWithContracts {
        /// Path to the solution file as a json `Solution`.
        solution: PathBuf,
        /// Paths to the contract files as a json `Contract`.
        contracts: Vec<PathBuf>,
    },
    /// Submit a solution to the server.
    SubmitSolution {
        /// Path to the solution file as a json `Solution`.
        solution: PathBuf,
    },
    /// Get the outcome of a solution.
    SolutionOutcome {
        /// Hash of the solution to get the outcome of as Base64.
        solution_hash: ContentAddress,
    },
    /// Get a predicate from a contract.
    GetPredicate {
        /// Address of the contract to get the predicate from as Base64.
        contract: ContentAddress,
        /// Address of the predicate to get as Base64.
        predicate: ContentAddress,
    },
    /// Get a contract from the server.
    GetContract {
        /// Address of the contract to get as Base64.
        address: ContentAddress,
    },
    /// List contracts on the server.
    ListContracts {
        /// Time range to list contracts in as `start..end` in unix timestamp seconds.
        #[arg(default_value(None))]
        time_range: Option<TimeRange>,
        /// Page number to list.
        #[arg(default_value(None))]
        page: Option<u64>,
    },
    /// List solutions in the pool on the server.
    ListSolutionsPool {
        /// Page number to list.
        #[arg(default_value(None))]
        page: Option<u64>,
    },
    /// List winning blocks on the server.
    ListWinningBlocks {
        /// Time range to list winning blocks in as `start..end` in unix timestamp seconds.
        #[arg(default_value(None))]
        time_range: Option<TimeRange>,
        /// Page number to list.
        #[arg(default_value(None))]
        page: Option<u64>,
    },
    /// Query the state of a contract.
    QueryState {
        /// Address of the contract to query as Base64.
        address: ContentAddress,
        /// Key to query as hex.
        key: Key,
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
    let Cli { address, commands } = cli;
    if let Some(commands) = commands {
        let client = EssentialClient::new(address)?;
        match commands {
            Commands::DeployContract { contract } => {
                let contract = from_file(contract).await?;
                let contract = serde_json::from_str::<SignedContract>(&contract)?;
                let output = client.deploy_contract(contract).await?;
                print!("{}", output);
            }
            Commands::CheckSolution { solution } => {
                let solution = serde_json::from_str::<Solution>(&from_file(solution).await?)?;
                let output = client.check_solution(solution).await?;
                print!("{:#?}", output);
            }
            Commands::CheckSolutionWithContracts {
                solution,
                contracts,
            } => {
                let solution = serde_json::from_str::<Solution>(&from_file(solution).await?)?;
                let mut c = Vec::new();
                for contract in contracts {
                    let contract = serde_json::from_str::<Contract>(&from_file(contract).await?)?;
                    c.push(contract);
                }
                let output = client.check_solution_with_contracts(solution, c).await?;
                print!("{:#?}", output);
            }
            Commands::SubmitSolution { solution } => {
                let solution = serde_json::from_str::<Solution>(&from_file(solution).await?)?;
                let output = client.submit_solution(solution).await?;
                print!("{}", output);
            }
            Commands::SolutionOutcome { solution_hash } => {
                let output = client.solution_outcome(&solution_hash.0).await?;
                print!("{:#?}", output);
            }
            Commands::GetPredicate {
                contract,
                predicate,
            } => {
                let address = PredicateAddress {
                    contract,
                    predicate,
                };
                let output = client.get_predicate(&address).await?;
                print!("{}", serde_json::to_string(&output)?);
            }
            Commands::GetContract { address } => {
                let output = client.get_contract(&address).await?;
                print!("{}", serde_json::to_string(&output)?);
            }
            Commands::ListContracts { time_range, page } => {
                let time_range = time_range.map(|time_range| {
                    Duration::from_secs(time_range.start)..Duration::from_secs(time_range.end)
                });
                let output = client.list_contracts(time_range, page).await?;
                print!("{}", serde_json::to_string(&output)?);
            }
            Commands::ListSolutionsPool { page } => {
                let output = client.list_solutions_pool(page).await?;
                print!("{}", serde_json::to_string(&output)?);
            }
            Commands::ListWinningBlocks { time_range, page } => {
                let time_range = time_range.map(|time_range| {
                    Duration::from_secs(time_range.start)..Duration::from_secs(time_range.end)
                });
                let output = client.list_winning_blocks(time_range, page).await?;
                print!("{}", serde_json::to_string(&output)?);
            }
            Commands::QueryState { address, key } => {
                let output = client.query_state(&address, &key.0).await?;
                print!(
                    "{}",
                    hex::encode_upper(
                        output
                            .into_iter()
                            .flat_map(bytes_from_word)
                            .collect::<Vec<_>>()
                    )
                );
            }
        }
    }
    Ok(())
}

async fn from_file(path: PathBuf) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}

#[derive(Clone, Debug)]
struct TimeRange {
    start: u64,
    end: u64,
}

impl FromStr for TimeRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("..");
        let start = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("No start time"))?;
        let end = split.next().ok_or_else(|| anyhow::anyhow!("No end time"))?;
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
