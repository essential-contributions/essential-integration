# Essential Dry Run 
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![license][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/essential-dry-run.svg
[crates-url]: https://crates.io/crates/essential-dry-run
[docs-badge]: https://docs.rs/essential-dry-run/badge.svg
[docs-url]: https://docs.rs/essential-dry-run
[apache-badge]: https://img.shields.io/badge/license-APACHE-blue.svg
[apache-url]: LICENSE
[actions-badge]: https://github.com/essential-contributions/essential-integration/workflows/ci/badge.svg
[actions-url]:https://github.com/essential-contributions/essential-integration/actions
This is a Rust library and CLI tool that allows dry running solutions on Essential server.

The dry run is performed through `essential-rest-client` that sends requests to `essential-rest-server`.

Under the hood, `essential-server`:
- Simulates the state transitions proposed by the solution.
- Checks the constraints of all predicates that the solution points to.

The Rust library provides modifications of functions that accept deserialized `Contract`/`Solution` objects (as well as functions that accept paths like the CLI tool does).

```
Usage: essential-dry-run [ADDRESS] <COMMAND>

Commands:
  check-with-contracts
  check
  help                  Print this message or the help of the given subcommand(s)

Arguments:
  [ADDRESS]  Server address to bind to [default: http://0.0.0.0:0]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Dry Run 

Dry runs a solution on the server. 

Before simulating state transitions and checking constraints, the contracts are read from storage.

If you already have the contracts and do not need to read them from storage, use [Dry Run With Contracts](#dry-run-with-contracts) instead.

```
Usage: essential-dry-run dry-run --server <SERVER> --solution <SOLUTION>

Options:
      --server <SERVER>      The address of the server to connect to
      --solution <SOLUTION>  Path to solution
  -h, --help                 Print help
```
### Example
```bash
essential-dry-run dry-run --server http://0.0.0.0:8080 --solution a_solution.json
```

## Dry Run With Contracts
Dry runs a solution on the server without reading contracts from storage.

If you do not have the contracts that the solution points to, use [Dry Run](#dry-run) instead.

```
Usage: essential-dry-run dry-run-with-contracts --server <SERVER> --contracts <CONTRACTS> --solution <SOLUTION>

Options:
      --server <SERVER>        The address of the server to connect to
      --contracts <CONTRACTS>  Path to compiled contracts
      --solution <SOLUTION>    Path to solution
  -h, --help                   Print help
```
### Example
```bash
essential-dry-run dry-run --server http://0.0.0.0:8080 --contracts src/contracts --solution a_solution.json
```
