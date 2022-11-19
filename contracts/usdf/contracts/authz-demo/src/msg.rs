use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    Transfer { from: String, to: String,amount:String }, // Bob pays Alice
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
