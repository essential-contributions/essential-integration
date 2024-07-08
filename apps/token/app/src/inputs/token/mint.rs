use essential_app_utils::inputs::{index_mutation, Instance, Int, WriteDecVars, B256};
use essential_types::solution::Mutation;

pub struct TransientData {
    pub key: B256,
    pub amount: Int,
    pub decimals: Int,
}

impl TransientData {
    pub fn encode(&self) -> Vec<Mutation> {
        let Self {
            key,
            amount,
            decimals,
        } = self;
        let mutations = vec![
            index_mutation(0, key.to_value()),
            index_mutation(1, amount.to_value()),
            index_mutation(2, decimals.to_value()),
        ];

        mutations
    }
}

pub struct DecVars {
    pub name: B256,
    pub symbol: B256,
    pub auth_addr: Instance,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self {
            name,
            symbol,
            auth_addr,
        } = self;
        let mut decision_variables = vec![];

        name.write_dec_var(&mut decision_variables);
        symbol.write_dec_var(&mut decision_variables);
        auth_addr.address.write_dec_var(&mut decision_variables);
        Int::from(auth_addr.path).write_dec_var(&mut decision_variables);

        decision_variables
    }
}
