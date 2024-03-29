# Web3 DNS Tutorial


## Step 3 - Compiling the contract
Now that we have the contract implemented, lets compile it to wasm.

Thanks to the `sc-meta` tool, we can compile the contract to wasm with a single command.
    
```bash
rustup override set nightly
rustup target add wasm32-unknown-unknown
sc-meta all build --target-dir ./target --path ./web3-dns
```

This command will compile the contract inside the `path` to wasm and place it in the `target` folder.

The `target` folder will have the following structure:
```text
target
└── wasm32-unknown-unknown
    └── release
        ├── web3_dns_wasm.d
        └── web3_dns_wasm.wasm
```
the `web3_dns_wasm.wasm` file is the compiled contract. This is the file that will be deployed to the blockchain.

[Step 4 - Deploying the contract](step4.md)