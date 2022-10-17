use cosmwasm_std::{Coin, DepsMut};
use provwasm_std::{ProvenanceQuerier, ProvenanceQuery};

pub fn is_restricted_marker(deps: &DepsMut<ProvenanceQuery>, coin: &Coin) -> bool {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let res = querier.get_marker_by_denom(&coin.denom);
    match res {
        Ok(marker) => return marker.bank_sends_disabled(),
        Err(_) => return false,
    }
}
