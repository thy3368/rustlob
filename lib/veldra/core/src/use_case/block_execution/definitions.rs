use std::collections::BTreeMap;

use cmd_handler::command_use_case_def2::IssuedByParty;
use thiserror::Error;

use crate::entity::{
    NewBlock, PendingRequest, ProductContext, ProductPluginError, ProductPluginRegistry,
    RequestExecutionResult,
};

/// 构建新区块的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildBlockFromPendingRequestsCommand {
    pub block_height: u64,
}

impl IssuedByParty for BuildBlockFromPendingRequestsCommand {}

/// 构建新区块前已加载的业务状态。
#[derive(Debug, Clone)]
pub struct BuildBlockFromPendingRequestsState {
    pub parent_height: u64,
    pub parent_block_hash: String,
    pub pending_requests: Vec<PendingRequest>,
    pub product_plugins: ProductPluginRegistry,
    pub product_contexts: BTreeMap<String, ProductContext>,
}

/// 区块构建的强类型输出。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildBlockFromPendingRequestsOutput {
    pub new_block: NewBlock,
    pub request_results: Vec<RequestExecutionResult>,
}

/// 区块构建业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BuildBlockError {
    #[error("block height must be greater than zero")]
    BlockHeightMustBePositive,
    #[error("pending requests batch is empty")]
    EmptyPendingRequests,
    #[error("block height {actual} is not continuous after parent height {parent_height}")]
    NonContinuousBlockHeight { parent_height: u64, actual: u64 },
    #[error("missing product plugin for '{product_id}'")]
    MissingProductPlugin { product_id: String },
    #[error("plugin for '{product_id}' does not support action '{action}'")]
    UnsupportedAction { product_id: String, action: String },
    #[error("missing product context for '{product_id}'")]
    MissingProductContext { product_id: String },
    #[error("product context '{actual}' does not match request product '{expected}'")]
    ProductContextMismatch { expected: String, actual: String },
    #[error("product plugin execution failed: {0}")]
    ProductPlugin(#[from] ProductPluginError),
    #[error("failed to apply request result back into product context: {product_id}")]
    ApplyResultFailed { product_id: String },
}
