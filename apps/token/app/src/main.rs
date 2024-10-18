//! Command-line interface for interacting with the token contract.

use anyhow::bail;
use clap::{Args, Parser, Subcommand};
use essential_app_utils::compile::compile_pint_project;
use essential_rest_client::{
    builder_client::EssentialBuilderClient, node_client::EssentialNodeClient,
};
use essential_signer::Signature;
use essential_types::{convert::word_4_from_u8_32, ContentAddress, PredicateAddress, Word};
use essential_wallet::Wallet;
use std::path::PathBuf;
use token::Query;

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

#[derive(Args)]
struct Mint {
    /// The account to mint from.
    account: String,
    /// The amount of token to mint.
    amount: Word,
    /// The name of the token.
    token_name: String,
    /// The symbol of the token.
    token_symbol: String,
    /// The address of the node to connect to.
    node_api: String,
    /// The address of the builder to connect to.
    builder_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct Transfer {
    /// The account to transfer from.
    from_account: String,
    /// The account to transfer to.
    /// Hashed key as hex.
    to_account: String,
    /// The amount of token to mint.
    amount: Word,
    /// The address of the node to connect to.
    node_api: String,
    /// The address of the builder to connect to.
    builder_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct Burn {
    /// The account to burn from.
    account: String,
    /// The amount of token to mint.
    amount: Word,
    /// The address of the node to connect to.
    node_api: String,
    /// The address of the builder to connect to.
    builder_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct Balance {
    /// The account name to get the balance of.
    account: String,
    /// The address of the node to connect to.
    node_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct ExternalBalance {
    /// The account hashed public key to get the balance of.
    /// Encoded as hex.
    account: String,
    /// The address of the node to connect to.
    node_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    Mint(Mint),
    Burn(Burn),
    Transfer(Transfer),
    Balance(Balance),
    ExternalBalance(ExternalBalance),
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
    let wallet = match &command {
        Command::ExternalBalance(_) => None,
        _ => {
            let pass = rpassword::prompt_password("Enter password to unlock wallet: ")?;
            let wallet = match wallet {
                Some(path) => essential_wallet::Wallet::new(&pass, path)?,
                None => essential_wallet::Wallet::with_default_path(&pass)?,
            };
            Some(wallet)
        }
    };
    match command {
        Command::Mint(args) => {
            println!(
                "minting {} for account: {}, token name: {}, token symbol: {}",
                args.amount, args.account, args.token_name, args.token_symbol
            );
            let wallet = wallet.unwrap();
            let addr = mint(wallet, args).await?;
            println!("sent mint solution: {}", addr);
        }
        Command::Burn(args) => {
            println!("burning {} for account: {}", args.amount, args.account);
            let wallet = wallet.unwrap();
            let addr = burn(wallet, args).await?;
            println!("sent burn solution: {}", addr);
        }
        Command::Transfer(args) => {
            println!(
                "transferring {} from account: {} to account: {}",
                args.amount, args.from_account, args.to_account
            );
            let wallet = wallet.unwrap();
            let addr = transfer(wallet, args).await?;
            println!("sent transfer solution: {}", addr);
        }
        Command::Balance(args) => {
            let Balance {
                account,
                node_api,
                pint_directory,
            } = args;
            println!("getting balance for account: {}", account);
            let mut wallet = wallet.unwrap();
            let hashed_key = hash_key(&mut wallet, &account);
            let balance = get_balance(hashed_key, node_api, pint_directory).await?;
            println!("balance is {}", balance);
        }
        Command::ExternalBalance(args) => {
            let ExternalBalance {
                account,
                node_api,
                pint_directory,
            } = args;
            println!("getting balance for account: {}", account);
            let hashed_key = word_4_from_u8_32(
                hex::decode(account)?
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("To key too large"))?,
            );
            let balance = get_balance(hashed_key, node_api, pint_directory).await?;
            println!("balance is {}", balance);
        }
    }
    Ok(())
}

/// Hashes the public key for an account.
fn hash_key(wallet: &mut Wallet, account_name: &str) -> [Word; 4] {
    let public_key = wallet.get_public_key(account_name).unwrap();
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        panic!("Invalid public key")
    };
    let encoded = essential_sign::encode::public_key(&public_key);
    word_4_from_u8_32(essential_hash::hash_words(&encoded))
}

async fn mint(mut wallet: Wallet, args: Mint) -> anyhow::Result<ContentAddress> {
    let Mint {
        account,
        amount,
        token_name,
        token_symbol,
        node_api,
        builder_api,
        pint_directory,
    } = args;
    let address = compile_address(pint_directory).await?;
    let hashed_key = hash_key(&mut wallet, &account);
    let node = EssentialNodeClient::new(node_api)?;
    let builder = EssentialBuilderClient::new(builder_api)?;

    let nonce_key = token::nonce_key(hashed_key);
    let nonce = node
        .query_state(address.contract.clone(), nonce_key)
        .await?;
    let init = token::mint::Init {
        hashed_key,
        amount,
        decimals: 18,
        nonce: token::Query(nonce),
    };
    let to_sign = token::mint::data_to_sign(init)?;
    let sig = wallet.sign_words(&to_sign.to_words(), &account)?;
    let Signature::Secp256k1(sig) = sig else {
        bail!("Invalid signature")
    };
    let balance_key = token::balance_key(hashed_key);
    let balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    let build_solution = token::mint::BuildSolution {
        new_nonce: to_sign.new_nonce,
        current_balance: Query(balance),
        hashed_key,
        amount: to_sign.amount,
        decimals: to_sign.decimals,
        signature: sig,
        token_name,
        token_symbol,
    };
    let solution = token::mint::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca)
}

async fn burn(mut wallet: Wallet, args: Burn) -> anyhow::Result<ContentAddress> {
    let Burn {
        account,
        amount,
        node_api,
        builder_api,
        pint_directory,
    } = args;
    let address = compile_address(pint_directory).await?;
    let hashed_key = hash_key(&mut wallet, &account);
    let node = EssentialNodeClient::new(node_api)?;
    let builder = EssentialBuilderClient::new(builder_api)?;

    let nonce_key = token::nonce_key(hashed_key);
    let nonce = node
        .query_state(address.contract.clone(), nonce_key)
        .await?;
    let init = token::burn::Init {
        hashed_key,
        amount,
        nonce: token::Query(nonce),
    };
    let to_sign = token::burn::data_to_sign(init)?;
    let sig = wallet.sign_words(&to_sign.to_words(), &account)?;
    let Signature::Secp256k1(sig) = sig else {
        bail!("Invalid signature")
    };
    let balance_key = token::balance_key(hashed_key);
    let balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    let build_solution = token::burn::BuildSolution {
        new_nonce: to_sign.new_nonce,
        current_balance: Query(balance),
        hashed_key,
        amount: to_sign.amount,
        signature: sig,
    };
    let solution = token::burn::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca)
}

async fn transfer(mut wallet: Wallet, args: Transfer) -> anyhow::Result<ContentAddress> {
    let Transfer {
        amount,
        node_api,
        builder_api,
        pint_directory,
        from_account,
        to_account,
    } = args;
    let address = compile_address(pint_directory).await?;
    let hashed_from_key = hash_key(&mut wallet, &from_account);
    let hashed_to_key = word_4_from_u8_32(
        hex::decode(to_account)?
            .try_into()
            .map_err(|_| anyhow::anyhow!("To key too large"))?,
    );
    let node = EssentialNodeClient::new(node_api)?;
    let builder = EssentialBuilderClient::new(builder_api)?;

    let nonce_key = token::nonce_key(hashed_from_key);
    let nonce = node
        .query_state(address.contract.clone(), nonce_key)
        .await?;
    let init = token::transfer::Init {
        amount,
        nonce: token::Query(nonce),
        hashed_from_key,
        hashed_to_key,
    };
    let to_sign = token::transfer::data_to_sign(init)?;
    let sig = wallet.sign_words(&to_sign.to_words(), &from_account)?;
    let Signature::Secp256k1(sig) = sig else {
        bail!("Invalid signature")
    };
    let balance_key = token::balance_key(hashed_from_key);
    let from_balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    let balance_key = token::balance_key(hashed_to_key);
    let to_balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    let build_solution = token::transfer::BuildSolution {
        new_nonce: to_sign.new_nonce,
        current_from_balance: Query(from_balance),
        current_to_balance: Query(to_balance),
        hashed_from_key,
        hashed_to_key,
        amount: to_sign.amount,
        signature: sig,
    };
    let solution = token::transfer::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca)
}

async fn get_balance(
    hashed_key: [Word; 4],
    node_api: String,
    pint_directory: PathBuf,
) -> anyhow::Result<Word> {
    let address = compile_address(pint_directory).await?;
    let node = EssentialNodeClient::new(node_api)?;

    let balance_key = token::balance_key(hashed_key);
    let balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    token::balance(Query(balance))
}

/// Compiles the contract and returns its address.
async fn compile_address(pint_directory: PathBuf) -> Result<PredicateAddress, anyhow::Error> {
    let counter = compile_pint_project(pint_directory).await?;
    let contract_address = essential_hash::contract_addr::from_contract(&counter);
    let predicate_address = essential_hash::content_addr(&counter.predicates[0]);
    let predicate_address = PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };
    Ok(predicate_address)
}
