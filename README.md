# essential-integration

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![license][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

Integration of the Pint constraint language and the Essential protocol.

To learn more about Pint and Essential, check out [the docs][essential-docs].

## Overview

This repo hosts a set of apps that serve as both a test-bed and demonstration
of how to use the Pint and Essential tooling together to build, deploy and
solve contracts.

By running these app tests in CI, we ensure a compatible set of Pint and
Essential tooling versions, while allowing both upstream projects to progress
at their own pace.

## Rust Apps and ABI-gen

Several of the Rust applications make use of the `pint-abi-gen` crate, which
generates Rust types and functions from a provided compiled
`contract-abi.json`. To ensure these contracts are present during development,
you can run the following command to build all `pint` apps in the repo:

```
nix run .#compile-all-contracts
```

[crates-badge]: https://img.shields.io/crates/v/essential-rest-client.svg
[crates-url]: https://crates.io/crates/essential-rest-client
[docs-badge]: https://docs.rs/essential-rest-client/badge.svg
[docs-url]: https://docs.rs/essential-rest-client
[apache-badge]: https://img.shields.io/badge/license-APACHE-blue.svg
[apache-url]: LICENSE
[actions-badge]: https://github.com/essential-contributions/essential-integration/workflows/ci/badge.svg
[actions-url]:https://github.com/essential-contributions/essential-integration/actions
[essential-docs]: https://docs.essential.builders
