#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use crate::{execute, query};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:exchange";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ProvenanceQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    if msg.exchange_rate.is_zero() {
        return Err(ContractError::InvalidExchangeRateError {});
    }
    // Update state
    let state = State {
        native_denom: msg.native_denom.clone(),
        private_denom: msg.private_denom.clone(),
        exchange_rate: msg.exchange_rate,
        owner: info.sender.clone(),
        marker_address: deps.api.addr_validate(msg.marker_address.as_str())?,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("action", "provwasm.contracts.exchange.init")
        .add_attribute("integration_test", "v1")
        .add_attribute("owner", info.sender)
        .add_attribute("native_denom", msg.native_denom)
        .add_attribute("private_denom", msg.private_denom)
        .add_attribute("exchange_rate", msg.exchange_rate.to_string())
        .add_attribute("marker_address", msg.marker_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let api = deps.api;
    match msg {
        ExecuteMsg::SetOwner { owner } => {
            execute::set_owner(deps, info, api.addr_validate(owner.as_str())?)
        }
        ExecuteMsg::SetExchangeRate { exchange_rate } => {
            execute::set_exchange_rate(deps, info, exchange_rate)
        }
        ExecuteMsg::Trade { coin } => execute::trade(deps, env, info, coin),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ProvenanceQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetExchangeInfo {} => to_binary(&query::get_exchange_info(deps)?),
        QueryMsg::GetOwner {} => to_binary(&query::get_owner(deps)?),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::msg::{GetExchangeInfoResponse, GetOwnerResponse};
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::CosmosMsg::Bank;
    use cosmwasm_std::{from_binary, Addr, Attribute, BankMsg, Coin, Decimal, Uint128};
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::{
        burn_marker_supply, mint_marker_supply, transfer_marker_coins, withdraw_coins, Marker,
    };

    fn create_marker(denom: &str, restricted: bool) -> Marker {
        Marker {
            address: Addr::unchecked("test".to_string()),
            coins: Vec::new(),
            account_number: 100,
            sequence: 100,
            manager: "test".to_string(),
            permissions: Vec::new(),
            status: provwasm_std::MarkerStatus::Active,
            denom: denom.to_string(),
            total_supply: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
            marker_type: if restricted {
                provwasm_std::MarkerType::Restricted
            } else {
                provwasm_std::MarkerType::Coin
            },
            supply_fixed: true,
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(10), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);

        // Verify we have all the attributes
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(7, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "provwasm.contracts.exchange.init"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("integration_test", "v1"), res.attributes[1]);
        assert_eq!(
            Attribute::new("owner", "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"),
            res.attributes[2]
        );
        assert_eq!(Attribute::new("native_denom", "denom1"), res.attributes[3]);
        assert_eq!(Attribute::new("private_denom", "denom2"), res.attributes[4]);
        assert_eq!(
            Attribute::new(
                "exchange_rate",
                Decimal::from_atomics(Uint128::new(10), 1)
                    .unwrap()
                    .to_string()
            ),
            res.attributes[5]
        );
        assert_eq!(
            Attribute::new(
                "marker_address",
                "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h"
            ),
            res.attributes[6]
        );

        // Check the owner
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: GetOwnerResponse = from_binary(&res).unwrap();
        assert_eq!("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", value.owner);

        // Check the native_denom, private_denom, and exchange_rate
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetExchangeInfo {}).unwrap();
        let value: GetExchangeInfoResponse = from_binary(&res).unwrap();
        assert_eq!("denom1", value.native_denom);
        assert_eq!("denom2", value.private_denom);
        assert_eq!(
            Decimal::from_atomics(Uint128::new(10), 1).unwrap(),
            value.exchange_rate
        );
        assert_eq!(
            "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h",
            value.marker_address
        );
    }

    #[test]
    fn invalid_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);

        // Verify we have all the attributes
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        match res {
            Err(ContractError::InvalidExchangeRateError {}) => {}
            _ => panic!("Must return invalid exchange rate error"),
        }
    }

    #[test]
    fn proper_set_owner() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(10), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::SetOwner {
            owner: "tp15h7xkfj4v549sfdu0gxl9wltc85lywhjdxt6xu".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Verify the attributes
        assert_eq!(0, res.messages.len());
        assert_eq!(4, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "provwasm.contracts.exchange.set_owner"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("integration_test", "v1"), res.attributes[1]);
        assert_eq!(
            Attribute::new(
                "previous_owner",
                "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"
            ),
            res.attributes[2]
        );
        assert_eq!(
            Attribute::new("new_owner", "tp15h7xkfj4v549sfdu0gxl9wltc85lywhjdxt6xu"),
            res.attributes[3]
        );

        // Verify the new owner
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: GetOwnerResponse = from_binary(&res).unwrap();
        assert_eq!(
            "tp15h7xkfj4v549sfdu0gxl9wltc85lywhjdxt6xu".to_string(),
            value.owner
        );

        // Attempt to set owner again and fail
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::SetOwner {
            owner: "tp15h7xkfj4v549sfdu0gxl9wltc85lywhjdxt6xu".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        // Verify we get an error
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return invalid unauthorized error"),
        }
    }

    #[test]
    fn proper_set_exchange_rate() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(10), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Set the exchange rate
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::SetExchangeRate {
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(4, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "provwasm.contracts.exchange.set_exchange_rate"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("integration_test", "v1"), res.attributes[1]);
        assert_eq!(
            Attribute::new(
                "previous_exchange_rate",
                Decimal::from_atomics(Uint128::new(10), 1)
                    .unwrap()
                    .to_string()
            ),
            res.attributes[2]
        );
        assert_eq!(
            Attribute::new(
                "new_exchange_rate",
                Decimal::from_atomics(Uint128::new(20), 1)
                    .unwrap()
                    .to_string()
            ),
            res.attributes[3]
        );

        // Verify the new exchange_rate
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetExchangeInfo {}).unwrap();
        let value: GetExchangeInfoResponse = from_binary(&res).unwrap();
        assert_eq!("denom1".to_string(), value.native_denom);
        assert_eq!("denom2".to_string(), value.private_denom);
        assert_eq!(
            Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            value.exchange_rate
        );
        assert_eq!(
            "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
            value.marker_address
        );

        // Verify only owner can set
        let info = mock_info("tp15h7xkfj4v549sfdu0gxl9wltc85lywhjdxt6xu", &[]);
        let msg = ExecuteMsg::SetExchangeRate {
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // Verify it needs to be greater than 0
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetExchangeInfo {}).unwrap();
        let value: GetExchangeInfoResponse = from_binary(&res).unwrap();
        assert_eq!("denom1".to_string(), value.native_denom);
        assert_eq!("denom2".to_string(), value.private_denom);
        assert_eq!(
            Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            value.exchange_rate
        );
        assert_eq!(
            "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
            value.marker_address
        );

        // Verify only owner can set
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::SetExchangeRate {
            exchange_rate: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::InvalidExchangeRateError {}) => {}
            _ => panic!("Must return exchange rate error"),
        }
    }

    #[test]
    fn invalid_trade_denom() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Error with invalid denom
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(1, "denom3"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::InvalidDenom {}) => {}
            _ => panic!("Must return invalid denom error"),
        }
    }

    #[test]
    fn invalid_trade_denom_amount() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Error with invalid denom
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(0, "denom1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::InvalidFundsAmountError {}) => {}
            _ => panic!("Must return invalid funds amount error"),
        }
    }

    #[test]
    fn trade_fund_denom_does_not_match() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Fund denom does not match
        let info = mock_info(
            "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h",
            &[Coin::new(1, "denom2")],
        );
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(1, "denom1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::InvalidFundsError {}) => {}
            _ => panic!("Must return invalid funds amount error"),
        }
    }

    #[test]
    fn trade_fund_amount_does_not_match() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Fund amount does not match
        let info = mock_info(
            "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h",
            &[Coin::new(2, "denom1")],
        );
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(1, "denom1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::InvalidFundsError {}) => {}
            _ => panic!("Must return invalid funds amount error"),
        }
    }

    #[test]
    fn trade_fund_must_exist_for_native() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Fund amount must be added for native
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(1, "denom1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::InvalidFundsError {}) => {}
            _ => panic!("Must return invalid funds amount error"),
        }
    }

    #[test]
    fn trade_fund_denom_must_match_native() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Fund must be native denom
        let info = mock_info(
            "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h",
            &[Coin::new(1, "denom2")],
        );
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(2, "denom2"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::InvalidFundsError {}) => {}
            _ => panic!("Must return invalid funds amount error"),
        }
    }

    #[test]
    fn proper_trade_native_to_private() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Exchange native to private
        let info = mock_info(
            "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h",
            &[Coin::new(10, "denom1")],
        );
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(10, "denom1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(2, res.messages.len());
        assert_eq!(4, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "provwasm.contracts.exchange.trade"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("integration_test", "v1"), res.attributes[1]);
        assert_eq!(
            Attribute::new("send", Coin::new(10, "denom1").to_string()),
            res.attributes[2]
        );
        assert_eq!(
            Attribute::new("receive", Coin::new(5, "denom2").to_string()),
            res.attributes[3]
        );

        let mint = mint_marker_supply(5 as u128, "denom2").unwrap();
        let withdraw = withdraw_coins(
            "denom2",
            5 as u128,
            "denom2",
            Addr::unchecked("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"),
        )
        .unwrap();
        assert_eq!(mint, res.messages[0].msg);
        assert_eq!(withdraw, res.messages[1].msg);
    }

    #[test]
    fn proper_trade_private_to_native() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Exchange private to native
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(10, "denom2"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(3, res.messages.len());
        assert_eq!(4, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "provwasm.contracts.exchange.trade"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("integration_test", "v1"), res.attributes[1]);
        assert_eq!(
            Attribute::new("send", Coin::new(10, "denom2").to_string()),
            res.attributes[2]
        );
        assert_eq!(
            Attribute::new("receive", Coin::new(20, "denom1").to_string()),
            res.attributes[3]
        );

        let burn = burn_marker_supply(10 as u128, "denom2").unwrap();
        let transfer_private = transfer_marker_coins(
            10 as u128,
            "denom2",
            Addr::unchecked("tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h"),
            Addr::unchecked("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"),
        )
        .unwrap();
        let transfer_native = Bank(BankMsg::Send {
            amount: vec![Coin::new(20, "denom1")],
            to_address: "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h".to_string(),
        });
        assert_eq!(transfer_private, res.messages[0].msg);
        assert_eq!(burn, res.messages[1].msg);
        assert_eq!(transfer_native, res.messages[2].msg);
    }

    #[test]
    fn proper_trade_native_unrestricted_marker_to_private() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Exchange native (Marker unrestricted) to private
        let marker = create_marker("denom1", false);
        deps.querier.with_markers(vec![marker]);
        let info = mock_info(
            "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h",
            &[Coin::new(10, "denom1")],
        );
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(10, "denom1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(2, res.messages.len());
        assert_eq!(4, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "provwasm.contracts.exchange.trade"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("integration_test", "v1"), res.attributes[1]);
        assert_eq!(
            Attribute::new("send", Coin::new(10, "denom1").to_string()),
            res.attributes[2]
        );
        assert_eq!(
            Attribute::new("receive", Coin::new(5, "denom2").to_string()),
            res.attributes[3]
        );

        let mint = mint_marker_supply(5 as u128, "denom2").unwrap();
        let withdraw = withdraw_coins(
            "denom2",
            5 as u128,
            "denom2",
            Addr::unchecked("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"),
        )
        .unwrap();
        assert_eq!(mint, res.messages[0].msg);
        assert_eq!(withdraw, res.messages[1].msg);
    }

    #[test]
    fn proper_trade_private_to_unrestricted_native_marker() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Exchange private to native (Marker unrestricted)
        let marker = create_marker("denom1", false);
        deps.querier.with_markers(vec![marker]);
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(10, "denom2"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(3, res.messages.len());
        assert_eq!(4, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "provwasm.contracts.exchange.trade"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("integration_test", "v1"), res.attributes[1]);
        assert_eq!(
            Attribute::new("send", Coin::new(10, "denom2").to_string()),
            res.attributes[2]
        );
        assert_eq!(
            Attribute::new("receive", Coin::new(20, "denom1").to_string()),
            res.attributes[3]
        );

        let burn = burn_marker_supply(10 as u128, "denom2").unwrap();
        let transfer_private = transfer_marker_coins(
            10 as u128,
            "denom2",
            Addr::unchecked("tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h"),
            Addr::unchecked("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"),
        )
        .unwrap();
        let transfer_native = Bank(BankMsg::Send {
            amount: vec![Coin::new(20, "denom1")],
            to_address: "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h".to_string(),
        });
        assert_eq!(transfer_private, res.messages[0].msg);
        assert_eq!(burn, res.messages[1].msg);
        assert_eq!(transfer_native, res.messages[2].msg);
    }

    #[test]
    fn proper_trade_restricted_native_marker_to_private() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Exchange native (Marker restricted) to private
        let marker = create_marker("denom1", true);
        deps.querier.with_markers(vec![marker]);
        let info = mock_info(
            "tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h",
            &[Coin::new(10, "denom1")],
        );
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(10, "denom1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(3, res.messages.len());
        assert_eq!(4, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "provwasm.contracts.exchange.trade"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("integration_test", "v1"), res.attributes[1]);
        assert_eq!(
            Attribute::new("send", Coin::new(10, "denom1").to_string()),
            res.attributes[2]
        );
        assert_eq!(
            Attribute::new("receive", Coin::new(5, "denom2").to_string()),
            res.attributes[3]
        );

        let transfer_native = transfer_marker_coins(
            10 as u128,
            "denom1",
            mock_env().contract.address,
            Addr::unchecked("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"),
        )
        .unwrap();
        let mint = mint_marker_supply(5 as u128, "denom2").unwrap();
        let withdraw = withdraw_coins(
            "denom2",
            5 as u128,
            "denom2",
            Addr::unchecked("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"),
        )
        .unwrap();
        assert_eq!(transfer_native, res.messages[0].msg);
        assert_eq!(mint, res.messages[1].msg);
        assert_eq!(withdraw, res.messages[2].msg);
    }

    #[test]
    fn proper_trade_private_to_restricted_native_marker() {
        // Create the contract
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            native_denom: "denom1".to_string(),
            private_denom: "denom2".to_string(),
            exchange_rate: Decimal::from_atomics(Uint128::new(20), 1).unwrap(),
            marker_address: "tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h".to_string(),
        };
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Exchange private to native (Marker restricted)
        let marker = create_marker("denom1", true);
        deps.querier.with_markers(vec![marker]);
        let info = mock_info("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h", &[]);
        let msg = ExecuteMsg::Trade {
            coin: Coin::new(10, "denom2"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(3, res.messages.len());
        assert_eq!(4, res.attributes.len());
        assert_eq!(
            Attribute::new("action", "provwasm.contracts.exchange.trade"),
            res.attributes[0]
        );
        assert_eq!(Attribute::new("integration_test", "v1"), res.attributes[1]);
        assert_eq!(
            Attribute::new("send", Coin::new(10, "denom2").to_string()),
            res.attributes[2]
        );
        assert_eq!(
            Attribute::new("receive", Coin::new(20, "denom1").to_string()),
            res.attributes[3]
        );

        let burn = burn_marker_supply(10 as u128, "denom2").unwrap();
        let transfer_private = transfer_marker_coins(
            10 as u128,
            "denom2",
            Addr::unchecked("tp1kn7phy33x5pqpax6t9n60tkjtuqf5jt37txe0h"),
            Addr::unchecked("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"),
        )
        .unwrap();
        let transfer_native = transfer_marker_coins(
            20 as u128,
            "denom1",
            Addr::unchecked("tp1w9fnesmguvlal3mp62na3f58zww9jtmtwfnx9h"),
            mock_env().contract.address,
        )
        .unwrap();
        assert_eq!(transfer_private, res.messages[0].msg);
        assert_eq!(burn, res.messages[1].msg);
        assert_eq!(transfer_native, res.messages[2].msg);
    }
}
