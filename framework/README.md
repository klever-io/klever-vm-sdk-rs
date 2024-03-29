# Klever Smart Contract Framework

The crates in this folder form the Klever smart contract framework.

They are as follows:
    - `klever-sc` - the base crate for smart contract libraries, it is the only dependency the smart contract code sees.
    - `klever-sc-derive` - procedural macros for friendlier SC code
    - `klever-sc-meta` - smart contract meta-programming: build system and other tools
    - `klever-sc-scenario` - the main testing tool, contracts are tested by via scenarios
    - `klever-sc-snippets` - base crate for tools that interact with the blockchain
    - `klever-sc-wasm-adapter` - the API that connects contracts to the WASM backend
