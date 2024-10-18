# Essential Applications 101

At the core, **Essential applications** are built around `contract`s. If you're familiar with imperative blockchain languages like Solidity, this terminology might sound familiar. However, a **declarative** contract in Essential is fundamentally different from its imperative counterpart.

### The Difference: Imperative vs. Declarative

In **imperative contracts**, a set of inputs is processed through a sequence of opcodes, which updates the state as a side effect. These contracts often use storage-related opcodes that directly manipulate the state.

However, **Essential applications** operate **declaratively**, meaning that state updates occur without execution. In contrast to the imperative approach, Essential applications work in reverse. We begin with a proposed atomic state mutation — a set of proposed new state values — and then check their validity against a contract. 

Essentially, a **Pint program** exists to **validate a given state mutation** against predefined rules. These rules form the core of a **Pint contract**.

---

### State Mutations and Solvers

You may be wondering, **where do these state mutations come from?**

- **State mutations** (or "solutions") are discovered by **solvers**.
- Solvers may be third-party entities competing to find optimal solutions, or they could be centralized programs like servers, front-end apps, or wallets, which provide solutions for specific applications.
- The techniques used by solvers to discover these solutions, and the mechanism for including them in blocks, are outside the scope of this guide. For now, know that incentivized actors in the system discover these solutions **off-chain**.

Later in this guide, we’ll explore a simple solution when we test our application.

---

### Storage and Contracts

A **Pint contract** may declare a **storage block**. If it does, the contract owns that state, and in general, state updates can only occur if the new values are validated by the contract.

> **Note**: A contract is not required to define a storage block. It may impose additional constraints on state mutations related to other contracts. In such cases, both the constraints of the current contract and the contract owning the state must be satisfied for a solution to be valid.

---

### Predicates and Constraints

Validation of a contract’s state occurs through the satisfaction of one of the contract's **predicates**. 

- Think of **predicates** as "pathways to validity." For a contract to be satisfied (and for its state to be updated), **one** of its predicates must be met.
- A **predicate** consists of one or more **constraints**. A constraint is a boolean expression that must evaluate to `True` for the predicate to be satisfied.

From a code organization perspective, predicates might look like functions, but there's a key difference: predicates are not **called** like functions. Instead, they are simply targets that individual solutions attempt to satisfy.

---

In the rest of this guide, we will implement these concepts in **Pint** using a simple counter application. You’ll see how contracts, predicates, and constraints come together in practice.
