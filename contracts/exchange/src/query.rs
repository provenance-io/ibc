use cosmwasm_std::{Deps, StdResult};
use provwasm_std::ProvenanceQuery;

use crate::{msg::GetExchangeInfoResponse, state::STATE};

pub fn get_exchange_info(deps: Deps<ProvenanceQuery>) -> StdResult<GetExchangeInfoResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetExchangeInfoResponse {
        collateral_denom: state.collateral_denom,
        native_denom: state.native_denom,
        marker_address: state.marker_address.to_string(),
    })
}
