use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidDenom")]
    InvalidDenom {},

    #[error("ConversionError")]
    ConversionError {},

    #[error("InvalidFundsError")]
    InvalidFundsError {},

    #[error("InvalidFundsAmountError")]
    InvalidFundsAmountError {},

    #[error("InvalidExchangeRateError")]
    InvalidExchangeRateError {},

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
