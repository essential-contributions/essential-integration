# Generate Address
Add the `compile_address` function that compiles the Pint project located in the specified directory and returns the PredicateAddress, which includes both the contract address and the predicate address:
```rust
{{#include ../../../../../../code/counter/counter-main.rs:comp}}
```

This function helps identify which contract and predicate to interact with. While we could simply pass in the addresses directly, generating them here adds convenience and ensures they are correctly derived from the compiled contract.