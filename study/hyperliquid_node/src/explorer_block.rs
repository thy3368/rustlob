//! Hyperliquid 公开 Explorer `blockDetails` adapter。

use std::error::Error;
use std::fmt;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// 官方公开 Explorer endpoint。
pub const OFFICIAL_EXPLORER_URL: &str = "https://rpc.hyperliquid.xyz/explorer";

#[derive(Debug, Clone, PartialEq)]
pub struct FetchedExplorerBlock {
    raw_response: Vec<u8>,
    response: ExplorerBlockResponse,
}

impl FetchedExplorerBlock {
    #[must_use]
    pub fn raw_response(&self) -> &[u8] {
        &self.raw_response
    }

    #[must_use]
    pub fn response(&self) -> &ExplorerBlockResponse {
        &self.response
    }

    #[must_use]
    pub fn block(&self) -> &ExplorerBlock {
        &self.response.block_details
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplorerBlockResponse {
    #[serde(rename = "type")]
    pub kind: BlockDetailsKind,
    #[serde(rename = "blockDetails")]
    pub block_details: ExplorerBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum BlockDetailsKind {
    #[serde(rename = "blockDetails")]
    BlockDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplorerBlock {
    pub height: u64,
    #[serde(rename = "blockTime")]
    pub block_time: u64,
    pub hash: String,
    pub proposer: String,
    #[serde(rename = "numTxs")]
    pub num_txs: usize,
    pub txs: Vec<ExplorerTransaction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplorerTransaction {
    pub time: u64,
    pub user: String,
    pub action: Action,
    pub block: u64,
    pub hash: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Action {
    #[serde(rename = "order")]
    Order(OrderAction),
    #[serde(rename = "cancel")]
    Cancel(CancelAction),
    #[serde(rename = "cancelByCloid")]
    CancelByCloid(CancelByCloidAction),
    #[serde(rename = "modify")]
    Modify(ModifyWire),
    #[serde(rename = "batchModify")]
    BatchModify(BatchModifyAction),
    #[serde(rename = "noop")]
    Noop(NoopAction),
    #[serde(rename = "scheduleCancel")]
    ScheduleCancel(ScheduleCancelAction),
    #[serde(rename = "updateLeverage")]
    UpdateLeverage(UpdateLeverageAction),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderAction {
    pub orders: Vec<OrderWire>,
    pub grouping: String,
    #[serde(default)]
    pub builder: Option<Builder>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelAction {
    pub cancels: Vec<CancelWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelByCloidAction {
    pub cancels: Vec<CancelByCloidWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchModifyAction {
    pub modifies: Vec<ModifyWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoopAction {}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleCancelAction {
    pub time: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateLeverageAction {
    pub asset: u32,
    #[serde(rename = "isCross")]
    pub is_cross: bool,
    pub leverage: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Builder {
    pub b: String,
    pub f: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelWire {
    pub a: u32,
    pub o: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelByCloidWire {
    pub asset: u32,
    pub cloid: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModifyWire {
    pub oid: OrderId,
    pub order: OrderWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum OrderId {
    Number(u64),
    Cloid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderWire {
    pub a: u32,
    pub b: bool,
    pub p: String,
    pub s: String,
    pub r: bool,
    pub t: OrderType,
    #[serde(default)]
    pub c: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Limit(LimitOrderType),
    Trigger(TriggerOrderType),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LimitOrderType {
    pub tif: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriggerOrderType {
    #[serde(rename = "isMarket")]
    pub is_market: bool,
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    pub tpsl: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplorerErrorResponse {
    #[serde(rename = "type")]
    pub kind: ExplorerErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ExplorerErrorKind {
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug)]
pub enum ExplorerError {
    Http(reqwest::Error),
    HttpStatus {
        url: String,
        status: StatusCode,
        body: String,
    },
    InvalidJson(serde_json::Error),
    NullResponse,
    Official(String),
    UnknownSchema(serde_json::Error),
    HeightMismatch {
        requested: u64,
        returned: u64,
    },
    TransactionCountMismatch {
        declared: usize,
        actual: usize,
    },
    TransactionHeightMismatch {
        index: usize,
        requested: u64,
        returned: u64,
    },
}

impl fmt::Display for ExplorerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => write!(f, "Explorer HTTP request failed: {error}"),
            Self::HttpStatus { url, status, body } => {
                write!(f, "Explorer returned HTTP {status} from {url}: {body}")
            }
            Self::InvalidJson(error) => write!(f, "Explorer returned invalid JSON: {error}"),
            Self::NullResponse => f.write_str("Explorer returned null"),
            Self::Official(message) => write!(f, "Explorer error: {message}"),
            Self::UnknownSchema(error) => write!(f, "Explorer response schema mismatch: {error}"),
            Self::HeightMismatch {
                requested,
                returned,
            } => write!(
                f,
                "Explorer returned height {returned}, requested {requested}"
            ),
            Self::TransactionCountMismatch { declared, actual } => write!(
                f,
                "Explorer declared {declared} transactions but returned {actual}"
            ),
            Self::TransactionHeightMismatch {
                index,
                requested,
                returned,
            } => write!(
                f,
                "Explorer transaction {index} has block {returned}, requested {requested}"
            ),
        }
    }
}

impl Error for ExplorerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            Self::InvalidJson(error) | Self::UnknownSchema(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Serialize)]
struct ExplorerRequest {
    #[serde(rename = "type")]
    kind: &'static str,
    height: u64,
}

/// 从指定 Explorer endpoint 下载并严格验证一个公开块。
#[allow(clippy::large_futures)]
pub async fn fetch_explorer_block(
    endpoint: &str,
    height: u64,
) -> Result<FetchedExplorerBlock, ExplorerError> {
    let raw_response = fetch_explorer_bytes(endpoint, height).await?;
    parse_explorer_block(raw_response, height)
}

/// 只执行 HTTP 下载，供需要在解析失败时保存原始证据的调用方使用。
#[allow(clippy::large_futures)]
pub async fn fetch_explorer_bytes(endpoint: &str, height: u64) -> Result<Vec<u8>, ExplorerError> {
    let response = Box::pin(
        reqwest::Client::new()
            .post(endpoint)
            .json(&ExplorerRequest {
                kind: "blockDetails",
                height,
            })
            .send(),
    )
    .await
    .map_err(ExplorerError::Http)?;
    let status = response.status();
    let raw_response = Box::pin(response.bytes())
        .await
        .map_err(ExplorerError::Http)?
        .to_vec();
    if !status.is_success() {
        return Err(ExplorerError::HttpStatus {
            url: endpoint.to_owned(),
            status,
            body: String::from_utf8_lossy(&raw_response).into_owned(),
        });
    }
    Ok(raw_response)
}

#[allow(clippy::disallowed_methods)] // 离线下载工具需要用严格 DTO 完整验证官方响应，不在交易热路径。
pub fn parse_explorer_block(
    raw_response: Vec<u8>,
    height: u64,
) -> Result<FetchedExplorerBlock, ExplorerError> {
    let value: serde_json::Value =
        serde_json::from_slice(&raw_response).map_err(ExplorerError::InvalidJson)?;
    if value.is_null() {
        return Err(ExplorerError::NullResponse);
    }
    if value.get("type").and_then(serde_json::Value::as_str) == Some("error") {
        let error: ExplorerErrorResponse =
            serde_json::from_value(value).map_err(ExplorerError::UnknownSchema)?;
        return Err(ExplorerError::Official(error.message));
    }
    let response: ExplorerBlockResponse =
        serde_json::from_value(value).map_err(ExplorerError::UnknownSchema)?;
    let block = &response.block_details;
    if block.height != height {
        return Err(ExplorerError::HeightMismatch {
            requested: height,
            returned: block.height,
        });
    }
    if block.num_txs != block.txs.len() {
        return Err(ExplorerError::TransactionCountMismatch {
            declared: block.num_txs,
            actual: block.txs.len(),
        });
    }
    if let Some((index, transaction)) = block
        .txs
        .iter()
        .enumerate()
        .find(|(_, transaction)| transaction.block != height)
    {
        return Err(ExplorerError::TransactionHeightMismatch {
            index,
            requested: height,
            returned: transaction.block,
        });
    }
    Ok(FetchedExplorerBlock {
        raw_response,
        response,
    })
}
