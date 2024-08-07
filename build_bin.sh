#!/bin/sh
TARGET_DIR=./sakuramml-bin
TARGET_ZIP=./mac-sakuramml-bin.zip

cargo build --release
# copy
mkdir -p $TARGET_DIR
cp README.md $TARGET_DIR/
cp README_ja.md $TARGET_DIR/
cp target/release/sakuramml $TARGET_DIR/

# zip
zip $TARGET_ZIP -r $TARGET_DIR
echo "OK"
