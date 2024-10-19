# Compete

In this final section test how the counter handles competing increments. We increment the counter twice with two solutions setting the count to the same value. Then verify that only one increment succeeds when we build the block. This demonstrates that when two solutions compete for the same state only one will win.
```rust
{{#include ../../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../../code/counter-test.rs:comp}}
{{#include ../../../../../../code/counter-test.rs:test-end}}
```
Currently the builder uses a simple FIFO approach to choosing which solutions wins but this will change to more advanced strategies in the future.