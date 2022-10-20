use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub collateral_denom: String,
    pub native_denom: String,
    pub marker_address: Addr,
}

pub const STATE: Item<State> = Item::new("state");
