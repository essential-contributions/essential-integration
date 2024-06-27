use std::path::PathBuf;

use clap::{Parser, Subcommand};
use essential_deploy_contract::{deploy_bytes, deploy_signed_bytes};
use tokio::io::{AsyncReadExt, BufReader};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
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
        account: String,
    },
    DeploySigned {
        /// The address of the server to connect to.
        server: String,
        /// The path to the signed contract to deploy.
        /// Serialized as json.
        signed_contract: PathBuf,
    },
    Deploy {
        /// Set the path to the wallet directory.
        /// If not set then a sensible default will be used (like ~/.essential-wallet).
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// The address of the server to connect to.
        server: String,
        /// The name of the account to deploy the app with.
        account: String,
        /// The path to the unsigned contract to deploy.
        /// Serialized as json.
        contract: PathBuf,
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
        Command::DeploySigned {
            server,
            signed_contract,
        } => {
            let signed_contract = read_bytes(signed_contract).await?;
            let addr = deploy_signed_bytes(server, signed_contract).await?;
            println!("Deployed signed contract to: {}", addr);
        }
        Command::Deploy {
            path,
            server,
            account,
            contract,
        } => {
            let mut wallet = get_wallet(path)?;
            let contract = read_bytes(contract).await?;
            let addr = deploy_bytes(server, &account, &mut wallet, contract).await?;
            println!("Deployed contract to: {}", addr);
        }
    }

    Ok(())
}

fn get_wallet(path: Option<PathBuf>) -> anyhow::Result<essential_wallet::Wallet> {
    let pass = rpassword::prompt_password("Enter password to unlock wallet: ")?;
    let wallet = match path {
        Some(path) => essential_wallet::Wallet::new(&pass, path)?,
        None => essential_wallet::Wallet::with_default_path(&pass)?,
    };
    Ok(wallet)
}

async fn read_bytes(path: PathBuf) -> anyhow::Result<Vec<u8>> {
    let file = tokio::fs::File::open(path).await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;
    Ok(bytes)
}
