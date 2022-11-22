use cosmwasm_std::{
    entry_point, to_binary, Coin, Deps, DepsMut, Env, IbcMsg, MessageInfo, QueryResponse, Response,
    StdResult,
};

use crate::ibc::PACKET_LIFETIME;
use crate::ibc_msg::PacketMsg;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StatusResponse};
use crate::state::{accounts, config, config_read, Config};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let cfg = Config {
        ack_received: false,
        nack_received: false,
        error_received: false,
    };
    config(deps.storage).save(&cfg)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UsdfSend {
            channel_id,
            funds,
            granter_address,
            to_address,
        } => handle_usdf_send(
            deps,
            env,
            info,
            channel_id,
            granter_address,
            to_address,
            funds,
        ),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Status {} => to_binary(&handle_status_query(deps)?),
    }
}

pub fn handle_status_query(deps: Deps) -> StdResult<StatusResponse> {
    let cfg = config_read(deps.storage).load()?;
    Ok(StatusResponse {
        ack_received: cfg.ack_received,
        nack_received: cfg.nack_received,
        error_received: cfg.error_received,
    })
}

pub fn handle_usdf_send(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    channel_id: String,
    granter_address: String,
    to_address: String,
    funds: Coin,
) -> StdResult<Response> {
    // ensure the channel exists (not found if not registered)
    accounts(deps.storage).load(channel_id.as_bytes())?;

    // construct a packet to send
    let packet = PacketMsg::UsdfSend {
        granter_address,
        to_address,
        funds,
    };
    let msg = IbcMsg::SendPacket {
        channel_id,
        data: to_binary(&packet)?,
        timeout: env.block.time.plus_seconds(PACKET_LIFETIME).into(),
    };

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "handle_usdf_send");
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const CREATOR: &str = "creator";

    #[test]
    fn instantiate_works() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info(CREATOR, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
