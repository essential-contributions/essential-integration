use essential_app_utils::inputs::B256;
use essential_types::{
    solution::{Mutation, SolutionData},
    Key, PredicateAddress, Value,
};

pint_abi::gen_from_file! {
    abi: "../pint/token/out/debug/token-abi.json",
    contract:  "../pint/token/out/debug/token.json",
}

// TODO: Remove the following after `pint-abi-gen` adds `keys()` builder.
pub fn query_balances(owner: B256) -> essential_types::Key {
    let mut k: Vec<Key> = storage::keys().balances(|map| map.entry(owner.0)).into();
    k.pop().unwrap()
}

pub fn query_nonce(key: B256) -> essential_types::Key {
    let mut k: Vec<Key> = storage::keys().nonce(|map| map.entry(key.0)).into();
    k.pop().unwrap()
}

#[allow(dead_code)]
pub fn query_token_name() -> essential_types::Key {
    let mut k: Vec<Key> = storage::keys().token_name().into();
    k.pop().unwrap()
}

#[allow(dead_code)]
pub fn query_token_symbol() -> essential_types::Key {
    let mut k: Vec<Key> = storage::keys().token_symbol().into();
    k.pop().unwrap()
}

#[allow(dead_code)]
pub fn query_decimals() -> essential_types::Key {
    let mut k: Vec<Key> = storage::keys().token_symbol().into();
    k.pop().unwrap()
}

pub type BurnData = Data<Burn::Vars, Burn::PubVars>;
pub type MintData = Data<Mint::Vars, Mint::PubVars>;
pub type TransferData = Data<Transfer::Vars, Transfer::PubVars>;

pub struct Data<D, P> {
    pub predicate_to_solve: PredicateAddress,
    pub decision_variables: D,
    pub transient_data: P,
    pub state_mutations: storage::Mutations,
}

impl<D, P> From<Data<D, P>> for SolutionData
where
    D: Into<Vec<Value>>,
    P: Into<Vec<Mutation>>,
{
    fn from(data: Data<D, P>) -> Self {
        SolutionData {
            predicate_to_solve: data.predicate_to_solve,
            decision_variables: data.decision_variables.into(),
            transient_data: data.transient_data.into(),
            state_mutations: data.state_mutations.into(),
        }
    }
}
