## Test our custom Interchain Contract

Now that the contract is ready, we can deploy it and test that it works as expected.

## Deploy the contract

As we did before with polytone contract, we'll use the same procedure as before.

First let's copy the wasm binary to the correct dockerized folder:

```bash
cp ./interchain_counter/artifacts/interchain_counter.wasm ${ICTEST_HOME}/homedirs/.juno/artifacts
```

Then we can proceed deploying it:

```bash
 junod-docker tx wasm store /root/.juno/artifacts/interchain_counter.wasm --from acc1
```

Once code is stored, we can instantiate it

```bash
junod-docker tx wasm instantiate 3 '{"note_address": "juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8", "count": 0}' --label "interchain_counter" --no-admin --from acc1
```

make sure to use the correct note address, and the correct code id.

## Delegate some tokens from acc2

To test the contract we need two accounts, one that has some ATOM staked and one that doesn't.

We'll use acc2 as the non-staked account, and acc1 as the staked account.

```bash
gaiad-docker tx staking delegate cosmosvaloper1zx0a35qhefzs8k6uvrry4ltqvnz5jcgwvra480 10uatomx --from acc1 --gas auto --gas-prices 1uatomx --gas-adjustment 2
```

make sure to replace the validator address with the correct one, that you can query with `gaiad-docker q staking validators`.

## Execute increment from acc1

First, let's check the current value for the counter:

```bash
dimi@192 polytone-workshop % junod-docker q wasm contract-state smart juno1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrq722sry '{"get_count":{}}'
data:
  count: 0
```

As you can see, the counter is 0 as expected.

Now let's try to call the increment function from acc1 which is a delegator:

```bash
 junod-docker tx wasm execute juno1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrq722sry '{"increment":{}}' --from acc2
```

After waiting a few seconds for IBC packets to be relayer, we can query the count again and see that the counter has been incremented:

```bash
dimi@192 polytone-workshop % junod-docker q wasm contract-state smart juno1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrq722sry '{"get_count":{}}'
data:
  count: 1
```

## Execute increment from acc2

We can try to do the same thing as before, but from acc2. The expected result is that the counter should not be incremented because acc2 doesn't have any ATOM staked.

```bash
 junod-docker tx wasm execute juno1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrq722sry '{"increment":{}}' --from acc2
```

If we wait a little bit, and we query again the counter, we can see that the counter is still 1.

```bash
dimi@192 polytone-workshop % junod-docker q wasm contract-state smart juno1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrq722sry '{"get_count":{}}'
data:
  count: 1
```

### Conclusion

You successfully created a cross-chain smart contract that interacts with both Juno and Cosmos Hub blockchains!
