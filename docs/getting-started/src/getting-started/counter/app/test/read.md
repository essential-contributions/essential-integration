# Read Count
Create the `read_count` function that queries the current count from the Essential node using the provided node client, contract address, and counter key.
```rust
{{#include ../../../../../../code/counter-test.rs:read-count}}
```
Add this section that reads the counter.
In this part, we're reading the initial count from our deployed counter contract. We create a key for the counter, read its value, and assert that it starts at zero. This verifies that our counter is deployed and hasn't been incremented yet.
```rust
{{#include ../../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../../code/counter-test.rs:read}}
{{#include ../../../../../../code/counter-test.rs:test-end}}
```
