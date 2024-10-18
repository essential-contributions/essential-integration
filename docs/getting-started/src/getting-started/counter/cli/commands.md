# Commands
If you have not yet created an essential wallet this will set your password for all keys stored locally in  the wallet. If you have already created a wallet you will need to enter the password you used to create it. If you have forgotten your password you delete the wallet at `~/.essential-wallet`. You will loose any keys you have already created but you can start over with a new password.

> **Warning:** [Essential wallet](https://github.com/essential-contributions/essential-wallet?tab=readme-ov-file#warning) is for testing purposes only. Do not use it for production. It has never been audited and should not ever be used to store real value.

Add in a `main.rs` file that will be used to run the counter-app CLI:
```bash
{{#include ../../../../../code/counter-cli.sh:main}}
```

Now in the `main.rs` file add the `use` statements:

```rust
{{#include ../../../../../code/counter-main.rs:use}}
```
Using the cli crate `clap` add two commands:
```rust
{{#include ../../../../../code/counter-main.rs:cli}}
```
The command `ReadCount` with become `read-count <SERVER> <PINT_DIRECTORY>` which will read the current count from the server. \
The command `IncrementCount` with become `increment-count <SERVER> <PINT_DIRECTORY>` which will increment the current count on the server. \
Both commands take the same arguments so we use a `Shared` struct to hold the arguments.

Add the main function to run the CLI:
```rust
{{#include ../../../../../code/counter-main.rs:main}}
```
This is fairly simple and just handles errors in a nice way for the user.

For both commands we need to compile the pint project to get the address of the predicate and create a new `App`. \
Add this helper function:
```rust
{{#include ../../../../../code/counter-main.rs:create}}
```

The core of the cli is the run function. \
It should handle each command and use the `App` to complete the actions like:
```rust
{{#include ../../../../../code/counter-main.rs:run}}
```

<details>
<summary>Check your `main.rs` matches this.</summary>

```rust
{{#include ../../../../../code/counter-main.rs:full}}
```
</details>