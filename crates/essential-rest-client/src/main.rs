use clap::{Args, Parser, Subcommand, ValueEnum};
use essential_rest_client::EssentialClient;
use essential_server_types::{QueryStateReads, SlotsRequest, StateReadRequestType};
use essential_types::{
    contract::{Contract, SignedContract},
    convert::{bytes_from_word, word_from_bytes},
    predicate::Predicate,
    solution::{Solution, SolutionData, SolutionDataIndex},
    ContentAddress, PredicateAddress, StateReadBytecode,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr, time::Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Essential REST Client
struct Cli {
    /// Server address to bind to. Default: "http://0.0.0.0:0"
    #[arg(default_value_t = String::from("http://0.0.0.0:0"))]
    address: String,
    #[command(subcommand)]
    commands: Commands,
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
    /// List blocks on the server.
    ListBlocks {
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
    /// Query the state of a contract by running state read programs.
    QueryStateReads {
        /// Path to the state reads file as a json `Vec<Vec<u8>>`.
        state_reads_path: PathBuf,
        #[command(flatten)]
        args: QueryStateReadsArgs,
    },
    /// Query the state of a contract by running the state read programs in a predicate.
    QueryPredicate {
        /// Path to the predicate file as a json `Predicate`.
        predicate_path: PathBuf,
        #[command(flatten)]
        args: QueryStateReadsArgs,
    },
    /// Query the state of a contract by running state read programs with a single solution data input.
    QueryInline {
        /// Path to the state reads file as a json `Vec<Vec<u8>>`.
        state_reads_path: PathBuf,
        /// Path to the solution data file as a json `SolutionData`.
        solution_data: PathBuf,
        #[command(flatten)]
        request: RequestArgs,
    },
    /// Query the state of an external contract by running state read programs.
    /// This uses an empty solution that doesn't solve anything.
    /// It only makes sense to query state that is in an external contract.
    QueryExtern {
        /// Path to the state reads file as a json `Vec<Vec<u8>>`.
        state_reads_path: PathBuf,
        #[command(flatten)]
        request: RequestArgs,
    },
}

#[derive(Args, Debug)]
struct QueryStateReadsArgs {
    /// Index of the solution data to use as an input to the state reads.
    index: SolutionDataIndex,
    /// Path to the solution file as a json `Solution`.
    solution: PathBuf,
    #[command(flatten)]
    request: RequestArgs,
}

#[derive(Args, Debug)]
struct RequestArgs {
    /// Whether to capture keys and values in the state reads.
    #[arg(value_enum)]
    capture_reads: CaptureReads,
    /// Whether to capture slots in the state reads.
    #[arg(value_enum)]
    capture_slots: CaptureSlots,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CaptureReads {
    /// Capture the keys and values in the state reads.
    Capture,
    /// Ignore the keys and values in the state reads.
    Ignore,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CaptureSlots {
    /// Capture the slots in the state reads.
    Capture,
    /// Capture only the pre slots in the state reads.
    CapturePre,
    /// Capture only the post slots in the state reads.
    CapturePost,
    /// Ignore the slots in the state reads.
    Ignore,
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
        Commands::ListBlocks { time_range, page } => {
            let time_range = time_range.map(|time_range| {
                Duration::from_secs(time_range.start)..Duration::from_secs(time_range.end)
            });
            let output = client.list_blocks(time_range, page).await?;
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
        Commands::QueryStateReads {
            state_reads_path,
            args:
                QueryStateReadsArgs {
                    index,
                    solution,
                    request,
                },
        } => {
            let state_read =
                serde_json::from_str::<StateReads>(&from_file(state_reads_path).await?)?.0;
            let solution = serde_json::from_str::<Solution>(&from_file(solution).await?)?;
            let query = QueryStateReads {
                state_read,
                index,
                solution,
                request_type: request.into(),
            };
            let output = client.query_state_reads(query).await?;
            print!("{}", serde_json::to_string(&output)?);
        }
        Commands::QueryPredicate {
            predicate_path,
            args:
                QueryStateReadsArgs {
                    index,
                    solution,
                    request,
                },
        } => {
            let predicate = serde_json::from_str::<Predicate>(&from_file(predicate_path).await?)?;
            let solution = serde_json::from_str::<Solution>(&from_file(solution).await?)?;
            let query = QueryStateReads::from_solution(solution, index, &predicate, request.into());
            let output = client.query_state_reads(query).await?;
            print!("{}", serde_json::to_string(&output)?);
        }
        Commands::QueryInline {
            state_reads_path,
            solution_data,
            request,
        } => {
            let state_read =
                serde_json::from_str::<StateReads>(&from_file(state_reads_path).await?)?.0;
            let data = serde_json::from_str::<SolutionData>(&from_file(solution_data).await?)?;
            let query = QueryStateReads::inline(state_read, data, request.into());
            let output = client.query_state_reads(query).await?;
            print!("{}", serde_json::to_string(&output)?);
        }
        Commands::QueryExtern {
            state_reads_path,
            request,
        } => {
            let state_read =
                serde_json::from_str::<StateReads>(&from_file(state_reads_path).await?)?.0;
            let query = QueryStateReads::inline_empty(state_read, request.into());
            let output = client.query_state_reads(query).await?;
            print!("{}", serde_json::to_string(&output)?);
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

#[derive(Serialize, Deserialize)]
struct StateReads(
    #[serde(
        serialize_with = "essential_types::serde::bytecode::serialize_vec",
        deserialize_with = "essential_types::serde::bytecode::deserialize_vec"
    )]
    Vec<StateReadBytecode>,
);

impl From<RequestArgs> for StateReadRequestType {
    fn from(value: RequestArgs) -> Self {
        match value {
            RequestArgs {
                capture_reads: CaptureReads::Capture,
                capture_slots: CaptureSlots::Capture,
            } => Self::All(SlotsRequest::All),
            RequestArgs {
                capture_reads: CaptureReads::Capture,
                capture_slots: CaptureSlots::CapturePre,
            } => Self::All(SlotsRequest::Pre),
            RequestArgs {
                capture_reads: CaptureReads::Capture,
                capture_slots: CaptureSlots::CapturePost,
            } => Self::All(SlotsRequest::Post),
            RequestArgs {
                capture_reads: CaptureReads::Capture,
                capture_slots: CaptureSlots::Ignore,
            } => Self::Reads,
            RequestArgs {
                capture_reads: CaptureReads::Ignore,
                capture_slots: CaptureSlots::Capture,
            } => Self::Slots(SlotsRequest::All),
            RequestArgs {
                capture_reads: CaptureReads::Ignore,
                capture_slots: CaptureSlots::CapturePre,
            } => Self::Slots(SlotsRequest::Pre),
            RequestArgs {
                capture_reads: CaptureReads::Ignore,
                capture_slots: CaptureSlots::CapturePost,
            } => Self::Slots(SlotsRequest::Post),
            RequestArgs {
                capture_reads: CaptureReads::Ignore,
                capture_slots: CaptureSlots::Ignore,
            } => panic!("Cannot have a query state reads where both reads and slots are ignored."),
        }
    }
}
