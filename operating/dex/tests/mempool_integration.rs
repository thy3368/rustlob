//! 内存池集成测试 - 验证 ReceiveAndAdmit → Mempool → ExecuteAndCommit 完整流程

use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

use cmd_handler::DomainEventSet;
use cmd_handler::use_case_def::{CommandUseCaseExecutor, DomainEventPipeline, LoadState};
use l1_adapter::{InMemoryMempool, MempoolReadingLoadPort, MempoolWritingPipeline};
use l1_core::{BlockStateChanges, ChainState, ExecuteAndCommitBlockError, ExecuteAndCommitBlockEvents, IngressDecision, MempoolPort, PendingRequest, ReceiveAndAdmitTransactionsCmd, ReceiveAndAdmitTransactionsError, ReceiveAndAdmitTransactionsStateSnapshot, StateRoot, VmCapability, VmKind};



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
                vm_kind: VmKind::RustVm,
                capability: VmCapability::new("dex.perp.place_order"),
                action_type: req.action_type.clone(),
                payload_hash: req.payload_hash.clone(),
            })
            .collect();

        let ingress_decisions: Vec<IngressDecision> =
            cmd.requests.iter().map(|_| IngressDecision::Admit).collect();

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
    fn persist(
        &self,
        events: &ExecuteAndCommitBlockEvents,
    ) -> Result<(), ExecuteAndCommitBlockError> {
        self.persist_count.fetch_add(1, Ordering::Relaxed);
        self.domain_event_count.store(events.domain_event_count(), Ordering::Relaxed);
        Ok(())
    }

    fn replay(
        &self,
        _events: &ExecuteAndCommitBlockEvents,
    ) -> Result<(), ExecuteAndCommitBlockError> {
        self.replay_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn publish(
        &self,
        _events: &ExecuteAndCommitBlockEvents,
    ) -> Result<(), ExecuteAndCommitBlockError> {
        self.publish_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use dex::core::use_case::execute_and_commit_block::default_execute_and_commit_block_use_case;
    use l1_core::{ExecuteAndCommitBlockCmd, ReceiveAndAdmitTransactionsEvents, ReceiveAndAdmitTransactionsUseCase, SignedTransactionRequest};
    use super::*;

    #[test]
    fn admitted_transactions_go_to_mempool_then_executed() {
        let mempool = Box::new(InMemoryMempool::new());
        let receive_pipeline =
            MempoolWritingPipeline::<ReceiveAndAdmitTransactionsEvents>::new(mempool.clone());
        let execute_load_port = MempoolReadingLoadPort::new(mempool.clone(), 10);
        let execute_pipeline = SpyPipeline::<ExecuteAndCommitBlockEvents>::default();
        let execute_use_case = default_execute_and_commit_block_use_case();
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

        let execute_cmd = ExecuteAndCommitBlockCmd { block_height: 42, pending_requests: vec![] };
        let execute_events = executor
            .execute(
                &execute_use_case,
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
        let receive_pipeline =
            MempoolWritingPipeline::<ReceiveAndAdmitTransactionsEvents>::new(mempool.clone());
        let execute_load_port = MempoolReadingLoadPort::new(mempool.clone(), 2);
        let execute_pipeline = SpyPipeline::<ExecuteAndCommitBlockEvents>::default();
        let execute_use_case = default_execute_and_commit_block_use_case();
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

        let execute_cmd = ExecuteAndCommitBlockCmd { block_height: 1, pending_requests: vec![] };
        let execute_events = executor
            .execute(
                &execute_use_case,
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
        let execute_use_case = default_execute_and_commit_block_use_case();
        let executor = CommandUseCaseExecutor;

        let execute_cmd = ExecuteAndCommitBlockCmd { block_height: 1, pending_requests: vec![] };
        let result = executor.execute(
            &execute_use_case,
            execute_cmd,
            &execute_load_port,
            &execute_pipeline,
        );

        assert_eq!(result.unwrap_err(), ExecuteAndCommitBlockError::EmptyPendingRequests);
    }

    #[test]
    fn evm_requests_execute_through_revm_runtime() {
        let mempool = Box::new(InMemoryMempool::new());
        let execute_load_port = MempoolReadingLoadPort::new(mempool.clone(), 10);
        let execute_pipeline = SpyPipeline::<ExecuteAndCommitBlockEvents>::default();
        let execute_use_case = default_execute_and_commit_block_use_case();
        let executor = CommandUseCaseExecutor;

        mempool
            .add_requests(vec![PendingRequest {
                request_id: "req-evm-1".to_string(),
                performer: "acct-evm-1".to_string(),
                vm_kind: VmKind::Evm,
                capability: VmCapability::new("evm.settlement.release"),
                action_type: "settlement_release".to_string(),
                payload_hash: "payload-evm-release".to_string(),
            }])
            .unwrap();

        let execute_cmd = ExecuteAndCommitBlockCmd { block_height: 7, pending_requests: vec![] };
        let execute_events = executor
            .execute(
                &execute_use_case,
                execute_cmd,
                &execute_load_port,
                &execute_pipeline,
            )
            .unwrap();

        assert_eq!(execute_events.block_events.len(), 1);
        assert_eq!(execute_events.state_changes, BlockStateChanges::default());
        assert_eq!(execute_events.committed_block.block_height, 7);
        assert!(mempool.is_empty());
    }
}
