# Build Block

Add a section that builds a block and verifies the results. We check that three solutions succeeded (the initial chain solution, the contract deployment and counter increment) and that no transactions failed. We then read the counter's value again to confirm it has been updated to the new count.
```rust
{{#include ../../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../../code/counter-test.rs:build}}
{{#include ../../../../../../code/counter-test.rs:test-end}}
```