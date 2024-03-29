# Web3 DNS Tutorial


## Step 4 - Deploying the contract
Now that we have the contract compiled, lets deploy it to the blockchain.


To deploy a smart contract is very simple. We just need to call the `sc create` command and pass the contract address and the wasm file.

```bash
operator --key-file=<./yourWallet.pem> sc create <yourAddress> --wasm="./hello_world_wasm.wasm"
```
Note that you must have enough balance to deploy the contract.

This command will send a transaction to the blockchain to deploy the contract. If everything goes well, you will receive the transaction hash.

### Let's check if the contract was deployed
Once the transaction is confirmed, we can check if the contract was deployed by calling the `tx-by-id` command.

```bash
operator tx-by-id <tx-hash>
```

Look at the logs section of the tx. You should see something like this:

```json

        "address": "...",
        "contractId": 0,
        "events": [
          {
            "address": "klv1qqqqqqqqqqqqqpgqghmn00u47ejq8jk37a7484sn7au3aqn6c0nqqtwsgw",
            "identifier": "SCDeploy",
            "topics": [
              "0000000000000000050045f737bf95f66403cad1f77d53d613f7791e827ac3e6",
              "f64e21227e8df59be638d00acfafdeb70d6a678d6eee4d929cbb143bb1edc3e6"
            ],
            "data": [
              
            ]
          },
          ...
        ]
      }
    }
```
Note that the `identifier` field is `SCDeploy`. This is the event that is emitted when a contract is deployed. The event `address` field is the contract address.

[Step 5 - Calling the contract](step5.md)