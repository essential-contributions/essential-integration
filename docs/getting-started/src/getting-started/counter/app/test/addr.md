# Generate Address
Define an asynchronous test function using the `#[tokio::test]` attribute. This sets up our test environment and allows us to use `async/await` syntax within our test.
```rust
{{#include ../../../../../../code/counter-test.rs:test-start}}
{{#include ../../../../../../code/counter-test.rs:test-end}}
```
Then add a section to generate the address. We're compiling our Pint project, which contains the counter contract. We then generate the contract address and predicate address from the compiled contract. These addresses will be used to interact with our counter in the builder / node.
```rust
{{#include ../../../../../../code/counter-test.rs:test-start}}
{{#include ../../../../../../code/counter-test.rs:addr}}
{{#include ../../../../../../code/counter-test.rs:test-end}}
```