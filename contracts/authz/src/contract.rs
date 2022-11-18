#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, QueryRequest, Response,
    StdResult, WasmMsg,
};

use cw2::set_contract_version;
use simple_ica::ReceiveIcaResponseMsg;

use crate::error::ContractError;
use crate::msg::{AdminResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ResultResponse};
use crate::state::{Config, CONFIG, RESULTS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:callback-capturer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let cfg = Config {
        admin: info.sender,
    };
    CONFIG.save(deps.storage, &cfg)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // create a send from alice to bob
    // create an authz exect msg
    // add to submsgs in response
    Ok(Response::default())
}


