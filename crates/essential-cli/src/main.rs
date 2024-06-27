use clap::Parser;
use essential_types::{contract::Contract, PredicateAddress};

/// A small suite of tools for simplifying the Essential application devex.
#[derive(Debug, Parser)]
#[command(name = "essential")]
#[command(bin_name = "essential")]
enum EssentialCli {
    /// Generate a random `secp256k1` key pair.
    ///
    /// The keypair is output via JSON, e.g.
    /// ```json
    /// {"private":<private-key>,"public":<public-key>}
    /// ```
    GenerateKeys,
    PredicateAddresses(PredicateAddresses),
    SignPredicateSet(SignPredicateSet),
}

/// Given a path to an contract JSON file, output a JSON list with an
/// `PredicateAddress` per predicate in the set.
#[derive(Debug, clap::Args)]
#[command(version, about, long_about = None)]
struct PredicateAddresses {
    /// The path to the contract (`Vec<Predicate>`) serialized to JSON.
    path: std::path::PathBuf,
}

/// Given a path to an contract JSON file, deserialize and sign the contract
/// and output the signed contract as JSON.
#[derive(Debug, clap::Args)]
struct SignPredicateSet {
    /// The [`secp256k1::PrivateKey`] in its JSON-serialized form (e.g. `[12, 211, 1, 4, /* ..etc */]`).
    #[arg(long)]
    private_key_json: String,
    path: std::path::PathBuf,
}

fn generate_keys() {
    use essential_sign::secp256k1::{rand::rngs::OsRng, Secp256k1};
    let secp = Secp256k1::new();
    let (sk, pk) = secp.generate_keypair(&mut OsRng);
    let map: std::collections::BTreeMap<_, _> = [
        ("private", sk.secret_bytes().to_vec()),
        ("public", pk.serialize().to_vec()),
    ]
    .into_iter()
    .collect();
    println!("{}", serde_json::to_string(&map).unwrap())
}

fn predicate_addresses(cmd: PredicateAddresses) {
    let contract = read_contract(&cmd.path);
    let contract_addr = essential_hash::contract_addr::from_contract(&contract);
    let predicate_addrs: Vec<_> = contract
        .iter()
        .map(|predicate| {
            let predicate = essential_hash::content_addr(predicate);
            let contract = contract_addr.clone();
            PredicateAddress {
                contract,
                predicate,
            }
        })
        .collect();
    println!("{}", serde_json::to_string(&predicate_addrs).unwrap());
}

fn sign_contract(cmd: SignPredicateSet) {
    let sk_bytes: [u8; 32] = serde_json::from_str(&cmd.private_key_json)
        .expect("failed to deserialize JSON private key to `[u8; 32]`");
    let sk = secp256k1::SecretKey::from_slice(&sk_bytes)
        .expect("failed to parse secp256k1 private key from bytes");
    let contract = read_contract(&cmd.path);
    let signed = essential_sign::contract::sign(contract, &sk);
    println!("{}", serde_json::to_string(&signed).unwrap());
}

fn read_contract(path: &std::path::Path) -> Contract {
    use std::{fs::File, io::BufReader};
    let file = File::open(path)
        .map_err(|e| format!("failed to open file {}: {e}", path.display()))
        .unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("failed to deserialize contract")
}

fn main() {
    match EssentialCli::parse() {
        EssentialCli::GenerateKeys => generate_keys(),
        EssentialCli::PredicateAddresses(cmd) => predicate_addresses(cmd),
        EssentialCli::SignPredicateSet(cmd) => sign_contract(cmd),
    }
}
