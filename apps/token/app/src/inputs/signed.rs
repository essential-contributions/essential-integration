use app_utils::inputs::{index_key, Int, B256};
use essential_types::{solution::Mutation, Key};

#[allow(dead_code)]
pub mod cancel;
pub mod transfer;
#[allow(dead_code)]
pub mod transfer_from;
#[allow(dead_code)]
pub mod transfer_from_to;

pub fn nonce(key: B256, nonce: Int) -> Mutation {
    Mutation {
        key: index_key(0, key.to_key()),
        value: nonce.to_value(),
    }
}

pub fn query_nonce(key: B256) -> Key {
    index_key(0, key.to_key())
}
