use app_utils::inputs::B256;
use essential_types::{solution::Mutation, PredicateAddress};

pub mod burn;
pub mod mint;
pub mod transfer;
#[allow(dead_code)]
pub mod transfer_from;

pub struct TransientData {
    pub token_address: PredicateAddress,
}

impl TransientData {
    pub fn encode(&self) -> Vec<Mutation> {
        let Self { token_address } = self;
        let mutations = vec![
            Mutation {
                key: vec![0, 0],
                value: B256::from(token_address.contract.0).to_value(),
            },
            Mutation {
                key: vec![0, 1],
                value: B256::from(token_address.predicate.0).to_value(),
            },
        ];

        mutations
    }
}
