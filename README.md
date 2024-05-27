# essential-integration

Integration of the Pint constraint language and the Essential protocol.

## Goals

The goal for this repo is to demonstrate (and test) the process of creating
end-to-end Essential applications via the command line.

The developer experience can be broken into the following stages:

1. **build**: Write and compile the intent set with Pint.
2. **sign**: Sign the compiled intent set (using `essential-cli`, included in
   this crate for now).
3. **deploy**: Deploy the application to an `essential-rest-server` instance.
4. **solve**: Create a solution and submit it to the `essential-rest-server`,
   updating the state as a result.

## Language Agnostic

Users should not require any knowledge beyond basic command line tooling to get
started with Pint and Essential.

In turn, we avoid letting Rust (or any other language besides Pint) leak into
the repository in order to emulate the experience of writing apps purely using
CLi tools.

The included Rust `essential-cli` crate contains a few small commands that
should be extracted from this repo into a more general-use essential tooling
suite.

## Using Nix

A Nix flake is included providing the `essential` package, providing `pintc`,
`pintfmt`, `essential-rest-server` and more.

1. Install Nix, easiest with the [Determinate Systems installer](https://github.com/DeterminateSystems/nix-installer).

2. Use Nix to enter a shell with the all Essential tools:
   ```console
   nix shell git+ssh://git@github.com/essential-contributions/essential-integration
   ```
   To include these along with the `jq` and `xxd` tools required to run included
   test scripts, use:
   ```console
   nix develop git+ssh://git@github.com/essential-contributions/essential-integration
   ```
   or if you have the repo cloned locally, simply `cd` in and use:
   ```console
   nix develop
   ```
