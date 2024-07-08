temp_dir=$(mktemp -d)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Setup the counter contract project.
cd $temp_dir
mkdir counter
pint new --name counter counter/contract
cp "$SCRIPT_DIR/counter.pnt" "$temp_dir/counter/contract/src/contract.pnt"

cd counter
# ANCHOR: cargo-new 
cargo new --lib counter-app 
# ANCHOR_END: cargo-new
cd ..
find counter -type f -not -path "*/.git/*" -not -path "*/.gitignore"
cd counter

# Remove this `if` once crates are published.
if false; then
# ANCHOR: cargo-add
cd counter-app
cargo add essential-app-utils
cargo add essential-app-utils --features test-utils --dev
cargo add essential-delpoy-contract
cargo add essential-hash
cargo add essential-rest-client
cargo add essential-types
cargo add essential-wallet --features test-utils --dev
cargo add anyhow
cargo add tokio --features full
cargo add clap
# ANCHOR_END: cargo-add
fi

# Remove when crates are published.
cd counter-app
# cargo add essential-app-utils --git "ssh://git@github.com/essential-contributions/essential-integration.git"
# cargo add essential-app-utils --git "ssh://git@github.com/essential-contributions/essential-integration.git" --features test-utils --dev
# cargo add essential-deploy-contract --git "ssh://git@github.com/essential-contributions/essential-integration.git"
# cargo add essential-rest-client --git "ssh://git@github.com/essential-contributions/essential-integration.git"
# cargo add essential-hash --git "ssh://git@github.com/essential-contributions/essential-base.git"
# cargo add essential-types --git "ssh://git@github.com/essential-contributions/essential-base.git"
# cargo add essential-wallet --git "ssh://git@github.com/essential-contributions/essential-wallet.git" --features test-utils --dev
# cargo add anyhow
# cargo add tokio --features full
# cargo add clap

cat << EOF > Cargo.toml
[package]
name = "counter-app"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.86"
clap = { version = "4.5.4", features = ["derive"] }
essential-app-utils = { git = "ssh://git@github.com/essential-contributions/essential-integration.git", version = "0.1.0" }
essential-deploy-contract = { git = "ssh://git@github.com/essential-contributions/essential-integration.git", version = "0.1.0" }
essential-hash = { git = "ssh://git@github.com/essential-contributions/essential-base.git", version = "0.1.0" }
essential-rest-client = { git = "ssh://git@github.com/essential-contributions/essential-integration.git", version = "0.1.0" }
essential-types = { git = "ssh://git@github.com/essential-contributions/essential-base.git", version = "0.1.0" }
tokio = { version = "1.38.0", features = ["full"] }

[dev-dependencies]
essential-app-utils = { git = "ssh://git@github.com/essential-contributions/essential-integration.git", features = ["test-utils"] }
essential-wallet = { git = "ssh://git@github.com/essential-contributions/essential-wallet.git", features = ["test-utils"] }
EOF

cat Cargo.toml

# ANCHOR: add-test
mkdir tests
touch tests/counter.rs
# ANCHOR_END: add-test

cd ../../
find counter -type f -not -path "*/.git/*" -not -path "*/.gitignore"
cp "$SCRIPT_DIR/counter.rs" "$temp_dir/counter/counter-app/src/lib.rs"
cd counter/counter-app
cargo check

cp "$SCRIPT_DIR/counter-test.rs" "$temp_dir/counter/counter-app/tests/counter.rs"

# ANCHOR: cargo-test
cargo test
# ANCHOR_END: cargo-test

echo "$temp_dir"