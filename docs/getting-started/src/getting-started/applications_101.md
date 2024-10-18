# Essential Applications 101

Essential applications at the highest level are composed of `contract`s. Although this terminology will be familiar to developers coming from imperative blockchain languages (e.g. Solidity), a _declarative_ contract is a fundamentally different thing.

Imperative "contracts" take a set of inputs, and update state as a side-effect of the execution of a sequence of opcodes over these inputs. In particular, a set of storage opcodes exist which directly act upon state.

You may have heard that Essential achieves state updates "_declaratively_"; without the need for execution. This means that from the point of view of an Essential application, things happen in reverse when compared to the imperative approach. We start with a (proposed) atomic state mutation (i.e. a set of proposed new state values), and then substitute those values into a contract to check their validity. A Pint program, then, exists to _validate a given state mutation against a set of predefined rules_. These predefined rules are what make up a Pint contract.

> We have said that the starting point for an Essential application is a state mutation. You may therefore be wondering where these state mutations come from. Discovery of optimal state mutations (or "solutions") is the responsibility of solvers. A solver may be a third-party entity which competes to find optimal solutions. It may also be simply a centralized program (e.g. a server, or a front-end app or wallet) which serves solutions for a specific application. The techniques solvers use to find optimal solutions (and the mechanism governing their inclusion in blocks) is beyond the scope of this guide. For now, it is sufficient to note that incentivized actors exist in the system to discover these solutions, and that this discovery occurs _off-chain_. We will see a simple solution later in this guide, when we come to test our application.

A Pint contract _may_ declare a storage block. If it does, this state belongs to that contract. In general, state can only be updated if the new values are validated by the contract which owns it.

> **Note**: A contract does not have to define a storage block. It may simply apply additional constraints to state mutations on other contracts. In this case, both the constraints of this contract _and the constraints contract which owns the state_ must be satisfied for a solution to be valid.

Validation occurs through the satisfaction of one of the contract's `predicate`s. You can think of predicates as "pathways to validity" for the contract: in order for the contract to be satisfied (and therefore, its state updated), _one_ of its predicates must be satisfied.

A predicate is a block of code comprising one or more `constraint`s. A constraint is simply a boolean expression which must evaluate to `True` for the predicate containing it to be satisfied. From a code organization point of view, a `predicate` may look a bit like a function. However, the distinction is very important. A `predicate` is in no sense "called" in the same way a function is. It is simply a target that individual solutions may seek to satisfy.

In the rest of this guide, we will see these concepts implemented in Pint for a simple counter application.

> **Note**: Dynamically sized [data types](https://essential-contributions.github.io/pint/book/basics/data_types.html) and [decision variables](https://essential-contributions.github.io/pint/book/basics/variables.html) are temporarily unsupported. Using hashes to staticize the data at hand may be a workaround for some use cases. Note that this limitation does not have to do with [dynamically-sized storage types](https://essential-contributions.github.io/pint/book/storage/dynamic.html). For more known issues and missing features, refer to the [relevant section](https://essential-contributions.github.io/pint/book/appendix/known_issues.html#missing-features) in the Pint book.
