use crate::{marker::collateral_matches_native_total_supply, state::STATE, ContractError};
use cosmwasm_std::CosmosMsg::Bank;
use cosmwasm_std::{BankMsg, Coin, DepsMut, Env, MessageInfo, Response};
use provwasm_std::{
    burn_marker_supply, mint_marker_supply, withdraw_coins, ProvenanceMsg, ProvenanceQuery,
};

pub fn trade(
    deps: DepsMut<ProvenanceQuery>,
    _env: Env,
    info: MessageInfo,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let state = STATE.load(deps.storage)?;

    let supply_matches = collateral_matches_native_total_supply(
        &deps,
        &state.collateral_denom,
        &state.native_denom,
        &state.marker_address,
    )?;
    if !supply_matches {
        return Err(ContractError::CollateralAndNativeSupplyMistmatchError {
            collateral_denom: state.collateral_denom,
            native_denom: state.native_denom,
            marker_address: state.marker_address.to_string(),
        });
    }

    // Make sure we have EXACTLY 1 item in funds
    if info.funds.is_empty() || info.funds.len() > 1 {
        return Err(ContractError::InvalidFundsLengthError {});
    }
    let coin = &info.funds[0];

    // Funds must match one of the contract's denoms
    if coin.denom != state.native_denom && coin.denom != state.collateral_denom {
        return Err(ContractError::InvalidFundsDenomError {});
    }

    // Fund amount must be greater than 0
    if coin.amount.is_zero() {
        return Err(ContractError::InvalidFundsAmountError {});
    }

    if coin.denom == state.collateral_denom {
        // We want to send collateral to the marker address
        let collateral_send = Bank(BankMsg::Send {
            amount: vec![coin.clone()],
            to_address: state.marker_address.to_string(),
        });
        let native = Coin::new(coin.amount.u128(), state.native_denom);

        // We want to mint native_denom for the marker
        let mint = mint_marker_supply(native.amount.u128(), native.denom.clone())?;

        // Give the new native_denom to the sender
        let withdraw = withdraw_coins(
            native.denom.clone(),
            native.amount.u128(),
            native.denom.clone(),
            info.sender,
        )?;

        Ok(Response::new()
            .add_message(collateral_send)
            .add_message(mint)
            .add_message(withdraw)
            .add_attribute("action", "provwasm.contracts.exchange.trade")
            .add_attribute("integration_test", "v1")
            .add_attribute("sent", coin.to_string())
            .add_attribute("received", native.to_string()))
    } else {
        // We want to send native to marker address
        let native_send = Bank(BankMsg::Send {
            amount: vec![coin.clone()],
            to_address: state.marker_address.to_string(),
        });
        let collateral = Coin::new(coin.amount.u128(), state.collateral_denom);

        // We want to burn native_denom for the marker
        let burn = burn_marker_supply(coin.amount.u128(), coin.denom.clone())?;

        // Give the collateral to the sender
        let withdraw = withdraw_coins(
            coin.denom.clone(),
            collateral.amount.u128(),
            collateral.denom.clone(),
            info.sender,
        )?;

        Ok(Response::new()
            .add_message(native_send)
            .add_message(burn)
            .add_message(withdraw)
            .add_attribute("action", "provwasm.contracts.exchange.trade")
            .add_attribute("integration_test", "v1")
            .add_attribute("sent", coin.to_string())
            .add_attribute("received", collateral.to_string()))
    }
}
