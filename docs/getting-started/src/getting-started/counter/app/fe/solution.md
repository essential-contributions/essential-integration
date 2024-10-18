# Create a solution

Add this function that takes the predicate address and the count we are trying to set the state to. \
The solution has a single `SolutionData` that solves this predicate (other solutions may solve multiple predicates). \
There's no `decision variables` or `transient data` so those are set to default. \
Add in a single state `Mutation`. The key is the `COUNTER_KEY` and the value is the new count.
```rust
{{#include ../../../../../../code/counter.rs:solution}}
```