use std::path::PathBuf;

use anyhow::bail;
use clap::{Args, Parser, Subcommand};
use nft_front_end::{compile_addresses, deploy_app, print_addresses, update_addresses, Nft};

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
    UpdateAddresses {
        /// The directory containing the pint files.
        pint_directory: PathBuf,
    },
    Mint {
        #[command(flatten)]
        server: ServerName,
        /// The hash of the token to mint.
        /// Encoded as a hex string of 32 bytes.
        hash: String,
    },
    DoIOwn {
        #[command(flatten)]
        server: ServerName,
        /// The hash of the token to check ownership of.
        /// Encoded as a hex string of 32 bytes.
        hash: String,
    },
    Transfer {
        #[command(flatten)]
        server: ServerName,
        /// The hash of the token to transfer.
        /// Encoded as a hex string of 32 bytes.
        hash: String,
        /// The account to transfer the token to.
        to: String,
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
    if let Err(e) = run(args).await {
        eprintln!("Command failed because: {}", e);
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    match &cli.command {
        Command::PrintAddresses { pint_directory } => {
            let deployed_intents = compile_addresses(pint_directory.clone()).await?;
            print_addresses(&deployed_intents);
            return Ok(());
        }
        Command::UpdateAddresses { pint_directory } => {
            update_addresses(pint_directory.clone()).await?;
            return Ok(());
        }
        _ => (),
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
            hash,
        } => {
            let deployed_intents = compile_addresses(pint_directory).await?;
            let mut nft = Nft::new(server, deployed_intents, wallet)?;
            let Ok(token): Result<[u8; 32], _> =
                essential_signer::decode_str(hash.clone(), essential_signer::Encoding::Hex)?
                    .try_into()
            else {
                bail!("Invalid hash: {}", hash);
            };
            nft.mint(&account, token).await?;
            println!("Minted token: {}", hash);
        }
        Command::DoIOwn {
            server:
                ServerName {
                    server,
                    account,
                    pint_directory,
                },
            hash,
        } => {
            let deployed_intents = compile_addresses(pint_directory).await?;
            let mut nft = Nft::new(server, deployed_intents, wallet)?;
            let Ok(token): Result<[u8; 32], _> =
                essential_signer::decode_str(hash.clone(), essential_signer::Encoding::Hex)?
                    .try_into()
            else {
                bail!("Invalid hash: {}", hash);
            };
            let owned = nft.do_i_own(&account, token).await?;
            if owned {
                println!("You own token: {}", hash);
            } else {
                println!("You do not own token: {}", hash);
            }
        }
        Command::Transfer {
            server:
                ServerName {
                    server,
                    account,
                    pint_directory,
                },
            hash,
            to,
        } => {
            let deployed_intents = compile_addresses(pint_directory).await?;
            let mut nft = Nft::new(server, deployed_intents, wallet)?;
            let Ok(token): Result<[u8; 32], _> =
                essential_signer::decode_str(hash.clone(), essential_signer::Encoding::Hex)?
                    .try_into()
            else {
                bail!("Invalid hash: {}", hash);
            };
            nft.transfer(&account, &to, token).await?;
            println!("Transferred token: {} to {}", hash, to);
        }
        Command::PrintAddresses { .. } => unreachable!(),
        Command::UpdateAddresses { .. } => unreachable!(),
    }
    Ok(())
}
