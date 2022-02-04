use crate::state::MintPrice;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub tokens: Vec<String>,
    pub mint_price: MintPrice,
    pub base: cw721_base::InstantiateMsg,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T> {
    BatchMint { amount: u64 },
    CW721(cw721_base::ExecuteMsg<T>),
}
