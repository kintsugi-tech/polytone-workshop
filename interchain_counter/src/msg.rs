use cosmwasm_schema::{cw_serde, QueryResponses};
use polytone::callbacks::CallbackMessage;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
    pub note_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset {
        count: i32,
    },
    /// Stores the callback in state and makes it queryable.
    Callback(CallbackMessage),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
