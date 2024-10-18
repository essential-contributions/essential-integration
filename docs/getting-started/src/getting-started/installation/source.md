# Build from Source

## Build Pint

Once you have `Cargo` installed, you can build `Pint` from source.

```bash
git clone https://github.com/essential-contributions/essential-integration.git
cd essential-integration
```

To build the `pint` binary, run:

```bash
cargo build --release -p pint-cli
```

The binary will be located at `target/release/pint`.

To run the `pint` binary, use:

```bash
cargo run --release -p pint-cli
```

To install `pint` on your system path, run:

```bash
cargo install --path crates/pint-cli/
```

---

## Build Essential REST Client

Clone the REST client repository:

```bash
git clone https://github.com/essential-contributions/essential-integration.git
cd essential-integration/crates/essential-rest-client
```

To build the `essential-rest-client` binary, run:

```bash
cargo build --release -p essential-rest-client
```

The binary will be located at `target/release/essential-rest-client`.

To run the `essential-rest-client` binary, use:

```bash
cargo run --release -p essential-rest-client
```

To install `essential-rest-client` on your system path, run:

```bash
cargo install --path crates/essential-rest-client/
```

---

## Build Essential Wallet
> **Warning:** [Essential wallet](https://github.com/essential-contributions/essential-wallet?tab=readme-ov-file#warning) is for testing purposes only. Do not use it for production. It has never been audited and should not be used to store real value.


Clone the wallet repository:

```bash
git clone https://github.com/essential-contributions/essential-integration.git
cd essential-integration/crates/essential-wallet
```

To build the `essential-wallet` binary, run:

```bash
cargo build --release -p essential-wallet
```

The binary will be located at `target/release/essential-wallet`.

To run the `essential-wallet` binary, use:

```bash
cargo run --release -p essential-wallet
```

To install `essential-wallet` on your system path, run:

```bash
cargo install --path crates/essential-wallet/
```