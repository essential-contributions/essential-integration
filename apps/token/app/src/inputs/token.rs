use super::{index_key, Int, B256};
use essential_types::{solution::Mutation, Key};

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
