use clap::{Parser, Subcommand};
use essential_rest_client::EssentialClient;
use essential_types::{intent::Intent, solution::Solution};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, BufReader};

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
    CreateAccount {
        /// Set the path to the wallet directory.
        /// If not set then a sensible default will be used (like ~/.essential-wallet).
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// The name of the account to create.
        #[arg(long)]
        account: String,
    },
    DeployAndCheck {
        /// Set the path to the wallet directory.
        /// If not set then a sensible default will be used (like ~/.essential-wallet).
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// The name of the account to deploy the app with.
        #[arg(long)]
        account: String,
        /// The address of the server to connect to.
        #[arg(long)]
        server: String,
        /// Path to compiled intents.
        #[arg(long)]
        intents: PathBuf,
        /// Solution to check.
        #[arg(long)]
        solution: String,
    },
    Check {
        /// Solution to check.
        #[arg(long)]
        solution: String,
        /// The address of the server to connect to.
        #[arg(long)]
        server: String,
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
        Command::CreateAccount { account, path } => {
            let mut wallet = get_wallet(path)?;
            wallet.new_key_pair(&account, essential_wallet::Scheme::Secp256k1)?;
            println!("Created account: {}", account);
        }
        Command::DeployAndCheck {
            path,
            account,
            server,
            intents,
            solution,
        } => {
            let mut wallet = get_wallet(path)?;
            let client = EssentialClient::new(server)?;
            let intents = sign_and_deploy_intents(&intents, &mut wallet, &account, &client).await?;
            let solution: Solution = serde_json::from_str(&solution)?;
            let output = client.check_solution_with_data(solution, intents).await?;
            println!("{}", serde_json::to_string(&output)?);
        }
        Command::Check { solution, server } => {
            let client = EssentialClient::new(server)?;
            let solution: Solution = serde_json::from_str(&solution)?;
            let output = client.check_solution(solution).await?;
            println!("{}", serde_json::to_string(&output)?);
        }
    }
    Ok(())
}

// TODO: triplicate in `essential-deploy-intent` and `essential-dry-run` (local server)
fn get_wallet(path: Option<PathBuf>) -> anyhow::Result<essential_wallet::Wallet> {
    let pass = rpassword::prompt_password("Enter password to unlock wallet: ")?;
    let wallet = match path {
        Some(path) => essential_wallet::Wallet::new(&pass, path)?,
        None => essential_wallet::Wallet::with_default_path(&pass)?,
    };
    Ok(wallet)
}

async fn sign_and_deploy_intents(
    intents_path: &Path,
    wallet: &mut essential_wallet::Wallet,
    account: &str,
    client: &essential_rest_client::EssentialClient,
) -> anyhow::Result<Vec<Intent>> {
    let mut intents: Vec<Intent> = vec![];
    for intent in intents_path.read_dir()? {
        let name = intent?.file_name();
        let name = name
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("invalid file name"))?;
        let path = intents_path.join(name);

        let intent_set = read_intents(&path).await?;
        let signed_set = wallet.sign_intent_set(intent_set.clone(), account)?;
        client.deploy_intent_set(signed_set).await?;

        intents.extend(intent_set);
    }
    Ok(intents)
}

async fn read_intents(path: &Path) -> anyhow::Result<Vec<Intent>> {
    let file = tokio::fs::File::open(path).await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;
    Ok(serde_json::from_slice::<Vec<Intent>>(&bytes)?)
}
