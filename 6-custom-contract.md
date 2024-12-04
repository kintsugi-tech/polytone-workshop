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

Select "full template" when asked.

## Add our custom logic

First, let's modify `msg.rs` adding the note address to the `InstantiateMsg` struct:

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
    pub note_address: String,
}
```

and to the same to state.rs so that we can actually save the note address in the state of the contract:

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
    pub note_address: String,
}
```

In the contract.rs file, we need to modify the instantiate function to get the note address from the message and save it to the state:

```rust

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
        note_address: msg.note_address,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}
```

Now that the note address is set, we need to modify the increment function so that it will _not_ increment the counter, but rather only trigger the interchain query. We want to increase the counter only after callback is received and we are sure the user has the minimum amount of atom staked.

here is how the increment function will look like now

```rust
    pub fn increment(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;

        // conver sender address from juno to cosmos
        let bech32_addr = bech32::decode(info.sender.as_str()).unwrap();
        let cosmos_addr = bech32::encode("cosmos", bech32_addr.1, bech32_addr.2).unwrap();

        let msg = PolytoneExecuteMsg::Query {
            msgs: vec![QueryRequest::Staking(StakingQuery::AllDelegations {
                delegator: cosmos_addr,
            })],
            callback: CallbackRequest {
                receiver: env.contract.address.into(),
                msg: to_json_binary(&"test")?,
            },
            timeout_seconds: Uint64::new(300), // Example timeout of 30 seconds
        };

        let note_sub_msg: Vec<SubMsg> = vec![SubMsg {
            id: 1,
            msg: WasmMsg::Execute {
                contract_addr: state.note_address,
                msg: to_json_binary(&msg)?,
                funds: info.funds,
            }
            .into(),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }];

        Ok(Response::new()
            .add_attribute("action", "increment")
            .add_submessages(note_sub_msg))
    }
```

In short:

- We are converting juno address to cosmos address
- We are preparing the interchain query message
- We are appending a submessage, triggering the note contract to do the interchain query and send us the callback

### Handling the callback and do the increment

Now that we have triggered the interchain query, we need to await for a callback and increment our counter only if the staked balance is greated than 0. We can use the following handler to deserialize the callback and do our logic

```rust
 pub fn callback(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        callback: CallbackMessage,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;

        // Only the note can execute the callback on this contract.
        if info.sender != state.note_address {
            return Err(ContractError::Unauthorized {});
        }

        // Check that we have at least some atom staked
        match callback.result {
            Callback::Query(Ok(results)) => {
                // Deserialize each Binary result
                for result in results {
                    let query_result: AllDelegationsResponse = from_json(result.clone())?;

                    // increase the counter if staked is greater than 0
                    if query_result.delegations[0].amount.amount > Uint128::zero() {
                        // Update state
                        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
                            state.count += 1;
                            Ok(state)
                        })?;
                    }
                }
            }
            Callback::Query(Err(err)) => {
                // use a proper error type here
                deps.api.debug(&format!("Query callback failed: {:?}", err));
                return Err(ContractError::Unauthorized {});
            }
            _ => {
                // use a proper error type here
                return Err(ContractError::Unauthorized {});
            }
        }

        Ok(Response::new().add_attribute("action", "callback"))
    }
```

Reference to `./interchain_counter` folder for the full code.

### Build the contract

Now we can build our contract using the following command:

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer-arm64:0.16.1
```

this will generate the wasm contract binary in the `artifacts` folder.

### Next steps

Test our contract! [here](./7-test-contract.md)
