use essential_types::{solution::Mutation, Key};

use super::B256;

pub mod swap;

pub fn token(token: B256) -> Mutation {
    Mutation {
        key: vec![0],
        value: token.to_value(),
    }
}

pub fn query_token() -> Key {
    vec![0]
}
