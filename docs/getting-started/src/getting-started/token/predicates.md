# Write Predicates

First, let's define a new type we'll name `BurnAuth` in the `contract/src/token.pnt` file:
```pint
{{#include ../../../../code/token/token.pnt:type_burn_auth}}
```

The `BurnAuth` type is a `union` of two possible types, themselves imported from `std::lib`: `Signed(SECP256K1)` and `Predicate(PredicateAddress)`.
This means that the `BurnAuth` type can be either a `Signed` type or a `Predicate` type.
`Signed` is what we used when we want to verify that a signature is valid to authorize an action. \
`Predicate` is used when we want to verify that another predicate is satisfied to authorize an action.

Now we'll add our first `predicate` called `Burn`.
```pint
{{#include ../../../../code/token/token.pnt:burn_start}}

{{#include ../../../../code/token/token.pnt:burn_end}}
```
Here, we've defined a new predicate called `Burn` with 3 arguments (known as predicate data). \
The first argument is the `key` which is the address of the account that is burning tokens. \
The second argument is the `amount` which is the number of tokens being burned. \
The third argument is the `BurnAuth` which is the authorization to burn tokens.

Next, we'll read some values from storage.
```pint
{{#include ../../../../code/token/token.pnt:burn_start}}
{{#include ../../../../code/token/token.pnt:read_storage_burn}}
{{#include ../../../../code/token/token.pnt:burn_end}}
```

Now let's add some constraints to our predicate.
```pint
{{#include ../../../../code/token/token.pnt:burn_start}}
{{#include ../../../../code/token/token.pnt:read_storage_burn}}

{{#include ../../../../code/token/token.pnt:burn_constraints}}
{{#include ../../../../code/token/token.pnt:burn_end}}
```
