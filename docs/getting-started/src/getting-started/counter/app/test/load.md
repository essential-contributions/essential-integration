# Generate Address
Define an asynchronous test function using the `#[tokio::test]` attribute. This sets up our test environment and allows us to use `async/await` syntax within our test.
```rust
{{#include ../../../../../../code/counter-test.rs:test-start}}
{{#include ../../../../../../code/counter-test.rs:test-end}}
```
Then add a section to read the "counter" contract from its bytecode which was generated earlier and
can be found under the `out/debug` directory.

```rust
{{#include ../../../../../../code/counter-test.rs:test-start}}
{{#include ../../../../../../code/counter-test.rs:counter}}
{{#include ../../../../../../code/counter-test.rs:test-end}}
```
