use crate::state::MintPrice;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub tokens: Vec<CustomNFT>,
    pub mint_price: MintPrice,
    pub base: cw721_base::InstantiateMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CustomNFT {
    pub uri: String,
    pub family_color: String,
    pub color_name: String,
    pub hex_code: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum ExecuteMsg {
//     Increment {},
//     Reset { count: i32 },
// }
//
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     // GetCount returns the current count as a json-encoded number
//     GetCount {},
// }
//
// // We define a custom struct for each query response
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct CountResponse {
//     pub count: i32,
// }
