# Increment Count

Add an `increment` function that increments the counter's value by one. The function reads the current count, increments it, and then creates a new solution with the updated count. The solution is then submitted to the builder to be included in a block.
```rust
{{#include ../../../../../../code/counter-test.rs:inc}}
```

Here, we're calling the `increment` function to increase the counter's value. This function interacts with the builder to update the counter's state.
```rust
{{#include ../../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../../code/counter-test.rs:incr}}
{{#include ../../../../../../code/counter-test.rs:test-end}}
```
