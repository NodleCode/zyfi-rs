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
        gas_limit: Option<String>,
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
            gas_limit,
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
        gas_limit: Option<String>,
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
            gas_limit,
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
        let status = response.status();
        if status.is_success() {
            let response = response.json::<ZyFiResponse>().await.map_err(|e| {
                error!("Failed to parse ZyFi response: {:?}", e);
                anyhow!("Failed to parse ZyFi response: {:?}", e)
            })?;
            debug!("ZyFi response: {:?}", response);
            Ok(response)
        } else {
            println!("{}", status);
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

    const MAINNET_TX_FROM: &str = "0xd1e5e09ef8f5ab7d59c14d8a0847e76a71163a82";
    const MAINNET_TX_TO: &str = "0x95b3641d549f719eb5105f9550eca4a7a2f305de";
    const MAINNET_TX_DATA: &str = "0xd204c45e000000000000000000000000d1e5e09ef8f5ab7d59c14d8a0847e76a71163a8200000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000035697066733a2f2f516d4e574d6e37586468514a426233376350334b59654659556d4538505a64373750754645734c4e66454b7150630000000000000000000000";

    const TESTNET_TX_FROM: &str = "0xd7aFa0aF9F93dbf58CF26ffA17f3e72D639c6483";
    const TESTNET_TX_TO: &str = "0x999368030Ba79898E83EaAE0E49E89B7f6410940";
    const TESTNET_TX_DATA: &str = "0xd204c45e000000000000000000000000d1e5e09ef8f5ab7d59c14d8a0847e76a71163a8200000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000035697066733a2f2f516d4e574d6e37586468514a426233376350334b59654659556d4538505a64373750754645734c4e66454b7150630000000000000000000000";

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_sponsored_mainnet() {
        let api_key = env::var("ZYFI_API_KEY").unwrap();
        let client = ClientZyFi {
            api_key: Some(api_key),
            testnet: false,
            ..Default::default()
        };

        let response = client
            .sponsored(
                MAINNET_TX_FROM.to_string(),
                MAINNET_TX_TO.to_string(),
                MAINNET_TX_DATA.to_string(),
                None,
            )
            .await;
        assert!(response.is_ok());

        let response = response.unwrap();
        println!("Mainnet sponsored response unwrapped: {:?}", response);
    }

    #[tokio::test]
    #[ignore = "requires API key"]
    async fn test_sponsored_testnet() {
        let api_key = env::var("ZYFI_API_KEY").unwrap();
        let client = ClientZyFi {
            api_key: Some(api_key),
            testnet: true,
            chain_id: 300,
            ..Default::default()
        };

        let response = client
            .sponsored(
                TESTNET_TX_FROM.to_string(),
                TESTNET_TX_TO.to_string(),
                TESTNET_TX_DATA.to_string(),
                None,
            )
            .await;
        println!("Testnet sponsored response: {:?}", response);
        assert!(response.is_ok());

        let response = response.unwrap();
        println!("Testnet sponsored response unwrapped: {:?}", response);
    }

    #[tokio::test]
    async fn test_paymaster_mainnet() {
        let client = ClientZyFi {
            testnet: false,
            fee_token_address: Some("0xBD4372e44c5eE654dd838304006E1f0f69983154".to_string()),
            ..Default::default()
        };

        let response = client
            .paymaster(
                MAINNET_TX_FROM.to_string(),
                MAINNET_TX_TO.to_string(),
                MAINNET_TX_DATA.to_string(),
                None,
            )
            .await;
        assert!(response.is_ok());

        let response = response.unwrap();
        println!("Mainnet paymaster response unwrapped: {:?}", response);
    }

    #[tokio::test]
    async fn test_paymaster_testnet() {
        let client = ClientZyFi {
            testnet: true,
            chain_id: 300,
            fee_token_address: Some("0xb4B74C2BfeA877672B938E408Bae8894918fE41C".to_string()), // Use appropriate testnet token address
            ..Default::default()
        };

        let response = client
            .paymaster(
                TESTNET_TX_FROM.to_string(),
                TESTNET_TX_TO.to_string(),
                TESTNET_TX_DATA.to_string(),
                None,
            )
            .await;
        println!("Testnet paymaster response: {:?}", response);
        assert!(response.is_ok());

        let response = response.unwrap();
        println!("Testnet paymaster response unwrapped: {:?}", response);
    }
}
