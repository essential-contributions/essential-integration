use app_utils::inputs::{index_key, index_mutation, Int, B256};
use essential_types::{solution::Mutation, Key};

pub mod mint;
pub mod transfer;

pub fn owners(token: Int, owner: B256) -> Mutation {
    Mutation {
        key: index_key(0, token.to_key()),
        value: owner.to_value(),
    }
}

pub fn query_owners(token: Int) -> Key {
    index_key(0, token.to_key())
}

pub struct Interface {
    pub key: B256,
    pub to: B256,
    pub token: Int,
    pub set: B256,
    pub intent_addr: B256,
    pub path: Int,
}

impl Interface {
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
