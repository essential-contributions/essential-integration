#!/usr/bin/env bash

temp_dir=$(mktemp -d)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd $temp_dir || exit
# ANCHOR: new_pint
mkdir counter
pint new --name counter counter/contract
# ANCHOR_END: new_pint

# Copy the counter contract to the contract directory.
cp "$SCRIPT_DIR/counter.pnt" "$temp_dir/counter/contract/src/contract.pnt"

# ANCHOR: build
cd counter/contract || exit
pint build
# ANCHOR_END: build

ls $temp_dir/counter/contract/out/debug
cat $temp_dir/counter/contract/out/debug/counter-abi.json