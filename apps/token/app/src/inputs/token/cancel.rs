use essential_app_utils::inputs::{index_mutation, B256};
use essential_types::solution::Mutation;

pub struct TransientData {
    pub key: B256,
}

impl TransientData {
    pub fn encode(&self) -> Vec<Mutation> {
        let Self { key } = self;
        let mutations = vec![index_mutation(0, key.to_value())];

        mutations
    }
}

pub type DecVars = super::DecVars;
