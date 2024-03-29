# Web3 DNS Tutorial


## Step 1 - Creating the project
To create a new project, we will use the `sc-meta` tool. This tool will create the project structure and the necessary files to start developing.

> **Note:**
> For now, keep your project in the `examples` folder. We have some relative paths in the project that will not work if you move it to another folder.

To create a new project, run the following command:
```bash
sc-meta new --name web3-dns --template empty
```

> **Note:**
> If you have problems with sc-meta or it is not installed yet, you can create your project copying the `empty` from the examples and changing the references for your project name.

It will create a new folder called `web3-dns` with the following structure:
```text
├── Cargo.toml
├── kleverkapp.json
├── meta
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── src
│   └── web3_dns.rs
└── wasm
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        └── lib.rs

```

Lets understand the folder structure
- `Cargo.toml`: This is the main file of the project. It contains the project metadata and dependencies.
- `kleverkapp.json`: This file contains the project metadata. It is used by the `sc-meta` and `operator` to deploy the contract.
- `meta`: This folder contains the contract metadata. 
- `src`: This folder contains the contract source code.
- `wasm`: This folder contains the contract reference endpoints to be compiled to wasm.

Feel free to explore the files and folders.

[Step 2 - Implementing the contract](step2.md)