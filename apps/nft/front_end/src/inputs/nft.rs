use app_utils::inputs::{index_key, Int, B256};
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
