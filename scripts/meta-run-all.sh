#!/bin/bash

cargo install klever-sc-meta

TARGET_DIR=$PWD/target

sc-meta all update --path ./contracts
