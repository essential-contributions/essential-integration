# Write the Predicate

Start by adding a new `predicate` called `Increment` to the `contract/src/contract.pnt` file:
```pint
{{#include ../../../../code/counter.pnt:pred_start}}

{{#include ../../../../code/counter.pnt:pred_end}}
```
### Storage
The first thing we want to do within this predicate is to read the `counter` storage value:
```pint
{{#include ../../../../code/counter.pnt:pred_start}}
{{#include ../../../../code/counter.pnt:read_storage}}
{{#include ../../../../code/counter.pnt:pred_end}}
```
### Constraint
Let's add our first constraint:
```pint
{{#include ../../../../code/counter.pnt:pred_start}}
{{#include ../../../../code/counter.pnt:read_storage}}

{{#include ../../../../code/counter_examples.pnt:constraint_simple}}
{{#include ../../../../code/counter.pnt:pred_end}}
```
A constraint is a boolean expression which must be true for the predicate to be satisfied. \
This constraint says that the post-state of the counter must be equal to the pre-state plus one.

**Note:** The post-state value of the counter storage variable is denoted by `counter'`. The trailing apostrophe `'` is the syntax for accessing the post-state value of a storage variable.

But there is a problem with this constraint. \
What if the pre-state of the counter is not yet set to anything?

We can use a `nil` check to handle this case:
```pint
{{#include ../../../../code/counter.pnt:pred_start}}
{{#include ../../../../code/counter.pnt:read_storage}}

{{#include ../../../../code/counter_examples.pnt:constraint_init}}
{{#include ../../../../code/counter.pnt:pred_end}}
```
This says that if the pre-state of the counter is `nil` then the post-state must be `1`.

Now let's put it all together:
```pint
{{#include ../../../../code/counter.pnt:pred_start}}
{{#include ../../../../code/counter.pnt:read_storage}}

{{#include ../../../../code/counter.pnt:constraint}}
{{#include ../../../../code/counter.pnt:pred_end}}
```
This constraint is satisfied if either the counter pre-state is `nil` and the post-state is `1`, or the counter pre-state is `n` and the post-state is `n + 1`.

Your complete `contract/src/contract.pnt` file should look like this:
```pint
{{#include ../../../../code/counter.pnt:counter_storage}}

{{#include ../../../../code/counter.pnt:pred_start}}
{{#include ../../../../code/counter.pnt:read_storage}}

{{#include ../../../../code/counter.pnt:constraint}}
{{#include ../../../../code/counter.pnt:pred_end}}
```
Congratulations on writing your first pint predicate!
