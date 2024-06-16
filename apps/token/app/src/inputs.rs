use essential_types::{
    convert::word_4_from_u8_32, solution::Mutation, ContentAddress, Hash, Key, Value, Word,
};

pub mod token;

pub struct Int(pub Word);

pub struct B256(pub [Word; 4]);

pub fn index_key(index: Word, key: Key) -> Key {
    let mut k = vec![index];
    k.extend(key);
    k
}

pub fn index_mutation(index: Word, value: Value) -> Mutation {
    Mutation {
        key: vec![index],
        value,
    }
}

impl B256 {
    pub fn to_key(&self) -> Key {
        self.0.to_vec()
    }

    pub fn to_value(&self) -> Value {
        self.0.to_vec()
    }
}

impl Int {
    pub fn to_value(&self) -> Value {
        vec![self.0]
    }
}

impl From<Hash> for B256 {
    fn from(value: Hash) -> Self {
        Self(essential_types::convert::word_4_from_u8_32(value))
    }
}

impl From<[Word; 4]> for B256 {
    fn from(value: [Word; 4]) -> Self {
        Self(value)
    }
}

impl From<Word> for Int {
    fn from(value: Word) -> Self {
        Self(value)
    }
}

impl From<ContentAddress> for B256 {
    fn from(value: ContentAddress) -> Self {
        Self(word_4_from_u8_32(value.0))
    }
}
