pub use crate::{
    BlockEvent, ChainState, CommittedBlock, ExecutionRuleSet, ExecutionTrace, NodeStateUpdate,
    StateDiff, StateRoot, VmCapability, VmKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedTransactionRequest {
    pub request_id: String,
    pub account: String,
    pub nonce: String,
    pub expires_at: String,
    pub action_type: String,
    pub payload_hash: String,
    pub signature_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngressDecision {
    Admit,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingRequest {
    pub request_id: String,
    pub performer: String,
    pub vm_kind: VmKind,
    pub capability: VmCapability,
    pub action_type: String,
    pub payload_hash: String,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    pub result_id: String,
    pub source_process: String,
    pub events_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderedBlockInput {
    pub sequence: u64,
    pub request_id: String,
    pub action_type: String,
    pub payload_hash: String,
    pub source_process: String,
}
