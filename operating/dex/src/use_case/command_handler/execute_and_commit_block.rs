use cmd_handler::{
    use_case_def::{CommandUseCase, LoadState, UseCaseReplyMapper},
    DomainEventSet,
};

use crate::entity::{
    BlockEvent, ChainState, CommittedBlock, ExecutionResult, ExecutionRuleSet, ExecutionTrace,
    NodeStateUpdate, PendingRequest, StateDiff,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecuteAndCommitBlockError {
    EmptyPendingRequests,
    InvalidBlockHeight,
    LoadStateFailed(String),
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
    pub block_events: Vec<BlockEvent>,
    pub node_state_updates: Vec<NodeStateUpdate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteAndCommitBlockEvents {
    pub committed_block: CommittedBlock,
    pub execution_trace: ExecutionTrace,
    pub state_diff: StateDiff,
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

    fn then(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Events, Self::Error> {
        Ok(ExecuteAndCommitBlockEvents {
            committed_block: state.committed_block,
            execution_trace: state.execution_trace,
            state_diff: state.state_diff,
            block_events: state.block_events,
            node_state_updates: state.node_state_updates,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{OrderedBlockInput, StateRoot};

    fn pending_request() -> PendingRequest {
        PendingRequest {
            request_id: "req-1".to_string(),
            performer: "acct-1".to_string(),
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
                    order_book_delta_hash: "book-delta-1".to_string(),
                    position_delta_hash: "position-delta-1".to_string(),
                    balance_delta_hash: "balance-delta-1".to_string(),
                    margin_delta_hash: "margin-delta-1".to_string(),
                },
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

    #[test]
    fn actor_is_block_executor() {
        assert_eq!(ExecuteAndCommitBlockUseCase.actor(), "BlockExecutor");
    }

    #[test]
    fn rejects_invalid_block_height() {
        let cmd = ExecuteAndCommitBlockCmd {
            block_height: 0,
            pending_requests: vec![pending_request()],
        };

        assert_eq!(
            ExecuteAndCommitBlockUseCase.pre_check_command(&cmd),
            Err(ExecuteAndCommitBlockError::InvalidBlockHeight)
        );
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
    fn rejects_empty_pending_requests_from_load_port() {
        let cmd = ExecuteAndCommitBlockCmd {
            block_height: 1,
            pending_requests: vec![],
        };

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
                order_book_delta_hash: "book-delta-1".to_string(),
                position_delta_hash: "position-delta-1".to_string(),
                balance_delta_hash: "balance-delta-1".to_string(),
                margin_delta_hash: "margin-delta-1".to_string(),
            },
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
    fn completes_minimal_command_path() {
        let cmd = ExecuteAndCommitBlockCmd {
            block_height: 1,
            pending_requests: vec![pending_request()],
        };

        let state = ExecuteAndCommitBlockUseCase
            .load_state(&cmd, &StubLoadPort)
            .unwrap();
        ExecuteAndCommitBlockUseCase
            .validate_against_state(&cmd, &state)
            .unwrap();
        let events = ExecuteAndCommitBlockUseCase.then(&cmd, state).unwrap();

        assert_eq!(events.committed_block.block_height, 1);
        assert_eq!(events.domain_event_count(), 3);
    }
}
