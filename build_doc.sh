#!/bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)
# wasm-pack build --target web

echo "--- build doc ---"
cd $SCRIPT_DIR/src
cnako3 batch_extract_command.nako3

