use cosmwasm_std::CosmosMsg::Bank;
use cosmwasm_std::{Addr, BankMsg, Coin, Decimal, Response};
use provwasm_std::{
    burn_marker_supply, mint_marker_supply, transfer_marker_coins, withdraw_coins, ProvenanceMsg,
};

use crate::state::State;
use crate::ContractError;

pub fn send_as_native(
    state: &State,
    coin: &Coin,
    to: &Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let native = convert(state, coin)?;
    let burn = burn_marker_supply(coin.amount.u128(), &coin.denom)?;
    let send_funds = Bank(BankMsg::Send {
        amount: vec![native.clone()],
        to_address: to.to_string(),
    });
    Ok(Response::new()
        .add_message(burn)
        .add_message(send_funds)
        .add_attribute("action", "provwasm.contracts.exchange.trade")
        .add_attribute("integration_test", "v1")
        .add_attribute("send", coin.to_string())
        .add_attribute("receive", native.to_string()))
}

pub fn send_as_private(
    state: &State,
    coin: &Coin,
    from: &Addr,
    to: &Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let private = convert(state, coin)?;
    let mint = mint_marker_supply(private.amount.u128(), &private.denom)?;
    let withdraw = withdraw_coins(
        private.denom.clone(),
        private.amount.u128(),
        private.denom.clone(),
        to.clone(),
    )?;
    let transfer = transfer_marker_coins(
        private.amount.u128(),
        &private.denom,
        to.clone(),
        from.clone(),
    )?;

    Ok(Response::new()
        .add_message(mint)
        .add_message(withdraw)
        .add_message(transfer)
        .add_attribute("action", "provwasm.contracts.exchange.trade")
        .add_attribute("integration_test", "v1")
        .add_attribute("send", coin.to_string())
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
