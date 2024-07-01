use app_utils::inputs::{index_mutation, Instance, Int, WriteDecVars, B256};
use essential_types::solution::Mutation;

pub struct TransientData {
    pub key: B256,
    pub to: B256,
    pub token: Int,
}

impl TransientData {
    pub fn encode(&self) -> Vec<Mutation> {
        let Self { key, to, token } = self;
        let mutations = vec![
            index_mutation(0, key.to_value()),
            index_mutation(1, to.to_value()),
            index_mutation(2, token.to_value()),
        ];

        mutations
    }
}

pub struct DecVars {
    pub auth_addr: Instance,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self { auth_addr } = self;
        let mut decision_variables = vec![];

        auth_addr.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
