# essential-rest-client

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

#### Essential REST Client
```
Usage: essential-rest-client <COMMAND>

Commands:
  node     Commands for calling node functions
  builder  Commands for calling builder functions
```

#### Commands for Essential Node
```
Usage: essential-rest-client node <ADDRESS> <COMMAND>

Commands:
  list-blocks  List blocks in the given block number range
  query-state  Query the state of a contract

Arguments:
  <ADDRESS>  The endpoint of node to bind to
```

#### Commands for Essential Builder
```
Usage: essential-rest-client builder <ADDRESS> <COMMAND>

Commands:
  deploy-contract           Deploy a contract
  submit-solution           Submit a solution
  latest-solution-failures  Get the latest failures for solution

Arguments:
  <ADDRESS>  The endpoint of builder to bind to
```

## Essential REST Client

This library provides a client for interacting with the Essential node and Essential builder.

### Essential Node

Block and state related endpoints.

### Essential Builder

Contract deployment and solution submission related endpoints.
