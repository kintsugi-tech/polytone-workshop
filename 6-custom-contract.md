## Custom Interchain Query Contract

Now that we are sure IBC Queries between our two local chains are working, we can actually create a custom contract to take advantage of it and do some cool stuff.

## Setup CosmWasm Template

To create a basic CosmWasm contract, you can use the [standard template](https://github.com/CosmWasm/cw-template) provided by CosmWasm.

```
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script

cd ~/polytone-workshop
cargo generate --git https://github.com/CosmWasm/cw-template.git --name interchain_counter
```

Select "minimal template" when asked.

## Add our custom logic
Let's build a real smart contract that does an interchain query and use the result in its logic [here](./6-custom-contract.md)
