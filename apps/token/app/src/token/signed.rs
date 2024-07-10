use essential_types::{
    solution::{Mutation, SolutionData},
    PredicateAddress, Value,
};

pint_abi::gen_from_file!("../pint/signed/out/debug/signed-abi.json");

pub type BurnData = Data<Burn::Vars, Burn::pub_vars::Mutations>;
pub type MintData = Data<Mint::Vars, Mint::pub_vars::Mutations>;
pub type TransferData = Data<Transfer::Vars, Transfer::pub_vars::Mutations>;

pub struct Data<D, P> {
    pub predicate_to_solve: PredicateAddress,
    pub decision_variables: D,
    pub transient_data: P,
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
            state_mutations: Default::default(),
        }
    }
}
