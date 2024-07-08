# Deploy to server
In this section you will learn how to deploy your counter app to the public server. The server is running at `https://server.essential.builders`.

Compared to the test this deploys the counter persistently to the test net. This means that the counter will be available to anyone who knows the contract address.

> Make sure you have [compiled](./compile.md) your app before deploying it. It may have changed since you last compiled.

enter the top level of your project:
```
counter/
```
Start by creating an account in essential wallet (you can skip this if you already have an account). Name your account something you will remember:
```bash
{{#include ../../../../code/deploy.sh:acc}}
```
You will be prompted with:
```bash
Enter password to unlock wallet:
```
If you have not yet created an essential wallet this will set your password for all keys stored locally in  the wallet. If you have already created a wallet you will need to enter the password you used to create it. If you have forgotten your password you delete the wallet at `~/.essential-wallet`. You will loose any keys you have already created but you can start over with a new password.

> **Warning** [Essential wallet](https://github.com/essential-contributions/essential-wallet?tab=readme-ov-file#warning) is for testing purposes only. Do not use it for production. It has never been audited and should not ever be used to store real value.

Now sign and deploy the counter app:
```bash
{{#include ../../../../code/deploy.sh:deploy}}
```
You will be prompted with:
```bash
Enter password to unlock wallet:
```
This is to unlock your wallet so the contract can be signed.
Then you should see something similar to:
```
Deployed contract to: B1D2E4A1CA7822903AF93E9D395ED7037A79AD8E10084BA25E75B18D6C92FAB8
```
*The address you see might be different.*