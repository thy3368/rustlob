use crate::types::{
    Block, BlockResponse, ClearinghouseState, OpenOrdersResponse, SpotClearinghouseState, UserDetails, UserFills,
};
use reqwest::Client;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("HTTP error status: {0}")]
    HttpError(reqwest::StatusCode),

    #[error("Block not found: {0}")]
    BlockNotFound(u64),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}

pub struct HyperliquidClient {
    client: Client,
    explorer_url: String,
    info_url: String,
}

impl HyperliquidClient {
    pub fn new() -> Result<Self, ClientError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            explorer_url: "https://rpc.hyperliquid.xyz/explorer".to_string(),
            info_url: "https://api.hyperliquid.xyz/info".to_string(),
        })
    }

    pub async fn fetch_block(&self, height: u64) -> Result<Block, ClientError> {
        let mut attempts = 0;
        let max_attempts = 3;

        loop {
            attempts += 1;
            match self.fetch_block_once(height).await {
                Ok(block) => return Ok(block),
                Err(e) if attempts >= max_attempts => return Err(e),
                Err(_) => {
                    let delay = Duration::from_millis(100 * 2_u64.pow(attempts - 1));
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    async fn fetch_block_once(&self, height: u64) -> Result<Block, ClientError> {
        let response = self
            .client
            .post(&self.explorer_url)
            .json(&serde_json::json!({
                "type": "blockDetails",
                "height": height
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                return Err(ClientError::BlockNotFound(height));
            }
            return Err(ClientError::HttpError(response.status()));
        }

        let block_response = response.json::<BlockResponse>().await?;
        Ok(block_response.block_details)
    }

    pub async fn fetch_latest_block(&self) -> Result<Block, ClientError> {
        eprintln!("警告: fetch_latest_block 暂未实现，使用固定区块高度 1000000000");
        self.fetch_block(1000000000).await
    }

    pub async fn fetch_clearinghouse_state(&self, user: &str) -> Result<ClearinghouseState, ClientError> {
        let response = self
            .client
            .post(&self.info_url)
            .json(&serde_json::json!({
                "type": "clearinghouseState",
                "user": user
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ClientError::HttpError(response.status()));
        }

        let state = response.json::<ClearinghouseState>().await?;
        Ok(state)
    }

    pub async fn fetch_spot_state(&self, user: &str) -> Result<SpotClearinghouseState, ClientError> {
        let response = self
            .client
            .post(&self.info_url)
            .json(&serde_json::json!({
                "type": "spotClearinghouseState",
                "user": user
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ClientError::HttpError(response.status()));
        }

        let state = response.json::<SpotClearinghouseState>().await?;
        Ok(state)
    }

    pub async fn fetch_user_details(&self, user: &str) -> Result<UserDetails, ClientError> {
        let response = self
            .client
            .post(&self.info_url)
            .json(&serde_json::json!({
                "type": "userDetails",
                "user": user
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ClientError::HttpError(response.status()));
        }

        let details = response.json::<UserDetails>().await?;
        Ok(details)
    }

    pub async fn fetch_open_orders(&self, user: &str) -> Result<Vec<OpenOrdersResponse>, ClientError> {
        let response = self
            .client
            .post(&self.info_url)
            .json(&serde_json::json!({
                "type": "openOrders",
                "user": user
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ClientError::HttpError(response.status()));
        }

        let orders = response.json::<Vec<OpenOrdersResponse>>().await?;
        Ok(orders)
    }

    pub async fn fetch_user_fills(&self, user: &str) -> Result<UserFills, ClientError> {
        let response = self
            .client
            .post(&self.info_url)
            .json(&serde_json::json!({
                "type": "userFills",
                "user": user
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ClientError::HttpError(response.status()));
        }

        // API returns array directly
        let fills_vec: Vec<crate::types::Fill> = response.json::<Vec<crate::types::Fill>>().await?;
        Ok(UserFills { fills: fills_vec })
    }
}
