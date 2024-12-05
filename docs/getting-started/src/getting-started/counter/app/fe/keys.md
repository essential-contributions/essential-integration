# Keys
Create a type that will be used to query the counter state. \
Also create a function to construct this type from the ABI. \
`Key`s are vectors of words so here we create a vector with a single word.
```rust
{{#include ../../../../../../code/counter/counter.rs:counter-key}}
```

Here, we're extract the storage key for the storage variable `counter` by using the method
`counter()`.
