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
    pub account_delta_hash: String,
    pub storage_delta_hash: String,
    pub code_delta_hash: String,
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
