# Keys
The `COUNTER_KEY` is the key that points to the `counter: int` storage.

Add a const to store this storage location.
```rust
{{#include ../../../../../../code/counter.rs:key}}
```
Create a type that will be used to query the counter state. \
Also create a function to construct this type from the `COUNTER_KEY`. \
Key's are vectors of words so here we create a vector with a single word.
```rust
{{#include ../../../../../../code/counter.rs:counter-key}}
```