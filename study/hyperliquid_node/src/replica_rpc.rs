//! Hyperliquid `hl_getBlock` JSON-RPC adapter.
//!
//! [`FetchedBlock`] keeps the complete wire response as well as a typed view.
//! This lets callers inspect newly added upstream fields without waiting for
//! this crate's DTOs to be updated.

use std::error::Error;
use std::fmt;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const JSON_RPC_VERSION: &str = "2.0";
const METHOD: &str = "hl_getBlock";
const STREAM: &str = "replica_cmds";
const REQUEST_ID: u64 = 1;

/// 完整 RPC 响应及其强类型块视图。
#[derive(Debug, Clone, PartialEq)]
pub struct FetchedBlock {
    raw_response: Value,
    raw_block: Value,
    block: ReplicaBlock,
}

impl FetchedBlock {
    /// 返回未经裁剪的 JSON-RPC 响应。
    #[must_use]
    pub fn raw_response(&self) -> &Value {
        &self.raw_response
    }

    /// 返回响应中未经裁剪的 `result` 块。
    #[must_use]
    pub fn raw_block(&self) -> &Value {
        &self.raw_block
    }

    /// 返回常用块字段的强类型视图。
    #[must_use]
    pub fn block(&self) -> &ReplicaBlock {
        &self.block
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct ReplicaBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abci_block: Option<AbciBlock>,
    #[serde(default)]
    pub signed_action_bundles: Vec<(String, ActionBundle)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resps: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct AbciBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub round: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct ActionBundle {
    #[serde(default)]
    pub signed_actions: Vec<SignedAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub broadcaster: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct SignedAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct Action {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default)]
    pub orders: Vec<OrderWire>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouping: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OrderWire {
    pub a: u32,
    pub b: bool,
    pub p: String,
    pub s: String,
    #[serde(default)]
    pub r: bool,
    pub t: OrderType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct OrderType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<LimitType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LimitType {
    pub tif: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug)]
pub enum RpcError {
    Http(reqwest::Error),
    HttpStatus { url: String, status: StatusCode, body: String },
    InvalidJson(serde_json::Error),
    JsonRpc(JsonRpcError),
    InvalidResponse(&'static str),
    InvalidBlock(serde_json::Error),
}

impl fmt::Display for RpcError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => write!(formatter, "HTTP request failed: {error}"),
            Self::HttpStatus { url, status, body } => {
                write!(formatter, "RPC returned HTTP {status} from {url}: {body}")
            }
            Self::InvalidJson(error) => write!(formatter, "RPC returned invalid JSON: {error}"),
            Self::JsonRpc(error) => {
                write!(formatter, "JSON-RPC error {}: {}", error.code, error.message)
            }
            Self::InvalidResponse(message) => {
                write!(formatter, "invalid JSON-RPC response: {message}")
            }
            Self::InvalidBlock(error) => write!(formatter, "invalid replica block: {error}"),
        }
    }
}

impl Error for RpcError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            Self::InvalidJson(error) | Self::InvalidBlock(error) => Some(error),
            Self::HttpStatus { .. } | Self::JsonRpc(_) | Self::InvalidResponse(_) => None,
        }
    }
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    jsonrpc: &'static str,
    method: &'static str,
    params: [GetBlockParams; 1],
    id: u64,
}

#[derive(Debug, Serialize)]
struct GetBlockParams {
    stream: &'static str,
    block_number: u64,
}

fn request(block_number: u64) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: JSON_RPC_VERSION,
        method: METHOD,
        params: [GetBlockParams { stream: STREAM, block_number }],
        id: REQUEST_ID,
    }
}

/// 从调用方指定的 endpoint 获取 `replica_cmds` 块。
#[allow(clippy::large_futures)] // nightly Clippy 无法计算 reqwest opaque future 的布局。
pub async fn get_block(rpc_url: &str, block_number: u64) -> Result<FetchedBlock, RpcError> {
    let response =
        Box::pin(reqwest::Client::new().post(rpc_url).json(&request(block_number)).send())
            .await
            .map_err(RpcError::Http)?;
    let status = response.status();
    let body = Box::pin(response.text()).await.map_err(RpcError::Http)?;
    if !status.is_success() {
        return Err(RpcError::HttpStatus { url: rpc_url.to_owned(), status, body });
    }

    let raw_response: Value = serde_json::from_str(&body).map_err(RpcError::InvalidJson)?;
    if let Some(error) = raw_response.get("error") {
        let error = serde_json::from_value(error.clone())
            .map_err(|_| RpcError::InvalidResponse("malformed error object"))?;
        return Err(RpcError::JsonRpc(error));
    }
    let raw_block = raw_response
        .get("result")
        .cloned()
        .ok_or(RpcError::InvalidResponse("missing result and error"))?;
    if raw_block.is_null() {
        return Err(RpcError::InvalidResponse("result is null"));
    }
    let block = serde_json::from_value(raw_block.clone()).map_err(RpcError::InvalidBlock)?;

    Ok(FetchedBlock { raw_response, raw_block, block })
}

#[cfg(test)]
#[allow(clippy::disallowed_methods, clippy::indexing_slicing)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn request_uses_replica_stream_and_requested_block() -> Result<(), serde_json::Error> {
        let value = serde_json::to_value(request(0))?;
        assert_eq!(value["jsonrpc"], json!("2.0"));
        assert_eq!(value["method"], json!("hl_getBlock"));
        assert_eq!(value["params"][0]["stream"], json!("replica_cmds"));
        assert_eq!(value["params"][0]["block_number"], json!(0));
        assert_eq!(value["id"], json!(1));
        Ok(())
    }
}
