use std::io::Cursor;

use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::MsgDelegate;
use cosmos_sdk_proto::cosmos::authz::v1beta1::MsgExec;
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::ibc::applications::transfer::v1::{
    QueryDenomTraceRequest, QueryDenomTraceResponse,
};
use cosmos_sdk_proto::traits::{Message, MessageExt};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:authz-demo";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let cfg = Config { admin: info.sender };
    CONFIG.save(deps.storage, &cfg)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // send from smart contract to a random address
    let send = MsgSend {
        from_address: info.sender.to_string(),
        to_address: "tp17dtl0mjt3t77kpuhg2edqzjpszulwhgzxhtkax".to_string(), // alice
        amount: vec![Coin {
            denom: "nhash".to_string(),
            amount: "1000000".to_string(),
        }],
    }
        .to_any()
        .unwrap();


    let exec = MsgExec {
        grantee: "tp13g9hxkljph90nt2waxtw3a40fkkz0dta3sgztv".to_string(), // bob's address
        msgs:vec![send],
    }
        .encode_to_vec();

    let msg = CosmosMsg::Stargate {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        value: Binary::from(exec),
    };

    Ok(Response::default().add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
    let bin = QueryDenomTraceRequest {
        hash: "295548A78785A1007F232DE286149A6FF512F180AF5657780FC89C009E2C348F".to_string(),
    }
        .encode_to_vec();

    let data = Binary::from(bin);

    let query = QueryRequest::Stargate {
        path: "/ibc.applications.transfer.v1.Query/DenomHash".to_string(),
        data,
    };

    let bin: Binary = deps.querier.query(&query)?;
    let response = QueryDenomTraceResponse::decode(&mut Cursor::new(bin.to_vec()))
        .map_err(ContractError::Decode)?;

    match response.denom_trace {
        None => Ok(to_binary("not_found")?),
        Some(trace) => Ok(to_binary(&trace.base_denom)?),
    }
}
