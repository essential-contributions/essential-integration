# essential-integration

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

Integration of the Pint constraint language and the Essential protocol.

## Goals

The goal for this repo is to demonstrate (and test) the process of creating
end-to-end Essential applications via the command line.

The developer experience can be broken into the following stages:

1. **build**: Write and compile the contract with Pint.
2. **run sever**: Start an [essential-rest-server](https://github.com/essential-contributions/essential-server/tree/main/crates/rest-server) instance running or use `https://server.essential.builders`.
2. **sign and deploy**: Sign and deploy the contract to using [essential-deploy-contract](./crates/essential-deploy-contract/README.md).
4. **solve**: Create a solution and submit it to the `essential-rest-server`,
   updating the state as a result.

## Language Agnostic Counter App Example

Users should not require any knowledge beyond basic command line tooling to get
started with Pint and Essential.

In turn, we avoid letting Rust (or any other language besides Pint) leak into
the repository in order to emulate the experience of writing apps purely using
CLi tools.

The included Rust `essential-cli` crate contains a few small commands that
should be extracted from this repo into a more general-use essential tooling
suite.

## Example Applications
Both these example applications use [Rust](https://www.rust-lang.org/) as their front end. They are much more in depth then the counter app example. This should give a good idea of how to build a more complex application using the Essential protocol.
- [NFT](./apps/nft/README.md)
- [Token](./apps/token/README.md)

## Using Nix

A Nix flake is included providing the `essential-all` package, providing `pint`, `pintc`,
`pintfmt`, `essential-rest-server` and more. There is also the `essential-minimal` package which contains just the bare minimal requirements to build an application.

1. Install Nix, easiest with the [Determinate Systems installer](https://github.com/DeterminateSystems/nix-installer).

2. Use Nix to enter a shell with the all Essential tools:
   ```console
   nix shell git+ssh://git@github.com/essential-contributions/essential-integration
   ```
   To include these along with the `jq` and `xxd` tools required to run included
   test scripts, use:
   ```console
   nix develop github:essential-contributions/essential-integration
   ```
   or if you have the repo cloned locally, simply `cd` in and use:
   ```console
   nix develop
   ```
