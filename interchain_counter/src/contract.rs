#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:interchain_counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps, info, env),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::Callback(callback) => execute::callback(deps, env, info, callback),
    }
}

pub mod execute {

    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{
        from_json, AllDelegationsResponse, CosmosMsg, Empty, QueryRequest, ReplyOn, StakingQuery,
        SubMsg, Uint128, Uint64, WasmMsg,
    };
    use polytone::{
        ack::Callback,
        callbacks::{CallbackMessage, CallbackRequest},
    };

    use super::*;

    #[cw_serde]
    pub enum PolytoneExecuteMsg {
        Query {
            msgs: Vec<QueryRequest<Empty>>,
            callback: CallbackRequest,
            timeout_seconds: Uint64,
        },
        Execute {
            msgs: Vec<CosmosMsg<Empty>>,
            callback: Option<CallbackRequest>,
            timeout_seconds: Uint64,
        },
    }

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

    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }
}
