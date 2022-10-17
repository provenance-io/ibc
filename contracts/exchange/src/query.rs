use cosmwasm_std::{Deps, StdResult};
use provwasm_std::ProvenanceQuery;

use crate::{
    msg::{GetExchangeInfoResponse, GetOwnerResponse},
    state::STATE,
};

pub fn get_owner(deps: Deps<ProvenanceQuery>) -> StdResult<GetOwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetOwnerResponse {
        owner: state.owner.to_string(),
    })
}

pub fn get_exchange_info(deps: Deps<ProvenanceQuery>) -> StdResult<GetExchangeInfoResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetExchangeInfoResponse {
        native_denom: state.native_denom,
        private_denom: state.private_denom,
        exchange_rate: state.exchange_rate,
        marker_address: state.marker_address.to_string(),
    })
}
