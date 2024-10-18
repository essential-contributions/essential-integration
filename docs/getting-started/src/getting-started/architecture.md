# Essential Architecture
---
> **Note:**  
> While efforts are made to maintain the Builder / Node state, schema changes may require wiping the database, resulting in a reset of stored data.
---

### Builder

The **Essential Builder** is responsible for block construction within the network. It gathers proposed solutions (such as state changes or transactions), validates them, and assembles new blocks for the blockchain. The Builder ensures state consistency by maintaining pre- and post-state views during block construction.

### Node

The **Essential Node** is the backbone of the blockchain network, handling core operations like ledger management, block validation, state synchronization, and peer-to-peer communication. The Node processes and validates transactions while maintaining the integrity and synchronization of the network.

### Essential Integration

The **Essential Integration** includes the **Essential REST Client**, which provides tools to interact with both the Node and Builder for managing contracts and state.

#### Essential REST Client Usage

```bash
essential-rest-client [NODE_ADDRESS] [BUILDER_ADDRESS] <COMMAND>
```

#### Commands:

- **Node Commands**:
    - `get-contract`: Retrieve a contract.
    - `get-predicate`: Fetch a predicate.
    - `list-blocks`: List blocks within a range.
    - `list-contracts`: List contracts in a block range.
    - `query-state`: Query the state at a contract address and key.
  
- **Builder Commands**:
    - `submit-solution`: Submit a solution for validation.
    - `latest-solution-failures`: Get the latest solution failures.

#### Arguments:

- `[NODE_ADDRESS]`: Optional Node address.
- `[BUILDER_ADDRESS]`: Optional Builder address.

This client simplifies interactions with the blockchain, offering key functionality for querying state and managing solutions.

> **Note:** If the `pintc` changes how it compiles your application or there is a change to our assembly, your contracts will have a new address. This means that if you deploy an app with the same source as another app that was compiled with a different compiler then the two apps will have different addresses and separate state.
