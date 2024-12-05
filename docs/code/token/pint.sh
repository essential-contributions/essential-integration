#!/usr/bin/env bash
temp_dir=$(mktemp -d)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd $temp_dir || exit
# ANCHOR: new_pint
mkdir token
pint new --name token token/contract
# ANCHOR_END: new_pint

# Copy the counter contract to the contract directory.
cp "$SCRIPT_DIR/token.pnt" "$temp_dir/token/contract/src/contract.pnt"

# ANCHOR: build
cd token/contract
pint build
# ANCHOR_END: build

ls $temp_dir/token/contract/out/debug
cat $temp_dir/token/contract/out/debug/token-abi.json