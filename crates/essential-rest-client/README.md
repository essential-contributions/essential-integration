# essential-rest-client

Version: 0.2.0
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

Block and state related endpoints.

### Essential Builder

Solution submission related endpoints.
