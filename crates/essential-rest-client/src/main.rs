use clap::{Parser, Subcommand};
use essential_rest_client::EssentialClient;
use essential_types::{
    intent::{Intent, SignedSet},
    solution::Solution,
    ContentAddress, IntentAddress, Key,
};
use std::{ops::Range, time::Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = String::from("http://0.0.0.0:0"))]
    address: String,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    DeployIntentSet {
        #[arg(long)]
        intents: String,
    },
    CheckSolution {
        #[arg(long)]
        solution: String,
    },
    CheckSolutionWithData {
        #[arg(long)]
        solution: String,
        #[arg(long)]
        intents: String,
    },
    SubmitSolution {
        #[arg(long)]
        solution: String,
    },
    SolutionOutcome {
        #[arg(long)]
        solution_hash: ContentAddress,
    },
    GetIntent {
        #[arg(long)]
        address: String,
    },
    GetIntentSet {
        #[arg(long)]
        address: ContentAddress,
    },
    ListIntentSets {
        #[arg(long, default_value(None))]
        time_range: Option<String>,
        #[arg(long, default_value(None))]
        page: Option<usize>,
    },
    ListSolutionsPool {
        #[arg(long, default_value(None))]
        page: Option<usize>,
    },
    ListWinningBlocks {
        #[arg(long, default_value(None))]
        time_range: Option<String>,
        #[arg(long, default_value(None))]
        page: Option<usize>,
    },
    QueryState {
        #[arg(long)]
        address: String,
        #[arg(long)]
        key: String,
    },
}

#[tokio::main]
async fn main() {
    let Cli { address, command } = Cli::parse();
    let client = EssentialClient::new(address).await;
    if let Some(command) = command {
        match command {
            Commands::DeployIntentSet { intents } => {
                let intents = serde_json::from_str::<SignedSet>(&intents).unwrap();
                let output = client.deploy_intent_set(intents).await.unwrap();
                print!("{}", output);
            }
            Commands::CheckSolution { solution } => {
                let solution = serde_json::from_str::<Solution>(&solution).unwrap();
                let output = client.check_solution(solution).await.unwrap();
                print!("{:#?}", output);
            }
            Commands::CheckSolutionWithData { solution, intents } => {
                let solution = serde_json::from_str::<Solution>(&solution).unwrap();
                let intents = serde_json::from_str::<Vec<Intent>>(&intents).unwrap();
                let output = client
                    .check_solution_with_data(solution, intents)
                    .await
                    .unwrap();
                print!("{:#?}", output);
            }
            Commands::SubmitSolution { solution } => {
                let solution = serde_json::from_str::<Solution>(&solution).unwrap();
                let output = client.submit_solution(solution).await.unwrap();
                print!("{}", output);
            }
            Commands::SolutionOutcome { solution_hash } => {
                let output = client.solution_outcome(&solution_hash.0).await.unwrap();
                print!("{:#?}", output);
            }
            Commands::GetIntent { address } => {
                let address = serde_json::from_str::<IntentAddress>(&address).unwrap();
                let output = client.get_intent(&address).await.unwrap();
                print!("{:#?}", output);
            }
            Commands::GetIntentSet { address } => {
                let output = client.get_intent_set(&address).await.unwrap();
                print!("{:#?}", output);
            }
            Commands::ListIntentSets { time_range, page } => {
                let time_range = time_range.map(|time_range| {
                    serde_json::de::from_str::<Range<Duration>>(&time_range).unwrap()
                });
                let output = client.list_intent_sets(time_range, page).await.unwrap();
                print!("{:#?}", output);
            }
            Commands::ListSolutionsPool { page } => {
                let output = client.list_solutions_pool(page).await.unwrap();
                print!("{:#?}", output);
            }
            Commands::ListWinningBlocks { time_range, page } => {
                let time_range = time_range.map(|time_range| {
                    serde_json::from_str::<Range<Duration>>(&time_range).unwrap()
                });
                let output = client.list_winning_blocks(time_range, page).await.unwrap();
                print!("{:#?}", output);
            }
            Commands::QueryState { address, key } => {
                let address = serde_json::from_str::<ContentAddress>(&address).unwrap();
                let key = serde_json::from_str::<Key>(&key).unwrap();
                let output = client.query_state(&address, &key).await.unwrap();
                print!("{:#?}", output);
            }
        }
    }
}
