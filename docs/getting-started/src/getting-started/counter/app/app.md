# App
We are going to write the application functionality. \
This is what we will use in our tests to interact with the contract. \
The aim of this code is to read state and create solutions.

Start by adding the imports you are going to need.
```rust
{{#include ../../../../../code/counter.rs:use}}
```
This is the main struct that allows us to interact with the `essential-server`.
It contains the `client` that let's us send requests to the server and the `predicate` address for our counter contract.
```rust
{{#include ../../../../../code/counter.rs:app}}
```
Add an impl block.
```rust
{{#include ../../../../../code/counter.rs:impl-start}}

{{#include ../../../../../code/counter.rs:impl-end}}
```
The `COUNTER_KEY` is the key that points to the `counter: int` storage.
```rust
{{#include ../../../../../code/counter.rs:impl-start}}
{{#include ../../../../../code/counter.rs:key}}
{{#include ../../../../../code/counter.rs:impl-end}}
```
Add a `new` method so the `App` can be created. \ 
This takes the address of the server and the contract.
```rust
{{#include ../../../../../code/counter.rs:impl-start}}
    // ...

{{#include ../../../../../code/counter.rs:new}}
{{#include ../../../../../code/counter.rs:impl-end}}
```
## Read storage
Read the current count from storage. \
Using the `essential-client` we make a query to the state at the address of the counter contract and the `COUNTER_KEY`.
```rust
{{#include ../../../../../code/counter.rs:impl-start}}
    // ...

{{#include ../../../../../code/counter.rs:read-start}}
{{#include ../../../../../code/counter.rs:read-state}}

        // ...
{{#include ../../../../../code/counter.rs:read-end}}

    // ...
{{#include ../../../../../code/counter.rs:impl-end}}
```
State can return a value of any size including empty. \
Add this `match` expression that maps:
- empty to `0`.
- a single word to the count.
- anything else to an error.

Then return the count.
```rust
{{#include ../../../../../code/counter.rs:impl-start}}
    // ...

{{#include ../../../../../code/counter.rs:read}}

    // ...
{{#include ../../../../../code/counter.rs:impl-end}}
```
# Create a solution
Add this function (outside the impl) that takes the predicate address and the count we are trying to set the state to. \
The solution has a single `SolutionData` that solves this predicate (other solutions may solve multiple predicates). \
There's no `decision variables` or `transient data` so those are set to default. \
Add in a single state `Mutation`. The key is the `COUNTER_KEY` and the value is the new count.
```rust
{{#include ../../../../../code/counter.rs:solution}}
```
Back in the `impl App` add a method to create and submit a solution that will increment the count.
```rust
{{#include ../../../../../code/counter.rs:impl-start}}
    // ...

{{#include ../../../../../code/counter.rs:increment}}

    // ...
{{#include ../../../../../code/counter.rs:impl-end}}
```


<details>
<summary>Check your `lib.rs` matches this.</summary>

```rust
{{#include ../../../../../code/counter.rs:full}}
```
</details>