use app_utils::inputs::{index_mutation, B256};
use essential_types::{solution::Mutation, IntentAddress};

pub mod burn;
pub mod mint;
pub mod transfer;
#[allow(dead_code)]
pub mod transfer_from;

pub struct TransientData {
    pub token_address: IntentAddress,
}

impl TransientData {
    pub fn encode(&self) -> Vec<Mutation> {
        let Self { token_address } = self;
        let mutations = vec![
            index_mutation(0, B256::from(token_address.set.0).to_value()),
            index_mutation(1, B256::from(token_address.intent.0).to_value()),
        ];

        mutations
    }
}
