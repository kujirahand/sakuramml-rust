#!/bin/sh

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
# wasm-pack build --target web

echo "--- build doc ---"
python3 "$SCRIPT_DIR/scripts/extract_command.py"
