use std::sync::Arc;

use cmd_handler::command_use_case_def2::{CommandUseCase2, IssuedByParty, UseCaseReplyMapper};
use cmd_handler::{EntityReplayableEvent, ReplayFieldChange};
use thiserror::Error;

use crate::{
    Account, AccountDelta, BlockEvent, BlockStateChanges, ChainState, CodeBlob, CodeDelta,
    CommittedBlock, ExecutionResult, ExecutionRuleSet, ExecutionTrace, NodeStateUpdate,
    PendingRequest, ProductEvent, Receipt, StateDiff, StorageDelta, VmExecutionInput,
    VmExecutionOutput, VmRegistry, VmRuntime, VmRuntimeError, VmRuntimeResolver,
};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ExecuteAndCommitBlockError {
    #[error("pending requests must not be empty")]
    EmptyPendingRequests,
    #[error("block height is inconsistent with current chain state")]
    InvalidBlockHeight,
    #[error("failed to load block execution state: {0}")]
    LoadStateFailed(String),
    #[error("vm execution failed: {0}")]
    VmExecutionFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteAndCommitBlockCmd {
    pub block_height: u64,
    pub trace_id: Option<String>,
    pub block_command_id: Option<String>,
    pub pending_requests: Vec<PendingRequest>,
}

impl IssuedByParty for ExecuteAndCommitBlockCmd {
    fn party_id(&self) -> Option<&str> {
        self.pending_requests.first().map(|request| request.performer.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteAndCommitBlockStateSnapshot {
    pub pending_requests: Vec<PendingRequest>,
    pub execution_results: Vec<ExecutionResult>,
    pub chain_state: ChainState,
    pub execution_rules: ExecutionRuleSet,
    pub committed_block: CommittedBlock,
    pub execution_trace: ExecutionTrace,
    pub state_diff: StateDiff,
    pub state_changes: BlockStateChanges,
    pub block_events: Vec<BlockEvent>,
    pub node_state_updates: Vec<NodeStateUpdate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteAndCommitBlockReply {
    pub block_height: u64,
    pub block_event_count: usize,
    pub node_state_update_count: usize,
    pub matched_trade_count: usize,
}

const COMMITTED_BLOCK_ENTITY_TYPE: u8 = 1;
const EXECUTION_TRACE_ENTITY_TYPE: u8 = 2;
const STATE_DIFF_ENTITY_TYPE: u8 = 3;
const ACCOUNT_DELTA_ENTITY_TYPE: u8 = 4;
const STORAGE_DELTA_ENTITY_TYPE: u8 = 5;
const CODE_DELTA_ENTITY_TYPE: u8 = 6;
const BLOCK_EVENT_ENTITY_TYPE: u8 = 7;
const NODE_STATE_UPDATE_ENTITY_TYPE: u8 = 8;
const PRODUCT_EVENT_ENTITY_TYPE: u8 = 9;
const FIELD_TYPE_STRING: u8 = 0;
const FIELD_TYPE_INT: u8 = 1;

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn next_sequence(sequence: &mut u64) -> u64 {
    let current = *sequence;
    *sequence += 1;
    current
}

fn new_event(entity_type: u8, entity_key: &str, sequence: &mut u64) -> EntityReplayableEvent {
    EntityReplayableEvent::new_created(
        0,
        next_sequence(sequence),
        stable_entity_id(entity_key),
        entity_type,
    )
}

fn string_field(name: &str, value: &str) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.as_bytes(),
        FIELD_TYPE_STRING,
    )
}

fn int_field(name: &str, value: u64) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.to_string().as_bytes(),
        FIELD_TYPE_INT,
    )
}

fn hex_field(name: &str, value: &str) -> ReplayFieldChange {
    let normalized = value.strip_prefix("0x").unwrap_or(value);
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        normalized.as_bytes(),
        FIELD_TYPE_STRING,
    )
}

fn bytes_field(name: &str, value: &[u8]) -> ReplayFieldChange {
    let value = String::from_utf8_lossy(value);
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.as_bytes(),
        FIELD_TYPE_STRING,
    )
}

fn event_field<'a>(event: &'a EntityReplayableEvent, name: &str) -> Option<&'a str> {
    event.field_changes.iter().find_map(|change| {
        let field_name = change.field_name_as_str().ok()?;
        if field_name != name {
            return None;
        }
        std::str::from_utf8(change.new_value_bytes()).ok()
    })
}

fn parse_u64_field(event: &EntityReplayableEvent, name: &str) -> Option<u64> {
    event_field(event, name)?.parse().ok()
}

#[cfg(test)]
fn restore_hex(value: &str) -> String {
    if value.starts_with("0x") || value.len() != 64 {
        value.to_string()
    } else {
        format!("0x{value}")
    }
}

fn vm_kind_label(vm_kind: crate::VmKind) -> &'static str {
    match vm_kind {
        crate::VmKind::Evm => "Evm",
        crate::VmKind::RustVm => "RustVm",
    }
}

#[cfg(test)]
fn parse_vm_kind(value: &str) -> Option<crate::VmKind> {
    match value {
        "Evm" => Some(crate::VmKind::Evm),
        "RustVm" => Some(crate::VmKind::RustVm),
        _ => None,
    }
}

fn encode_committed_block(block: &CommittedBlock, sequence: &mut u64) -> EntityReplayableEvent {
    let mut event = new_event(
        COMMITTED_BLOCK_ENTITY_TYPE,
        &format!("committed-block:{}", block.block_height),
        sequence,
    );
    event.add_field_change(int_field("block_height", block.block_height));
    event.add_field_change(string_field("input_root", &block.input_root));
    event.add_field_change(string_field("state_root", &block.state_root.0));
    event.add_field_change(string_field("execution_trace_hash", &block.execution_trace_hash));
    event.add_field_change(string_field("state_diff_hash", &block.state_diff_hash));
    event
}

fn encode_execution_trace(trace: &ExecutionTrace, sequence: &mut u64) -> EntityReplayableEvent {
    let mut event = new_event(
        EXECUTION_TRACE_ENTITY_TYPE,
        &format!("execution-trace:{}", trace.block_height),
        sequence,
    );
    event.add_field_change(int_field("block_height", trace.block_height));
    event.add_field_change(string_field("input_root", &trace.input_root));
    event.add_field_change(string_field("events_hash", &trace.events_hash));
    event.add_field_change(string_field("state_diff_hash", &trace.state_diff_hash));
    event
}

fn encode_state_diff(state_diff: &StateDiff, sequence: &mut u64) -> EntityReplayableEvent {
    let mut event = new_event(
        STATE_DIFF_ENTITY_TYPE,
        &format!("state-diff:{}", state_diff.block_height),
        sequence,
    );
    event.add_field_change(int_field("block_height", state_diff.block_height));
    event.add_field_change(hex_field("account_delta_hash", &state_diff.account_delta_hash));
    event.add_field_change(hex_field("storage_delta_hash", &state_diff.storage_delta_hash));
    event.add_field_change(hex_field("code_delta_hash", &state_diff.code_delta_hash));
    event
        .add_field_change(string_field("order_book_delta_hash", &state_diff.order_book_delta_hash));
    event.add_field_change(string_field("position_delta_hash", &state_diff.position_delta_hash));
    event.add_field_change(string_field("balance_delta_hash", &state_diff.balance_delta_hash));
    event.add_field_change(string_field("margin_delta_hash", &state_diff.margin_delta_hash));
    event
}

fn encode_account_delta(delta: &AccountDelta, sequence: &mut u64) -> EntityReplayableEvent {
    let mut event =
        new_event(ACCOUNT_DELTA_ENTITY_TYPE, &format!("account-delta:{}", delta.address), sequence);
    let account = delta.current.as_ref().expect("account delta current should exist");
    event.add_field_change(string_field("address", &delta.address.to_string()));
    event.add_field_change(int_field("nonce", account.nonce));
    event.add_field_change(string_field("balance", &account.balance.to_string()));
    event.add_field_change(hex_field("code_hash", &account.code_hash.to_string()));
    event.add_field_change(hex_field("storage_root", &account.storage_root.to_string()));
    event.add_field_change(string_field("vm_kind", vm_kind_label(account.vm_kind)));
    event
}

fn encode_storage_delta(delta: &StorageDelta, sequence: &mut u64) -> EntityReplayableEvent {
    let mut event = new_event(
        STORAGE_DELTA_ENTITY_TYPE,
        &format!("storage-delta:{}:{}", delta.address, delta.key),
        sequence,
    );
    event.add_field_change(string_field("address", &delta.address.to_string()));
    event.add_field_change(hex_field("key", &delta.key.to_string()));
    event.add_field_change(hex_field("previous", &delta.previous.to_string()));
    event.add_field_change(hex_field("current", &delta.current.to_string()));
    event
}

fn encode_code_delta(delta: &CodeDelta, sequence: &mut u64) -> EntityReplayableEvent {
    let mut event =
        new_event(CODE_DELTA_ENTITY_TYPE, &format!("code-delta:{}", delta.code_hash), sequence);
    let code_blob = delta.current.as_ref().expect("code delta current should exist");
    event.add_field_change(hex_field("code_hash", &delta.code_hash.to_string()));
    event.add_field_change(string_field("vm_kind", vm_kind_label(code_blob.vm_kind)));
    event.add_field_change(bytes_field("bytes", &code_blob.bytes));
    event
}

fn encode_block_event(block_event: &BlockEvent, sequence: &mut u64) -> EntityReplayableEvent {
    let mut event = new_event(
        BLOCK_EVENT_ENTITY_TYPE,
        &format!("block-event:{}", block_event.event_id),
        sequence,
    );
    event.add_field_change(string_field("event_id", &block_event.event_id));
    event.add_field_change(int_field("block_height", block_event.block_height));
    event.add_field_change(string_field("event_type", &block_event.event_type));
    event.add_field_change(string_field("payload_hash", &block_event.payload_hash));
    event
}

fn encode_node_state_update(update: &NodeStateUpdate, sequence: &mut u64) -> EntityReplayableEvent {
    let mut event = new_event(
        NODE_STATE_UPDATE_ENTITY_TYPE,
        &format!("node-update:{}:{}", update.block_height, update.update_hash),
        sequence,
    );
    event.add_field_change(int_field("block_height", update.block_height));
    event.add_field_change(string_field("state_root", &update.state_root.0));
    event.add_field_change(string_field("update_hash", &update.update_hash));
    event
}

fn encode_product_event(product_event: &ProductEvent, sequence: &mut u64) -> EntityReplayableEvent {
    let mut event = new_event(
        PRODUCT_EVENT_ENTITY_TYPE,
        &format!("product-event:{}:{}", product_event.product_type, product_event.event_type),
        sequence,
    );
    event.add_field_change(string_field("product_type", &product_event.product_type));
    event.add_field_change(string_field("event_type", &product_event.event_type));
    event.add_field_change(bytes_field("payload", &product_event.payload));
    event
}

fn committed_block_from_events(events: &[EntityReplayableEvent]) -> Option<CommittedBlock> {
    let event = events.iter().find(|event| event.entity_type == COMMITTED_BLOCK_ENTITY_TYPE)?;
    Some(CommittedBlock {
        block_height: parse_u64_field(event, "block_height")?,
        input_root: event_field(event, "input_root")?.to_string(),
        state_root: crate::StateRoot(event_field(event, "state_root")?.to_string()),
        execution_trace_hash: event_field(event, "execution_trace_hash")?.to_string(),
        state_diff_hash: event_field(event, "state_diff_hash")?.to_string(),
    })
}

#[cfg(test)]
fn state_diff_from_events(events: &[EntityReplayableEvent]) -> Option<StateDiff> {
    let event = events.iter().find(|event| event.entity_type == STATE_DIFF_ENTITY_TYPE)?;
    Some(StateDiff {
        block_height: parse_u64_field(event, "block_height")?,
        account_delta_hash: restore_hex(event_field(event, "account_delta_hash")?),
        storage_delta_hash: restore_hex(event_field(event, "storage_delta_hash")?),
        code_delta_hash: restore_hex(event_field(event, "code_delta_hash")?),
        order_book_delta_hash: event_field(event, "order_book_delta_hash")?.to_string(),
        position_delta_hash: event_field(event, "position_delta_hash")?.to_string(),
        balance_delta_hash: event_field(event, "balance_delta_hash")?.to_string(),
        margin_delta_hash: event_field(event, "margin_delta_hash")?.to_string(),
    })
}

#[cfg(test)]
fn state_changes_from_events(events: &[EntityReplayableEvent]) -> BlockStateChanges {
    let account_deltas = events
        .iter()
        .filter(|event| event.entity_type == ACCOUNT_DELTA_ENTITY_TYPE)
        .filter_map(|event| {
            let address = event_field(event, "address")?.parse().ok()?;
            let nonce = parse_u64_field(event, "nonce")?;
            let balance = event_field(event, "balance")?.parse().ok()?;
            let code_hash = event_field(event, "code_hash")?.parse().ok()?;
            let storage_root = event_field(event, "storage_root")?.parse().ok()?;
            let vm_kind = parse_vm_kind(event_field(event, "vm_kind")?)?;
            Some(AccountDelta {
                address,
                previous: None,
                current: Some(Account { nonce, balance, code_hash, storage_root, vm_kind }),
            })
        })
        .collect();

    let storage_deltas = events
        .iter()
        .filter(|event| event.entity_type == STORAGE_DELTA_ENTITY_TYPE)
        .filter_map(|event| {
            Some(StorageDelta {
                address: event_field(event, "address")?.parse().ok()?,
                key: event_field(event, "key")?.parse().ok()?,
                previous: event_field(event, "previous")?.parse().ok()?,
                current: event_field(event, "current")?.parse().ok()?,
            })
        })
        .collect();

    let code_deltas = events
        .iter()
        .filter(|event| event.entity_type == CODE_DELTA_ENTITY_TYPE)
        .filter_map(|event| {
            let code_hash = event_field(event, "code_hash")?.parse().ok()?;
            let vm_kind = parse_vm_kind(event_field(event, "vm_kind")?)?;
            let bytes = event_field(event, "bytes")?.as_bytes().to_vec();
            Some(CodeDelta {
                code_hash,
                previous: None,
                current: Some(CodeBlob { code_hash, vm_kind, bytes }),
            })
        })
        .collect();

    BlockStateChanges { account_deltas, storage_deltas, code_deltas }
}

fn product_events_from_events(events: &[EntityReplayableEvent]) -> Vec<ProductEvent> {
    events
        .iter()
        .filter(|event| event.entity_type == PRODUCT_EVENT_ENTITY_TYPE)
        .filter_map(|event| {
            Some(ProductEvent {
                product_type: event_field(event, "product_type")?.to_string(),
                event_type: event_field(event, "event_type")?.to_string(),
                payload: event_field(event, "payload")?.as_bytes().to_vec(),
            })
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteAndCommitBlockReplyMapper;

impl UseCaseReplyMapper for ExecuteAndCommitBlockReplyMapper {
    type Reply = ExecuteAndCommitBlockReply;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        ExecuteAndCommitBlockReply {
            block_height: committed_block_from_events(&events)
                .map(|block| block.block_height)
                .unwrap_or_default(),
            block_event_count: events
                .iter()
                .filter(|event| event.entity_type == BLOCK_EVENT_ENTITY_TYPE)
                .count(),
            node_state_update_count: events
                .iter()
                .filter(|event| event.entity_type == NODE_STATE_UPDATE_ENTITY_TYPE)
                .count(),
            matched_trade_count: count_matched_trades(&product_events_from_events(&events)),
        }
    }
}

fn count_matched_trades(product_events: &[ProductEvent]) -> usize {
    product_events
        .iter()
        .filter_map(|event| {
            if event.product_type == "Spot" {
                parse_accepted_trade_count(&event.event_type)
            } else {
                None
            }
        })
        .sum()
}

fn parse_accepted_trade_count(event_type: &str) -> Option<usize> {
    let mut parts = event_type.split(':');
    if parts.next()? != "accepted" {
        return None;
    }
    parts.next()?;
    parts.next()?;
    parts.next()?.parse().ok()
}

pub struct ExecuteAndCommitBlockUseCase {
    vm_resolver: Arc<dyn VmRuntimeResolver<PendingRequest>>,
}

impl ExecuteAndCommitBlockUseCase {
    pub fn new(vm_resolver: Arc<dyn VmRuntimeResolver<PendingRequest>>) -> Self {
        Self { vm_resolver }
    }

    pub fn with_vm_registry(vm_registry: VmRegistry<PendingRequest>) -> Self {
        Self::new(Arc::new(vm_registry))
    }

    fn default_vm_registry() -> VmRegistry<PendingRequest> {
        let mut registry = VmRegistry::new();
        registry.register_runtime(
            crate::VmKind::RustVm,
            Arc::new(StaticVmRuntime { gas_used: 11, event_prefix: "rustvm" }),
        );
        registry.register_runtime(
            crate::VmKind::Evm,
            Arc::new(StaticVmRuntime { gas_used: 29, event_prefix: "evm" }),
        );
        registry
    }

    fn execute_pending_requests(
        &self,
        pending_requests: &[PendingRequest],
    ) -> Result<Vec<VmExecutionOutput>, ExecuteAndCommitBlockError> {
        pending_requests
            .iter()
            .cloned()
            .map(|request| {
                self.vm_resolver
                    .execute(VmExecutionInput::from_pending_request(
                        request.vm_kind,
                        request.capability.clone(),
                        request,
                    ))
                    .map_err(|error| {
                        ExecuteAndCommitBlockError::VmExecutionFailed(format!("{error:?}"))
                    })
            })
            .collect()
    }

    fn merge_state_changes(outputs: &[VmExecutionOutput]) -> BlockStateChanges {
        let mut merged = BlockStateChanges::default();
        for output in outputs {
            merged.account_deltas.extend(output.state_changes.account_deltas.clone());
            merged.storage_deltas.extend(output.state_changes.storage_deltas.clone());
            merged.code_deltas.extend(output.state_changes.code_deltas.clone());
        }
        merged
    }

    fn merge_product_events(outputs: &[VmExecutionOutput]) -> Vec<ProductEvent> {
        let mut merged = Vec::new();
        for output in outputs {
            merged.extend(output.product_events.clone());
        }
        merged
    }

    fn hash_to_b256(input: &str) -> alloy_primitives::B256 {
        use std::hash::{Hash, Hasher};

        let mut bytes = [0u8; 32];
        for (chunk_index, chunk) in bytes.chunks_mut(8).enumerate() {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            chunk_index.hash(&mut hasher);
            input.hash(&mut hasher);
            chunk.copy_from_slice(&hasher.finish().to_be_bytes());
        }
        alloy_primitives::B256::new(bytes)
    }

    fn hash_strings(parts: impl IntoIterator<Item = String>) -> String {
        let joined = parts.into_iter().collect::<Vec<_>>().join("|");
        format!("{:?}", Self::hash_to_b256(&joined))
    }

    fn account_delta_hash(state_changes: &BlockStateChanges) -> String {
        Self::hash_strings(state_changes.account_deltas.iter().map(|delta| {
            let current = delta.current.as_ref();
            format!(
                "{:?}:{:?}:{:?}:{:?}:{:?}",
                delta.address,
                current.map(|account| account.nonce),
                current.map(|account| account.balance),
                current.map(|account| account.code_hash),
                current.map(|account| account.storage_root),
            )
        }))
    }

    fn storage_delta_hash(state_changes: &BlockStateChanges) -> String {
        Self::hash_strings(state_changes.storage_deltas.iter().map(|delta| {
            format!("{:?}:{:?}:{:?}:{:?}", delta.address, delta.key, delta.previous, delta.current)
        }))
    }

    fn code_delta_hash(state_changes: &BlockStateChanges) -> String {
        Self::hash_strings(state_changes.code_deltas.iter().map(|delta| {
            let current = delta.current.as_ref();
            format!(
                "{:?}:{:?}:{:?}:{:?}",
                delta.code_hash,
                current.map(|code| code.vm_kind),
                current.map(|code| code.bytes.len()),
                current.map(|code| code.bytes.as_slice()),
            )
        }))
    }

    fn address_from_request(performer: &str, salt: u64) -> alloy_primitives::Address {
        let hash = Self::hash_to_b256(&format!("{}:{}", performer, salt));
        alloy_primitives::Address::from_slice(&hash.as_slice()[..20])
    }
}

impl Default for ExecuteAndCommitBlockUseCase {
    fn default() -> Self {
        Self::with_vm_registry(Self::default_vm_registry())
    }
}

impl CommandUseCase2 for ExecuteAndCommitBlockUseCase {
    type Command = ExecuteAndCommitBlockCmd;
    type GivenState = ExecuteAndCommitBlockStateSnapshot;
    type Error = ExecuteAndCommitBlockError;

    fn role(&self) -> &'static str {
        "BlockExecutor"
    }

    fn format_error(&self, error: &Self::Error) -> Option<String> {
        match error {
            ExecuteAndCommitBlockError::EmptyPendingRequests => {
                Some("execute_and_commit_block: empty pending requests".to_string())
            }
            ExecuteAndCommitBlockError::InvalidBlockHeight => {
                Some("execute_and_commit_block: invalid block height".to_string())
            }
            ExecuteAndCommitBlockError::LoadStateFailed(message) => {
                Some(format!("execute_and_commit_block load_state_failed: {message}"))
            }
            ExecuteAndCommitBlockError::VmExecutionFailed(message) => {
                Some(format!("execute_and_commit_block vm_execution_failed: {message}"))
            }
        }
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.block_height == 0 {
            return Err(ExecuteAndCommitBlockError::InvalidBlockHeight);
        }

        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if cmd.block_height != state.committed_block.block_height {
            return Err(ExecuteAndCommitBlockError::InvalidBlockHeight);
        }

        Ok(())
    }

    fn compute_replayable_events(
        &self,
        _cmd: &Self::Command,
        mut state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let outputs = self.execute_pending_requests(&state.pending_requests)?;
        let product_events = Self::merge_product_events(&outputs);
        state.state_changes = Self::merge_state_changes(&outputs);
        state.state_diff.account_delta_hash = Self::account_delta_hash(&state.state_changes);
        state.state_diff.storage_delta_hash = Self::storage_delta_hash(&state.state_changes);
        state.state_diff.code_delta_hash = Self::code_delta_hash(&state.state_changes);
        let mut sequence = 0;
        let mut events = Vec::new();
        events.push(encode_committed_block(&state.committed_block, &mut sequence));
        events.push(encode_execution_trace(&state.execution_trace, &mut sequence));
        events.push(encode_state_diff(&state.state_diff, &mut sequence));
        events.extend(
            state
                .state_changes
                .account_deltas
                .iter()
                .map(|delta| encode_account_delta(delta, &mut sequence)),
        );
        events.extend(
            state
                .state_changes
                .storage_deltas
                .iter()
                .map(|delta| encode_storage_delta(delta, &mut sequence)),
        );
        events.extend(
            state
                .state_changes
                .code_deltas
                .iter()
                .map(|delta| encode_code_delta(delta, &mut sequence)),
        );
        events.extend(
            state.block_events.iter().map(|event| encode_block_event(event, &mut sequence)),
        );
        events.extend(
            state
                .node_state_updates
                .iter()
                .map(|update| encode_node_state_update(update, &mut sequence)),
        );
        events
            .extend(product_events.iter().map(|event| encode_product_event(event, &mut sequence)));
        Ok(events)
    }
}

struct StaticVmRuntime {
    gas_used: u64,
    event_prefix: &'static str,
}

impl VmRuntime<PendingRequest> for StaticVmRuntime {
    fn execute(
        &self,
        input: VmExecutionInput<PendingRequest>,
    ) -> Result<VmExecutionOutput, VmRuntimeError> {
        let address = ExecuteAndCommitBlockUseCase::address_from_request(
            &input.transaction.performer,
            self.gas_used,
        );
        let code_hash = ExecuteAndCommitBlockUseCase::hash_to_b256(&format!(
            "{}:{}:{}",
            input.capability.0, input.transaction.payload_hash, self.event_prefix
        ));
        let storage_key = ExecuteAndCommitBlockUseCase::hash_to_b256(&input.transaction.request_id);
        let storage_value = ExecuteAndCommitBlockUseCase::hash_to_b256(&format!(
            "{}:{}",
            input.transaction.action_type, input.transaction.payload_hash
        ));

        Ok(VmExecutionOutput {
            vm_kind: input.vm_kind,
            capability: input.capability.clone(),
            state_changes: BlockStateChanges {
                account_deltas: vec![AccountDelta {
                    address,
                    previous: None,
                    current: Some(Account {
                        nonce: self.gas_used,
                        balance: alloy_primitives::U256::from(self.gas_used),
                        code_hash,
                        storage_root: storage_value,
                        vm_kind: input.vm_kind,
                    }),
                }],
                storage_deltas: vec![StorageDelta {
                    address,
                    key: storage_key,
                    previous: alloy_primitives::B256::ZERO,
                    current: storage_value,
                }],
                code_deltas: vec![CodeDelta {
                    code_hash,
                    previous: None,
                    current: Some(CodeBlob {
                        code_hash,
                        vm_kind: input.vm_kind,
                        bytes: format!(
                            "{}:{}:{}",
                            self.event_prefix,
                            input.transaction.action_type,
                            input.transaction.payload_hash
                        )
                        .into_bytes(),
                    }),
                }],
            },
            receipts: vec![Receipt {
                success: true,
                cumulative_gas_used: self.gas_used,
                logs: vec![],
                bloom: alloy_primitives::Bloom::ZERO,
            }],
            gas_used: self.gas_used,
            product_events: vec![ProductEvent {
                product_type: format!("{:?}", input.vm_kind),
                event_type: input.capability.0,
                payload: input.transaction.payload_hash.clone().into_bytes(),
            }],
        })
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{Address, B256, Bloom, U256};
    use proptest::prelude::*;

    use super::*;
    use crate::StateRoot;

    struct StubVmResolver {
        output: VmExecutionOutput,
    }

    impl VmRuntimeResolver<PendingRequest> for StubVmResolver {
        fn execute(
            &self,
            _input: VmExecutionInput<PendingRequest>,
        ) -> Result<VmExecutionOutput, VmRuntimeError> {
            Ok(self.output.clone())
        }
    }

    fn stub_vm_output() -> VmExecutionOutput {
        let address = Address::repeat_byte(0x11);
        let code_hash = B256::repeat_byte(0x22);
        let storage_key = B256::repeat_byte(0x33);
        let storage_value = B256::repeat_byte(0x44);

        VmExecutionOutput {
            vm_kind: crate::VmKind::RustVm,
            capability: crate::VmCapability::new("dex.prep.place_order"),
            state_changes: BlockStateChanges {
                account_deltas: vec![AccountDelta {
                    address,
                    previous: None,
                    current: Some(Account {
                        nonce: 77,
                        balance: U256::from(123u64),
                        code_hash,
                        storage_root: storage_value,
                        vm_kind: crate::VmKind::RustVm,
                    }),
                }],
                storage_deltas: vec![StorageDelta {
                    address,
                    key: storage_key,
                    previous: B256::ZERO,
                    current: storage_value,
                }],
                code_deltas: vec![CodeDelta {
                    code_hash,
                    previous: None,
                    current: Some(CodeBlob {
                        code_hash,
                        vm_kind: crate::VmKind::RustVm,
                        bytes: b"stub-runtime".to_vec(),
                    }),
                }],
            },
            receipts: vec![Receipt {
                success: true,
                cumulative_gas_used: 77,
                logs: vec![],
                bloom: Bloom::ZERO,
            }],
            gas_used: 77,
            product_events: vec![ProductEvent {
                product_type: "StubProduct".to_string(),
                event_type: "stub.executed".to_string(),
                payload: b"stub-payload".to_vec(),
            }],
        }
    }

    fn pending_request() -> PendingRequest {
        PendingRequest {
            trace_id: Some("trace-1".to_string()),
            request_id: "req-1".to_string(),
            performer: "acct-1".to_string(),
            vm_kind: crate::VmKind::RustVm,
            capability: crate::VmCapability::new("dex.prep.place_order"),
            action_type: "order".to_string(),
            payload_hash: "payload-1".to_string(),
            payload: None,
        }
    }

    fn committed_block(height: u64) -> CommittedBlock {
        CommittedBlock {
            block_height: height,
            input_root: "input-root-1".to_string(),
            state_root: StateRoot("state-root-1".to_string()),
            execution_trace_hash: "trace-hash-1".to_string(),
            state_diff_hash: "diff-hash-1".to_string(),
        }
    }

    fn state_snapshot(
        block_height: u64,
        pending_requests: Vec<PendingRequest>,
    ) -> ExecuteAndCommitBlockStateSnapshot {
        ExecuteAndCommitBlockStateSnapshot {
            pending_requests,
            execution_results: vec![ExecutionResult {
                result_id: "result-1".to_string(),
                source_process: "SingleNodeBlockExecutionProcess".to_string(),
                events_hash: "events-1".to_string(),
            }],
            chain_state: ChainState {
                height: block_height.saturating_sub(1),
                state_root: StateRoot("parent-root-1".to_string()),
            },
            execution_rules: ExecutionRuleSet {
                version: "v1".to_string(),
                rule_hash: "rules-1".to_string(),
            },
            committed_block: committed_block(block_height),
            execution_trace: ExecutionTrace {
                block_height,
                input_root: "input-root-1".to_string(),
                events_hash: "events-1".to_string(),
                state_diff_hash: "diff-hash-1".to_string(),
            },
            state_diff: StateDiff {
                block_height,
                account_delta_hash: "account-delta-1".to_string(),
                storage_delta_hash: "storage-delta-1".to_string(),
                code_delta_hash: "code-delta-1".to_string(),
                order_book_delta_hash: "book-delta-1".to_string(),
                position_delta_hash: "position-delta-1".to_string(),
                balance_delta_hash: "balance-delta-1".to_string(),
                margin_delta_hash: "margin-delta-1".to_string(),
            },
            state_changes: BlockStateChanges::default(),
            block_events: vec![BlockEvent {
                event_id: "event-1".to_string(),
                block_height,
                event_type: "block_committed".to_string(),
                payload_hash: "payload-1".to_string(),
            }],
            node_state_updates: vec![NodeStateUpdate {
                block_height,
                state_root: StateRoot("state-root-1".to_string()),
                update_hash: "update-1".to_string(),
            }],
        }
    }

    fn vm_kind_strategy() -> impl Strategy<Value = crate::VmKind> {
        prop_oneof![Just(crate::VmKind::RustVm), Just(crate::VmKind::Evm)]
    }

    fn pending_request_strategy() -> impl Strategy<Value = PendingRequest> {
        (
            any::<u64>(),
            any::<u64>(),
            vm_kind_strategy(),
            any::<u16>(),
            any::<u16>(),
            any::<u64>(),
            proptest::option::of(any::<u32>()),
        )
            .prop_map(
                |(
                    request_id,
                    performer,
                    vm_kind,
                    capability,
                    action_type,
                    payload_hash,
                    payload,
                )| {
                    PendingRequest {
                        trace_id: Some(format!("trace-{request_id}")),
                        request_id: format!("req-{request_id}"),
                        performer: format!("acct-{performer}"),
                        vm_kind,
                        capability: crate::VmCapability::new(format!("cap-{capability}")),
                        action_type: format!("action-{action_type}"),
                        payload_hash: format!("payload-{payload_hash}"),
                        payload: payload.map(|value| format!("body-{value}")),
                    }
                },
            )
    }

    fn product_event_case_strategy() -> impl Strategy<Value = (ProductEvent, usize)> {
        prop_oneof![
            (0usize..32).prop_map(|trade_count| (
                ProductEvent {
                    product_type: "Spot".to_string(),
                    event_type: format!("accepted:maker:taker:{trade_count}"),
                    payload: Vec::new(),
                },
                trade_count,
            )),
            any::<u16>().prop_map(|suffix| (
                ProductEvent {
                    product_type: "Spot".to_string(),
                    event_type: format!("rejected:{suffix}"),
                    payload: Vec::new(),
                },
                0,
            )),
            (
                prop_oneof![
                    Just("RustVm".to_string()),
                    Just("Evm".to_string()),
                    Just("Perp".to_string())
                ],
                any::<u16>(),
                any::<u8>(),
            )
                .prop_map(|(product_type, suffix, trade_count)| (
                    ProductEvent {
                        product_type,
                        event_type: format!("accepted:x:y:{trade_count}:{suffix}"),
                        payload: Vec::new(),
                    },
                    0,
                )),
        ]
    }

    fn expected_gas_used(vm_kind: crate::VmKind) -> u64 {
        match vm_kind {
            crate::VmKind::RustVm => 11,
            crate::VmKind::Evm => 29,
        }
    }

    struct StubLoadPort;

    impl StubLoadPort {
        fn load_state(
            &self,
            cmd: &ExecuteAndCommitBlockCmd,
        ) -> Result<ExecuteAndCommitBlockStateSnapshot, ExecuteAndCommitBlockError> {
            Ok(state_snapshot(cmd.block_height, vec![pending_request()]))
        }
    }

    struct EmptyLoadPort;

    impl EmptyLoadPort {
        fn load_state(
            &self,
            _cmd: &ExecuteAndCommitBlockCmd,
        ) -> Result<ExecuteAndCommitBlockStateSnapshot, ExecuteAndCommitBlockError> {
            Err(ExecuteAndCommitBlockError::EmptyPendingRequests)
        }
    }

    #[test]
    fn role_is_block_executor() {
        let use_case = ExecuteAndCommitBlockUseCase::default();
        assert_eq!(use_case.role(), "BlockExecutor");
    }

    #[test]
    fn rejects_invalid_block_height() {
        let use_case = ExecuteAndCommitBlockUseCase::default();
        let cmd = ExecuteAndCommitBlockCmd {
            block_height: 0,
            trace_id: None,
            block_command_id: None,
            pending_requests: vec![pending_request()],
        };

        assert_eq!(
            use_case.pre_check_command(&cmd),
            Err(ExecuteAndCommitBlockError::InvalidBlockHeight)
        );
    }

    #[test]
    fn rejects_empty_pending_requests_from_load_port() {
        let cmd = ExecuteAndCommitBlockCmd {
            block_height: 1,
            trace_id: None,
            block_command_id: None,
            pending_requests: vec![],
        };

        let result = EmptyLoadPort.load_state(&cmd);

        assert_eq!(result.unwrap_err(), ExecuteAndCommitBlockError::EmptyPendingRequests);
    }

    #[test]
    fn maps_events_to_reply() {
        let mut sequence = 0;
        let events = vec![
            encode_committed_block(&committed_block(1), &mut sequence),
            encode_execution_trace(
                &ExecutionTrace {
                    block_height: 1,
                    input_root: "input-root-1".to_string(),
                    events_hash: "events-1".to_string(),
                    state_diff_hash: "diff-hash-1".to_string(),
                },
                &mut sequence,
            ),
            encode_state_diff(
                &StateDiff {
                    block_height: 1,
                    account_delta_hash: "account-delta-1".to_string(),
                    storage_delta_hash: "storage-delta-1".to_string(),
                    code_delta_hash: "code-delta-1".to_string(),
                    order_book_delta_hash: "book-delta-1".to_string(),
                    position_delta_hash: "position-delta-1".to_string(),
                    balance_delta_hash: "balance-delta-1".to_string(),
                    margin_delta_hash: "margin-delta-1".to_string(),
                },
                &mut sequence,
            ),
            encode_block_event(
                &BlockEvent {
                    event_id: "event-1".to_string(),
                    block_height: 1,
                    event_type: "block_committed".to_string(),
                    payload_hash: "payload-1".to_string(),
                },
                &mut sequence,
            ),
            encode_node_state_update(
                &NodeStateUpdate {
                    block_height: 1,
                    state_root: StateRoot("state-root-1".to_string()),
                    update_hash: "update-1".to_string(),
                },
                &mut sequence,
            ),
            encode_product_event(
                &ProductEvent {
                    product_type: "Spot".to_string(),
                    event_type: "accepted:1:1:1".to_string(),
                    payload: b"payload-1".to_vec(),
                },
                &mut sequence,
            ),
        ];

        let reply = ExecuteAndCommitBlockReplyMapper.map(events);

        assert_eq!(reply.block_height, 1);
        assert_eq!(reply.block_event_count, 1);
        assert_eq!(reply.node_state_update_count, 1);
        assert_eq!(reply.matched_trade_count, 1);
    }

    #[test]
    fn command_envelope_keeps_trace_meta_outside_business_command() {
        let envelope = cmd_handler::command_use_case_def2::CommandEnvelope {
            meta: cmd_handler::command_use_case_def2::CommandMeta {
                trace_id: Some("trace-block-101".to_string()),
                command_id: Some("execute-block-101".to_string()),
            },
            command: ExecuteAndCommitBlockCmd {
                block_height: 101,
                trace_id: None,
                block_command_id: None,
                pending_requests: vec![pending_request()],
            },
        };

        assert_eq!(envelope.meta.trace_id.as_deref(), Some("trace-block-101"));
        assert_eq!(envelope.meta.command_id.as_deref(), Some("execute-block-101"));
        assert_eq!(envelope.command.block_height, 101);
        assert_eq!(envelope.command.pending_requests.len(), 1);
    }

    #[test]
    fn completes_minimal_command_path_through_vm_registry() {
        let use_case = ExecuteAndCommitBlockUseCase::default();
        let cmd = ExecuteAndCommitBlockCmd {
            block_height: 1,
            trace_id: None,
            block_command_id: None,
            pending_requests: vec![pending_request()],
        };

        let state = StubLoadPort.load_state(&cmd).unwrap();
        use_case.validate_against_state(&cmd, &state).unwrap();
        let events = use_case.compute_replayable_events(&cmd, state).unwrap();
        let committed_block = committed_block_from_events(&events).unwrap();
        let state_diff = state_diff_from_events(&events).unwrap();
        let account_events = events
            .iter()
            .filter(|event| event.entity_type == ACCOUNT_DELTA_ENTITY_TYPE)
            .collect::<Vec<_>>();
        let storage_events = events
            .iter()
            .filter(|event| event.entity_type == STORAGE_DELTA_ENTITY_TYPE)
            .collect::<Vec<_>>();
        let code_events = events
            .iter()
            .filter(|event| event.entity_type == CODE_DELTA_ENTITY_TYPE)
            .collect::<Vec<_>>();

        assert_eq!(committed_block.block_height, 1);
        assert_eq!(account_events.len(), 1);
        assert_eq!(storage_events.len(), 1);
        assert_eq!(code_events.len(), 1);
        assert_ne!(state_diff.account_delta_hash, "account-delta-1");
        assert_ne!(state_diff.storage_delta_hash, "storage-delta-1");
        assert_ne!(state_diff.code_delta_hash, "code-delta-1");
        assert_eq!(event_field(account_events[0], "vm_kind"), Some("RustVm"));
    }

    #[test]
    fn injected_vm_resolver_overrides_default_runtime() {
        let use_case = ExecuteAndCommitBlockUseCase::new(Arc::new(StubVmResolver {
            output: stub_vm_output(),
        }));
        let cmd = ExecuteAndCommitBlockCmd {
            block_height: 1,
            trace_id: None,
            block_command_id: None,
            pending_requests: vec![pending_request()],
        };

        let state = StubLoadPort.load_state(&cmd).unwrap();
        let events = use_case.compute_replayable_events(&cmd, state).unwrap();
        let state_diff = state_diff_from_events(&events).unwrap();
        let account_events = events
            .iter()
            .filter(|event| event.entity_type == ACCOUNT_DELTA_ENTITY_TYPE)
            .collect::<Vec<_>>();
        let code_events = events
            .iter()
            .filter(|event| event.entity_type == CODE_DELTA_ENTITY_TYPE)
            .collect::<Vec<_>>();

        assert_eq!(account_events.len(), 1);
        assert_eq!(event_field(account_events[0], "nonce"), Some("77"));
        assert_eq!(event_field(code_events[0], "bytes"), Some("stub-runtime"));
        assert_ne!(state_diff.account_delta_hash, "account-delta-1");
    }

    proptest! {
        #[test]
        fn property_merges_state_changes_and_recomputes_hashes(
            block_height in 1u64..1024,
            pending_requests in proptest::collection::vec(pending_request_strategy(), 1..8),
        ) {
            let use_case = ExecuteAndCommitBlockUseCase::default();
            let cmd = ExecuteAndCommitBlockCmd {
                block_height,
                trace_id: None,
                block_command_id: None,
                pending_requests: pending_requests.clone(),
            };
            let state = state_snapshot(block_height, pending_requests.clone());

            prop_assert_eq!(use_case.validate_against_state(&cmd, &state), Ok(()));

            let events = use_case.compute_replayable_events(&cmd, state).unwrap();
            let committed_block = committed_block_from_events(&events).unwrap();
            let state_diff = state_diff_from_events(&events).unwrap();
            let account_events = events
                .iter()
                .filter(|event| event.entity_type == ACCOUNT_DELTA_ENTITY_TYPE)
                .collect::<Vec<_>>();
            let storage_events = events
                .iter()
                .filter(|event| event.entity_type == STORAGE_DELTA_ENTITY_TYPE)
                .collect::<Vec<_>>();
            let code_events = events
                .iter()
                .filter(|event| event.entity_type == CODE_DELTA_ENTITY_TYPE)
                .collect::<Vec<_>>();
            let product_events = events
                .iter()
                .filter(|event| event.entity_type == PRODUCT_EVENT_ENTITY_TYPE)
                .collect::<Vec<_>>();
            let expected_outputs = use_case.execute_pending_requests(&pending_requests).unwrap();
            let expected_state_changes = ExecuteAndCommitBlockUseCase::merge_state_changes(&expected_outputs);

            prop_assert_eq!(committed_block.block_height, block_height);
            prop_assert_eq!(account_events.len(), pending_requests.len());
            prop_assert_eq!(storage_events.len(), pending_requests.len());
            prop_assert_eq!(code_events.len(), pending_requests.len());
            prop_assert_eq!(product_events.len(), pending_requests.len());
            prop_assert_eq!(
                state_diff.account_delta_hash,
                ExecuteAndCommitBlockUseCase::account_delta_hash(&expected_state_changes),
            );
            prop_assert_eq!(
                state_diff.storage_delta_hash,
                ExecuteAndCommitBlockUseCase::storage_delta_hash(&expected_state_changes),
            );
            prop_assert_eq!(
                state_diff.code_delta_hash,
                ExecuteAndCommitBlockUseCase::code_delta_hash(&expected_state_changes),
            );

            for (((request, account_event), storage_event), (code_event, product_event)) in pending_requests
                .iter()
                .zip(account_events.iter())
                .zip(storage_events.iter())
                .zip(code_events.iter().zip(product_events.iter()))
            {
                let gas_used = expected_gas_used(request.vm_kind);
                let expected_code_bytes = format!(
                    "{}:{}:{}",
                    match request.vm_kind {
                        crate::VmKind::RustVm => "rustvm",
                        crate::VmKind::Evm => "evm",
                    },
                    request.action_type,
                    request.payload_hash
                )
                .into_bytes();
                let expected_nonce = gas_used.to_string();
                let expected_balance = U256::from(gas_used).to_string();
                let expected_code_bytes_text = String::from_utf8_lossy(&expected_code_bytes).into_owned();
                let expected_product_type = format!("{:?}", request.vm_kind);

                prop_assert_eq!(event_field(account_event, "vm_kind"), Some(vm_kind_label(request.vm_kind)));
                prop_assert_eq!(event_field(account_event, "nonce"), Some(expected_nonce.as_str()));
                prop_assert_eq!(event_field(account_event, "balance"), Some(expected_balance.as_str()));
                prop_assert_eq!(event_field(storage_event, "address"), event_field(account_event, "address"));
                prop_assert_eq!(event_field(code_event, "vm_kind"), Some(vm_kind_label(request.vm_kind)));
                prop_assert_eq!(event_field(code_event, "bytes"), Some(expected_code_bytes_text.as_str()));
                prop_assert_eq!(event_field(product_event, "product_type"), Some(expected_product_type.as_str()));
                prop_assert_eq!(event_field(product_event, "event_type"), Some(request.capability.0.as_str()));
                prop_assert_eq!(event_field(product_event, "payload"), Some(request.payload_hash.as_str()));
            }
        }

        #[test]
        fn property_reply_mapper_counts_only_spot_accepted_trades(
            cases in proptest::collection::vec(product_event_case_strategy(), 0..16),
        ) {
            let expected_trade_count = cases.iter().map(|(_, count)| *count).sum::<usize>();
            let product_events: Vec<ProductEvent> =
                cases.into_iter().map(|(event, _)| event).collect();
            let mut sequence = 0;
            let mut events = vec![encode_committed_block(&committed_block(7), &mut sequence)];
            events.extend(product_events.iter().map(|event| encode_product_event(event, &mut sequence)));

            let reply = ExecuteAndCommitBlockReplyMapper.map(events);

            prop_assert_eq!(reply.matched_trade_count, expected_trade_count);
        }
    }
}
