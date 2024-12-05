#!/usr/bin/env bash
temp_dir=$(mktemp -d)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Setup the counter contract project.
cd $temp_dir || exit
mkdir counter
pint new --name counter counter/contract
cp "$SCRIPT_DIR/counter.pnt" "$temp_dir/counter/contract/src/contract.pnt"
cd counter/contract || exit
pint build

front_end_temp_dir=$("$SCRIPT_DIR/front-end.sh" | tail -n 1)
cd $front_end_temp_dir/counter || exit
# ANCHOR: main
cd counter-app
touch src/main.rs
# ANCHOR_END: main
cp "$SCRIPT_DIR/counter-main.rs" "$front_end_temp_dir/counter/counter-app/src/main.rs"

# ANCHOR: read
cargo run -- read-count "https://node.essential.builders" "../contract"
# ANCHOR_END: read
# ANCHOR: inc
cargo run -- increment-count "https://node.essential.builders" "https://node.essential.builders" "../contract"
# ANCHOR_END: inc
# ANCHOR: read-again
cargo run -- read-count "https://node.essential.builders" "../contract"
# ANCHOR_END: read-again