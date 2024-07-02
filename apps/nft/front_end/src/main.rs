use app_utils::cli::ServerName;
use clap::{Parser, Subcommand};
use essential_types::Word;
use nft::{compile_addresses, deploy_app, print_addresses, Nft};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Set the path to the wallet directory.
    /// If not set then a sensible default will be used (like ~/.essential-wallet).
    #[arg(short, long)]
    path: Option<PathBuf>,
    /// Select a subcommand to run
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    CreateAccount {
        /// The name of the account to create.
        account: String,
    },
    DeployApp {
        #[command(flatten)]
        server: ServerName,
    },
    PrintAddresses {
        /// The directory containing the pint files.
        pint_directory: PathBuf,
    },
    Mint {
        #[command(flatten)]
        server: ServerName,
        /// The token id
        token: Word,
    },
    DoIOwn {
        #[command(flatten)]
        server: ServerName,
        /// The token id
        token: Word,
    },
    Transfer {
        #[command(flatten)]
        server: ServerName,
        /// The token id
        token: Word,
        /// The account to transfer the token to.
        to: String,
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
    if let Command::PrintAddresses { pint_directory } = &cli.command {
        let deployed_predicates = compile_addresses(pint_directory.clone()).await?;
        print_addresses(&deployed_predicates);
        return Ok(());
    }
    let pass = rpassword::prompt_password("Enter password to unlock wallet: ")?;
    let mut wallet = match cli.path {
        Some(path) => essential_wallet::Wallet::new(&pass, path)?,
        None => essential_wallet::Wallet::with_default_path(&pass)?,
    };

    match cli.command {
        Command::CreateAccount { account } => {
            wallet.new_key_pair(&account, essential_wallet::Scheme::Secp256k1)?;
            println!("Created account: {}", account);
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
            token,
        } => {
            let deployed_predicates = compile_addresses(pint_directory).await?;
            let mut nft = Nft::new(server, deployed_predicates, wallet)?;
            nft.mint(&account, token).await?;
            println!("Minted token: {}", token);
        }
        Command::DoIOwn {
            server:
                ServerName {
                    server,
                    account,
                    pint_directory,
                },
            token,
        } => {
            let deployed_predicates = compile_addresses(pint_directory).await?;
            let mut nft = Nft::new(server, deployed_predicates, wallet)?;
            let owned = nft.do_i_own(&account, token).await?;
            if owned {
                println!("You own token: {}", token);
            } else {
                println!("You do not own token: {}", token);
            }
        }
        Command::Transfer {
            server:
                ServerName {
                    server,
                    account,
                    pint_directory,
                },
            token,
            to,
        } => {
            let deployed_predicates = compile_addresses(pint_directory).await?;
            let mut nft = Nft::new(server, deployed_predicates, wallet)?;
            nft.transfer(&account, &to, token).await?;
            println!("Transferred token: {} to {}", token, to);
        }
        Command::PrintAddresses { .. } => unreachable!(),
    }
    Ok(())
}
