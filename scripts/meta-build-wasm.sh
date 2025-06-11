
#!/bin/sh

# builds all wasm targets

SC_META=./target/release/sc-meta

TARGET_DIR=$PWD/target

$SC_META all build --target-dir-all $TARGET_DIR --path ./contracts