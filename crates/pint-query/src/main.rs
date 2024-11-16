use clap::Parser;
use essential_rest_client::node_client::EssentialNodeClient;
use essential_types::{convert::word_from_bytes, ContentAddress};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Tool to query state from an Essential node endpoint.
struct Args {
    /// The endpoint of node to bind to.
    #[arg(long)]
    node_address: String,
    /// The contract address to query, encoded as hex.
    #[arg(long)]
    contract_address: ContentAddress,
    /// The key to query, encoded as hex.
    #[arg(long)]
    key: Key,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(err) = run(args).await {
        eprintln!("Command failed because: {}", err);
    }
}

async fn run(args: Args) -> anyhow::Result<()> {
    let Args {
        node_address,
        contract_address,
        key,
    } = args;

    let node_client = EssentialNodeClient::new(node_address)?;
    let output = node_client
        .query_state(contract_address.to_owned(), key.0.to_owned())
        .await?;
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

// Should be made obsolete by https://github.com/essential-contributions/essential-base/issues/228
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
