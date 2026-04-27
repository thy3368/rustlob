use std::{
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering},
};

use cmd_handler::{
    use_case_def::{CommandUseCaseExecutor, DomainEventPipeline, LoadState},
    DomainEventSet,
};
use dex::{
    entity::{
        BlockEvent, ChainState, CommittedBlock, ExecutionResult, ExecutionRuleSet, ExecutionTrace,
        IngressDecision, NodeStateUpdate, PendingRequest, SignedTransactionRequest, StateDiff,
        StateRoot,
    },
    use_case::command_handler::{
        ExecuteAndCommitBlockCmd, ExecuteAndCommitBlockError, ExecuteAndCommitBlockEvents,
        ExecuteAndCommitBlockStateSnapshot,
        ExecuteAndCommitBlockUseCase, ReceiveAndAdmitTransactionsCmd,
        ReceiveAndAdmitTransactionsError, ReceiveAndAdmitTransactionsEvents,
        ReceiveAndAdmitTransactionsStateSnapshot,
        ReceiveAndAdmitTransactionsUseCase,
    },
};

struct StubReceiveAdmissionLoadPort;

impl LoadState<ReceiveAndAdmitTransactionsCmd, ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError>
    for StubReceiveAdmissionLoadPort
{
    fn load_state(
        &self,
        cmd: &ReceiveAndAdmitTransactionsCmd,
    ) -> Result<ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError> {
        let request = cmd.requests.first().expect("test command has one request");

        Ok(ReceiveAndAdmitTransactionsStateSnapshot {
            chain_state: ChainState {
                height: 41,
                state_root: StateRoot("state-root-41".to_string()),
            },
            admitted_requests: vec![PendingRequest {
                request_id: request.request_id.clone(),
                performer: request.account.clone(),
                action_type: request.action_type.clone(),
                payload_hash: request.payload_hash.clone(),
            }],
            ingress_decisions: vec![IngressDecision::Admit],
        })
    }
}

struct StubExecuteCommitLoadPort;

impl
    LoadState<
        ExecuteAndCommitBlockCmd,
        ExecuteAndCommitBlockStateSnapshot,
        ExecuteAndCommitBlockError,
    > for StubExecuteCommitLoadPort
{
    fn load_state(
        &self,
        cmd: &ExecuteAndCommitBlockCmd,
    ) -> Result<ExecuteAndCommitBlockStateSnapshot, ExecuteAndCommitBlockError> {
        assert_eq!(cmd.pending_requests.len(), 1);
        let request = cmd.pending_requests[0].clone();

        Ok(ExecuteAndCommitBlockStateSnapshot {
            pending_requests: vec![request.clone()],
            execution_results: vec![ExecutionResult {
                result_id: "result-42".to_string(),
                source_process: "ReceiveAndAdmitTransactionsUseCase".to_string(),
                events_hash: "events-42".to_string(),
            }],
            chain_state: ChainState {
                height: 41,
                state_root: StateRoot("state-root-41".to_string()),
            },
            execution_rules: ExecutionRuleSet {
                version: "v1".to_string(),
                rule_hash: "rules-1".to_string(),
            },
            committed_block: CommittedBlock {
                block_height: cmd.block_height,
                input_root: "input-root-42".to_string(),
                state_root: StateRoot("state-root-42".to_string()),
                execution_trace_hash: "trace-42".to_string(),
                state_diff_hash: "diff-42".to_string(),
            },
            execution_trace: ExecutionTrace {
                block_height: cmd.block_height,
                input_root: "input-root-42".to_string(),
                events_hash: "events-42".to_string(),
                state_diff_hash: "diff-42".to_string(),
            },
            state_diff: StateDiff {
                block_height: cmd.block_height,
                order_book_delta_hash: "order-book-delta-42".to_string(),
                position_delta_hash: "position-delta-42".to_string(),
                balance_delta_hash: "balance-delta-42".to_string(),
                margin_delta_hash: "margin-delta-42".to_string(),
            },
            block_events: vec![BlockEvent {
                event_id: "block-event-42".to_string(),
                block_height: cmd.block_height,
                event_type: "block_committed".to_string(),
                payload_hash: request.payload_hash,
            }],
            node_state_updates: vec![NodeStateUpdate {
                block_height: cmd.block_height,
                state_root: StateRoot("state-root-42".to_string()),
                update_hash: "node-update-42".to_string(),
            }],
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

impl DomainEventPipeline<ReceiveAndAdmitTransactionsEvents, ReceiveAndAdmitTransactionsError>
    for SpyPipeline<ReceiveAndAdmitTransactionsEvents>
{
    fn persist(&self, events: &ReceiveAndAdmitTransactionsEvents) -> Result<(), ReceiveAndAdmitTransactionsError> {
        self.persist_count.fetch_add(1, Ordering::Relaxed);
        self.domain_event_count
            .store(events.domain_event_count(), Ordering::Relaxed);
        Ok(())
    }

    fn replay(&self, _events: &ReceiveAndAdmitTransactionsEvents) -> Result<(), ReceiveAndAdmitTransactionsError> {
        self.replay_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn publish(&self, _events: &ReceiveAndAdmitTransactionsEvents) -> Result<(), ReceiveAndAdmitTransactionsError> {
        self.publish_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
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

#[test]
fn admitted_transactions_are_executed_and_committed_as_a_block() {
    // Given
    let receive_cmd = ReceiveAndAdmitTransactionsCmd {
        requests: vec![SignedTransactionRequest {
            request_id: "req-42".to_string(),
            account: "acct-1".to_string(),
            nonce: "42".to_string(),
            expires_at: "2026-04-25T00:00:00Z".to_string(),
            action_type: "order".to_string(),
            payload_hash: "payload-42".to_string(),
            signature_hash: "sig-42".to_string(),
        }],
    };
    let receive_pipeline = SpyPipeline::<ReceiveAndAdmitTransactionsEvents>::default();
    let execute_pipeline = SpyPipeline::<ExecuteAndCommitBlockEvents>::default();
    let executor = CommandUseCaseExecutor;

    // When
    let receive_events = executor
        .execute(
            &ReceiveAndAdmitTransactionsUseCase,
            receive_cmd,
            &StubReceiveAdmissionLoadPort,
            &receive_pipeline,
        )
        .unwrap();
    let execute_cmd = ExecuteAndCommitBlockCmd {
        block_height: 42,
        pending_requests: receive_events.admitted_requests.clone(),
    };
    let execute_events = executor
        .execute(
            &ExecuteAndCommitBlockUseCase,
            execute_cmd,
            &StubExecuteCommitLoadPort,
            &execute_pipeline,
        )
        .unwrap();

    // Then
    assert_eq!(receive_events.admitted_requests.len(), 1);
    assert_eq!(receive_events.ingress_decisions, vec![IngressDecision::Admit]);
    assert_eq!(execute_events.committed_block.block_height, 42);
    assert_eq!(execute_events.block_events.len(), 1);
    assert_eq!(execute_events.node_state_updates.len(), 1);

    assert_eq!(receive_pipeline.persist_count.load(Ordering::Relaxed), 1);
    assert_eq!(receive_pipeline.replay_count.load(Ordering::Relaxed), 1);
    assert_eq!(receive_pipeline.publish_count.load(Ordering::Relaxed), 1);
    assert_eq!(receive_pipeline.domain_event_count.load(Ordering::Relaxed), 1);

    assert_eq!(execute_pipeline.persist_count.load(Ordering::Relaxed), 1);
    assert_eq!(execute_pipeline.replay_count.load(Ordering::Relaxed), 1);
    assert_eq!(execute_pipeline.publish_count.load(Ordering::Relaxed), 1);
    assert_eq!(execute_pipeline.domain_event_count.load(Ordering::Relaxed), 3);
}
