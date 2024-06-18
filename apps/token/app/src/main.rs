use app_utils::cli::ServerName;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use token::{
    actions::{compile_addresses, deploy_app, print_addresses},
    token::Token,
};

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
        name: String,
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
    Mint {
        #[command(flatten)]
        server: ServerName,
        /// The amount of token to mint.
        amount: u64,
    },
    Burn {
        #[command(flatten)]
        server: ServerName,
        /// The amount of token to burn.
        amount: u64,
    },
    Transfer {
        #[command(flatten)]
        server: ServerName,
        /// The amount of transfer.
        amount: u64,
        /// The account to transfer the token to.
        to: String,
    },
    Balance {
        #[command(flatten)]
        server: ServerName,
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
    let Cli { wallet, command } = cli;
    if let Command::PrintAddresses { pint_directory } = &command {
        let deployed_intents = compile_addresses(pint_directory.clone()).await?;
        print_addresses(&deployed_intents);
        return Ok(());
    }
    let pass = rpassword::prompt_password("Enter password to unlock wallet: ")?;
    let mut wallet = match wallet {
        Some(path) => essential_wallet::Wallet::new(&pass, path)?,
        None => essential_wallet::Wallet::with_default_path(&pass)?,
    };
    match command {
        Command::CreateAccount { name } => {
            wallet.new_key_pair(&name, essential_wallet::Scheme::Secp256k1)?;
            println!("Created account: {}", name);
        }
        Command::DeployApp {
            server:
                ServerName {
                    server,
                    account,
                    pint_directory,
                },
        } => {
            let addrs = deploy_app(server, &mut wallet, &account, pint_directory).await?;
            print_addresses(&addrs);
        }
        Command::Mint {
            server:
                ServerName {
                    server,
                    account,
                    pint_directory,
                },
            amount,
        } => {
            let deployed_intents = compile_addresses(pint_directory).await?;
            let mut token = Token::new(server, deployed_intents, wallet)?;
            token.mint(&account, amount.try_into().unwrap()).await?;
            println!("Minted {} of token to {}", amount, account);
        }
        Command::Burn {
            server:
                ServerName {
                    server,
                    account,
                    pint_directory,
                },
            amount,
        } => {
            let deployed_intents = compile_addresses(pint_directory).await?;
            let mut token = Token::new(server, deployed_intents, wallet)?;
            token.burn(&account, amount.try_into().unwrap()).await?;
            println!("Burned {} of token from {}", amount, account);
        }
        Command::Transfer {
            server:
                ServerName {
                    server,
                    account,
                    pint_directory,
                },
            amount,
            to,
        } => {
            let deployed_intents = compile_addresses(pint_directory).await?;
            let mut token = Token::new(server, deployed_intents, wallet)?;
            token
                .transfer(&account, &to, amount.try_into().unwrap())
                .await?;
            println!("Transferred {} of token from {} to {}", amount, account, to);
        }
        Command::Balance {
            server:
                ServerName {
                    server,
                    account,
                    pint_directory,
                },
        } => {
            let deployed_intents = compile_addresses(pint_directory).await?;
            let mut token = Token::new(server, deployed_intents, wallet)?;
            let balance = token.balance(&account).await?.unwrap_or_default();
            println!("Account {} has balance: {}", account, balance);
        }
        Command::PrintAddresses { .. } => unreachable!(),
    }
    Ok(())
}
