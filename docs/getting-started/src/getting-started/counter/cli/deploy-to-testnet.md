# Deploy to Testnet
In this section you will learn how to deploy your counter app to the public testnet builder. The builder is running at `https://node.essential.builders`.

To do this you can use the `pint-deploy` tool available in the [essential-integration](https://github.com/essential-contributions/essential-integration) repo.

Compared to the test this deploys the counter persistently to the testnet. This means that the counter will be available to anyone who knows the contract address.

> Make sure you have [compiled](./compile.md) your app before deploying it. It may have changed since you last compiled.

enter the top level of your project:
```
counter/
```
Now deploy the counter app:
```bash
{{#include ../../../../../code/counter/deploy.sh:deploy}}
```

> It's very possible that someone else has already deployed this contract as contracts are stored via their content hash but don't worry, deploying the same contract twice will not cause any issues.
