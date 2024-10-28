# Compile
Now that we have our predicate written, we can compile the code using the `pint` tool.

```bash
{{#include ../../../../code/pint.sh:build}}
```
This will create a new directory inside the `contract` directory called `out`. \
Because this is a debug build, you can find the compiled contract at `counter/contract/out/debug/counter.json`.

There is also a `counter-abi.json` file in the same directory that contains the counter contract's ABI.

The ABI for the counter looks like this:
```json
{{#include ../../../../code/counter-abi.json}}
```
> Note that yours may look slightly different depending on the version of the compiler you are using.

To interact with the contract you will need to create a front end application. \
This can be done in any language. We will demonstrate in the [Rust](https://www.rust-lang.org/) programming language.

It is not a requirement to use Rust in order to use `Essential`, we just happen to be Rust devs. The following section is completely optional but you may find it useful to see how we interact with the contract (even if you don't know Rust).
