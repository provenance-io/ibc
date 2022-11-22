use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

/// This needs no info. Owner of the contract is whoever signed the InstantiateMsg.
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    UsdfSend {
        channel_id: String,
        funds: Coin,
        from_address: String,
        to_address: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StatusResponse)]
    Status {},
}

#[cw_serde]
pub struct StatusResponse {
    pub ack_received: bool,
    pub nack_received: bool,
    pub error_received: bool,
}
