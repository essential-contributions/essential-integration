# Extract Count
State will be read using the data from the keys page however state returns an optional vector of words. \
This is because state can return any size of data including empty.

Create a type to represent this.
```rust
{{#include ../../../../../../code/counter.rs:query}}
```
Now create a function to extract the count from the state return `QueryCount`.

Add this `match` expression that maps:
- empty to `0`.
- a single word to the count.
- anything else to an error.

Then return the count.
```rust
{{#include ../../../../../../code/counter.rs:extract}}
```
