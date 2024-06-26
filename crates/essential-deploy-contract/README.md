# Essential Deploy Contract 
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
The json intents should be saved as a file. \
The file should deserialize into a `SignedSet`.
```
Usage: essential-deploy-contract deploy-signed <SERVER> <SIGNED_INTENTS>

Arguments:
  <SERVER>          The address of the server to connect to
  <SIGNED_INTENTS>  The path to the signed intents to deploy. Serialized as json

Options:
  -h, --help  Print help
```
### Example
```bash
essential-deploy-contract deploy-signed 0.0.0.0:8080 ./signed_intents.json
```
## Deploy
This command allows you to sign an unsigned contract using your `essential-wallet` account and then deploy them to the server. \
The unsigned intents should be saved as a file as json. \
The file should deserialize into a `Vec<Intent>`.
```
Usage: essential-deploy-contract deploy [OPTIONS] <SERVER> <ACCOUNT> <INTENTS>

Arguments:
  <SERVER>   The address of the server to connect to
  <ACCOUNT>  The name of the account to deploy the app with
  <INTENTS>  The path to the unsigned intents to deploy. Serialized as json

Options:
  -p, --path <PATH>  Set the path to the wallet directory. If not set then a sensible default will be used (like ~/.essential-wallet)
  -h, --help         Print help
```
```bash
essential-deploy-contract deploy 0.0.0.0:8080 "alice" ./unsigned_intents.json
```