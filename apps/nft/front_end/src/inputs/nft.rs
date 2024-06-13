use essential_types::{solution::Mutation, Key};

use super::{index_key, B256};

pub mod mint;
pub mod transfer;

pub fn owners(token: B256, owner: B256) -> Mutation {
    Mutation {
        key: index_key(0, token.to_key()),
        value: owner.to_value(),
    }
}

pub fn query_owners(token: B256) -> Key {
    index_key(0, token.to_key())
}
