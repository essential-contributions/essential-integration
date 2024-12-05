# Deploy to Test Builder
Add these lines to deploy the contract. \
Here, we're setting up our test databases and deploying the counter contract to our test builder. This step is crucial for having a contract to interact with in our subsequent tests.
```rust
{{#include ../../../../../../code/counter/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../../code/counter/counter-test.rs:dep}}
{{#include ../../../../../../code/counter/counter-test.rs:test-end}}
```
