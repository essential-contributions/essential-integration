# Generate Address
Add the `compile_address` function that compiles the Pint project located in the specified directory and returns the PredicateAddress, which includes both the contract address and the predicate address:
```rust
{{#include ../../../../../../code/counter-main.rs:comp}}
```

This is used to know which contract and predicate to interact with. We could also just pass the addresses in but generating them is done for convenience.