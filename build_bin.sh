#!/bin/sh
TARGET_DIR=./sakuramml-bin

cargo build --release
# copy
mkdir -p $TARGET_DIR
cp README.md $TARGET_DIR/
cp target/release/sakuramml $TARGET_DIR/

echo "OK"
