# Change Log

This file contains a centralizes a trace of all published crate versions, with their changes in short.

## Versioning the crates

The `klever-vm-sdk-rs` repo contains many crates, grouped into several families. Crates in these families always have the same version with one another.

For brevity, the changelog will only mention a short version of their name.

They are:
- `klever-sc`, in short `sc`, the smart contract framework, 6 crates + 3 for contracts/modules:
	- `klever-sc`
    - `klever-sc-derive`
    - `klever-sc-meta`
    - `klever-sc-scenario`
    - `klever-sc-snippets`
    - `klever-sc-wasm-adapter`
    - `klever-sc-modules` - *standard contract modules*
- `klever-sc-codec`, in short `codec`, the serializer/deserializer, 2 crates:
	- `klever-sc-codec`
	- `klever-sc-codec-derive`
- `klever-chain-vm`, in short `vm`, a Rust VM implementation, 1 crate.
- `klever-chain-scenario-format`, in short `scenario-format`, scenario JSON serializer/deserializer, 1 crate.
- `klever-vm-sdk`, in short `sdk`, allows communication with the chain(s), 1 crate.



## [sc 0.45.0,codec 0.19.0,vm 0.6.0] - 2025-06-11

### Features
- Add KSC cloud release workflow and dependency updates (#8)
- Add manual workflow dispatch to release action with tag validation
- Add managed_get_kda_roles hook implementation
- Implement base structure for sft
- Add additional checks for check nft balance
- Add pause-admin-module

### Fixes
- fix: typos && typos checker workflow
- fix: CHANGELOG order

### Other Changes
- tools: Check dependencies script
- Add version bump helper script for automating version management
- Add scripts for building and updating wasm targets with sc-meta
- chore: KDA Roles improvements (#9)
- apply linting fixes
- Change implementation to be SFTMeta instead of SFTMetadata
- chore: add return to bet endpoint in dice sc
- Change ManagedOption type name on proxy generation
- chore: improved code formatting
- add kda roles helpers
- add scenario helpers to set kda burn and roles

## [sc 0.44.0, vm 0.5.3, codec 0.18.2] - 2025-04-07

- Support contract deletion (`delete_contract`) and new deploy-delete example
- Introduce `upgrade()` endpoint for on-chain state updates in smart contracts
- Replace `klever_sc::imports!()` macros with direct `use` statements across all contracts
- Introduce auto-generated proxy modules (e.g., `AdderProxy`, `LinkedListRepeatProxy`)
- Rewrite test scenarios to use builder-based transactions and new `.kleversc.json` artifacts
- Add benchmarks using new `mb_builder_benchmark()` for testing `ManagedBuffer` writes
- Cleanup: remove outdated wasm comments, unused macros, and internal features
- Upgrade dependencies: `syn`, `proc-macro2`, `sha2`, `base64`, and others
- MultiversX SDK sync upgrades: from `0.43.4` to `0.50.4`

## [sc 0.43.3, vm 0.5.2] - 2024-04-01 FIRST KLEVER RELEASE
- Initial release of the framework
- Rename crates to klever
- Refactor blockchain calls and builtin functions
- Main features at this time:
