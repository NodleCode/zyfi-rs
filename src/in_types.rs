use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TxData {
    pub from: String,
    pub to: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub chain_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsorship_ratio: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_limit: Option<u8>,
    pub tx_data: TxData,
    pub is_testnet: bool,
}
