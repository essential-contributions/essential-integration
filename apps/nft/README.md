# NFT Example Application
This is an example of creating an NFT contract using the Pint constraint language.

The front end is written in Rust.

## Abi Gen
When you first clone this repo or change the contracts the abi gen will not have the file it needs to run so rust will not compile the app. To fix this run the following command in the root of the repo.
```bash
nix run .#compile-all-contracts
```
Or if you are not using nix you can go to the directory of each contract and run the following command.
```bash
pint build
```
