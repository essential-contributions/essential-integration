# essential-rest-client

Version: 0.1.0
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
[actions-url]: https://github.com/essential-contributions/essential-integration/actions

This is a Rust library and CLI tool that allows you to easily make rest requests to `essential-node` and `essential-builder`.

```
Essential REST Client

Usage: essential-rest-client [NODE_ADDRESS] [BUILDER_ADDRESS] <COMMAND>

Commands:
  Node Commands:
      get-contract                 Get a contract
      get-predicate                Get a predicate
      list-blocks                  List blocks in the given range
      list-contracts               List contracts in the given block range
      query-state                  Query state at contract address and key
  Builder Commands:
      submit-solution              Submit a solution
      latest-solution-failures     Get the latest failures for a solution
  help                             Print this message or the help of the given subcommand(s)

Arguments:
  [NODE_ADDRESS]  Optional node address to bind to
  [BUILDER_ADDRESS]  Optional builder address to bind to

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Essential REST Client

This library provides a client for interacting with the Essential node and Essential builder.

### Essential Node

#### Get Contract

This allows retrieving a deployed contract.
Very similar to `Get Predicate` but gets you the entire contract.

#### Get Predicate

This allows retrieving a deployed predicate.
It might be useful to do this if you want to debug a solution.

#### List Blocks

This allows listing all blocks in the given block range.
Blocks are only created if there are valid solutions.
Blocks are created on a regular interval.

#### List Contracts

This allows listing all deployed contracts in the given block range.

#### Query State

This allows querying the state of a contract.
It is the main way the front end application will interact with state.
It only really makes sense to query state where you know what the ABI of the contract is.
The state that's returned is a list of words of variable size.
The keys are also variable sized lists of words.
To make use of this API you need to know what type of contract you are querying.

### Essential Builder

#### Submit Solution

This allows submitting a solution to be included in an upcoming block.
Once a solution is submitted it is added to the pool.
The block builder runs on a regular loop interval and will include the solution in a block in FIFO order if it satisfies the constraints.

The block builder is likely to become more sophisticated in the future.

Note that currently if you submit a solution that conflicts with another solution then whichever solution is submitted first will be included in the block and the other solution will fail. Failed solutions are not retried and will eventually be pruned.

A solution can conflict with another solution when one solution is built on top of pre-state that the other solution changes. For example if a counter can only increment by 1 and is currently set to 5 then you submit a solution setting it to 6 but another solution is submitted before yours that sets the counter to 6 then your solution will fail to satisfy the constraints.
In fact in this example your solution will never satisfy again unless you update the state mutation to the current count + 1. But to do this you have to resubmit your solution.

Submitting the same solution twice (even by different user) is idempotent.

#### Latest Solution Failures

This allows querying the latest failures of a solution.
A solution is either successfully included in a block or it fails with a reason.

One thing to keep in mind is solutions are not necessarily unique.
It is possible for the same solution to be submitted multiple times.
For example if the counter example also allowed decrementing by 1 then a solution could increment the count from 4 to 5 and another solution could decrement the count from 5 to 4.
Then a solution that increments the count from 4 to 5 could be submitted again.
These two solutions would have the exact same content address.
This results in the same solution hash returning multiple outcomes.

This might make it difficult to know if it was the solution that you submitted that
was successful or failed. But actually it doesn't really matter because there is no
real ownership over a solution. Remember if two of the same solution are submitted
at the same time then it is as if only one was submitted.

If you are interested in "has my solution worked" then it probably makes more
sense to query the state of the contract that you were trying to change.
