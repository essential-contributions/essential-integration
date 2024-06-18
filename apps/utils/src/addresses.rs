use essential_types::{intent::Intent, ContentAddress, IntentAddress, Word};

pub fn get_addresses(intents: &[Intent]) -> (ContentAddress, Vec<IntentAddress>) {
    let set = essential_hash::intent_set_addr::from_intents(intents);
    let intents = intents
        .iter()
        .map(|intent| IntentAddress {
            set: set.clone(),
            intent: essential_hash::content_addr(intent),
        })
        .collect();
    (set, intents)
}

pub fn contract_hash(contract: &IntentAddress) -> [Word; 4] {
    let set_hash = essential_types::convert::word_4_from_u8_32(contract.set.0);
    let intent_hash = essential_types::convert::word_4_from_u8_32(contract.intent.0);
    let mut words = set_hash.to_vec();
    words.extend_from_slice(&intent_hash);
    let contract_hash = essential_hash::hash_words(&words);
    essential_types::convert::word_4_from_u8_32(contract_hash)
}
