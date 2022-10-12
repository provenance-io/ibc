use cosmwasm_std::{Addr, Coin, Decimal, Response};
use provwasm_std::{
    burn_marker_supply, mint_marker_supply, transfer_marker_coins, withdraw_coins, ProvenanceMsg,
};

use crate::state::State;
use crate::ContractError;

pub fn send_as_native(
    state: &State,
    private: &Coin,
    contract: &Addr,
    sender: &Addr,
    marker: &Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let native = convert(state, private)?;
    let burn = burn_marker_supply(private.amount.u128(), &private.denom)?;

    let private_transfer = transfer_marker_coins(
        private.amount.u128(),
        &private.denom,
        marker.clone(),
        sender.clone(),
    )?;
    let native_transfer = transfer_marker_coins(
        native.amount.u128(),
        &native.denom,
        sender.clone(),
        contract.clone(),
    )?;

    Ok(Response::new()
        .add_message(private_transfer)
        .add_message(burn)
        .add_message(native_transfer)
        .add_attribute("action", "provwasm.contracts.exchange.trade")
        .add_attribute("integration_test", "v1")
        .add_attribute("send", private.to_string())
        .add_attribute("receive", native.to_string()))
}

pub fn send_as_private(
    state: &State,
    native: &Coin,
    contract: &Addr,
    sender: &Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let private = convert(state, native)?;
    let mint = mint_marker_supply(private.amount.u128(), &private.denom)?;
    let withdraw = withdraw_coins(
        private.denom.clone(),
        private.amount.u128(),
        private.denom.clone(),
        sender.clone(),
    )?;
    let transfer = transfer_marker_coins(
        native.amount.u128(),
        &native.denom,
        contract.clone(),
        sender.clone(),
    )?;

    Ok(Response::new()
        .add_message(transfer)
        .add_message(mint)
        .add_message(withdraw)
        .add_attribute("action", "provwasm.contracts.exchange.trade")
        .add_attribute("integration_test", "v1")
        .add_attribute("send", native.to_string())
        .add_attribute("receive", private.to_string()))
}

fn convert(state: &State, coin: &Coin) -> Result<Coin, ContractError> {
    let denom;
    let mut amount = Decimal::from_atomics(coin.amount, 0).unwrap();

    if coin.denom == state.native_denom {
        denom = state.private_denom.clone();

        if let Ok(new_amount) = amount.checked_div(state.exchange_rate) {
            amount = new_amount.floor();
        } else {
            return Err(ContractError::ConversionError {});
        }
    } else if coin.denom == state.private_denom {
        denom = state.native_denom.clone();

        if let Ok(new_amount) = amount.checked_mul(state.exchange_rate) {
            amount = new_amount.floor();
        } else {
            return Err(ContractError::ConversionError {});
        }
    } else {
        // We need to verify that we accept the coin
        return Err(ContractError::InvalidDenom {});
    }

    Ok(Coin::new(dec_to_u128(&amount), denom))
}

fn dec_to_u128(decimal: &Decimal) -> u128 {
    decimal.to_string().parse::<u128>().unwrap()
}
