use anyhow::{bail, ensure};
use essential_app_utils::{
    addresses::contract_hash,
    compile::compile_pint_project,
    inputs::Encode,
    print::{print_contract_address, print_predicate_address},
};
use essential_rest_client::EssentialClient;
use essential_types::{
    contract::Contract,
    convert::word_4_from_u8_32,
    solution::{Mutation, Solution, SolutionData},
    ContentAddress, PredicateAddress, Word,
};
use std::{path::Path, vec};

pub struct App {
    client: EssentialClient,
    predicate: PredicateAddress,
}

impl App {
    pub const NONCE_KEY: Word = 0;

    pub fn new(addr: String, predicate: PredicateAddress) -> anyhow::Result<Self> {
        let client = EssentialClient::new(addr)?;
        Ok(Self { client, predicate })
    }

    pub async fn read_nonce(&self) -> anyhow::Result<Word> {
        let output = self
            .client
            .query_state(&self.predicate.contract, &vec![Self::NONCE_KEY])
            .await?;

        let count = match &output[..] {
            [] => 0,
            [count] => *count,
            _ => bail!("Expected one word, got: {:?}", output),
        };
        Ok(count)
    }

    pub async fn increment(&self) -> anyhow::Result<Word> {
        let new_count = self.read_nonce().await? + 1;
        let solution = create_solution(self.predicate.clone(), new_count);
        self.client.submit_solution(solution).await?;
        Ok(new_count)
    }
}

// mod unions {
//     pint_abi::gen_from_file! {
//         abi: "../out/debug/unions-abi.json",
//         contract:  "../out/debug/unions.json",
//     }
// }

pub fn create_solution(predicate: PredicateAddress, new_count: Word) -> Solution {
    Solution {
        data: vec![SolutionData {
            predicate_to_solve: predicate,
            // decision_variables: unions::Increment::Vars {
            //     current_thing: 42.into(),
            // },
            // decision_variables: vec![vec![42]],
            decision_variables: Default::default(),
            transient_data: Default::default(),
            state_mutations: vec![Mutation {
                key: vec![App::NONCE_KEY],
                value: vec![new_count],
            }],
        }],
    }
}
