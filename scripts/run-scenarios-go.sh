#!/bin/bash

### Use this to build all contracts and test them using the VM.

./scripts/build-wasm.sh

cargo test --features run-go-tests
