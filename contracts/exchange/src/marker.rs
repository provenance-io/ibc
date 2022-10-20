use cosmwasm_std::{Addr, Coin, DepsMut};
use provwasm_std::{ProvenanceQuerier, ProvenanceQuery};

use crate::error::ContractError;

pub fn is_restricted_marker(deps: &DepsMut<ProvenanceQuery>, coin: &Coin) -> bool {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let res = querier.get_marker_by_denom(&coin.denom);
    match res {
        Ok(marker) => marker.bank_sends_disabled(),
        Err(_) => false,
    }
}

pub fn collateral_matches_native_total_supply(
    deps: &DepsMut<ProvenanceQuery>,
    collateral_denom: &str,
    native_denom: &str,
    marker_address: &Addr,
) -> Result<bool, ContractError> {
    let native_supply = deps.querier.query_supply(native_denom)?;
    let collateral = deps
        .querier
        .query_balance(marker_address, collateral_denom.to_string())?;
    Ok(collateral.amount == native_supply.amount)
}
