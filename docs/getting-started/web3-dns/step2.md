# Web3 DNS Tutorial


## Step 2 - Implementing the contract
Now that we have the project structure, lets implement the contract.

### First, lets start creating an event

```rust
    #[event("message")]
    fn message(&self, msg: &str);

```
This is a simple event. It will emit a message when called. The event name is `message` and it has one parameter called `msg` of type `&str`.
Note that we dont need to implement the function body. The compiler will do that for us. It will automatically emit the event when the function is called.

### Now, lets create a function that will emit the event

```rust
    #[endpoint]
    fn get_message(&self) -> SCResult<()> {
        self.message("Hello World!");
        Ok(())
    }
```

This is a simple function that will emit the event we just created. Note that we are using the `view` annotation. That means, this function can be called by anyone at any time. Also, note that we are calling the `message` function we just created.

These two functions are more than enough to create a simple contract. The first will be responsible for emitting the event and the second will be responsible for calling the first when someone calls the second function.

[Step 3 - Compiling the contract](step3.md)