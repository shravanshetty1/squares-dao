#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{CustomNFT, InstantiateMsg};
use crate::state::{State, STATE};
use cw721_base::state::TokenInfo;
use cw721_base::Cw721Contract;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:squares";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let token_count = msg.tokens.len();
    let owner = info.sender.clone();

    STATE.save(
        deps.storage,
        &State {
            owner: info.sender.clone(),
            tokens: msg.tokens,
        },
    )?;

    let tract = Cw721Contract::<CustomNFT, Empty>::default();
    let resp = tract.instantiate(deps, _env, info, msg.base).unwrap();

    Ok(resp
        .add_attribute("owner", owner)
        .add_attribute("token_count", token_count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: cw721_base::ExecuteMsg<CustomNFT>,
) -> Result<Response, cw721_base::ContractError> {
    let tract = Cw721Contract::<CustomNFT, Empty>::default();
    match msg {
        cw721_base::ExecuteMsg::Mint(_msg) => mint(tract, deps, _env, info),
        _ => tract.execute(deps, _env, info, msg),
    }
}

pub fn mint(
    tract: Cw721Contract<CustomNFT, Empty>,
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, cw721_base::ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let token_extension = state.tokens.pop().ok_or(cw721_base::ContractError::Std(
        cosmwasm_std::StdError::generic_err("contract is out of tokens"),
    ))?;
    STATE.save(deps.storage, &state)?;
    let token = TokenInfo {
        owner: info.sender.clone(),
        approvals: vec![],
        token_uri: Some(token_extension.uri.clone()),
        extension: token_extension,
    };

    let id = tract.token_count(deps.storage)? + 1;
    tract
        .tokens
        .update(deps.storage, &id.to_string(), |old| match old {
            Some(_) => Err(cw721_base::ContractError::Claimed {}),
            None => Ok(token),
        })?;
    tract.increment_tokens(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", info.sender)
        .add_attribute("token_id", id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: cw721_base::QueryMsg) -> StdResult<Binary> {
    let tract = Cw721Contract::<CustomNFT, Empty>::default();
    tract.query(deps, _env, msg)
}
