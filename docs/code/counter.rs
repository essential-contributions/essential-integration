
// ANCHOR: full
// ANCHOR: use
use anyhow::bail;
use essential_rest_client::EssentialClient;
use essential_types::{
    solution::{Mutation, Solution, SolutionData},
    PredicateAddress, Word,
};
// ANCHOR_END: use

// ANCHOR: app
pub struct App {
    client: EssentialClient,
    predicate: PredicateAddress,
}
// ANCHOR_END: app

// ANCHOR: impl
// ANCHOR: impl-start
impl App {
// ANCHOR_END: impl-start
    // ANCHOR: key
    pub const COUNTER_KEY: Word = 0;
    // ANCHOR_END: key

    // ANCHOR: new
    pub fn new(addr: String, predicate: PredicateAddress) -> anyhow::Result<Self> {
        let client = EssentialClient::new(addr)?;
        Ok(Self {
            client,
            predicate,
        })
    }
    // ANCHOR_END: new

    // ANCHOR: read
    // ANCHOR: read-start
    pub async fn read_count(&self) -> anyhow::Result<Word> {
    // ANCHOR_END: read-start
        // ANCHOR: read-state
        let output = self
            .client
            .query_state(&self.predicate.contract, &vec![Self::COUNTER_KEY])
            .await?;
        // ANCHOR_END: read-state

        let count = match &output[..] {
            [] => 0,
            [count] => *count,
            _ => bail!("Expected one word, got: {:?}", output),
        };
        Ok(count)
    // ANCHOR: read-end
    }
    // ANCHOR_END: read-end
    // ANCHOR_END: read

    // ANCHOR: increment
    pub async fn increment(&self) -> anyhow::Result<Word> {
        let new_count = self.read_count().await? + 1;
        let solution = create_solution(self.predicate.clone(), new_count);
        self.client.submit_solution(solution).await?;
        Ok(new_count)
    }
    // ANCHOR_END: increment
// ANCHOR: impl-end
}
// ANCHOR_END: impl-end
// ANCHOR_END: impl

// ANCHOR: solution
pub fn create_solution(predicate: PredicateAddress, new_count: Word) -> Solution {
    Solution {
        data: vec![SolutionData {
            predicate_to_solve: predicate,
            decision_variables: Default::default(),
            transient_data: Default::default(),
            state_mutations: vec![Mutation {
                key: vec![App::COUNTER_KEY],
                value: vec![new_count],
            }],
        }],
    }
}
// ANCHOR_END: solution
// ANCHOR_END: full