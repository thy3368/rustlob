use crate::types::Block;
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
    base_url: String,
}

impl HyperliquidClient {
    pub fn new() -> Result<Self, ClientError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            base_url: "https://api.hyperliquid.xyz".to_string(),
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
            .post(&format!("{}/info", self.base_url))
            .json(&serde_json::json!({
                "type": "blockDetails",
                "block": height
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                return Err(ClientError::BlockNotFound(height));
            }
            return Err(ClientError::HttpError(response.status()));
        }

        let block = response.json::<Block>().await?;
        Ok(block)
    }

    pub async fn fetch_latest_block(&self) -> Result<Block, ClientError> {
        // 暂时使用一个较新的固定区块高度
        // TODO: 实现真正的最新区块获取逻辑
        eprintln!("警告: fetch_latest_block 暂未实现，使用固定区块高度 1000000000");
        self.fetch_block(1000000000).await
    }
}
