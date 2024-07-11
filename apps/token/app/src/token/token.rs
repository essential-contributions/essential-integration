use essential_app_utils::inputs::B256;
use essential_types::{
    solution::{Mutation, SolutionData},
    PredicateAddress, Value,
};

pint_abi::gen_from_file!("../pint/token/out/debug/token-abi.json");

// TODO: Remove the following after `pint-abi-gen` adds `keys()` builder.
pub fn query_balances(owner: B256) -> essential_types::Key {
    let mut m: Vec<Mutation> = storage::mutations()
        .balances(|map| map.entry(owner.0, Default::default()))
        .into();
    m.pop().unwrap().key
}

pub fn query_nonce(key: B256) -> essential_types::Key {
    let mut m: Vec<Mutation> = storage::mutations()
        .nonce(|map| map.entry(key.0, Default::default()))
        .into();
    m.pop().unwrap().key
}

#[allow(dead_code)]
pub fn query_token_name() -> essential_types::Key {
    let mut m: Vec<Mutation> = storage::mutations().token_name(Default::default()).into();
    m.pop().unwrap().key
}

#[allow(dead_code)]
pub fn query_token_symbol() -> essential_types::Key {
    let mut m: Vec<Mutation> = storage::mutations().token_symbol(Default::default()).into();
    m.pop().unwrap().key
}

#[allow(dead_code)]
pub fn query_decimals() -> essential_types::Key {
    let mut m: Vec<Mutation> = storage::mutations().token_symbol(Default::default()).into();
    m.pop().unwrap().key
}

pub type BurnData = Data<Burn::Vars, Burn::pub_vars::Mutations>;
pub type MintData = Data<Mint::Vars, Mint::pub_vars::Mutations>;
pub type TransferData = Data<Transfer::Vars, Transfer::pub_vars::Mutations>;

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