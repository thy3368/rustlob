use cmd_handler::use_case_def::LoadState;
use l1_core::{
    BlockEvent, BlockStateChanges, ChainState, CommittedBlock, ExecuteAndCommitBlockCmd,
    ExecuteAndCommitBlockError, ExecuteAndCommitBlockStateSnapshot, ExecutionResult,
    ExecutionRuleSet, ExecutionTrace, MempoolPort, NodeStateUpdate, StateDiff, StateRoot,
};

pub struct MempoolReadingLoadPort {
    mempool: Box<dyn MempoolPort>,
    batch_size: usize,
}

impl MempoolReadingLoadPort {
    pub fn new(mempool: Box<dyn MempoolPort>, batch_size: usize) -> Self {
        Self { mempool, batch_size }
    }
}

impl LoadState<ExecuteAndCommitBlockCmd, ExecuteAndCommitBlockStateSnapshot, ExecuteAndCommitBlockError>
    for MempoolReadingLoadPort
{
    fn load_state(
        &self,
        cmd: &ExecuteAndCommitBlockCmd,
    ) -> Result<ExecuteAndCommitBlockStateSnapshot, ExecuteAndCommitBlockError> {
        let pending_requests = self
            .mempool
            .fetch_requests(self.batch_size)
            .map_err(|e| ExecuteAndCommitBlockError::LoadStateFailed(format!("{:?}", e)))?;

        if pending_requests.is_empty() {
            return Err(ExecuteAndCommitBlockError::EmptyPendingRequests);
        }

        Ok(ExecuteAndCommitBlockStateSnapshot {
            pending_requests: pending_requests.clone(),
            execution_results: pending_requests
                .iter()
                .map(|req| ExecutionResult {
                    result_id: format!("result-{}", req.request_id),
                    source_process: "BlockExecutor".to_string(),
                    events_hash: format!("events-{}", req.request_id),
                })
                .collect(),
            chain_state: ChainState {
                height: cmd.block_height - 1,
                state_root: StateRoot(format!("state-root-{}", cmd.block_height - 1)),
            },
            execution_rules: ExecutionRuleSet {
                version: "v1".to_string(),
                rule_hash: "rules-1".to_string(),
            },
            committed_block: CommittedBlock {
                block_height: cmd.block_height,
                input_root: format!("input-root-{}", cmd.block_height),
                state_root: StateRoot(format!("state-root-{}", cmd.block_height)),
                execution_trace_hash: format!("trace-{}", cmd.block_height),
                state_diff_hash: format!("diff-{}", cmd.block_height),
            },
            execution_trace: ExecutionTrace {
                block_height: cmd.block_height,
                input_root: format!("input-root-{}", cmd.block_height),
                events_hash: format!("events-{}", cmd.block_height),
                state_diff_hash: format!("diff-{}", cmd.block_height),
            },
            state_diff: StateDiff {
                block_height: cmd.block_height,
                account_delta_hash: format!("account-delta-{}", cmd.block_height),
                storage_delta_hash: format!("storage-delta-{}", cmd.block_height),
                code_delta_hash: format!("code-delta-{}", cmd.block_height),
                order_book_delta_hash: format!("order-book-delta-{}", cmd.block_height),
                position_delta_hash: format!("position-delta-{}", cmd.block_height),
                balance_delta_hash: format!("balance-delta-{}", cmd.block_height),
                margin_delta_hash: format!("margin-delta-{}", cmd.block_height),
            },
            state_changes: BlockStateChanges::default(),
            block_events: pending_requests
                .iter()
                .map(|req| BlockEvent {
                    event_id: format!("block-event-{}", req.request_id),
                    block_height: cmd.block_height,
                    event_type: "transaction_executed".to_string(),
                    payload_hash: req.payload_hash.clone(),
                })
                .collect(),
            node_state_updates: vec![NodeStateUpdate {
                block_height: cmd.block_height,
                state_root: StateRoot(format!("state-root-{}", cmd.block_height)),
                update_hash: format!("node-update-{}", cmd.block_height),
            }],
        })
    }
}
