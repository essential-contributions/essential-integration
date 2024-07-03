# Rust front end
In this section we will build a simple front end application in [Rust](https://www.rust-lang.org/) to interact with the counter contract.

As previously mentioned there is no reason you have to use Rust to build a front end application. \
You could use any language you like. \
This chapter is optional but you may find it useful to see how to interact with the contract (even if you don't know Rust).

## Install Rust
If you don't have Rust and Cargo installed you can follow the instructions [here](https://www.rust-lang.org/tools/install).

<details>
<summary>If you are using Nix you can follow these instructions to get Rust.</summary>

If you are using `nix` you can simply run the following command to launch a dev shell with all the necessary tools installed:
```bash
nix develop todo-put-essential-integration-url
```

Or you can create your own `flake` using:
```bash
cd counter
nix flake init
```
Something like this should get you going:
```nix
# TODO add nix flake example
```
</details>