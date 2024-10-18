# Deploy to testnet
In this section you will learn how to deploy your counter app to the public testnet builder. The builder is running at `https://bigbangblock.builders`.

Compared to the test this deploys the counter persistently to the testnet. This means that the counter will be available to anyone who knows the contract address.

> Make sure you have [compiled](./compile.md) your app before deploying it. It may have changed since you last compiled.

enter the top level of your project:
```
counter/
```
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
