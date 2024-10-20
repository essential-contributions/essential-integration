# Essential Architecture

---

> **Note:**  
> While efforts are made to maintain the Builder / Node state, schema changes may require wiping the database, resulting in a reset of stored data.

---

### Builder

The **Essential Builder** is responsible for block construction within the network. It gathers proposed solutions (such as state changes or transactions), validates them, and assembles new blocks for the blockchain. The Builder ensures state consistency by maintaining pre- and post-state views during block construction.

### Node

The **Essential Node** is the backbone of the blockchain network, handling core operations like block validation, state synchronization, and peer-to-peer communication. The Node processes and validates transactions while maintaining the integrity and synchronization of the network.

### Essential Integration

The **Essential Integration** includes the **Essential REST Client**, which provides tools to interact with both the Node and Builder for managing contracts and state.

#### Essential REST Client Usage

``` bash
Usage: essential-rest-client <COMMAND>

Commands:
  list-blocks                     List blocks in the given block number range
  query-state                     Query the state of a contract
  deploy-contract                 Deploy a contract
  submit-solution                 Submit a solution
  latest-solution-failures        Get the latest failures for solution
```

This client simplifies interactions with the blockchain, offering key functionality for querying state and managing solutions.

> **Note:** If the `pintc` changes how it compiles your application or there is a change to our assembly, your contracts will have a new address. This means that if you deploy an app with the same source as another app that was compiled with a different compiler then the two apps will have different addresses and separate state.
