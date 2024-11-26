# Define the Storage
Define the storage for the token.
The storage block is where we define the data that the contract will store. \
Here, we define five types that this contract can store; 2 `mapping`s, 2 `b256` values and an `int`. 
Add the following to the `contract/src/token.pnt` file:

```pint
{{#include ../../../../code/token/token.pnt:token_storage}}
```
