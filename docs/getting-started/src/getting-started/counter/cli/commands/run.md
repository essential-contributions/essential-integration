# Run
Create the `run` function that handles the execution of the chosen subcommand (ReadCount or IncrementCount). For ReadCount, it compiles the Pint project, queries the current count, and displays it. For IncrementCount, it compiles the project, queries the current count, creates an incremented solution, submits it to the builder, and displays the new count:
```rust
{{#include ../../../../../../code/counter-main.rs:run}}
```
Add the main entry point of the application. It uses tokio for asynchronous execution, parses the command-line arguments, and calls the run function to execute the appropriate command:
```rust
{{#include ../../../../../../code/counter-main.rs:main}}
```