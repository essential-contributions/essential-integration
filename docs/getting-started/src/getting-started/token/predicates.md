# Write Predicates

Predicates are the main building block of Pint contracts, and are how we enable others to interact with our contract. \
lets add some predicates to our token contract now.
We'll start with the Mint predicate:

```pint
{{#include ../../../../code/token/token.pnt:mint_start}}

{{#include ../../../../code/token/token.pnt:mint_end}}
```

Next, we'll read some values from storage:

```pint
{{#include ../../../../code/token/token.pnt:mint_start}}
{{#include ../../../../code/token/token.pnt:read_storage_mint}}
{{#include ../../../../code/token/token.pnt:mint_end}}
```

Now let's add some constraints to our `Mint` predicate:

```pint
{{#include ../../../../code/token/token.pnt:mint_start}}

    // ...

{{#include ../../../../code/token/token.pnt:mint_constraints}}
{{#include ../../../../code/token/token.pnt:mint_end}}
```

Now we'll add our second `predicate` called `Burn`.

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
    // ...

{{#include ../../../../code/token/token.pnt:burn_constraints}}
{{#include ../../../../code/token/token.pnt:burn_end}}
```

We can go ahead now and add the `Transfer` predicate:

```pint
{{#include ../../../../code/token/token.pnt:predicate_transfer}}
```

Finally, let's add the `Cancel` predicate:

```pint
{{#include ../../../../code/token/token.pnt:predicate_cancel}}
```

The only thing were missing now is the macro definitions for `@check_if_predicate_is_owner()`. These could also be imported from an library, but for now we'll just define them in our contract:

```pint
{{#include ../../../../code/token/token.pnt:macros}}

```pint
{{#include ../../../../code/token/token.pnt:check_if_predicate_is_owner}}
```

Your complete `contract/src/contract.pnt` file should look like this:

```pint
{{#include ../../../../code/token/token.pnt:token_complete}}
```
