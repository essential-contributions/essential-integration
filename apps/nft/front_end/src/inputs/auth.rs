use app_utils::inputs::{index_mutation, Int, WriteDecVars, B256};
use essential_types::{solution::Mutation, IntentAddress, Value};

pub struct DecVars {
    pub auth_addr: IntentAddress,
    pub authi_auth_pathway: Int,
}

pub struct TransientData {
    pub key: B256,
    pub to: B256,
    pub token: Int,
    pub set: B256,
    pub intent_addr: B256,
    pub path: Int,
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
        let Self {
            key,
            to,
            token,
            set,
            intent_addr,
            path,
        } = self;
        let mutations = vec![
            index_mutation(0, key.to_value()),
            index_mutation(1, to.to_value()),
            index_mutation(2, token.to_value()),
            index_mutation(3, set.to_value()),
            index_mutation(4, intent_addr.to_value()),
            index_mutation(5, path.to_value()),
        ];

        mutations
    }
}
