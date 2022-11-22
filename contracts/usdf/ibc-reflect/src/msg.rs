use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, ContractResult};

/// Just needs to know the code_id of a reflect contract to spawn sub-accounts
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub enum PacketMsg {
    UsdfSend {
        from_address: String,
        to_address: String,
        funds: Coin,
    },
}
#[cw_serde]
pub struct UsdfSendResponse {}

/// All acknowledgements are wrapped in `ContractResult`.
/// The success value depends on the PacketMsg variant.
pub type AcknowledgementMsg<T> = ContractResult<T>;
