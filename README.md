# The Klever Rust Tool Set


[![Build Status](https://img.shields.io/github/actions/workflow/status/klever-io/klever-vm-sdk-rs/actions.yml?branch=master)](https://github.com/klever-io/klever-vm-sdk-rs/actions/workflows/actions.yml?query=branch%3Amaster) [![Dependency Status](https://deps.rs/repo/github/klever-io/klever-vm-sdk-rs/status.svg)](https://deps.rs/repo/github/klever-vm-sdk-rs) [![Contributors](https://img.shields.io/github/contributors/klever-io/klever-vm-sdk-rs)](https://github.com/klever-io/klever-vm-sdk-rs/graphs/contributors)

This repository contains a wide variety of tools, aimed primarily at smart contract developers.

The repo contains:
- The most complete smart contract framework on Klever:
    - The base framework;
    - A complete build system, which relies on the smart contract code directly;
    - A powerful debugger, based on a partial implementation of the Klever VM, in Rust.
    - A framework for writing both black-box and white-box tests. They rely on the standard Klever blockchain scenario format.
    - The official data serializer and deserializer for smart contract data. Can be used both on- and off-chain.
- A large collection of smart contract examples and feature tests, together with some of the core smart contracts used on the blockchain
- A framework for interacting with the blockchain, based on the smart contract logic, especially suitable for developers.
- A code snippet generator.

# Documentation & Getting started

The documentation can be found [here](docs/README.md)

# Building contracts

A comprehensive build guide can be found here: https://docs.klever.com/developers/developer-reference/sc-build-reference/

# Debugging contracts

The debugger guide: https://docs.klever.com/developers/developer-reference/sc-debugging/
