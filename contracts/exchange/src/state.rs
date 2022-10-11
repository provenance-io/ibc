use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub native_denom: String,
    pub private_denom: String,
    pub exchange_rate: Decimal,
    pub marker_account: Addr,
}

pub const STATE: Item<State> = Item::new("state");
