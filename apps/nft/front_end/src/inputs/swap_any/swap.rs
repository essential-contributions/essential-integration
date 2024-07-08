use essential_app_utils::inputs::{Int, WriteDecVars, B256};
use essential_types::{solution::Mutation, PredicateAddress};

pub struct TransientData {
    pub nft: PredicateAddress,
}

impl TransientData {
    pub fn encode(&self) -> Vec<Mutation> {
        let Self { nft } = self;
        let mutations = vec![
            Mutation {
                key: vec![0, 0],
                value: B256::from(nft.contract.0).to_value(),
            },
            Mutation {
                key: vec![0, 1],
                value: B256::from(nft.predicate.0).to_value(),
            },
        ];

        mutations
    }
}

pub struct DecVars {
    pub nft_path: Int,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self { nft_path } = self;
        let mut decision_variables = vec![];

        nft_path.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
