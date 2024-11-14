# Rust Items Generation Using the ABI

The Application Binary Interface (ABI) is genearted after compiling your Pint project and lives
under `out/debug`. The ABI can be used to generate Rust types and modules that will help you
generate solutions. Add the following macro call produce all the required Rust items from the ABI:

```rust
{{#include ../../../../../../code/counter.rs:abi_gen}}
```
