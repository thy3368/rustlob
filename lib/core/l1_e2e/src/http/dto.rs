use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitTransactionsRequest {
    pub requests: Vec<SubmitTransactionItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitTransactionItem {
    pub request_id: String,
    pub account: String,
    pub nonce: String,
    pub expires_at: String,
    pub action_type: String,
    pub payload_hash: String,
    pub signature_hash: String,
    pub vm_kind: VmKindDto,
    pub capability: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VmKindDto {
    Evm,
    RustVm,
}

impl From<VmKindDto> for l1_core::VmKind {
    fn from(value: VmKindDto) -> Self {
        match value {
            VmKindDto::Evm => l1_core::VmKind::Evm,
            VmKindDto::RustVm => l1_core::VmKind::RustVm,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteBlockRequest {
    pub block_height: u64,
    pub batch_size: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitTransactionsResponse {
    pub admitted_count: usize,
    pub rejected_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecuteBlockResponse {
    pub block_height: u64,
    pub block_event_count: usize,
    pub node_state_update_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
