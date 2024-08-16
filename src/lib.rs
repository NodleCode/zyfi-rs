//! A simple HTTP client to talk to the sponsorship ZyFi API.

use anyhow::{anyhow, bail, Result};
use tracing::{debug, error};

mod in_types;
mod out_types;

pub use in_types::TxData as ZyFiRequest;
pub use out_types::Response as ZyFiResponse;

const ZYFI_SPONSORED_URL: &str = "https://api.zyfi.org/api/erc20_sponsored_paymaster/v1";
const ZYFI_PAYMASTER_URL: &str = "https://api.zyfi.org/api/erc20_paymaster/v1";

pub struct ClientZyFi {
    /// API Key to authenticate with ZyFi
    pub api_key: Option<String>,

    /// Address of the token to use when paying for fees
    pub fee_token_address: Option<String>,

    /// Whether to use the testnet or mainnet
    pub testnet: bool,

    /// Chain ID to use, defaults to ZkSync mainnet
    pub chain_id: u32,
}

impl Default for ClientZyFi {
    fn default() -> Self {
        Self {
            api_key: None,
            fee_token_address: None,
            testnet: false,
            chain_id: 324, // ZkSync mainnet
        }
    }
}
impl ClientZyFi {
    pub async fn sponsored(
        &self,
        tx_from: String,
        tx_to: String,
        tx_data: String,
    ) -> Result<ZyFiResponse> {
        let request = in_types::Request {
            chain_id: self.chain_id,
            sponsorship_ratio: Some(100),
            replay_limit: Some(1),
            tx_data: in_types::TxData {
                from: tx_from,
                to: tx_to,
                data: tx_data,
            },
            is_testnet: self.testnet,
            ..Default::default()
        };

        let client = reqwest::Client::new();
        let response = client
            .post(ZYFI_SPONSORED_URL)
            .header("Content-Type", "application/json")
            .header(
                "X-API-Key",
                self.api_key.clone().ok_or(anyhow!(
                    "API key not set - which is necessary to sponsor ZyFi transactions"
                ))?,
            )
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn paymaster(
        &self,
        tx_from: String,
        tx_to: String,
        tx_data: String,
    ) -> Result<ZyFiResponse> {
        let request = in_types::Request {
            chain_id: self.chain_id,
            tx_data: in_types::TxData {
                from: tx_from,
                to: tx_to,
                data: tx_data,
            },
            is_testnet: self.testnet,
            fee_token_address: self.fee_token_address.clone(),
            ..Default::default()
        };

        let client = reqwest::Client::new();
        let response = client
            .post(ZYFI_PAYMASTER_URL)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn handle_response(&self, response: reqwest::Response) -> Result<ZyFiResponse> {
        if response.status().is_success() {
            let response = response.json::<ZyFiResponse>().await.map_err(|e| {
                error!("Failed to parse ZyFi response: {:?}", e);
                anyhow!("Failed to parse ZyFi response: {:?}", e)
            })?;
            debug!("ZyFi response: {:?}", response);
            Ok(response)
        } else {
            let error = response.text().await?;
            error!("ZyFi error: {:?}", error);
            bail!("ZyFi error: {:?}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const TX_FROM: &str = "0xd1e5e09ef8f5ab7d59c14d8a0847e76a71163a82";
    const TX_TO: &str = "0x95b3641d549f719eb5105f9550eca4a7a2f305de";
    const TX_DATA: &str = "0xd204c45e000000000000000000000000d1e5e09ef8f5ab7d59c14d8a0847e76a71163a8200000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000035697066733a2f2f516d4e574d6e37586468514a426233376350334b59654659556d4538505a64373750754645734c4e66454b7150630000000000000000000000";

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_sponsored() {
        let api_key = env::var("ZYFI_API_KEY").unwrap();
        let client = ClientZyFi {
            api_key: Some(api_key),
            testnet: false,
            ..Default::default()
        };

        let response = client
            .sponsored(TX_FROM.to_string(), TX_TO.to_string(), TX_DATA.to_string())
            .await;
        println!("{:?}", response);
        assert!(response.is_ok());

        let response = response.unwrap();
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_paymaster() {
        let client = ClientZyFi {
            testnet: false,
            fee_token_address: Some("0xBD4372e44c5eE654dd838304006E1f0f69983154".to_string()),
            ..Default::default()
        };

        let response = client
            .paymaster(TX_FROM.to_string(), TX_TO.to_string(), TX_DATA.to_string())
            .await;
        println!("{:?}", response);
        assert!(response.is_ok());

        let response = response.unwrap();
        println!("{:?}", response);
    }
}
