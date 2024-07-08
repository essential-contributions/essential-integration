# Essential server
The essential server is a public centralized test server that implements the declarative constraint checking system. The server is built on top of the same software stack that the node will be using. This allows app developers and solvers who want to experiment with building declarative applications to get started in a realistic environment.

Functionally the server acts like a single block builder but does not post blocks to an L1.

The plan is to incrementally swap out parts of the server until there is a fully decentralized node and builder. We are aiming to keep the changes to the API and constraint checking system as minimal as possible although it is still early in the project and there will most likely be some breaking changes. These will be well documented and communicated so developers can easily update their applications.

The server is using a persistent database so that you can interact with other applications that have been deployed.

In the following section we will walk you through building a simple counter app. At first you will use the server running locally on your machine but then you will try using the public server.

The server has the same API as the local server you will be running. The API is defined [here.](https://github.com/essential-contributions/essential-server/blob/main/crates/rest-server/README.md#api)

## Address
The server is running at `https://server.essential.builders`. It only accepts HTTPS connections and requires HTTP/2. You don't need to worry about this unless you are writing your own client libraries.

> **Note:** You may be wondering what happens in you deploy the same app to the server as someone else. This is completely fine but you should keep in mind that the state of the app **doesn't** reset on a redeploy (redeploying is idempotent). So the state may already contain values from other users. It will be interesting to see what the count is up to once you go finish the tutorial.

> **Note:** Although we will make an effort to keep the state of the server, there may be times where the databases need to be wiped due to changes in the database schema.

> **Note:** If the `pintc` changes how it compiles your application or there is a change to our assembly you contracts will have a new address. This means that if you deploy an app with the same source as another app that was compiled with a different compiler then the two apps will have different addresses and separate state. 