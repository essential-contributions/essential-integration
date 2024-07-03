use anyhow::bail;
use essential_rest_client::EssentialClient;
use essential_types::{
    solution::{Mutation, Solution, SolutionData},
    Key, PredicateAddress, Word,
};

const COUNTER_KEY: Word = 0;

pub struct App {
    client: EssentialClient,
    predicate: PredicateAddress,
    counter_key: Key,
}

impl App {
    pub fn new(addr: String, predicate: PredicateAddress) -> anyhow::Result<Self> {
        let client = EssentialClient::new(addr)?;
        Ok(Self {
            client,
            predicate,
            counter_key: vec![COUNTER_KEY],
        })
    }

    pub fn counter_key(&self) -> &Key {
        &self.counter_key
    }

    pub async fn read_count(&self) -> anyhow::Result<Word> {
        let output = self
            .client
            .query_state(&self.predicate.contract, self.counter_key())
            .await?;

        let count = match &output[..] {
            [] => 0,
            [count] => *count,
            _ => bail!("Expected one word, got: {:?}", output),
        };
        Ok(count)
    }

    pub async fn increment(&self) -> anyhow::Result<Word> {
        let solution = self.incremented_solution().await?;
        let count = solution.data[0].state_mutations[0].value[0];
        self.submit_solution(solution).await?;
        Ok(count)
    }

    pub async fn incremented_solution(&self) -> anyhow::Result<Solution> {
        let count = self.read_count().await?;
        Ok(create_solution(self.predicate.clone(), count + 1))
    }

    pub async fn submit_solution(&self, solution: Solution) -> anyhow::Result<()> {
        self.client.submit_solution(solution).await?;
        Ok(())
    }
}

pub fn create_solution(predicate: PredicateAddress, new_count: Word) -> Solution {
    Solution {
        data: vec![SolutionData {
            predicate_to_solve: predicate,
            decision_variables: Default::default(),
            transient_data: Default::default(),
            state_mutations: vec![Mutation {
                key: vec![COUNTER_KEY],
                value: vec![new_count],
            }],
        }],
    }
}
