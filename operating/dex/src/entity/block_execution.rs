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
    pub action_type: String,
    pub payload_hash: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateRoot(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainState {
    pub height: u64,
    pub state_root: StateRoot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionRuleSet {
    pub version: String,
    pub rule_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionTrace {
    pub block_height: u64,
    pub input_root: String,
    pub events_hash: String,
    pub state_diff_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateDiff {
    pub block_height: u64,
    pub order_book_delta_hash: String,
    pub position_delta_hash: String,
    pub balance_delta_hash: String,
    pub margin_delta_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommittedBlock {
    pub block_height: u64,
    pub input_root: String,
    pub state_root: StateRoot,
    pub execution_trace_hash: String,
    pub state_diff_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockEvent {
    pub event_id: String,
    pub block_height: u64,
    pub event_type: String,
    pub payload_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeStateUpdate {
    pub block_height: u64,
    pub state_root: StateRoot,
    pub update_hash: String,
}
