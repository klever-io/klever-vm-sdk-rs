#!/bin/sh

# builds all wasm targets

cargo install klever-sc-meta

TARGET_DIR=$PWD/target

sc-meta all build --target-dir $TARGET_DIR --path ./contracts
