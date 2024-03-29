# Klever SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/klever-vm-sdk)](https://crates.io/crates/klever-vm-sdk)

General purpose collection of tools & SDKs to interact with the Klever blockchain from Rust projects.

## Example

```rust
use klever_vm_sdk::blockchain::rpc::{CommunicationProxy, DEVNET_GATEWAY};

#[tokio::test]
async fn get_network_config() {
    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {:?}", network_config)
}
```

More examples in `./examples`.

## Acknowledgements

Project originally started by [Bicarus labs](https://github.com/bicarus-labs/elrond-sdk-erdrs), later integrated into the Klever official codebase.
