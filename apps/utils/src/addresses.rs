use essential_types::{contract::Contract, ContentAddress, PredicateAddress, Word};

pub fn get_addresses(contract: &Contract) -> (ContentAddress, Vec<PredicateAddress>) {
    let contract_addr = essential_hash::contract_addr::from_contract(contract);
    let predicates = contract
        .iter()
        .map(|predicate| PredicateAddress {
            contract: contract_addr.clone(),
            predicate: essential_hash::content_addr(predicate),
        })
        .collect();
    (contract_addr, predicates)
}

pub fn contract_hash(contract: &PredicateAddress) -> [Word; 4] {
    let set_hash = essential_types::convert::word_4_from_u8_32(contract.contract.0);
    let predicate_hash = essential_types::convert::word_4_from_u8_32(contract.predicate.0);
    let mut words = set_hash.to_vec();
    words.extend_from_slice(&predicate_hash);
    let contract_hash = essential_hash::hash_words(&words);
    essential_types::convert::word_4_from_u8_32(contract_hash)
}
