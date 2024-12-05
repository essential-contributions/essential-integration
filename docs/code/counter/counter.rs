// ANCHOR: full
// ANCHOR: use
use anyhow::bail;
use essential_types::{
    solution::{Mutation, Solution, SolutionData},
    Value, Word,
};
// ANCHOR_END: use

// ANCHOR: abi_gen
pint_abi::gen_from_file! {
    abi: "../contract/out/debug/counter-abi.json",
    contract: "../contract/out/debug/counter.json",
}
// ANCHOR_END: abi_gen

// ANCHOR: counter-key
#[derive(Clone)]
pub struct CounterKey(pub Vec<Word>);

pub fn counter_key() -> CounterKey {
    let keys: Vec<_> = storage::keys().counter().into();
    CounterKey(keys.first().unwrap().clone())
}
// ANCHOR_END: counter-key

// ANCHOR: increment
pub fn incremented_solution(count: Option<Value>) -> anyhow::Result<(Solution, Word)> {
    let count = extract_count(count)?;
    let new_count = count + 1;
    Ok((create_solution(new_count), new_count))
}
// ANCHOR_END: increment

// ANCHOR: extract
/// Given a query of the current count, extract the count.
pub fn extract_count(count: Option<Value>) -> anyhow::Result<Word> {
    match count {
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
pub fn create_solution(new_count: Word) -> Solution {
    let state_mutations: Vec<Mutation> = storage::mutations().counter(new_count).into();
    Solution {
        data: vec![SolutionData {
            predicate_to_solve: Increment::ADDRESS,
            decision_variables: Default::default(),
            state_mutations,
        }],
    }
}
// ANCHOR_END: solution
// ANCHOR_END: full