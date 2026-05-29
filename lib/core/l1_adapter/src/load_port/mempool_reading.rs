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

    pub fn load_state(
        &self,
        cmd: &ExecuteAndCommitBlockCmd,
    ) -> Result<ExecuteAndCommitBlockStateSnapshot, ExecuteAndCommitBlockError> {
        use minstant::Instant;

        let started_at = Instant::now();
        tracing::trace!(
            adapter = "MempoolReadingLoadPort",
            adapter_kind = "outbound",
            port = "MempoolPort",
            action = "load_state",
            block_height = cmd.block_height,
            batch_size = self.batch_size as u64,
            status = "start",
            "l1 adapter load_state started"
        );

        let pending_requests = self
            .mempool
            .fetch_requests(self.batch_size)
            .map_err(|e| ExecuteAndCommitBlockError::LoadStateFailed(format!("{:?}", e)))?;

        if pending_requests.is_empty() {
            tracing::trace!(
                call_stack = true,
                layer = "outbound",
                component = "MempoolReadingLoadPort",
                operation = "load_state",
                trace_id = cmd.trace_id.as_deref().unwrap_or("-"),
                command_id = cmd.block_command_id.as_deref().unwrap_or("-"),
                request_block_height = cmd.block_height,
                request_batch_size = self.batch_size as u64,
                response_result = "err",
                adapter = "MempoolReadingLoadPort",
                adapter_kind = "outbound",
                port = "MempoolPort",
                action = "load_state",
                block_height = cmd.block_height,
                batch_size = self.batch_size as u64,
                status = "err",
                latency_ns = started_at.elapsed().as_nanos().min(u64::MAX as u128) as u64,
                error_message = "mempool returned no pending requests",
                "l1 adapter load_state failed"
            );
            return Err(ExecuteAndCommitBlockError::EmptyPendingRequests);
        }

        let first_request = pending_requests.first();
        tracing::trace!(
            call_stack = true,
            layer = "outbound",
            component = "MempoolReadingLoadPort",
            operation = "load_state",
            trace_id = cmd.trace_id.as_deref().unwrap_or("-"),
            command_id = cmd.block_command_id.as_deref().unwrap_or("-"),
            request_block_height = cmd.block_height,
            request_batch_size = self.batch_size as u64,
            response_fetched_request_count = pending_requests.len() as u64,
            adapter = "MempoolReadingLoadPort",
            adapter_kind = "outbound",
            port = "MempoolPort",
            action = "load_state",
            block_height = cmd.block_height,
            batch_size = self.batch_size as u64,
            fetched_request_count = pending_requests.len() as u64,
            first_trace_id =
                first_request.and_then(|request| request.trace_id.as_deref()).unwrap_or("-"),
            status = "ok",
            latency_ns = started_at.elapsed().as_nanos().min(u64::MAX as u128) as u64,
            "l1 adapter load_state completed"
        );

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
