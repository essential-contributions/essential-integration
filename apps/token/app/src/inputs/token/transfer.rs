use essential_app_utils::inputs::{index_mutation, Int, B256};
use essential_types::solution::Mutation;

pub struct TransientData {
    pub key: B256,
    pub to: B256,
    pub amount: Int,
}

impl TransientData {
    pub fn encode(&self) -> Vec<Mutation> {
        let Self { key, to, amount } = self;
        let mutations = vec![
            index_mutation(0, key.to_value()),
            index_mutation(1, to.to_value()),
            index_mutation(2, amount.to_value()),
        ];

        mutations
    }
}

pub type DecVars = super::DecVars;
