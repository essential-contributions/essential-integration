use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use token::actions::{compile_addresses, print_addresses};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Essential wallet directory.
    /// If not set then a sensible default will be used (like ~/.essential-wallet).
    #[arg(short, long)]
    wallet: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    CreateAccount {
        /// The name of the account to create.
        account: String,
    },
    PrintAddresses {
        /// The directory containing the pint files.
        pint_directory: PathBuf,
    },
    // TODO: add UpdateAddresses once Auth is in
    DeployApp {
        #[command(flatten)]
        server: ServerName,
    },
    Init {
        #[command(flatten)]
        server: ServerName,
        /// The name of the token encoded as a hex string of 32 bytes.
        hash: String,
    },
    Mint {
        #[command(flatten)]
        server: ServerName,
        amount: u64,
    },
    Burn {
        #[command(flatten)]
        server: ServerName,
        amount: u64,
    },
    Transfer {
        #[command(flatten)]
        server: ServerName,
        /// The account to transfer the token to.
        to: String,
    },
    Balance {
        #[command(flatten)]
        server: ServerName,
    },
}

#[derive(Args)]
struct ServerName {
    /// The address of the server to connect to.
    server: String,
    /// The name of the account to deploy the app with.
    account: String,
    /// The directory containing the pint files.
    pint_directory: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if let Err(err) = run(args).await {
        eprintln!("Command failed because: {}", err);
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let Cli { wallet, command } = cli;
    if let Command::PrintAddresses { pint_directory } = &command {
        let deployed_intents = compile_addresses(pint_directory.clone()).await?;
        print_addresses(&deployed_intents);
        return Ok(());
    }
    let pass = rpassword::prompt_password("Enter password to unlock wallet: ")?;
    let mut _wallet = match wallet {
        Some(path) => essential_wallet::Wallet::new(&pass, path)?,
        None => essential_wallet::Wallet::with_default_path(&pass)?,
    };

    // TODO: implement the rest of the commands

    Ok(())
}
