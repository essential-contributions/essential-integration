# Test
Continuing on from where we left off in the last test we will now read and increment the counter.

Continue adding to the bottom of the same test.
<details>
<summary>`tests/counter.rs` code from previous section.</summary>

```rust
{{#include ../../../../../code/counter-test.rs:use}}

{{#include ../../../../../code/counter-test.rs:test-start}}
{{#include ../../../../../code/counter-test.rs:p1}}

    // Add new code here.

{{#include ../../../../../code/counter-test.rs:test-end}}
```
</details>

Create a new `App` we defined earlier in the `lib.rs`.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:app}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
Read the current count value and assert that it's `0`.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:read}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
Increment the counter. Remember this creates and submits the solution.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:inc}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
Stepping out of the test function for a second. We want to check for new state however the state only changes once a solution has been included in a block. \
Add this `wait_for_change` function that will check if the state is the expected value. If it is not then it will wait one second and check again.
```rust
{{#include ../../../../../code/counter-test.rs:wait-fn}}
```
Back in the test function now let's use the `wait_for_change` we just wrote to wait for the count to reach `1`.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:wait}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
Exciting the your solution has successfully satisfied the constraints and the state has been mutated.

Now just to make sure let's increment it again.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:inc-again}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
And wait for the state to change.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:wait-again}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
The state has changed again to `2`!

<details>
<summary>Check your `tests/counter.rs` matches this.</summary>

```rust
{{#include ../../../../../code/counter-test.rs:full}}
```
</details>

## Run the test
Run the test and check it all works.
```bash
{{#include ../../../../../code/front-end.sh:cargo-test}}
```

Congratulations on building your first `essential` application. \
It may be a very simple example but this declarative way of creating applications is very powerful. \

In future sections we will dive into more complex and interesting applications.

This concludes the "hello world" introduction to the `essential` system.