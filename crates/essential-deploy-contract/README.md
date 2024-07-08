# Essential Deploy Contract 
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![license][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/essential-deploy-contract.svg
[crates-url]: https://crates.io/crates/essential-deploy-contract
[docs-badge]: https://docs.rs/essential-deploy-contract/badge.svg
[docs-url]: https://docs.rs/essential-deploy-contract
[apache-badge]: https://img.shields.io/badge/license-APACHE-blue.svg
[apache-url]: LICENSE
[actions-badge]: https://github.com/essential-contributions/essential-integration/workflows/ci/badge.svg
[actions-url]:https://github.com/essential-contributions/essential-integration/actions

This is a rust library and cli tool that allows you to easily deploy your contract to the server.
Additionally it provides functionality to sign the contract if it's not already signed.


```
Usage: essential-deploy-contract <COMMAND>

Commands:
  create-account  
  deploy-signed   
  deploy          
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
## Create Account
This command allows you to create a new account in your `essential-wallet` if you don't have one already. \
You can then use this account to sign and deploy your contract.

```
Usage: essential-deploy-contract create-account [OPTIONS] <ACCOUNT>

Arguments:
  <ACCOUNT>  The name of the account to create

Options:
  -p, --path <PATH>  Set the path to the wallet directory. If not set then a sensible default will be used (like ~/.essential-wallet)
  -h, --help         Print help
```
### Example
```bash
essential-deploy-contract create-account "alice"
```
## Deployed Signed
This command allows you to deploy a contract that you've already signed and serialized as json. \
The json contract should be saved as a file. \
The file should deserialize into a `SignedContract`.
```
Usage: essential-deploy-contract deploy-signed <SERVER> <SIGNED_CONTRACT>

Arguments:
  <SERVER>          The address of the server to connect to
  <SIGNED_CONTRACT>  The path to the signed contract to deploy. Serialized as json

Options:
  -h, --help  Print help
```
### Example
```bash
essential-deploy-contract deploy-signed 0.0.0.0:8080 ./signed_contract.json
```
## Deploy
This command allows you to sign an unsigned contract using your `essential-wallet` account and then deploy them to the server. \
The unsigned contract should be saved as a file as json. \
The file should deserialize into a `Contract`.
```
Usage: essential-deploy-contract deploy [OPTIONS] <SERVER> <ACCOUNT> <CONTRACT>

Arguments:
  <SERVER>   The address of the server to connect to
  <ACCOUNT>  The name of the account to deploy the app with
  <CONTRACT>  The path to the unsigned contract to deploy. Serialized as json

Options:
  -p, --path <PATH>  Set the path to the wallet directory. If not set then a sensible default will be used (like ~/.essential-wallet)
  -h, --help         Print help
```
```bash
essential-deploy-contract deploy 0.0.0.0:8080 "alice" ./unsigned_contract.json
```