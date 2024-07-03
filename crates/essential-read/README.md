# Essential Read

This is a Rust library that contains utilities for reading and deserializing contracts and solutions.
Functions skip subdirectories, non-JSON files and files that have non-valid UTF-8 names.

Return types are all wrapped in `anyhow::Result`, omitted in the rest of the README for simplicity.

## Read Contract

Reads and deserializes predicates from a file.

Argument:

-   File as `PathBuf`.

Returns:

-   `Vec<Predicate>`, that is, a contract.

## Read Contracts

Reads and deserializes contracts in a directory.

Argument:

-   Directory as `PathBuf`.

Returns:

-   `Vec<Vec<Predicate>>`, where the inner vector is a single contract.

## Read Solution

Reads and deserializes a solution from a file.

Argument:

-   File as `PathBuf`.

Returns:

-   `Solution`.

## Read Solutions

Reads and deserializes solutions in a directory.

Argument:

-   Directory as `PathBuf`.

Returns:

-   `Vec<Solution>`.

## Read Bytes

Reads the contents of a file as bytes.
This function can be used in cases where deserialization is not necessary.

Argument:

-   File as `PathBuf`.

Returns:

-   `Vec<u8>`.

## Read Bytes (Directory)

Reads the contents of files in a directory as a vector of bytes.
This function can be used in cases where deserialization is not necessary.

Argument:

-   Directory as `PathBuf`.

Returns:

-   `Vec<Vec<u8>>`, where each inner vector is the contents of a file.

## Deserialize Contract

Deserializes a contract from bytes.

Argument:

-   `Vec<u8>`

Returns:

-   `Vec<Predicate>`, that is, a contract.

## Deserialize Solution

Deserializes a solution from bytes.

Argument:

-   `Vec<u8>`

Returns:

-   `Solution`
