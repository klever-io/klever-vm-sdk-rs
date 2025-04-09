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

# Documentation 

Most documentation can be found at https://docs.klever.org/klever-vm

# Getting started

The crowdfunding tutorial is a great place to start: https://docs.klever.org/tutorials/crowdfunding

# IDE

The framework is designed to be easiest to use with the KleverChain IDE VSCode extension: https://marketplace.visualstudio.com/items?itemName=Klever-org.vscode-kvm-ide

You can find a KleverChain IDE tutorial at https://docs.klever.org/tutorials/kleverchain-ide

# Building contracts

A comprehensive build guide can be found here: https://docs.klever.org/smart-contracts/config-and-tooling/build-reference

# Testing contracts

The Smart Contracts Testing guide: https://docs.klever.org/smart-contracts/testing
