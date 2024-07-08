# {{crate}}
Verion: {{version}}
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

{{readme}}