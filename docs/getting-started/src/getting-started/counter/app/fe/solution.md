# Create Solution

In this step, we will add a function that accepts the predicate address and the desired count for setting the state.

The solution includes a single `Solution` that satisfies the given predicate (although other solutions may solve multiple predicates). In this case the predicate requires no input data so the `predicate_data` field is set to the default.

We will also add a single state mutation where the counter is updated to some `new_count`.

```rust
{{#include ../../../../../../code/counter.rs:solution}}
```

Here, we're using method `counter(..)` to provide the new value for the counter. Note that
`storage::mutations()`, `counter(..)`, and `Increment::ADDRESS` are available from the expansion of the `gen_from_file` macro.
