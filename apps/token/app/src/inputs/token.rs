use app_utils::inputs::{index_key, index_mutation, Int, B256};
use essential_types::{solution::Mutation, Key, Word};

pub mod burn;
pub mod mint;
pub mod transfer;

pub fn balances(owner: B256, amount: Int) -> Mutation {
    Mutation {
        key: index_key(1, owner.to_key()),
        value: amount.to_value(),
    }
}

pub fn query_balances(owner: B256) -> Key {
    index_key(1, owner.to_key())
}

pub fn init() -> Mutation {
    Mutation {
        key: vec![0],
        value: vec![Word::from(true)],
    }
}

pub fn query_init() -> Key {
    vec![0]
}

pub struct Interface {
    pub key: B256,
    pub to: B256,
    pub amount: Int,
    pub set: B256,
    pub intent_addr: B256,
    pub path: Int,
}

impl Interface {
    pub fn encode(&self) -> Vec<Mutation> {
        let Self {
            key,
            to,
            amount,
            set,
            intent_addr,
            path,
        } = self;
        let mutations = vec![
            index_mutation(0, key.to_value()),
            index_mutation(1, to.to_value()),
            index_mutation(2, amount.to_value()),
            index_mutation(3, set.to_value()),
            index_mutation(4, intent_addr.to_value()),
            index_mutation(5, path.to_value()),
        ];

        mutations
    }
}
