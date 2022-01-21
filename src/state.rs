use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::CustomNFT;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub tokens: Vec<CustomNFT>,
    pub mint_price: MintPrice,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintPrice {
    pub denom: String,
    pub amount: u64,
}

pub const STATE: Item<State> = Item::new("state");
