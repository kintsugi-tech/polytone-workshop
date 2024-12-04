## Relayer setup

To handle cross chain comunication between Local Juno and Local Cosmos Hub, we need to setup a relayer. In this guide we'll use [hermes](https://github.com/informalsystems/hermes)

In this case we assume hermes is already installed and configured in your system, and we are going to run it directly without a docker container.

### Configuration

A configuration file is already provided in `./local-ic-config/hermes/config.toml`, this includes already all the configuration we need to run a relayer between our two local chains.

You can copy it in your local hermes base directory with this command: `cp ${ICTEST_HOME}/hermes/config.toml ~/.hermes/config.toml`

You can validate the configuration by running `hermes config validate`

### Setup realyer keys

To run a relayer we need to import the keys for both relayer accounts, you can do so using the following command:

```bash
hermes keys add --key-name relayer_key_chain1 --chain localjuno-1 --mnemonic-file ${ICTEST_HOME}/mnemonic1.txt
hermes keys add --key-name relayer_key_chain2 --chain localcosmoshub-1 --mnemonic-file ${ICTEST_HOME}/mnemonic2.txt
```

### Create IBC connection and run relayer

Now that we have keys imported we can proceed creating a communication channel between the two chains

First setup an IBC connection between the two chains

```bash
hermes create connection --a-chain localjuno-1 --b-chain localcosmoshub-1
```

once done, you'll have connection-0 set-up and ready to be used to create channels.

Now we can create a channel specific for our two polyton contracts instance (note->voice)

```bash
hermes create channel --a-chain localjuno-1 --a-connection connection-0 --a-port wasm.juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 --b-port wasm.cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr --channel-version polytone-1
```

Notice how we set chain a port and chain b port to be respectively the port of the note and listener contracts addresses.

Once done we can finally start the realyer, and keep it running in the background:

```bash
hermes start
```

The relayer now will listen to events on both chaains and will relay messages between them.

## Next Step

Let's try to send a cross-chain query! [here](./5-test-callback.md)
