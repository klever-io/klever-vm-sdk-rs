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


## [sc 0.43.3, vm 0.5.2] - 2024-04-01 FIRST KLEVER RELEASE
- Initial release of the framework
- Rename crates to klever
- Refactor blockchain calls and builtin functions
- Main features at this time:
