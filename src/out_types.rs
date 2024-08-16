use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TxData {
    pub chain_id: u32,
    pub from: String,
    pub to: String,
    pub data: String,
    pub value: String,
    pub custom_data: CustomData,
    pub max_fee_per_gas: String,
    pub gas_limit: u64,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomData {
    pub paymaster_params: PaymasterParams,
    pub gas_per_pubdata: u64,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymasterParams {
    pub paymaster: String,
    pub paymaster_input: String,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub tx_data: TxData,
    pub gas_limit: String,
    pub gas_price: String,
    pub token_address: String,
    pub token_price: String,
    pub fee_token_amount: String,
    #[serde(rename = "feeTokendecimals")]
    pub fee_token_decimals: String,
    #[serde(rename = "feeUSD")]
    pub fee_usd: String,
    pub markup: String,
    pub expiration_time: String,
    pub expires_in: String,
    pub max_nonce: Option<String>,
    pub protocol_address: Option<String>,
    pub sponsorship_ratio: Option<String>,
    pub estimated_final_fee_token_amount: Option<String>,
    pub estimated_final_fee_usd: Option<String>,
}
