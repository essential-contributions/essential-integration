# Essential Applications

At the core, **Essential applications** are built around `contracts`. If you're familiar with imperative blockchain languages like Solidity, this terminology might sound familiar. However, a **declarative** contract in Essential is fundamentally different from its imperative counterpart.

## The Difference: Imperative vs. Declarative

In **imperative contracts**, a set of inputs is processed through a sequence of opcodes, which updates the state as a side effect. These contracts often rely on storage-related opcodes that directly manipulate the state.

However, **Essential applications** operate **declaratively**, meaning that state updates occur without direct execution. Unlike the imperative approach, Essential applications work in reverse. They begin with a proposed atomic state mutation — a set of proposed new state values — and then check their validity against a contract.

In short, a **Pint program** exists to **validate a given state mutation** against predefined rules. These rules form the core of a **Pint contract**.

In the rest of this guide, we will implement these concepts in **Pint** using a simple counter application. You’ll see how contracts, predicates, and constraints come together in practice.
