# Compile and deploy
Now we will create a test that compiles, signs and deploys the counter app on a locally running `essential-server`.

Start by adding the imports you will need.
```rust
{{#include ../../../../../code/counter-test.rs:use}}
```
Add a tokio test because we are using async code.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}

{{#include ../../../../../code/counter-test.rs:test-end}}
```
Run a local `essential-server` in the background.
> Note that this requires that the `essential-server` binary be your `$PATH`.
See the [installation](../../installation/index.md) section for more details.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
{{#include ../../../../../code/counter-test.rs:setup}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
## Compile
Compile the pint project.
> Note that this requires that the `pint` binary be your `$PATH`.
See the [installation](../../installation/index.md) section for more details.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:compile}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
Create the `PredicateAddress`. This is the `ContentAddress` of the overall contract and the `ContentAddress` if the predicate.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:address}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
> Our contract only has a single predicate but other contracts can have multiple  predicates and therefor multiple `PredicateAddress`s.
## Sign and deploy
Using the `essential-wallet` create a temporary wallet and a new key for `alice` using the `Secp256k1` scheme. \
This is the key that you will use to sign the contract before deploying it.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:key}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
> Note that the `essential-wallet` has not been audited and is only for testing purposes. In this test we create a temporary key that is deleted at the end of the test. \
To interact with the hosted `essential-server` you will want to create a key pair that's stored locally using `essential-wallet`. \
Our wallet is just a convenience for testing.

Sign and deploy the contract using `alice`'s key.
```rust
{{#include ../../../../../code/counter-test.rs:test-start}}
    // ...

{{#include ../../../../../code/counter-test.rs:deploy}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
Now we are done getting everything setup and deployed. \
In the next section we will interact with the deployed contract.

<details>
<summary>Check your `tests/counter.rs` matches this.</summary>

```rust
{{#include ../../../../../code/counter-test.rs:use}}

{{#include ../../../../../code/counter-test.rs:test-start}}
{{#include ../../../../../code/counter-test.rs:p1}}
{{#include ../../../../../code/counter-test.rs:test-end}}
```
</details>