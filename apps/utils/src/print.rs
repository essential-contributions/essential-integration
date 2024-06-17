use essential_types::{ContentAddress, IntentAddress};

pub fn print_intent_address(name: &str, address: &IntentAddress) {
    println!(
        "{}: set: {}, intent: {}",
        name,
        hex::encode_upper(address.set.0),
        hex::encode_upper(address.intent.0),
    );
}

pub fn print_set_address(name: &str, address: &ContentAddress) {
    println!("{}: set: {}", name, hex::encode_upper(address.0),);
}
