# Define Types

Let's define some new types we'll need in our contract in the `contract/src/token.pnt` file:
```pint
{{#include ../../../../code/token/token.pnt:types}}
```

The `MintAuth` type is a `union` of two possible types, themselves imported from `std::lib`: `Signed(SECP256K1)` and `Predicate(PredicateAddress)`.
This means that the `MintAuth` type can be either a `Signed` type or a `Predicate` type.
The `Signed` variant is used when we want to verify that a signature is valid to authorize an action. \
The `Predicate` variant is used when we want to verify that another predicate is satisfied to authorize an action.
