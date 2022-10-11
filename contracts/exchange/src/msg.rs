use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct InstantiateMsg {
    pub native_denom: String,
    pub private_denom: String,
    pub exchange_rate: Decimal,
    pub marker_account: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Trade {},
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

    #[returns(GetMarkerAccountResponse)]
    GetMarkerAccount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetExchangeInfoResponse {
    pub native_denom: String,
    pub private_denom: String,
    pub exchange_rate: Decimal,
}

#[cw_serde]
pub struct GetOwnerResponse {
    pub owner: String,
}

#[cw_serde]
pub struct GetMarkerAccountResponse {
    pub marker_account: String,
}
