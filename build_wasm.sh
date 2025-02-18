#!/bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)

cnako3 $SCRIPT_DIR/src/batch_version.nako3
wasm-pack build --target web

# update version to mmlbbs and picosakura
cnako3 $SCRIPT_DIR/update_version.nako3


