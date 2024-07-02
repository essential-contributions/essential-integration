use essential_types::{ContentAddress, PredicateAddress};

pub fn print_predicate_address(name: &str, address: &PredicateAddress) {
    println!(
        "{}: contract:{}, predicate: {}",
        name,
        hex::encode_upper(address.contract.0),
        hex::encode_upper(address.predicate.0),
    );
}

pub fn print_contract_address(name: &str, address: &ContentAddress) {
    println!("{}: contract:{}", name, hex::encode_upper(address.0),);
}
