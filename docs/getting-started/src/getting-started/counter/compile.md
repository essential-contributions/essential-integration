# Compile
Now that we have our predicate written, we can compile the code using the `pint` tool.

```bash
{{#include ../../../../code/counter/pint.sh:build}}
```
This will create a new directory inside the `contract` directory called `out`. \
Because this is a debug build, you can find the compiled contract at `counter/contract/out/debug/counter.json`.

There is also a `counter-abi.json` file in the same directory that contains the counter contract's ABI.

The ABI for the counter looks like this:
```json
{{#include ../../../../code/counter/counter-abi.json}}
```
> Note that yours may look slightly different depending on the version of the compiler you are using.

In the next section, we'll learn how to run a local test node, deploy our contract, and update the onchain counter state by solving its predicate.
