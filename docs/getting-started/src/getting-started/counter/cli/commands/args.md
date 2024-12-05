# Args
Define the command-line interface structure using the clap library. Set up two subcommands: ReadCount and IncrementCount, each with their respective arguments. The Shared struct is used to define common arguments for both subcommands.
```rust
{{#include ../../../../../../code/counter/counter-main.rs:cli}}
```