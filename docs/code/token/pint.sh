#!/usr/bin/env bash

temp_dir=$(mktemp -d)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd $temp_dir || exit
# ANCHOR: new_pint
mkdir token 
mkdir token/contracts
pint new --name token token/contracts/token
# ANCHOR_END: new_pint

find token -type f -not -path "*/.git/*" -not -path "*/.gitignore" > $SCRIPT_DIR/out1.txt
cat $SCRIPT_DIR/out1.txt
