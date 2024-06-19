use app_utils::inputs::{index_key, Instance, Int, WriteDecVars, B256};
use essential_types::{solution::Mutation, Key};

pub mod burn;
pub mod cancel;
pub mod mint;
pub mod transfer;

pub fn balances(owner: B256, amount: Int) -> Mutation {
    Mutation {
        key: index_key(0, owner.to_key()),
        value: amount.to_value(),
    }
}

pub fn query_balances(owner: B256) -> Key {
    index_key(0, owner.to_key())
}

pub fn nonce(key: B256, nonce: Int) -> Mutation {
    Mutation {
        key: index_key(1, key.to_key()),
        value: nonce.to_value(),
    }
}

pub fn query_nonce(key: B256) -> Key {
    index_key(1, key.to_key())
}

pub fn token_name(name: B256) -> Mutation {
    Mutation {
        key: vec![2],
        value: name.to_value(),
    }
}

pub fn query_name() -> Key {
    vec![2]
}

pub fn token_symbol(symbol: B256) -> Mutation {
    Mutation {
        key: vec![3],
        value: symbol.to_value(),
    }
}

pub fn query_symbol() -> Key {
    vec![3]
}

pub fn decimals(decimals: Int) -> Mutation {
    Mutation {
        key: vec![4],
        value: decimals.to_value(),
    }
}

pub fn query_decimals() -> Key {
    vec![4]
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
