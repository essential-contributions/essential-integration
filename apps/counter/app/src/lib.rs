use anyhow::bail;
use essential_types::{
    solution::{Mutation, Solution, SolutionData},
    PredicateAddress, Value, Word,
};

const COUNTER_KEY: Word = 0;

#[derive(Clone)]
pub struct CounterKey(pub Vec<Word>);
#[derive(Clone)]
pub struct QueryCount(pub Option<Value>);

pub fn counter_key() -> CounterKey {
    CounterKey(vec![COUNTER_KEY])
}

pub fn incremented_solution(
    predicate: PredicateAddress,
    count: QueryCount,
) -> anyhow::Result<(Solution, Word)> {
    let count = extract_count(count)?;
    let new_count = count + 1;
    Ok((create_solution(predicate, new_count), new_count))
}

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
