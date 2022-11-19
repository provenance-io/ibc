use cosmos_sdk_proto::prost::{DecodeError, EncodeError};
use cosmwasm_std::StdError;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Decode(#[from] DecodeError),

    #[error("{0}")]
    Encode(#[from] EncodeError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
