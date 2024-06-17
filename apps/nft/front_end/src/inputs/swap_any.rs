use app_utils::inputs::B256;
use essential_types::{solution::Mutation, Key};

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
