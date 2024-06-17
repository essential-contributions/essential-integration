use crate::read::read_pint_file;
use essential_types::{intent::Intent, ContentAddress, IntentAddress, Word};
use std::path::PathBuf;

pub async fn replace_intent_address(
    pint_directory: PathBuf,
    name: &str,
    address: &IntentAddress,
) -> anyhow::Result<()> {
    let mut intent = read_pint_file(pint_directory.clone(), name).await?;
    let set =
        find_address(&intent, 1).ok_or_else(|| anyhow::anyhow!("{} missing set address", name))?;
    intent = intent.replace(set, &hex::encode_upper(address.set.0));
    let intent_addr = find_address(&intent, 2)
        .ok_or_else(|| anyhow::anyhow!("{} missing intent address", name))?;
    intent = intent.replace(intent_addr, &hex::encode_upper(address.intent.0));
    tokio::fs::write(pint_directory.join(name), intent).await?;
    Ok(())
}

pub async fn replace_set_address(
    pint_directory: PathBuf,
    name: &str,
    address: &ContentAddress,
) -> anyhow::Result<()> {
    let mut intent = read_pint_file(pint_directory.clone(), name).await?;
    let set =
        find_address(&intent, 1).ok_or_else(|| anyhow::anyhow!("{} missing set address", name))?;
    intent = intent.replace(set, &hex::encode_upper(address.0));
    tokio::fs::write(pint_directory.join(name), intent).await?;
    Ok(())
}

pub fn find_address(intent: &str, num: usize) -> Option<&str> {
    intent
        .split("0x")
        .nth(num)
        .and_then(|s| s.split(&[' ', ')', ',']).next())
        .map(|s| s.trim())
}

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
