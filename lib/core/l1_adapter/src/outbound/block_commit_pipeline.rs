use std::sync::{Arc, Mutex};

use cmd_handler::EntityReplayableEvent;
use l1_core::{
    Account, AccountDelta, BlockStateChanges, CodeBlob, CodeDelta, ExecuteAndCommitBlockError,
    StorageDelta, VmKind,
};

use crate::MdbxStateStore;

const COMMITTED_BLOCK_ENTITY_TYPE: u8 = 1;
const ACCOUNT_DELTA_ENTITY_TYPE: u8 = 4;
const STORAGE_DELTA_ENTITY_TYPE: u8 = 5;
const CODE_DELTA_ENTITY_TYPE: u8 = 6;
const BLOCK_EVENT_ENTITY_TYPE: u8 = 7;
const NODE_STATE_UPDATE_ENTITY_TYPE: u8 = 8;

pub struct ExecuteAndCommitBlockStatePipeline {
    pub state_store: Arc<Mutex<MdbxStateStore>>,
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

fn restore_hex(value: &str) -> String {
    if value.starts_with("0x") || value.len() != 64 {
        value.to_string()
    } else {
        format!("0x{value}")
    }
}

fn parse_vm_kind(value: &str) -> Option<VmKind> {
    match value {
        "Evm" => Some(VmKind::Evm),
        "RustVm" => Some(VmKind::RustVm),
        _ => None,
    }
}

fn committed_block_height(events: &[EntityReplayableEvent]) -> Option<u64> {
    let event = events.iter().find(|event| event.entity_type == COMMITTED_BLOCK_ENTITY_TYPE)?;
    parse_u64_field(event, "block_height")
}

fn state_changes_from_events(events: &[EntityReplayableEvent]) -> BlockStateChanges {
    let account_deltas = events
        .iter()
        .filter(|event| event.entity_type == ACCOUNT_DELTA_ENTITY_TYPE)
        .filter_map(|event| {
            let address = event_field(event, "address")?.parse().ok()?;
            let nonce = parse_u64_field(event, "nonce")?;
            let balance = event_field(event, "balance")?.parse().ok()?;
            let code_hash = restore_hex(event_field(event, "code_hash")?).parse().ok()?;
            let storage_root = restore_hex(event_field(event, "storage_root")?).parse().ok()?;
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
                key: restore_hex(event_field(event, "key")?).parse().ok()?,
                previous: restore_hex(event_field(event, "previous")?).parse().ok()?,
                current: restore_hex(event_field(event, "current")?).parse().ok()?,
            })
        })
        .collect();

    let code_deltas = events
        .iter()
        .filter(|event| event.entity_type == CODE_DELTA_ENTITY_TYPE)
        .filter_map(|event| {
            let code_hash = restore_hex(event_field(event, "code_hash")?).parse().ok()?;
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

impl ExecuteAndCommitBlockStatePipeline {
    pub fn persist(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ExecuteAndCommitBlockError> {
        use minstant::Instant;

        let started_at = Instant::now();
        let block_height = committed_block_height(events).ok_or_else(|| {
            ExecuteAndCommitBlockError::LoadStateFailed(
                "missing committed block event in replayable events".to_string(),
            )
        })?;
        let state_changes = state_changes_from_events(events);

        tracing::trace!(
            adapter = "ExecuteAndCommitBlockStatePipeline",
            adapter_kind = "outbound",
            port = "MdbxStateStore",
            action = "persist",
            block_height,
            account_delta_count = state_changes.account_deltas.len() as u64,
            storage_delta_count = state_changes.storage_deltas.len() as u64,
            code_delta_count = state_changes.code_deltas.len() as u64,
            block_event_count =
                events.iter().filter(|event| event.entity_type == BLOCK_EVENT_ENTITY_TYPE).count()
                    as u64,
            node_state_update_count = events
                .iter()
                .filter(|event| event.entity_type == NODE_STATE_UPDATE_ENTITY_TYPE)
                .count() as u64,
            status = "start",
            "l1 adapter persist started"
        );

        let mut state_store = self.state_store.lock().map_err(|error| {
            let error_message = error.to_string();
            tracing::trace!(
                call_stack = true,
                layer = "outbound",
                component = "ExecuteAndCommitBlockStatePipeline",
                operation = "persist",
                request_block_height = block_height,
                request_account_delta_count = state_changes.account_deltas.len() as u64,
                request_storage_delta_count = state_changes.storage_deltas.len() as u64,
                request_code_delta_count = state_changes.code_deltas.len() as u64,
                response_result = "err",
                adapter = "ExecuteAndCommitBlockStatePipeline",
                adapter_kind = "outbound",
                port = "MdbxStateStore",
                action = "persist",
                block_height,
                status = "err",
                latency_ns = started_at.elapsed().as_nanos().min(u64::MAX as u128) as u64,
                error_message = %error_message,
                "l1 adapter persist failed"
            );
            ExecuteAndCommitBlockError::LoadStateFailed(error_message)
        })?;

        state_store.apply_block_state_changes(block_height, &state_changes).map_err(|error| {
            let error_message = error.to_string();
            tracing::trace!(
                call_stack = true,
                layer = "outbound",
                component = "ExecuteAndCommitBlockStatePipeline",
                operation = "persist",
                request_block_height = block_height,
                request_account_delta_count = state_changes.account_deltas.len() as u64,
                request_storage_delta_count = state_changes.storage_deltas.len() as u64,
                request_code_delta_count = state_changes.code_deltas.len() as u64,
                response_result = "err",
                adapter = "ExecuteAndCommitBlockStatePipeline",
                adapter_kind = "outbound",
                port = "MdbxStateStore",
                action = "persist",
                block_height,
                status = "err",
                latency_ns = started_at.elapsed().as_nanos().min(u64::MAX as u128) as u64,
                error_message = %error_message,
                "l1 adapter persist failed"
            );
            ExecuteAndCommitBlockError::LoadStateFailed(error_message)
        })?;

        tracing::trace!(
            call_stack = true,
            layer = "outbound",
            component = "ExecuteAndCommitBlockStatePipeline",
            operation = "persist",
            request_block_height = block_height,
            request_account_delta_count = state_changes.account_deltas.len() as u64,
            request_storage_delta_count = state_changes.storage_deltas.len() as u64,
            request_code_delta_count = state_changes.code_deltas.len() as u64,
            response_applied = true,
            adapter = "ExecuteAndCommitBlockStatePipeline",
            adapter_kind = "outbound",
            port = "MdbxStateStore",
            action = "persist",
            block_height,
            status = "ok",
            latency_ns = started_at.elapsed().as_nanos().min(u64::MAX as u128) as u64,
            "l1 adapter persist completed"
        );
        Ok(())
    }

    pub fn replay(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ExecuteAndCommitBlockError> {
        let block_height = committed_block_height(events).unwrap_or_default();
        tracing::trace!(
            adapter = "ExecuteAndCommitBlockStatePipeline",
            adapter_kind = "outbound",
            port = "MdbxStateStore",
            action = "replay",
            block_height,
            status = "ok",
            "l1 adapter replay completed"
        );
        Ok(())
    }

    pub fn publish(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ExecuteAndCommitBlockError> {
        let block_height = committed_block_height(events).unwrap_or_default();
        tracing::trace!(
            adapter = "ExecuteAndCommitBlockStatePipeline",
            adapter_kind = "outbound",
            port = "MdbxStateStore",
            action = "publish",
            block_height,
            status = "ok",
            "l1 adapter publish completed"
        );
        Ok(())
    }
}
