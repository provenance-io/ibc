use crate::{
    exchange::{exchange_for_native, exchange_for_private},
    marker::is_restricted_marker,
    state::STATE,
    ContractError,
};
use cosmwasm_std::{Addr, Coin, Decimal, DepsMut, Env, MessageInfo, Response, Uint128};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};

pub fn trade(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    coin: Coin,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let state = STATE.load(deps.storage)?;
    let response;

    // Verify we have a valid amount to be traded
    if coin.amount == Uint128::new(0) {
        return Err(ContractError::InvalidFundsAmountError {});
    }

    // Make sure we don't have more than one thing in amount
    if info.funds.len() > 1 {
        return Err(ContractError::InvalidFundsError {});
    }

    // If there is one thing make sure the coins match
    if !info.funds.is_empty() && info.funds[0] != coin {
        return Err(ContractError::InvalidFundsError {});
    }

    // If funds are specified then it must be the native denom
    if !info.funds.is_empty() && info.funds[0].denom != state.native_denom {
        return Err(ContractError::InvalidFundsError {});
    }

    // Must specify funds for native
    if coin.denom == state.native_denom
        && info.funds.is_empty()
        && !is_restricted_marker(&deps, &coin)
    {
        return Err(ContractError::InvalidFundsError {});
    }

    if coin.denom == state.private_denom {
        response = exchange_for_native(
            &state,
            &deps,
            &coin,
            &env.contract.address,
            &info.sender,
            &state.marker_address,
        )?;
    } else if coin.denom == state.native_denom {
        // We want to pass in is_marker here
        response = exchange_for_private(&state, &deps, &coin, &env.contract.address, &info.sender)?;
    } else {
        return Err(ContractError::InvalidDenom {});
    }

    Ok(response)
}

pub fn set_exchange_rate(
    deps: DepsMut<ProvenanceQuery>,
    info: MessageInfo,
    exchange_rate: Decimal,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let previous_rate = STATE.load(deps.storage)?.exchange_rate;

    // Verify the format and check that it is not 0
    if exchange_rate.is_zero() {
        return Err(ContractError::InvalidExchangeRateError {});
    }

    // Update the exchange_rate
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }

        state.exchange_rate = exchange_rate;
        Ok(state)
    })?;

    Ok(Response::new()
        .add_attribute("action", "provwasm.contracts.exchange.set_exchange_rate")
        .add_attribute("integration_test", "v1")
        .add_attribute("previous_exchange_rate", previous_rate.to_string())
        .add_attribute("new_exchange_rate", exchange_rate.to_string()))
}

pub fn set_owner(
    deps: DepsMut<ProvenanceQuery>,
    info: MessageInfo,
    owner: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let previous_owner = STATE.load(deps.storage)?.owner;
    let new_owner = owner.clone();

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }

        state.owner = owner;
        Ok(state)
    })?;

    Ok(Response::new()
        .add_attribute("action", "provwasm.contracts.exchange.set_owner")
        .add_attribute("integration_test", "v1")
        .add_attribute("previous_owner", previous_owner)
        .add_attribute("new_owner", new_owner))
}
