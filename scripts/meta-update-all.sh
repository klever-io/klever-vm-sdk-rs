#!/bin/bash

SC_META=./target/release/sc-meta

TARGET_DIR=$PWD/target

$SC_META all update --target-dir-all $TARGET_DIR --path ./contracts