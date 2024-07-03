# From source
### Cargo
There are instructions on how to install `Cargo` and `Rust` imperatively [here.](https://www.rust-lang.org/tools/install)
### Pint
Once you have `Cargo` installed you can build `Pint` from source.
```bash
git clone git@github.com:essential-contributions/pint.git
cd pint
```
To build the `pint` binary run:
```bash
cargo build --release -p pint
```
The binary will be located at `target/release/pint`.

To run the `pint` binary you can use:
```bash
cargo run --release -p pint
```
To install `pint` on your path you can run:
```bash
cargo install --path pint/
```
### Essential Server
Clone the server repo
```bash
git clone git@github.com:essential-contributions/essential-server.git
cd essential-server
```
To build the `essential-rest-server` binary run:
```bash
cargo build --release -p essential-rest-server
```
The binary will be located at `target/release/essential-rest-server`.

To run the `essential-rest-server` binary you can use:
```bash
cargo run --release -p essential-rest-server
```
To install `essential-rest-server` on your path you can run:
```bash
cargo install -p crates/rest-server/
```
## Optional
These are not strictly necessary but are useful for testing and deploying contracts.
### Essential Wallet
Clone the wallet repo
```bash
git clone git@github.com:essential-contributions/essential-wallet.git
cd essential-wallet
```
To build the `essential-wallet` binary run:
```bash
cargo build --release -p essential-wallet
```
The binary will be located at `target/release/essential-wallet`.

To run the `essential-wallet` binary you can use:
```bash
cargo run --release -p essential-wallet
```
To install `essential-wallet` on your path you can run:
```bash
cargo install -p crates/wallet/
```
### Essential Deploy Contract
Clone the integration repo
```bash
git clone git@github.com:essential-contributions/essential-integration.git
cd essential-integration
```
To build the `essential-deploy-contract` binary run:
```bash
cargo build --release -p essential-deploy-contract
```
The binary will be located at `target/release/essential-deploy-contract`.

To run the `essential-deploy-contract` binary you can use:
```bash
cargo run --release --p essential-deploy-contract
```
To install `essential-deploy-contract` on your path you can run:
```bash
cargo install -p crates/essential-deploy-contract/
```