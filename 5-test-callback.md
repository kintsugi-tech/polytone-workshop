## Test the Interchain Query & Callback

Now that finally we have a full environment setup, we can test to see if the cross-chain query is working using polytone.

For this demonstration we are going to use polytone contracts by calling them directly. In a production scenario you want to have your own smart contract to the required submessages automatically, making all this process transparent to the user.

### Let's do the query

To initiate the interchain query, we need to call the method `query`on the note contract on juno, passing it the required params that will ask for a specific query on the remote chain.

In our case, we are going to check the balance of one of our test accounts on cosmos hub.

```bash
junod-docker tx wasm execute juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 '{"query": {"msgs": [{"bank": {"all_balances": {"address": "cosmos1hj5fveer5cjtn4wd6wstzugjfdxzl0xpxvjjvr"}}}],"callback": {"receiver": "juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p","msg": "dGVzdA=="},"timeout_seconds": "300"}}' --from acc1 -y
```

This will generate the first transaction, where we ask the note smart contract to send an interchain query to cosmos hub.

Once this is confirmed, relayers will catch it and relay the packets cross chain.

### Check te result

To check the result we got back from the interchain query, we can check the status of the listener contract on juno. It should have the result of the query saved in it.

```bash
junod-docker q wasm contract-state smart juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p '{"result": {"initiator":"juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl", "initiator_msg":"dGVzdA=="}}'
```

In this command you can see that we are using as initiator parameter the address who asked for the query (acc1), and initiator_msg the same we used when requesting the query.

the result should look like this:

```bash
data:
  callback:
    initiator: juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl
    initiator_msg: dGVzdA==
    result:
      query:
        Ok:
        - eyJhbW91bnQiOlt7ImRlbm9tIjoidWF0b214IiwiYW1vdW50IjoiODcyNTkyIn1dfQ==
```

If we deserialize the Ok value, we get the raw data of the query:

```bash
dimi@192 % echo "eyJhbW91bnQiOlt7ImRlbm9tIjoidWF0b214IiwiYW1vdW50IjoiODcyNTkyIn1dfQ==" | base64 -d
{"amount":[{"denom":"uatomx","amount":"872592"}]}
```

The listener contract successufly saved the balance of the remote address, to be 872592uatomx!

If for some reason you don't see the result, probably your relayer is stuck. I suggest to stop and restart hermes to make sure all packets are relayed.

## Next Step

Let's build a real smart contract that does an interchain query and use the result in its logic [here](./6-custom-contract.md)
