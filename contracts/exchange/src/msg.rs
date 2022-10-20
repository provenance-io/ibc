use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub collateral_denom: String,
    pub native_denom: String,
    pub marker_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Trade {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetExchangeInfoResponse)]
    GetExchangeInfo {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetExchangeInfoResponse {
    pub collateral_denom: String,
    pub native_denom: String,
    pub marker_address: String,
}
