use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("InvalidFundsDenomError")]
    InvalidFundsDenomError {},

    #[error("InvalidFundsLengthError")]
    InvalidFundsLengthError {},

    #[error("InvalidFundsAmountError")]
    InvalidFundsAmountError {},

    #[error("The amount of [{collateral_denom}] does not match the total supply of [{native_denom}] for marker address [{marker_address}].")]
    CollateralAndNativeSupplyMistmatchError {
        collateral_denom: String,
        native_denom: String,
        marker_address: String,
    },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
