#!/usr/bin/env bash

temp_dir=$(mktemp -d)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Setup the counter contract project.
cd $temp_dir || exit
mkdir counter
pint new --name counter counter/contract
cp "$SCRIPT_DIR/counter.pnt" "$temp_dir/counter/contract/src/contract.pnt"

cd counter || exit
# ANCHOR: cargo-new
cargo new --lib counter-app
# ANCHOR_END: cargo-new
cd ..
find counter -type f -not -path "*/.git/*" -not -path "*/.gitignore"
cd counter || exit

# ANCHOR: cargo-add
cd counter-app || exit
cargo add essential-app-utils
cargo add essential-app-utils --features test-utils --dev
cargo add essential-deploy-contract
cargo add essential-hash
cargo add essential-rest-client
cargo add essential-types
cargo add essential-wallet --features test-utils --dev
cargo add anyhow
cargo add tokio --features full
cargo add clap --features derive
# ANCHOR_END: cargo-add

cat Cargo.toml

# ANCHOR: add-test
mkdir tests
touch tests/counter.rs
# ANCHOR_END: add-test

cd ../../
find counter -type f -not -path "*/.git/*" -not -path "*/.gitignore"
cp "$SCRIPT_DIR/counter.rs" "$temp_dir/counter/counter-app/src/lib.rs"
cd counter/counter-app || exit
cargo check

cp "$SCRIPT_DIR/counter-test.rs" "$temp_dir/counter/counter-app/tests/counter.rs"

# ANCHOR: cargo-test
cargo test -- --nocapture
# ANCHOR_END: cargo-test

echo "$temp_dir"
