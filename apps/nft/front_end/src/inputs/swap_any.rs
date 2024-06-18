use app_utils::inputs::Int;
use essential_types::{solution::Mutation, Key};

pub mod swap;

pub fn token(token: Int) -> Mutation {
    Mutation {
        key: vec![0],
        value: token.to_value(),
    }
}

pub fn query_token() -> Key {
    vec![0]
}
