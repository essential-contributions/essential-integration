use clap::Parser;
use essential_rest_client::builder_client::EssentialBuilderClient;
use essential_types::contract::Contract;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Tool to deploy a contract to a Essential builder endpoint.
struct Args {
    /// The endpoint of builder to bind to.
    #[arg(long)]
    builder_address: String,
    /// Path to the contract file as a json `Contract`.
    #[arg(long)]
    contract: PathBuf,
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
        builder_address,
        contract,
    } = args;

    let builder_client = EssentialBuilderClient::new(builder_address)?;
    let contract = serde_json::from_str::<Contract>(&from_file(contract).await?)?;
    let output = builder_client.deploy_contract(&contract).await?;
    println!("{}", output);
    Ok(())
}

/// Read file contents to a string.
async fn from_file(path: PathBuf) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}
