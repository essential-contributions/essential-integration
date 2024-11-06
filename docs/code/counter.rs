
// ANCHOR: full
// ANCHOR: use
use anyhow::bail;
use essential_types::{
    solution::{Mutation, Solution, SolutionData},
    PredicateAddress, Value, Word,
};
// ANCHOR_END: use

// ANCHOR: key
const COUNTER_KEY: Word = 0;
// ANCHOR_END: key

// ANCHOR: query
#[derive(Clone)]
pub struct QueryCount(pub Option<Value>);
// ANCHOR_END: query


// ANCHOR: counter-key
#[derive(Clone)]
pub struct CounterKey(pub Vec<Word>);

pub fn counter_key() -> CounterKey {
    CounterKey(vec![COUNTER_KEY])
}
// ANCHOR_END: counter-key

// ANCHOR: increment
pub fn incremented_solution(
    predicate: PredicateAddress,
    count: QueryCount,
) -> anyhow::Result<(Solution, Word)> {
    let count = extract_count(count)?;
    let new_count = count + 1;
    Ok((create_solution(predicate, new_count), new_count))
}
// ANCHOR_END: increment

// ANCHOR: extract
/// Given a query of the current count, extract the count.
pub fn extract_count(count: QueryCount) -> anyhow::Result<Word> {
    match count.0 {
        Some(count) => match &count[..] {
            [] => Ok(0),
            [count] => Ok(*count),
            _ => bail!("Expected single word, got: {:?}", count),
        },
        None => Ok(0),
    }
}
// ANCHOR_END: extract

// ANCHOR: solution
pub fn create_solution(predicate: PredicateAddress, new_count: Word) -> Solution {
    Solution {
        data: vec![SolutionData {
            predicate_to_solve: predicate,
            decision_variables: Default::default(),
            state_mutations: vec![Mutation {
                key: vec![COUNTER_KEY],
                value: vec![new_count],
            }],
        }],
    }
}
// ANCHOR_END: solution
// ANCHOR_END: full