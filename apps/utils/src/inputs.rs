use essential_types::{
    convert::word_4_from_u8_32, solution::Mutation, ContentAddress, Hash, Key, PredicateAddress,
    Value, Word,
};

#[derive(Clone)]
pub struct Instance {
    pub address: PredicateAddress,
    pub path: Word,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int(pub Word);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub fn to_key(&self) -> Key {
        vec![self.0]
    }

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

trait Slots {
    fn to_slot<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Word>;
}

pub trait WriteDecVars {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>);
}

impl WriteDecVars for B256 {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        decision_variables.to_slot(self.0);
    }
}

impl WriteDecVars for Int {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        decision_variables.push(vec![self.0]);
    }
}

impl WriteDecVars for essential_signer::secp256k1::ecdsa::RecoverableSignature {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        let sig = essential_sign::encode::signature(self);
        decision_variables.to_slot(sig[..4].to_vec());
        decision_variables.to_slot(sig[4..8].to_vec());
        decision_variables.to_slot(sig[8..].to_vec());
    }
}

impl WriteDecVars for essential_signer::secp256k1::PublicKey {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        let k = essential_sign::encode::public_key(self);
        decision_variables.to_slot(k[..4].to_vec());
        decision_variables.to_slot(k[4..].to_vec());
    }
}

impl WriteDecVars for PredicateAddress {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        self.contract.write_dec_var(decision_variables);
        self.predicate.write_dec_var(decision_variables);
    }
}

impl WriteDecVars for ContentAddress {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        decision_variables.to_slot(word_4_from_u8_32(self.0));
    }
}

impl WriteDecVars for Instance {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        self.address.contract.write_dec_var(decision_variables);
        self.address.predicate.write_dec_var(decision_variables);
        Int::from(self.path).write_dec_var(decision_variables);
    }
}

impl Slots for Vec<Value> {
    fn to_slot<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Word>,
    {
        self.push(iter.into_iter().collect());
    }
}
