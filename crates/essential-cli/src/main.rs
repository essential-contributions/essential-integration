use clap::Parser;

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
    IntentAddresses(IntentAddresses),
    SignIntentSet(SignIntentSet),
}

/// Given a path to an intent set JSON file, output a JSON list with an
/// `IntentAddress` per intent in the set.
#[derive(Debug, clap::Args)]
#[command(version, about, long_about = None)]
struct IntentAddresses {
    /// The path to the intent set (`Vec<Intent>`) serialized to JSON.
    path: std::path::PathBuf,
}

/// Given a path to an intent set JSON file, deserialize and sign the intent set
/// and output the signed intent set as JSON.
#[derive(Debug, clap::Args)]
struct SignIntentSet {
    /// The [`secp256k1::PrivateKey`] in its JSON-serialized form (e.g. `[12, 211, 1, 4, /* ..etc */]`).
    #[arg(long)]
    private_key_json: String,
    path: std::path::PathBuf,
}

fn generate_keys() {
    use essential_sign::secp256k1::{Secp256k1, rand::rngs::OsRng};
    let secp = Secp256k1::new();
    let (sk, pk) = secp.generate_keypair(&mut OsRng);
    let map: std::collections::BTreeMap<_, _> = [
        ("private", sk.secret_bytes().to_vec()),
        ("public", pk.serialize().to_vec()),
    ].into_iter().collect();
    println!("{}", serde_json::to_string(&map).unwrap())
}

fn intent_addresses(cmd: IntentAddresses) {
    let intent_set = read_intent_set(&cmd.path);
    let set_addr = essential_hash::content_addr(&intent_set);
    let intent_addrs: Vec<_> = intent_set.iter().map(|intent| {
        let intent = essential_hash::content_addr(intent);
        let set = set_addr.clone();
        essential_types::IntentAddress { set, intent }
    }).collect();
    println!("{}", serde_json::to_string(&intent_addrs).unwrap());
}

fn sign_intent_set(cmd: SignIntentSet) {
    let sk_bytes: [u8; 32] = serde_json::from_str(&cmd.private_key_json)
        .expect("failed to deserialize JSON private key to `[u8; 32]`");
    let sk = secp256k1::SecretKey::from_slice(&sk_bytes)
        .expect("failed to parse secp256k1 private key from bytes");
    let intent_set = read_intent_set(&cmd.path);
    let signed = essential_sign::sign(intent_set, sk);
    println!("{}", serde_json::to_string(&signed).unwrap());
}

fn read_intent_set(path: &std::path::Path) -> Vec<essential_types::intent::Intent> {
    use std::{fs::File, io::BufReader};
    let file = File::open(path)
        .map_err(|e| format!("failed to open file {}: {e}", path.display()))
        .unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
        .expect("failed to deserialize intent set")
}

fn main() {
    match EssentialCli::parse() {
        EssentialCli::GenerateKeys => generate_keys(),
        EssentialCli::IntentAddresses(cmd) => intent_addresses(cmd),
        EssentialCli::SignIntentSet(cmd) => sign_intent_set(cmd),
    }
}
