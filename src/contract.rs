#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use cw721_base::state::TokenInfo;
use cw721_base::{Cw721Contract, MintMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:squares";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct CustomNFT {
    pub uri: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    STATE.save(
        deps.storage,
        &State {
            owner: info.sender.clone(),
            tokens: msg.tokens,
        },
    );

    let tract = Cw721Contract::<CustomNFT, Empty>::default();
    let resp = tract.instantiate(deps, _env, info, msg.base).unwrap();

    Ok(resp
        .add_attribute("owner", info.sender.clone())
        .add_attribute("token_count", msg.tokens.len()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: cw721_base::ExecuteMsg<CustomNFT>,
) -> Result<Response, ContractError> {
    let tract = Cw721Contract::<CustomNFT, Empty>::default();
    match msg {
        cw721_base::ExecuteMsg::Mint(msg) => mint(tract, deps, _env, info),
        _ => tract.execute(deps, _env, info, msg),
    }
}

pub fn mint(
    tract: Cw721Contract<CustomNFT, Empty>,
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response<C>, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let token_extension = state.tokens.pop().ok_or(Err(()))?;
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
            Some(_) => Err(ContractError::Claimed {}),
            None => Ok(token),
        })?;
    tract.increment_tokens(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", info.sender)
        .add_attribute("token_id", id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: cw721_base::QueryMsg) -> StdResult<Binary> {
    let tract = Cw721Contract::<CustomNFT, Empty>::default();
    tract.query(deps, _env, msg)
}
