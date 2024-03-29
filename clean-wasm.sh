#!/bin/sh

# cleans all wasm targets

cargo install klever-sc-meta

sc-meta all clean --path ./contracts
