use anyhow::bail;
use essential_types::{
    solution::{Mutation, Solution},
    PredicateAddress, Value, Word,
};

/// The location in storage where the counter is stored.
const COUNTER_KEY: Word = 0;

#[derive(Clone)]
/// The key used to access the counter in storage.
pub struct CounterKey(pub Vec<Word>);

#[derive(Clone)]
/// The data returned when querying the current count.
pub struct QueryCount(pub Option<Value>);

/// The key used to access the counter in storage.
pub fn counter_key() -> CounterKey {
    CounterKey(vec![COUNTER_KEY])
}

/// Given a query of the current count,
/// create a new solution that increments the count by one.
pub fn incremented_solution(
    predicate: PredicateAddress,
    count: QueryCount,
) -> anyhow::Result<(Solution, Word)> {
    let count = extract_count(count)?;
    let new_count = count + 1;
    Ok((create_solution(predicate, new_count), new_count))
}

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

/// Create a solution that sets the count to a new value.
pub fn create_solution(predicate: PredicateAddress, new_count: Word) -> Solution {
    Solution {
        predicate_to_solve: predicate,
        predicate_data: Default::default(),
        state_mutations: vec![Mutation {
            key: vec![COUNTER_KEY],
            value: vec![new_count],
        }],
    }
}
