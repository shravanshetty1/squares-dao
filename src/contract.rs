#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    BankMsg, Binary, Coin, Decimal, Deps, DepsMut, Empty, Env, MessageInfo, Order, Response,
    StdResult, Uint128,
};

use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{State, STATE};
use cw721::Cw721Execute;
use cw721_base::state::TokenInfo;
use cw721_base::{Cw721Contract, Extension};
use std::ops::Mul;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, cw721_base::ContractError> {
    let token_count = msg.tokens.len();
    STATE.save(
        deps.storage,
        &State {
            config: msg.config.clone(),
            tokens: msg.tokens,
        },
    )?;

    let tract = Cw721Contract::<Extension, Empty>::default();
    let resp = tract.instantiate(deps.branch(), env, info, msg.base)?;

    batch_mint(
        &tract,
        deps.branch(),
        MessageInfo {
            sender: msg.config.creator_fund.clone(),
            funds: vec![],
        },
        msg.config.creator_fund_nft_count,
    )?;

    batch_mint(
        &tract,
        deps.branch(),
        MessageInfo {
            sender: msg.config.dev_fund.clone(),
            funds: vec![],
        },
        msg.config.dev_fund_nft_count,
    )?;

    Ok(resp
        .add_attribute("creator_fund", msg.config.creator_fund)
        .add_attribute("dev_fund", msg.config.dev_fund)
        .add_attribute("token_count", token_count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg<Extension>,
) -> Result<Response, cw721_base::ContractError> {
    let tract = Cw721Contract::<Extension, Empty>::default();
    match msg {
        ExecuteMsg::BatchMint { amount } => try_batch_mint(&tract, deps, info, amount),
        ExecuteMsg::Mint(_) => try_batch_mint(&tract, deps, info, 1),
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => tract.approve(deps, env, info, spender, token_id, expires),
        ExecuteMsg::Revoke { spender, token_id } => {
            tract.revoke(deps, env, info, spender, token_id)
        }
        ExecuteMsg::ApproveAll { operator, expires } => {
            tract.approve_all(deps, env, info, operator, expires)
        }
        ExecuteMsg::RevokeAll { operator } => tract.revoke_all(deps, env, info, operator),
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => tract.transfer_nft(deps, env, info, recipient, token_id),
        ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => tract.send_nft(deps, env, info, contract, token_id, msg),
    }
}

pub fn try_batch_mint(
    tract: &Cw721Contract<Extension, Empty>,
    deps: DepsMut,
    info: MessageInfo,
    n: u64,
) -> Result<Response, cw721_base::ContractError> {
    let conf = STATE.load(deps.storage)?.config;
    let expected_fee = conf.mint_fee.clone();
    let got_fee = info
        .funds
        .iter()
        .find(|got| got.denom == expected_fee.denom)
        .ok_or_else(|| {
            cosmwasm_std::StdError::generic_err(format!(
                "could not find funds with denom {}",
                expected_fee.denom
            ))
        })?;
    let expected_fee_amount = expected_fee
        .amount
        .checked_mul(Uint128::new(n as u128))
        .map_err(|_| cosmwasm_std::StdError::generic_err("overflow error"))?;
    if got_fee.amount.le(&expected_fee_amount) {
        return Err(cosmwasm_std::StdError::generic_err(format!(
            "insufficient fee, expected - {} {}, got - {} {}",
            expected_fee_amount, expected_fee.denom, got_fee.amount, got_fee.denom
        ))
        .into());
    }

    let owned_tokens = tract
        .tokens
        .idx
        .owner
        .prefix(info.sender.clone())
        .keys(deps.storage, None, None, Order::Ascending)
        .count();

    if owned_tokens as u64 + n > 10 {
        return Err(cw721_base::ContractError::Std(
            cosmwasm_std::StdError::generic_err("a single wallet cannot own more than 10 tokens"),
        ));
    }

    let token_ids: Vec<String> = batch_mint(tract, deps, info.clone(), n)?;

    let mut resp = Response::new();
    resp = resp.add_message(BankMsg::Send {
        to_address: String::from(conf.dev_fund.clone()),
        amount: vec![Coin {
            denom: conf.mint_fee.denom.clone(),
            amount: got_fee.amount.mul(Decimal::percent(conf.dev_fund_percent)),
        }],
    });
    resp = resp.add_message(BankMsg::Send {
        to_address: String::from(conf.creator_fund.clone()),
        amount: vec![Coin {
            denom: conf.mint_fee.denom,
            amount: got_fee
                .amount
                .mul(Decimal::percent(conf.creator_fund_percent)),
        }],
    });

    Ok(resp
        .add_attribute("action", "mint")
        .add_attribute("minter", info.sender)
        .add_attribute("token_ids", token_ids.join(",")))
}

pub fn batch_mint(
    tract: &Cw721Contract<Extension, Empty>,
    deps: DepsMut,
    info: MessageInfo,
    n: u64,
) -> Result<Vec<String>, cw721_base::ContractError> {
    let mut token_ids: Vec<String> = Vec::new();
    for _ in 0..n {
        let mut state = STATE.load(deps.storage)?;
        let token_uri = state.tokens.pop().ok_or_else(|| {
            cw721_base::ContractError::Std(cosmwasm_std::StdError::generic_err(
                "contract is out of tokens",
            ))
        })?;
        STATE.save(deps.storage, &state)?;
        let token = TokenInfo {
            owner: info.sender.clone(),
            approvals: vec![],
            token_uri: Some(token_uri.clone()),
            extension: None,
        };

        let id = tract.token_count(deps.storage)? + 1;
        tract
            .tokens
            .update(deps.storage, &id.to_string(), |old| match old {
                Some(_) => Err(cw721_base::ContractError::Claimed {}),
                None => Ok(token),
            })?;
        tract.increment_tokens(deps.storage)?;

        token_ids.push(id.to_string())
    }

    Ok(token_ids)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: cw721_base::QueryMsg) -> StdResult<Binary> {
    let tract = Cw721Contract::<Extension, Empty>::default();
    tract.query(deps, _env, msg)
}
