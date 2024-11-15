use clap::Parser;
use essential_rest_client::builder_client::EssentialBuilderClient;
use essential_types::contract::Contract;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Tool to deploy contracts in a Pint project to a Essential builder endpoint.
struct Args {
    /// The endpoint of builder to bind to.
    #[arg(long)]
    builder_address: String,
    /// The path to the package manifest.
    ///
    /// If not provided, the current directory is checked and then each parent
    /// recursively until a manifest is found.
    #[arg(long)]
    manifest_path: Option<PathBuf>,
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
        manifest_path,
    } = args;
    let builder_client = EssentialBuilderClient::new(builder_address)?;
    let contract_path = find_manifest(manifest_path)?;
    let contract = serde_json::from_str::<Contract>(&from_file(contract_path).await?)?;
    let output = builder_client.deploy_contract(&contract).await?;
    println!("{}", output);
    Ok(())
}

async fn from_file(path: PathBuf) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}

// Find the file within the current directory or parent directories with the given name.
fn find_file(mut dir: PathBuf, file_name: &str) -> Option<PathBuf> {
    loop {
        let path = dir.join(file_name);
        if path.exists() {
            return Some(path);
        }
        if !dir.pop() {
            return None;
        }
    }
}

// Determine the manifest location.
fn find_manifest(path: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    const MANIFEST_FILE_NAME: &str = "pint.toml";
    match path {
        Some(path) => Ok(path),
        None => {
            let current_dir = std::env::current_dir()?;
            match find_file(current_dir, MANIFEST_FILE_NAME) {
                None => anyhow::bail!("no `pint.toml` in the current or parent directories"),
                Some(path) => Ok(path),
            }
        }
    }
}
