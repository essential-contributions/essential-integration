use essential_types::{solution::Mutation, IntentAddress, Value};

use super::{index_mutation, Int, WriteDecVars, B256};

pub struct DecVars {
    pub auth_addr: IntentAddress,
    pub authi_auth_pathway: Int,
}

pub struct TransientData {
    pub key: B256,
    pub token: B256,
    pub to: B256,
}

impl DecVars {
    pub fn encode(&self) -> Vec<Value> {
        let Self {
            auth_addr,
            authi_auth_pathway,
        } = self;
        let mut decision_variables = vec![];

        auth_addr.write_dec_var(&mut decision_variables);
        authi_auth_pathway.write_dec_var(&mut decision_variables);
        decision_variables
    }
}

impl TransientData {
    pub fn encode(&self) -> Vec<Mutation> {
        let Self { key, token, to } = self;
        let mutations = vec![
            index_mutation(0, key.to_value()),
            index_mutation(1, token.to_value()),
            index_mutation(2, to.to_value()),
        ];

        mutations
    }
}
