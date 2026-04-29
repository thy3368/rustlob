use cmd_handler::DomainEventSet;
use cmd_handler::use_case_def::{CommandUseCase, LoadState, UseCaseReplyMapper};
use std::sync::Arc;

use crate::{
    Account, AccountDelta, BlockEvent, BlockStateChanges, ChainState, CodeBlob, CodeDelta,
    CommittedBlock, ExecutionResult, ExecutionRuleSet, ExecutionTrace, NodeStateUpdate,
    OrderedBlockInput, PendingRequest, ProductEvent, Receipt, StateDiff, StorageDelta, StateRoot,
    VmExecutionInput, VmExecutionOutput, VmRegistry, VmRuntime, VmRuntimeError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecuteAndCommitBlockError {
    EmptyPendingRequests,
    InvalidBlockHeight,
    LoadStateFailed(String),
    VmExecutionFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteAndCommitBlockCmd {
    pub block_height: u64,
    pub pending_requests: Vec<PendingRequest>,
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
pub struct ExecuteAndCommitBlockEvents {
    pub committed_block: CommittedBlock,
    pub execution_trace: ExecutionTrace,
    pub state_diff: StateDiff,
    pub state_changes: BlockStateChanges,
    pub block_events: Vec<BlockEvent>,
    pub node_state_updates: Vec<NodeStateUpdate>,
}

impl DomainEventSet for ExecuteAndCommitBlockEvents {
    fn domain_event_count(&self) -> usize {
        1 + self.block_events.len() + self.node_state_updates.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteAndCommitBlockReply {
    pub block_height: u64,
    pub block_event_count: usize,
    pub node_state_update_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteAndCommitBlockReplyMapper;

impl UseCaseReplyMapper<ExecuteAndCommitBlockEvents> for ExecuteAndCommitBlockReplyMapper {
    type Reply = ExecuteAndCommitBlockReply;

    fn map(&self, events: ExecuteAndCommitBlockEvents) -> Self::Reply {
        ExecuteAndCommitBlockReply {
            block_height: events.committed_block.block_height,
            block_event_count: events.block_events.len(),
            node_state_update_count: events.node_state_updates.len(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteAndCommitBlockUseCase;

impl ExecuteAndCommitBlockUseCase {
    fn vm_registry() -> VmRegistry<PendingRequest> {
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
        pending_requests: &[PendingRequest],
    ) -> Result<Vec<VmExecutionOutput>, ExecuteAndCommitBlockError> {
        let registry = Self::vm_registry();
        pending_requests
            .iter()
            .cloned()
            .map(|request| {
                registry
                    .execute(VmExecutionInput::from_pending_request(
                        request.vm_kind,
                        request.capability.clone(),
                        request,
                    ))
                    .map_err(|error| ExecuteAndCommitBlockError::VmExecutionFailed(format!("{error:?}")))
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

impl CommandUseCase for ExecuteAndCommitBlockUseCase {
    type Command = ExecuteAndCommitBlockCmd;
    type GivenState = ExecuteAndCommitBlockStateSnapshot;
    type Events = ExecuteAndCommitBlockEvents;
    type Error = ExecuteAndCommitBlockError;
    type LoadPort = dyn LoadState<
            ExecuteAndCommitBlockCmd,
            ExecuteAndCommitBlockStateSnapshot,
            ExecuteAndCommitBlockError,
        >;

    fn actor(&self) -> &'static str {
        "BlockExecutor"
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

    fn then_event_4_new_state(
        &self,
        _cmd: &Self::Command,
        mut state: Self::GivenState,
    ) -> Result<Self::Events, Self::Error> {
        let outputs = Self::execute_pending_requests(&state.pending_requests)?;
        state.state_changes = Self::merge_state_changes(&outputs);
        state.state_diff.account_delta_hash = Self::account_delta_hash(&state.state_changes);
        state.state_diff.storage_delta_hash = Self::storage_delta_hash(&state.state_changes);
        state.state_diff.code_delta_hash = Self::code_delta_hash(&state.state_changes);
        Ok(ExecuteAndCommitBlockEvents {
            committed_block: state.committed_block,
            execution_trace: state.execution_trace,
            state_diff: state.state_diff,
            state_changes: state.state_changes,
            block_events: state.block_events,
            node_state_updates: state.node_state_updates,
        })
    }
}

struct StaticVmRuntime {
    gas_used: u64,
    event_prefix: &'static str,
}

impl VmRuntime<PendingRequest> for StaticVmRuntime {
    fn execute(&self, input: VmExecutionInput<PendingRequest>) -> Result<VmExecutionOutput, VmRuntimeError> {
        let address = ExecuteAndCommitBlockUseCase::address_from_request(&input.transaction.performer, self.gas_used);
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
                            self.event_prefix, input.transaction.action_type, input.transaction.payload_hash
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
    use super::*;

    fn pending_request() -> PendingRequest {
        PendingRequest {
            request_id: "req-1".to_string(),
            performer: "acct-1".to_string(),
            vm_kind: crate::VmKind::RustVm,
            capability: crate::VmCapability::new("dex.prep.place_order"),
            action_type: "order".to_string(),
            payload_hash: "payload-1".to_string(),
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

    struct StubLoadPort;

    impl
        LoadState<
            ExecuteAndCommitBlockCmd,
            ExecuteAndCommitBlockStateSnapshot,
            ExecuteAndCommitBlockError,
        > for StubLoadPort
    {
        fn load_state(
            &self,
            cmd: &ExecuteAndCommitBlockCmd,
        ) -> Result<ExecuteAndCommitBlockStateSnapshot, ExecuteAndCommitBlockError> {
            let request = pending_request();
            let ordered_input = OrderedBlockInput {
                sequence: 1,
                request_id: request.request_id.clone(),
                action_type: request.action_type.clone(),
                payload_hash: request.payload_hash.clone(),
                source_process: "SingleNodeBlockExecutionProcess".to_string(),
            };

            Ok(ExecuteAndCommitBlockStateSnapshot {
                pending_requests: vec![request],
                execution_results: vec![ExecutionResult {
                    result_id: "result-1".to_string(),
                    source_process: ordered_input.source_process,
                    events_hash: "events-1".to_string(),
                }],
                chain_state: ChainState {
                    height: cmd.block_height - 1,
                    state_root: StateRoot("parent-root-1".to_string()),
                },
                execution_rules: ExecutionRuleSet {
                    version: "v1".to_string(),
                    rule_hash: "rules-1".to_string(),
                },
                committed_block: committed_block(cmd.block_height),
                execution_trace: ExecutionTrace {
                    block_height: cmd.block_height,
                    input_root: "input-root-1".to_string(),
                    events_hash: "events-1".to_string(),
                    state_diff_hash: "diff-hash-1".to_string(),
                },
                state_diff: StateDiff {
                    block_height: cmd.block_height,
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
                    block_height: cmd.block_height,
                    event_type: "block_committed".to_string(),
                    payload_hash: "payload-1".to_string(),
                }],
                node_state_updates: vec![NodeStateUpdate {
                    block_height: cmd.block_height,
                    state_root: StateRoot("state-root-1".to_string()),
                    update_hash: "update-1".to_string(),
                }],
            })
        }
    }

    struct EmptyLoadPort;

    impl
        LoadState<
            ExecuteAndCommitBlockCmd,
            ExecuteAndCommitBlockStateSnapshot,
            ExecuteAndCommitBlockError,
        > for EmptyLoadPort
    {
        fn load_state(
            &self,
            _cmd: &ExecuteAndCommitBlockCmd,
        ) -> Result<ExecuteAndCommitBlockStateSnapshot, ExecuteAndCommitBlockError> {
            Err(ExecuteAndCommitBlockError::EmptyPendingRequests)
        }
    }

    #[test]
    fn actor_is_block_executor() {
        assert_eq!(ExecuteAndCommitBlockUseCase.actor(), "BlockExecutor");
    }

    #[test]
    fn rejects_invalid_block_height() {
        let cmd =
            ExecuteAndCommitBlockCmd { block_height: 0, pending_requests: vec![pending_request()] };

        assert_eq!(
            ExecuteAndCommitBlockUseCase.pre_check_command(&cmd),
            Err(ExecuteAndCommitBlockError::InvalidBlockHeight)
        );
    }

    #[test]
    fn rejects_empty_pending_requests_from_load_port() {
        let cmd = ExecuteAndCommitBlockCmd { block_height: 1, pending_requests: vec![] };

        let result = ExecuteAndCommitBlockUseCase.load_state(&cmd, &EmptyLoadPort);

        assert_eq!(result.unwrap_err(), ExecuteAndCommitBlockError::EmptyPendingRequests);
    }

    #[test]
    fn maps_events_to_reply() {
        let events = ExecuteAndCommitBlockEvents {
            committed_block: committed_block(1),
            execution_trace: ExecutionTrace {
                block_height: 1,
                input_root: "input-root-1".to_string(),
                events_hash: "events-1".to_string(),
                state_diff_hash: "diff-hash-1".to_string(),
            },
            state_diff: StateDiff {
                block_height: 1,
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
                block_height: 1,
                event_type: "block_committed".to_string(),
                payload_hash: "payload-1".to_string(),
            }],
            node_state_updates: vec![NodeStateUpdate {
                block_height: 1,
                state_root: StateRoot("state-root-1".to_string()),
                update_hash: "update-1".to_string(),
            }],
        };

        let reply = ExecuteAndCommitBlockReplyMapper.map(events);

        assert_eq!(reply.block_height, 1);
        assert_eq!(reply.block_event_count, 1);
        assert_eq!(reply.node_state_update_count, 1);
    }

    #[test]
    fn completes_minimal_command_path_through_vm_registry() {
        let cmd =
            ExecuteAndCommitBlockCmd { block_height: 1, pending_requests: vec![pending_request()] };

        let state = ExecuteAndCommitBlockUseCase.load_state(&cmd, &StubLoadPort).unwrap();
        ExecuteAndCommitBlockUseCase.validate_against_state(&cmd, &state).unwrap();
        let events = ExecuteAndCommitBlockUseCase.then_event_4_new_state(&cmd, state).unwrap();

        assert_eq!(events.committed_block.block_height, 1);
        assert_eq!(events.domain_event_count(), 3);
        assert_eq!(events.state_changes.account_deltas.len(), 1);
        assert_eq!(events.state_changes.storage_deltas.len(), 1);
        assert_eq!(events.state_changes.code_deltas.len(), 1);
        assert_ne!(events.state_diff.account_delta_hash, "account-delta-1");
        assert_ne!(events.state_diff.storage_delta_hash, "storage-delta-1");
        assert_ne!(events.state_diff.code_delta_hash, "code-delta-1");
        assert_eq!(events.state_changes.account_deltas[0].current.as_ref().unwrap().vm_kind, crate::VmKind::RustVm);
    }
}
