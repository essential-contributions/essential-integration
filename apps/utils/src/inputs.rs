use essential_sign::secp256k1::{ecdsa::RecoverableSignature, PublicKey};
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
        decision_variables.to_slot(sig);
    }
}

impl WriteDecVars for essential_signer::secp256k1::PublicKey {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        let k = essential_sign::encode::public_key(self);
        decision_variables.to_slot(k);
    }
}

impl WriteDecVars for PredicateAddress {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        let mut slot = Vec::new();
        self.contract.write_dec_var(&mut slot);
        self.predicate.write_dec_var(&mut slot);
        decision_variables.to_slot(slot.into_iter().flatten());
    }
}

impl WriteDecVars for ContentAddress {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        decision_variables.to_slot(word_4_from_u8_32(self.0));
    }
}

impl WriteDecVars for Instance {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        let mut slot = Vec::new();
        self.address.contract.write_dec_var(&mut slot);
        self.address.predicate.write_dec_var(&mut slot);
        Int::from(self.path).write_dec_var(&mut slot);
        decision_variables.to_slot(slot.into_iter().flatten());
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

pub trait Encode {
    type Output;

    fn encode(&self) -> Self::Output;
}

impl Encode for RecoverableSignature {
    type Output = ([Word; 4], [Word; 4], Word);

    /// Convert an ECDSA signature into the form expected by the generated ABI type.
    fn encode(&self) -> Self::Output {
        let [a0, a1, a2, a3, b0, b1, b2, b3, c1] = essential_sign::encode::signature(self);
        ([a0, a1, a2, a3], [b0, b1, b2, b3], c1)
    }
}

impl Encode for PublicKey {
    type Output = ([Word; 4], Word);

    /// Convert a public key into the form expected by the generated ABI type.
    fn encode(&self) -> Self::Output {
        let [a0, a1, a2, a3, b0] = essential_sign::encode::public_key(self);
        ([a0, a1, a2, a3], b0)
    }
}

impl Encode for ContentAddress {
    type Output = [Word; 4];

    fn encode(&self) -> Self::Output {
        word_4_from_u8_32(self.0)
    }
}

impl Encode for PredicateAddress {
    type Output = ([Word; 4], [Word; 4]);

    fn encode(&self) -> Self::Output {
        (self.contract.encode(), self.predicate.encode())
    }
}
