## counter

A simple constraint that allows for updating a counter in state under the
condition that the solver can provide the answer to 6 * 7.

The goal for this app is to be a simple-as-possible demonstration of how to
build, sign, deploy and solve a simple pint contract.

Run from the root of this repo with `nix run .#test-app-counter`.

## Abi Gen
When you first clone this repo or change the contracts the abi gen will not have the file it needs to run so rust will not compile the app. To fix this run the following command in the root of the repo.
```bash
nix run .#compile-all-contracts
```
Or if you are not using nix you can go to the directory of each contract and run the following command.
```bash
pint build
```
