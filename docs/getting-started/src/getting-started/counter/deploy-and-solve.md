# Deploy & Solve

Now that we've built our simple counter contract, let's deploy it to a local
test network, and update the counter by solving our `Increment` predicate.

Before we begin, we'll make the `essential-builder`, `pint`, and
`essential-rest-client` tools available to our current shell:

```
nix shell github:essential-contributions/essential-integration#essential
```

## Running a Test Builder

Before we can deploy our contract, we need somewhere to deploy it to. Let's run
a local, in-memory test instance of the `essential-builder` tool.

The builder will run forever so let's run it in a seperate terminal. Open a
new terminal, run the nix shell command above to make the builder tool
available, and run it like so:

```console
essential-builder --node-api-bind-address "0.0.0.0:3553" --builder-api-bind-address "0.0.0.0:3554"
```

Now we're running a local instance of the builder. We should see some output
like the following:

```console
2024-11-14T11:58:14.359260Z  INFO essential_builder_cli: Initializing node DB
2024-11-14T11:58:14.364424Z  INFO essential_builder_cli: Starting node API server at 0.0.0.0:3553
2024-11-14T11:58:14.364439Z  INFO essential_builder_cli: Initializing builder DB
2024-11-14T11:58:14.365863Z  INFO essential_builder_cli: Starting builder API server at 0.0.0.0:3554
2024-11-14T11:58:14.365887Z  INFO essential_builder_cli: Running the block builder
```

We can see that the builder exposes 2 APIs:

1. **A node API** at **localhost** on **port 3553**. This allows for querying
   the state of our local single-node blockchain.
2. **A builder API** at **localhost** on **port 3554**. This allows for
   submitting solutions to have them included in a block.

> **Note:** If we do not specify the `--node-api-bind-address` or
> `--builder-api-bind-address` options, the builder will randomly select
> available ports for the node and builder APIs respectively.

> **Tip:** To see more detailed logs from the builder, we can run it with the
> `RUST_LOG` environment variable set to `debug` or `trace`, e.g.
> ```console
> RUST_LOG=trace essential-builder
> ```


## Contract Deployment

Using the `pint deploy` plugin, we can deploy our built counter
contract to the local test builder:

```console
pint deploy --builder-address "http://127.0.0.1:3554" --contract "./out/debug/counter.json"
```

Upon success, the builder will send us the content address of the solution used
to deploy the contract. We'll learn more about solutions in the following
sections. For now, this is enough to know our contract is deployed!


## Solving Predicates

In order to update contract state, a solution must solve one of the contract's
predicates.

Let's try updating our contract's `counter` state by solving its `Increment`
predicate.

A solution requires 3 components:

1. **`predicate_to_solve`**: The full address of the predicate we wish to
   solve. This includes both the contract and predicate content addresses. If
   we look back at our `pint build` step, we'll notice that these addresses are
   printed to the command
   line:
   ```console
   $ pint build
      Compiling counter [contract] (/Users/mindtree/Desktop/counter/contract)
       Finished build [debug] in 13.38825ms
       contract counter            1899743AA94972DDD137D039C2E670ADA63969ABF93191FA1A4506304D4033A2
            └── counter::Increment 355A12DCB600C302FFD5D69C4B7B79E60BA3C72DDA553B7D43F4C36CB7CC0948
   ```
   Note that your addresses might be slightly different if using a newer
   compiler version.

2. **`predicate_data`**: A list of input data values (aka "decision variables"
   in pint) that are expected by the predicate. Our `Increment` predicate takes
   no parameters, so here we can specify an empty list.

3. **`state_mutations`**: The state mutations we wish to propose. In our case,
   we want to initialise the `counter` storage variable to `1`.

   > **Note:** In state mutations, both keys and values are variable-sized
   > arrays of `Word`s. As `counter` is the only variable, we can assume it is
   > at key `[0]`. As the type of `counter` is `int`, we only need a single
   > `Word` to represent the value, e.g. `[1]`.

When represented as JSON, our full solution looks as follows:

```json
{
    "solutions": [
        {
            "predicate_to_solve": {
                "contract": "1899743AA94972DDD137D039C2E670ADA63969ABF93191FA1A4506304D4033A2",
                "predicate": "355A12DCB600C302FFD5D69C4B7B79E60BA3C72DDA553B7D43F4C36CB7CC0948"
            },
            "predicate_data": [],
            "state_mutations": [
                {
                    "key": [0],
                    "value": [1]
                }
            ]
        }
    ]
}
```

Lets put the above JSON in a `solutions.json` file.

> **Note:** We use the plural "solutions" here, as the following `pint submit`
> command actually supports submitting whole sets of solutions simultaneously.

To submit our solution to the local builder, we can now use the following command:

```
pint submit --builder-address "http://127.0.0.1:3554" --solutions "./solutions.json"
```

As confirmation that the builder received our solution, it responds with the
content address of the set.

However, this is not enough to know whether or not our solution was included in
a block, or whether it passed the contract's constraints at all.

To check whether or not our solution was successful, we can query the state of
our contract's `counter` storage variable using the builder's node API:

```console
pint query --node-address "http://127.0.0.1:3553" --contract-address "1899743AA94972DDD137D039C2E670ADA63969ABF93191FA1A4506304D4033A2" counter
```

Here, we're providing the address of the node API:

```
--node-address "http://127.0.0.1:3553"
```

the counter's contract address:

```
--contract-address "1899743AA94972DDD137D039C2E670ADA63969ABF93191FA1A4506304D4033A2"
```

and the name of the storage variable which we want to query (currently this is only supported for [scalar types](https://essential-contributions.github.io/pint/book/basics/data_types.html#scalar-types)):

```
counter
```

As an alternative to the name of the variable, you can provide an 8-byte hex-formatted key, as such:

```
--key 0000000000000000
```

> **Tip**: Refer to [the relevant section](https://essential-contributions.github.io/pint/book/appendix/storage_keys.html) in The Book of Pint
> for more information on storage keys.

Upon success, the node responds with:

```
[1]
```

And that's it! We can continue to submit solutions and update state in this
manner - as long as our solutions satisfy the contract's predicates.

> **Tip**: Use `essential-rest-client --help` to find more useful queries,
> including `list-blocks` and `latest-solution-failures`.


## Constructing Solutions for Apps

Rather than manually writing JSON files to construct solutions and interact
with contracts, it will be more common to automatically create solutions from
application code.

In the following chapter, we will demonstrate how to do so with the
[Rust](https://www.rust-lang.org/) programming language.

It is not a requirement to use Rust in order to use `Essential`, we just happen
to be Rust devs. The following section is completely optional but you may find
it useful to see how we interact with the contract (even if you don't know
Rust).
