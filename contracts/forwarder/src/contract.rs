#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Order::Ascending;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg};
use cosmwasm_std::BankMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Forward { recipient, amount } => {
            execute_forward(deps, env, info, recipient, amount)
        }
        ExecuteMsg::ForwardToContract { contract, recipient, amount } => {
            execute_forward_to_contract(deps, env, info, contract, recipient, amount)
        }
    }
}

pub fn execute_forward(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let rcpt_addr = deps.api.addr_validate(&recipient)?;

    let msg = BankMsg::Send { to_address: rcpt_addr.to_string(), amount: vec![Coin {
        denom: "uluna".to_string(),
        amount: amount,
    }] };

    let res = Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("from", info.sender)
        .add_attribute("to", recipient)
        .add_attribute("amount", amount);
    Ok(res.add_message(msg))
}

pub fn execute_forward_to_contract(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: String,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let contract_addr = deps.api.addr_validate(&contract)?;
    let rcpt_addr = deps.api.addr_validate(&recipient)?;

    let msg = WasmMsg::Execute { contract_addr: contract_addr.to_string(), msg: to_binary(&ExecuteMsg::Forward { recipient: rcpt_addr.to_string(), amount: amount.saturating_sub(Uint128::from(10u128)) })?, funds: vec![Coin {
        denom: "uluna".to_string(),
        amount: amount,
    }] };

    let res = Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("from", info.sender)
        .add_attribute("to", recipient)
        .add_attribute("amount", amount);
    Ok(res.add_message(msg))
}




#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(to_binary("lmao")?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
