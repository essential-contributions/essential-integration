use essential_types::{
    convert::word_4_from_u8_32, solution::Mutation, ContentAddress, Hash, IntentAddress, Key,
    Value, Word,
};

#[derive(Clone)]
pub struct Instance {
    pub address: IntentAddress,
    pub path: Word,
}

#[derive(Clone, Copy)]
pub struct Int(pub Word);

#[derive(Clone, Copy)]
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
    fn to_slots<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Word>;
    fn to_slots_ref<'a, I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a Word>;
}

pub trait WriteDecVars {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>);
}

impl WriteDecVars for B256 {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        decision_variables.to_slots(self.0);
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
        decision_variables.to_slots(sig);
    }
}

impl WriteDecVars for essential_signer::secp256k1::PublicKey {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        let k = essential_sign::encode::public_key(self);
        decision_variables.to_slots(k);
    }
}

impl WriteDecVars for IntentAddress {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        self.set.write_dec_var(decision_variables);
        self.intent.write_dec_var(decision_variables);
    }
}

impl WriteDecVars for ContentAddress {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        decision_variables.to_slots(word_4_from_u8_32(self.0));
    }
}

impl WriteDecVars for Instance {
    fn write_dec_var(&self, decision_variables: &mut Vec<Value>) {
        self.address.set.write_dec_var(decision_variables);
        self.address.intent.write_dec_var(decision_variables);
        Int::from(self.path).write_dec_var(decision_variables);
    }
}

impl Slots for Vec<Value> {
    fn to_slots<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Word>,
    {
        self.extend(to_slots(iter));
    }

    fn to_slots_ref<'a, I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a Word>,
    {
        self.extend(to_slots_ref(iter));
    }
}

pub fn to_slots(iter: impl IntoIterator<Item = Word>) -> impl Iterator<Item = Vec<Word>> {
    iter.into_iter().map(|w| vec![w])
}

pub fn to_slots_ref<'a>(
    iter: impl IntoIterator<Item = &'a Word>,
) -> impl Iterator<Item = Vec<Word>> {
    to_slots(iter.into_iter().copied())
}
