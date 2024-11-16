use clap::Parser;
use essential_rest_client::node_client::EssentialNodeClient;
use essential_types::{convert::word_from_bytes, ContentAddress};
use pint_abi::types::{ContractABI, TypeABI};
use pint_manifest::ManifestFile;
use std::{fs::read_dir, path::PathBuf, str::FromStr};

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
    /// The key name to query.
    #[arg(long)]
    key: Option<String>,
    /// The path to the package manifest.
    ///
    /// If not provided, the current directory is checked and then each parent
    /// recursively until a manifest is found.
    #[arg(long)]
    manifest_path: Option<PathBuf>,
    /// The key to query, encoded as hex.
    #[arg(long)]
    key_hex: Option<Key>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(err) = run(args).await {
        let bold = Style::new().bold();
        eprintln!("{}Error:{} {err:?}", bold.render(), bold.render_reset());
    }
}

async fn run(args: Args) -> anyhow::Result<()> {
    let Args {
        node_address,
        contract_address,
        key,
        manifest_path,
        key_hex,
    } = args;

    let query_key = match (key, key_hex) {
        (Some(_), Some(_)) => {
            anyhow::bail!("Only one of key name or key hex should be provided.")
        }
        (Some(key_name), None) => {
            let manifest_path = match manifest_path {
                Some(path) => path,
                None => match find_file(std::env::current_dir()?, ManifestFile::FILE_NAME) {
                    Some(path) => path,
                    None => anyhow::bail!(
                        "Pint manifest could not be found in current or parent directories."
                    ),
                },
            };
            let manifest = ManifestFile::from_path(&manifest_path)?;
            let contract_abi = get_contract_abi(manifest)?;
            get_key_from_abi(contract_abi, key_name)?
        }
        (None, Some(key_hex)) => key_hex.0,
        (None, None) => anyhow::bail!("At least one of key name or key hex should be provided."),
    };

    let node_client = EssentialNodeClient::new(node_address)?;
    let output = node_client
        .query_state(contract_address.to_owned(), query_key)
        .await?;
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

// FIXME: Should be made obsolete by https://github.com/essential-contributions/essential-base/issues/228
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

/// Find the file within the current directory or parent directories with the given name.
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

/// Given a `ManifestFile`, return the `ContractABI` of the already compiled contract.
fn get_contract_abi(manifest: ManifestFile) -> anyhow::Result<ContractABI> {
    let out_dir = manifest.out_dir();
    for entry in read_dir(out_dir.clone())? {
        let entry = entry?;
        let entry_file_name = entry.file_name();
        let name = entry_file_name
            .to_str()
            .expect("file name should be convertible to string");
        if name.ends_with("-abi.json") {
            return pint_abi::from_path(&entry.path()).map_err(|err| anyhow::anyhow!("{}", err));
        }
    }
    Err(anyhow::anyhow!(
        "Could not find *-abi.json file in {:?}",
        out_dir
    ))
}

/// Given a `ContractABI` and a key name, return the `Key`.
fn get_key_from_abi(abi: ContractABI, key_name: String) -> anyhow::Result<essential_types::Key> {
    abi.storage
        .iter()
        .enumerate()
        .find(|(_, storage)| storage.name == key_name)
        .map(|(index, storage)| {
            if matches!(
                storage.ty,
                TypeABI::Bool | TypeABI::Int | TypeABI::Real | TypeABI::String | TypeABI::B256
            ) {
                let key = vec![index.try_into()?];
                Ok(key)
            } else {
                // FIXME: support complex types
                Err(anyhow::anyhow!(
            "Querying key of type {:?} with name is not supported. Try providing a key_hex.",
                    storage.ty
                ))
            }
        })
        .unwrap_or_else(|| {
            Err(anyhow::anyhow!(
                "Could not find key {key_name} in given ABI"
            ))
        })
}
