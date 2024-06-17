use super::{index_key, Int, B256};
use essential_types::{solution::Mutation, Key};

pub mod burn;
pub mod init;
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

pub fn name(name: B256) -> Mutation {
    Mutation {
        key: vec![0],
        value: name.to_value(),
    }
}
