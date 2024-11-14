# Create Solution

In this step, we will add a function that accepts the predicate address and the desired count for setting the state.

The solution includes a single `SolutionData` that satisfies the given predicate (although other solutions may solve multiple predicates). In this case, there are no `decision variables` so that field is set to the default.

We will also add a single state `Mutation`, where the key is `COUNTER_KEY` and the value is the new count.

```rust
{{#include ../../../../../../code/counter.rs:solution}}
