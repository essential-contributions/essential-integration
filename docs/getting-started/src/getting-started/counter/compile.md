# Compile
Now that we have our predicate written, we can compile the code using the `pint` tool.

```bash
{{#include ../../../../code/pint.sh:build}}
```
This will create a new directory inside the `contract` directory called `out`. \
Because this is a debug build, you can find the compiled contract at `counter/contract/out/debug/counter.json`.

This is the file you can sign and deploy.

There is also a `counter-abi.json` file in the same directory that contains the counter contract's ABI.

The ABI for the counter looks like this:
```json
{
  "predicates": [
    {
      "name": "",
      "vars": [],
      "pub_vars": []
    },
    {
      "name": "::Increment",
      "vars": [],
      "pub_vars": []
    }
  ],
  "storage": [
    {
      "name": "counter",
      "ty": {
        "Int": [
          0
        ]
      }
    }
  ]
}
```
Note that yours may look slightly different depending on the version of the compiler you are using.

Now that you have compiled your counter contract you could go ahead and deploy it to the test server at `https://server.essential.builders`.

To do this you can use the `essential-deploy-contract` tool available in the [essential-integration](todo-put-link) repo.

It's very possible that someone else has already deployed this contract as contracts are stored via their content hash but don't worry, deploying the same contract twice will not cause any issues.

We will cover how to make contracts unique later on.

Deploying a contract doesn't really let you do much with it though. \
To interact with the contract you will need to create a front end application. \
This can be done in any language. We will demonstrate in the [Rust](https://www.rust-lang.org/) programming language.

It is not a requirement to use Rust in order to use `Essential`, we just happen to be Rust devs. The following section is completely optional but you may find it useful to see how we interact with the contract (even if you don't know Rust).