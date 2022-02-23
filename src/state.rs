use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub config: Config,
    pub tokens: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub creator_fund: Addr,
    pub dev_fund: Addr,
    pub creator_fund_percent: u64,
    pub dev_fund_percent: u64,
    pub creator_fund_nft_count: u64,
    pub dev_fund_nft_count: u64,
    pub mint_fee: Coin,
}

pub const STATE: Item<State> = Item::new("state");
