//! 内存池集成测试 - 验证 ReceiveAndAdmit → Mempool → ExecuteAndCommit 完整流程

use cmd_handler::{
    use_case_def::{CommandUseCaseExecutor, DomainEventPipeline, LoadState},
    DomainEventSet,
};
use dex::{
    entity::{
        ChainState, CommittedBlock, ExecutionResult, ExecutionRuleSet, ExecutionTrace,
        IngressDecision, NodeStateUpdate, PendingRequest, SignedTransactionRequest, StateDiff,
        StateRoot,
    },
    mempool::{InMemoryMempool, MempoolPort},
    use_case::command_handler::{
        ExecuteAndCommitBlockCmd, ExecuteAndCommitBlockError, ExecuteAndCommitBlockEvents,
        ExecuteAndCommitBlockStateSnapshot,
        ExecuteAndCommitBlockUseCase, ReceiveAndAdmitTransactionsCmd,
        ReceiveAndAdmitTransactionsError, ReceiveAndAdmitTransactionsEvents,
        ReceiveAndAdmitTransactionsStateSnapshot,
        ReceiveAndAdmitTransactionsUseCase,
    },
};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

/// 将准入的请求写入内存池的 Pipeline
pub struct MempoolWritingPipeline<E> {
    mempool: Box<dyn MempoolPort>,
    _events: PhantomData<E>,
}

impl<E> MempoolWritingPipeline<E> {
    pub fn new(mempool: Box<dyn MempoolPort>) -> Self {
        Self {
            mempool,
            _events: PhantomData,
        }
    }
}

impl DomainEventPipeline<ReceiveAndAdmitTransactionsEvents, ReceiveAndAdmitTransactionsError>
    for MempoolWritingPipeline<ReceiveAndAdmitTransactionsEvents>
{
    fn persist(&self, _events: &ReceiveAndAdmitTransactionsEvents) -> Result<(), ReceiveAndAdmitTransactionsError> {
        Ok(())
    }

    fn replay(&self, events: &ReceiveAndAdmitTransactionsEvents) -> Result<(), ReceiveAndAdmitTransactionsError> {
        self.mempool
            .add_requests(events.admitted_requests.clone())
            .map_err(|e| ReceiveAndAdmitTransactionsError::LoadStateFailed(format!("{:?}", e)))
    }

    fn publish(&self, _events: &ReceiveAndAdmitTransactionsEvents) -> Result<(), ReceiveAndAdmitTransactionsError> {
        Ok(())
    }
}

/// 从内存池读取请求用于区块执行的 LoadPort
pub struct MempoolReadingLoadPort {
    mempool: Box<dyn MempoolPort>,
    batch_size: usize,
}

impl MempoolReadingLoadPort {
    pub fn new(mempool: Box<dyn MempoolPort>, batch_size: usize) -> Self {
        Self {
            mempool,
            batch_size,
        }
    }
}

impl
    LoadState<
        ExecuteAndCommitBlockCmd,
        ExecuteAndCommitBlockStateSnapshot,
        ExecuteAndCommitBlockError,
    > for MempoolReadingLoadPort
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
                order_book_delta_hash: format!("order-book-delta-{}", cmd.block_height),
                position_delta_hash: format!("position-delta-{}", cmd.block_height),
                balance_delta_hash: format!("balance-delta-{}", cmd.block_height),
                margin_delta_hash: format!("margin-delta-{}", cmd.block_height),
            },
            block_events: pending_requests
                .iter()
                .map(|req| dex::entity::BlockEvent {
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

struct StubReceiveAdmissionLoadPort;

impl
    LoadState<
        ReceiveAndAdmitTransactionsCmd,
        ReceiveAndAdmitTransactionsStateSnapshot,
        ReceiveAndAdmitTransactionsError,
    > for StubReceiveAdmissionLoadPort
{
    fn load_state(
        &self,
        cmd: &ReceiveAndAdmitTransactionsCmd,
    ) -> Result<ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError> {
        let admitted_requests: Vec<PendingRequest> = cmd
            .requests
            .iter()
            .map(|req| PendingRequest {
                request_id: req.request_id.clone(),
                performer: req.account.clone(),
                action_type: req.action_type.clone(),
                payload_hash: req.payload_hash.clone(),
            })
            .collect();

        let ingress_decisions: Vec<IngressDecision> = cmd
            .requests
            .iter()
            .map(|_| IngressDecision::Admit)
            .collect();

        Ok(ReceiveAndAdmitTransactionsStateSnapshot {
            chain_state: ChainState {
                height: 1,
                state_root: StateRoot("state-root-1".to_string()),
            },
            admitted_requests,
            ingress_decisions,
        })
    }
}

struct SpyPipeline<E> {
    persist_count: AtomicUsize,
    replay_count: AtomicUsize,
    publish_count: AtomicUsize,
    domain_event_count: AtomicUsize,
    _events: PhantomData<E>,
}

impl<E> Default for SpyPipeline<E> {
    fn default() -> Self {
        Self {
            persist_count: AtomicUsize::new(0),
            replay_count: AtomicUsize::new(0),
            publish_count: AtomicUsize::new(0),
            domain_event_count: AtomicUsize::new(0),
            _events: PhantomData,
        }
    }
}

impl DomainEventPipeline<ExecuteAndCommitBlockEvents, ExecuteAndCommitBlockError>
    for SpyPipeline<ExecuteAndCommitBlockEvents>
{
    fn persist(&self, events: &ExecuteAndCommitBlockEvents) -> Result<(), ExecuteAndCommitBlockError> {
        self.persist_count.fetch_add(1, Ordering::Relaxed);
        self.domain_event_count
            .store(events.domain_event_count(), Ordering::Relaxed);
        Ok(())
    }

    fn replay(&self, _events: &ExecuteAndCommitBlockEvents) -> Result<(), ExecuteAndCommitBlockError> {
        self.replay_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn publish(&self, _events: &ExecuteAndCommitBlockEvents) -> Result<(), ExecuteAndCommitBlockError> {
        self.publish_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admitted_transactions_go_to_mempool_then_executed() {
        let mempool = Box::new(InMemoryMempool::new());
        let receive_pipeline = MempoolWritingPipeline::<ReceiveAndAdmitTransactionsEvents>::new(mempool.clone());
        let execute_load_port = MempoolReadingLoadPort::new(mempool.clone(), 10);
        let execute_pipeline = SpyPipeline::<ExecuteAndCommitBlockEvents>::default();
        let executor = CommandUseCaseExecutor;

        let receive_cmd = ReceiveAndAdmitTransactionsCmd {
            requests: vec![
                SignedTransactionRequest {
                    request_id: "req-1".to_string(),
                    account: "acct-1".to_string(),
                    nonce: "1".to_string(),
                    expires_at: "2026-04-25T00:00:00Z".to_string(),
                    action_type: "order".to_string(),
                    payload_hash: "payload-1".to_string(),
                    signature_hash: "sig-1".to_string(),
                },
                SignedTransactionRequest {
                    request_id: "req-2".to_string(),
                    account: "acct-2".to_string(),
                    nonce: "2".to_string(),
                    expires_at: "2026-04-25T00:00:00Z".to_string(),
                    action_type: "cancel".to_string(),
                    payload_hash: "payload-2".to_string(),
                    signature_hash: "sig-2".to_string(),
                },
            ],
        };

        let receive_events = executor
            .execute(
                &ReceiveAndAdmitTransactionsUseCase,
                receive_cmd,
                &StubReceiveAdmissionLoadPort,
                &receive_pipeline,
            )
            .unwrap();

        assert_eq!(receive_events.admitted_requests.len(), 2);
        assert_eq!(mempool.len(), 2);

        let execute_cmd = ExecuteAndCommitBlockCmd {
            block_height: 42,
            pending_requests: vec![],
        };
        let execute_events = executor
            .execute(
                &ExecuteAndCommitBlockUseCase,
                execute_cmd,
                &execute_load_port,
                &execute_pipeline,
            )
            .unwrap();

        assert_eq!(execute_events.committed_block.block_height, 42);
        assert_eq!(execute_events.block_events.len(), 2);
        assert!(mempool.is_empty());

        assert_eq!(execute_pipeline.persist_count.load(Ordering::Relaxed), 1);
        assert_eq!(execute_pipeline.replay_count.load(Ordering::Relaxed), 1);
        assert_eq!(execute_pipeline.publish_count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn batch_size_limits_requests_fetched_from_mempool() {
        let mempool = Box::new(InMemoryMempool::new());
        let receive_pipeline = MempoolWritingPipeline::<ReceiveAndAdmitTransactionsEvents>::new(mempool.clone());
        let execute_load_port = MempoolReadingLoadPort::new(mempool.clone(), 2);
        let execute_pipeline = SpyPipeline::<ExecuteAndCommitBlockEvents>::default();
        let executor = CommandUseCaseExecutor;

        let receive_cmd = ReceiveAndAdmitTransactionsCmd {
            requests: vec![
                SignedTransactionRequest {
                    request_id: "req-1".to_string(),
                    account: "acct-1".to_string(),
                    nonce: "1".to_string(),
                    expires_at: "2026-04-25T00:00:00Z".to_string(),
                    action_type: "order".to_string(),
                    payload_hash: "payload-1".to_string(),
                    signature_hash: "sig-1".to_string(),
                },
                SignedTransactionRequest {
                    request_id: "req-2".to_string(),
                    account: "acct-2".to_string(),
                    nonce: "2".to_string(),
                    expires_at: "2026-04-25T00:00:00Z".to_string(),
                    action_type: "order".to_string(),
                    payload_hash: "payload-2".to_string(),
                    signature_hash: "sig-2".to_string(),
                },
                SignedTransactionRequest {
                    request_id: "req-3".to_string(),
                    account: "acct-3".to_string(),
                    nonce: "3".to_string(),
                    expires_at: "2026-04-25T00:00:00Z".to_string(),
                    action_type: "order".to_string(),
                    payload_hash: "payload-3".to_string(),
                    signature_hash: "sig-3".to_string(),
                },
            ],
        };

        executor
            .execute(
                &ReceiveAndAdmitTransactionsUseCase,
                receive_cmd,
                &StubReceiveAdmissionLoadPort,
                &receive_pipeline,
            )
            .unwrap();

        assert_eq!(mempool.len(), 3);

        let execute_cmd = ExecuteAndCommitBlockCmd {
            block_height: 1,
            pending_requests: vec![],
        };
        let execute_events = executor
            .execute(
                &ExecuteAndCommitBlockUseCase,
                execute_cmd,
                &execute_load_port,
                &execute_pipeline,
            )
            .unwrap();

        assert_eq!(execute_events.block_events.len(), 2);
        assert_eq!(mempool.len(), 1);
    }

    #[test]
    fn empty_mempool_causes_execution_error() {
        let mempool = Box::new(InMemoryMempool::new());
        let execute_load_port = MempoolReadingLoadPort::new(mempool.clone(), 10);
        let execute_pipeline = SpyPipeline::<ExecuteAndCommitBlockEvents>::default();
        let executor = CommandUseCaseExecutor;

        let execute_cmd = ExecuteAndCommitBlockCmd {
            block_height: 1,
            pending_requests: vec![],
        };
        let result = executor.execute(
            &ExecuteAndCommitBlockUseCase,
            execute_cmd,
            &execute_load_port,
            &execute_pipeline,
        );

        assert_eq!(
            result.unwrap_err(),
            ExecuteAndCommitBlockError::EmptyPendingRequests
        );
    }
}
