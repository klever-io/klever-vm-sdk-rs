# Web3 DNS Tutorial


## Step 5 - Calling the contract
Now that we have the contract deployed, lets call it.

```bash
operator --key-file=<./yourWallet.pem> sc invoke <contractAddress> --message=get_message
```
This command will send a transaction to the blockchain to call the `get_message` function of the contract. If everything goes well, you will receive the transaction hash.

### Let's check if the contract was called
Once the transaction is confirmed, we can check if the contract was called by calling the `tx-by-id` command.

```bash
operator tx-by-id <tx-hash>
```

Look at the logs section of the tx. You should see something like this:

```json

        "address": "...",
        "contractId": 0,
        "events": [
          {
            "address": "...",
            "identifier": "get_message",
            "topics": [
              "6d657373616765"
            ],
            "data": [
              "48656c6c6f20576f726c6421"
            ]
          },
          ...
        ]
      }
    }
```
The `events` field contains all the events emitted by the contract. Each event has the following fields:
- `address`: The contract address that emitted that event.
- `identifier`: The event identifier. This is the name of the function that emitted the event.
- `topics`: The event arguments names. (Hex encoded)
- `data`: The event arguments values. (Hex encoded)

**So lets understand the example above.**

The contract emitted an event from a function called `get_message`. The event has 1 parameter, the parameter name is in the topics field and the parameter value is in the data field. So:
- The parameter name is `message` (Hex encoded: `6d657373616765`)
- The parameter value is `Hello World!` (Hex encoded: `48656c6c6f20576f726c6421`)

The parameter name will always match the array index with the value. So, `topic[0]` is the name of the first parameter and `data[0]` is the value of the first parameter.

### Congratulations!
You just created your first smart contract and deployed it to the blockchain. You also called the contract and read the event emitted by it.

