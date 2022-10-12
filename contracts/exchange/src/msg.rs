use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub native_denom: String,
    pub private_denom: String,
    pub exchange_rate: Decimal,
    pub marker_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Trade { coin: Coin },
    SetExchangeRate { exchange_rate: Decimal },
    SetOwner { owner: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetExchangeInfoResponse)]
    GetExchangeInfo {},

    #[returns(GetOwnerResponse)]
    GetOwner {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetExchangeInfoResponse {
    pub native_denom: String,
    pub private_denom: String,
    pub exchange_rate: Decimal,
    pub marker_address: String,
}

#[cw_serde]
pub struct GetOwnerResponse {
    pub owner: String,
}
