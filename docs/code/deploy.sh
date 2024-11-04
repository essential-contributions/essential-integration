#!/usr/bin/env bash
set -eo pipefail

temp_dir=$(mktemp -d)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd $temp_dir
mkdir counter
pint new --name counter counter/contract

# Copy the counter contract to the contract directory.
cp "$SCRIPT_DIR/counter.pnt" "$temp_dir/counter/contract/src/contract.pnt"

cd counter/contract
pint build
cd ../

# Deploy the contract
# ANCHOR: deploy
essential-rest-client deploy-contract "https://node.essential.builders" "contract/out/debug/counter.json"
# ANCHOR_END: deploy
