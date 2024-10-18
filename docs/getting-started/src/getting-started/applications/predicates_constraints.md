## Predicates and Constraints

Validation of a contract’s state occurs through the satisfaction of one of the contract's **predicates**.

- Think of **predicates** as "pathways to validity." For a contract to be satisfied (and for its state to be updated), **one** of its predicates must be met.
- A **predicate** consists of one or more **constraints**. A constraint is a boolean expression that must evaluate to `True` for the predicate to be satisfied.

From a code organization perspective, predicates might look like functions, but there's a key difference: predicates are not **called** like functions. Instead, they are targets that individual solutions attempt to satisfy.