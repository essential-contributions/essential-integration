use std::path::PathBuf;

use clap::{Parser, Subcommand};
use essential_rest_client::node_client::EssentialNodeClient;
use essential_types::{convert::word_4_from_u8_32, Word};
use essential_wallet::Wallet;

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
    Mint {
        /// The amount of token to mint.
        amount: u64,
        /// The name of the token.
        token_name: String,
        /// The symbol of the token.
        token_symbol: String,
        /// The address of the node to connect to.
        node_api: String,
        /// The address of the builder to connect to.
        builder_api: String,
    },
    Burn {
        /// The amount of token to burn.
        amount: u64,
    },
    Transfer {
        /// The amount of transfer.
        amount: u64,
        /// The account to transfer the token to.
        to: String,
    },
    Balance {},
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
    let pass = rpassword::prompt_password("Enter password to unlock wallet: ")?;
    let mut wallet = match wallet {
        Some(path) => essential_wallet::Wallet::new(&pass, path)?,
        None => essential_wallet::Wallet::with_default_path(&pass)?,
    };
    match command {
        Command::Mint {
            amount,
            token_name,
            token_symbol,
            node_api,
            builder_api,
        } => {
            println!("mint {} {} {}", amount, token_name, token_symbol);
        }
        Command::Burn { amount } => {
            println!("burn {}", amount);
        }
        Command::Transfer { amount, to } => {
            println!("transfer {} to {}", amount, to);
        }
        Command::Balance {} => {
            println!("balance");
        }
    }
    Ok(())
}

fn hash_key(wallet: &mut Wallet, account_name: &str) -> [Word; 4] {
    let public_key = wallet.get_public_key(account_name).unwrap();
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        panic!("Invalid public key")
    };
    let encoded = essential_sign::encode::public_key(&public_key);
    word_4_from_u8_32(essential_hash::hash_words(&encoded))
}

fn mint(
    wallet: Wallet,
    account: &str,
    amount: Word,
    token_name: &str,
    token_symbol: &str,
    node_api: String,
    builder_api: String,
) -> anyhow::Result<()> {
    // let key = hash_key(&mut wallet, account);
    // let node = EssentialNodeClient::new(node_api)?;
    // let nonce = node.query_state(contract_ca, key)
    // let account = token::mint::Init {
    //     hashed_key: key,
    //     amount,
    //     decimals: 18,
    //     nonce: token::Query(nonce),
    // };
    Ok(())
}
