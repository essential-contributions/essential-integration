## Storage and Contracts

A **Pint contract** may declare a **storage block**. If it does, the contract owns that state, and in general, state updates can only occur if the new values are validated by the contract.

> **Note**: A contract is not required to define a storage block. It may impose additional constraints on state mutations related to other contracts. In such cases, both the constraints of the current contract and the contract owning the state must be satisfied for a solution to be valid.