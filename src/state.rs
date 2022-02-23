use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub creator_fund: Addr,
    pub dev_fund: Addr,
    pub tokens: Vec<String>,
    pub mint_price: Coin,
}

pub const STATE: Item<State> = Item::new("state");
