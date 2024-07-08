# essential-rest-client
Verion: 0.1.0
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![license][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/essential-rest-client.svg
[crates-url]: https://crates.io/crates/essential-rest-client
[docs-badge]: https://docs.rs/essential-rest-client/badge.svg
[docs-url]: https://docs.rs/essential-rest-client
[apache-badge]: https://img.shields.io/badge/license-APACHE-blue.svg
[apache-url]: LICENSE
[actions-badge]: https://github.com/essential-contributions/essential-integration/workflows/ci/badge.svg
[actions-url]:https://github.com/essential-contributions/essential-integration/actions

This is a rust library and cli tool that allows you to easily make rest requests to a `essential-rest-server`.

```
Essential REST Client

Usage: essential-rest-client [ADDRESS] <COMMAND>

Commands:
  deploy-contract                Deploy a contract to the server
  check-solution                 Check a solution against the server
  check-solution-with-contracts  Check a solution against the server with data
  submit-solution                Submit a solution to the server
  solution-outcome               Get the outcome of a solution
  get-predicate                  Get a predicate from a contract
  get-contract                   Get a contract from the server
  list-contracts                 List contracts on the server
  list-solutions-pool            List solutions in the pool on the server
  list-winning-blocks            List winning blocks on the server
  query-state                    Query the state of a contract
  query-state-reads              Query the state of a contract by running state read programs
  query-predicate                Query the state of a contract by running the state read programs in a predicate
  query-inline                   Query the state of a contract by running state read programs with a single solution data input
  query-extern                   Query the state of an external contract by running state read programs. This uses an empty solution that doesnt solve anything. It only makes sense to query state that is in an external contract
  help                           Print this message or the help of the given subcommand(s)

Arguments:
  [ADDRESS]  Server address to bind to [default: http://0.0.0.0:0]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Essential REST Client

This library provides a client for interacting with the Essential REST Server.

### Deploy Contract
Contracts must be signed before they can be deployed.
`essential-wallet` can be used to sign contracts.
Deploying the same contract multiple times is a no-op.

### Check Solution
This allows checking a lotion without it actually being included in a block.
The solution with use the pre-state that is currently on the server.
Any contracts that the solution solves must be deployed already.

This is useful when you want to check a solution before submitting it
and want to use the state that is currently on the server.

### Check Solution With Contracts
This allows checking a solution with the set of contracts it is solving.
All contracts that the solution solves must be included in the set of contracts.
The solution will use the state that is currently on the server.

This is useful when you are building contracts that aren't ready to be deployed
and you want to test them with a solution.

### Submit Solution
This allows submitting a solution to be included in an upcoming block.
Once a solution is submitted it is added to the pool.
The block builder runs on a regular loop interval and will include the solution in a block
in FIFO order if it satisfies the constraints.

The block builder is likely to become more sophisticated in the future.

Note that currently if you submit a solution that conflicts with another solution then
which ever solution is submitted first will be included in the block and the other solution
will fail. Failed solutions are not retried and will eventually be pruned.

A solution can conflict with another solution when one solution is built on top of pre state
that the other solution changes. For example if a counter can only increment by 1 and is
currently set to 5 then you submit a solution setting it to 6 but another solution is submitted
before yours that sets the counter to 6 then your solution will fail to satisfy the constraints.
In fact in this example your solution will never satisfy again unless you update the state mutation
to the current count + 1. But to do this you have to resubmit your solution.

Submitting the same solution twice (even by different user) is idempotent.

### Solution Outcome
This allows querying the outcome of a solution.
A solution is either successfully included in a block or it fails with a reason.

One thing to keep in mind is solutions aren't necessarily unique.
It's possible for the same solution to be submitted multiple times.
For example if the counter example also allowed decrementing by 1 then
a solution could increment the count from 4 to 5 and another solution could decrement the count from 5 to 4.
Then a solution that increments the count from 4 to 5 could be submitted again.
These two solutions would have the exact same content address.
This results in the same solution hash returning multiple outcomes.

This might make it difficult to know if it was the solution that you submitted that
was successful or failed. But actually it doesn't really matter because there is no
real ownership over a solution. Remember if two of the same solution are submitted
at the same time then it is as if only one was submitted.

If you are interested in "has my solution worked" then it probably makes more
sense to query the state of the contract that you were trying to change.

Keep in mind this is all very application specific.

### Get Predicate
This allows retrieving a deployed predicate.
It might be useful to do this if you want to debug a solution.

### Get Contract
This allows retrieving a deployed contract.
Very similar to `Get Predicate` but gets you the entire contract.

### List Contracts
This allows listing all deployed contracts.
The results are paged so you can only get a maximum number of contracts per query.
The contracts can also be filtered by the time range that they were deployed.

### List Solutions Pool
This allows listing all solutions currently in the pool.
The results are also paged.
Depending on the backlog of solutions an individual solution might not be in the pool for long.

### List Winning Blocks
This allows listing all blocks that have been successfully created.
The results are also paged.
The blocks can also be filtered by time.
Blocks are only created if there are solutions in the pool.
Blocks are created on a regular interval.

### Query State
This allows querying the state of a contract.
It is the main way the front end application will interact with state.
It only really makes sense to query state where you know what the abi of the contract is.
The state that's returned is a list of words of variable size.
The keys are also variable sized lists of words.
To make use of this api you need to know what type of contract you are querying.

### Query State Reads
This allows querying the state of a contract using state read programs.
This is a more advanced way of querying state.
It allows you to query the state of a contract using the state read programs from a predicate.
Custom state read programs can be also be written.
Pint can be used to create custom state reads.

This api is also very useful if you are trying to solve a predicate but need to know what the pre-state
that the solution will read is.
For example if you want to run a debugger you will need this pre-state.

The api can return which keys were read and which values were returned.
It can also return that values that were read into state slots on the pre-state read
and post-state read.

Note that it doesn't return the keys and values that were read on the post-state read
because it is trivial to compute this locally using the state mutations in the solution.
