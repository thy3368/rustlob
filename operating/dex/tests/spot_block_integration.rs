//! 现货下单与出块集成测试 - 验证 spot 请求经 mempool 进入 Rust VM 并在区块中执行/撮合

use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

use cmd_handler::DomainEventSet;
use cmd_handler::use_case_def::{CommandUseCaseExecutor, DomainEventPipeline, LoadState};
use base_types::handler::handler_update::CmdHandlerForUpdate;
use l1_adapter::{InMemoryMempool, MempoolReadingLoadPort, MempoolWritingPipeline};
use l1_core::{
    ChainState, ExecuteAndCommitBlockError, ExecuteAndCommitBlockEvents, IngressDecision,
    MempoolPort, PendingRequest, ReceiveAndAdmitTransactionsCmd, ReceiveAndAdmitTransactionsError,
    ReceiveAndAdmitTransactionsStateSnapshot, StateRoot, VmCapability, VmKind,
};

struct SpotAdmissionLoadPort {
    capability: &'static str,
}

impl LoadState<
    ReceiveAndAdmitTransactionsCmd,
    ReceiveAndAdmitTransactionsStateSnapshot,
    ReceiveAndAdmitTransactionsError,
> for SpotAdmissionLoadPort
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
                capability: VmCapability::new(self.capability),
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
    use dex::adapter::rust_vm_runtime::RustVmRuntimeAdapter;
    use dex::core::use_case::execute_and_commit_block::default_execute_and_commit_block_use_case;
    use dex::core::{
        ExchangeCommand, ExchangeCommandEnvelope, ExecuteTradingBatchHandler, ProductType,
        SpotCommand, SpotPlaceOrderCmd, SpotSide, TradingCommand,
    };
    use l1_core::{
        ExecuteAndCommitBlockCmd, ReceiveAndAdmitTransactionsEvents,
        ReceiveAndAdmitTransactionsUseCase, SignedTransactionRequest, VmExecutionInput, VmRuntime,
    };

    use super::*;

    fn signed_request(request_id: &str, account: &str, payload_hash: &str) -> SignedTransactionRequest {
        SignedTransactionRequest {
            request_id: request_id.to_string(),
            account: account.to_string(),
            nonce: "1".to_string(),
            expires_at: "2026-04-25T00:00:00Z".to_string(),
            action_type: "order".to_string(),
            payload_hash: payload_hash.to_string(),
            signature_hash: format!("sig-{request_id}"),
        }
    }

    fn spot_place_order(
        command_id: u64,
        trader_id: u64,
        side: SpotSide,
        price: u64,
        quantity: u64,
    ) -> ExchangeCommandEnvelope {
        ExchangeCommandEnvelope {
            command_id,
            trader_id,
            nonce: command_id,
            timestamp_ns: 1_000 + command_id,
            product_type: ProductType::Spot,
            command: ExchangeCommand::TradingCommand(TradingCommand::Spot(SpotCommand::PlaceOrder(
                SpotPlaceOrderCmd {
                    trader_id,
                    market: "BTC-USDT".into(),
                    side,
                    price,
                    quantity,
                },
            ))),
        }
    }

    #[test]
    fn spot_requests_go_to_mempool_then_execute_in_block() {
        let mempool = Box::new(InMemoryMempool::new());
        let receive_pipeline =
            MempoolWritingPipeline::<ReceiveAndAdmitTransactionsEvents>::new(mempool.clone());
        let execute_load_port = MempoolReadingLoadPort::new(mempool.clone(), 10);
        let execute_pipeline = SpyPipeline::<ExecuteAndCommitBlockEvents>::default();
        let execute_use_case = default_execute_and_commit_block_use_case();
        let executor = CommandUseCaseExecutor;

        let receive_events = executor
            .execute(
                &ReceiveAndAdmitTransactionsUseCase,
                ReceiveAndAdmitTransactionsCmd {
                    requests: vec![signed_request("req-1", "acct-1", "payload-spot-1")],
                },
                &SpotAdmissionLoadPort {
                    capability: "dex.spot.place_order",
                },
                &receive_pipeline,
            )
            .unwrap();

        assert_eq!(receive_events.admitted_requests.len(), 1);
        assert_eq!(mempool.len(), 1);

        let execute_events = executor
            .execute(
                &execute_use_case,
                ExecuteAndCommitBlockCmd {
                    block_height: 42,
                    pending_requests: vec![],
                },
                &execute_load_port,
                &execute_pipeline,
            )
            .unwrap();

        assert_eq!(execute_events.committed_block.block_height, 42);
        assert_eq!(execute_events.block_events.len(), 1);
        assert!(mempool.is_empty());
        assert_eq!(execute_pipeline.persist_count.load(Ordering::Relaxed), 1);
        assert_eq!(execute_pipeline.replay_count.load(Ordering::Relaxed), 1);
        assert_eq!(execute_pipeline.publish_count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn spot_orders_match_across_two_blocks() {
        let adapter = RustVmRuntimeAdapter::new();

        let sell_output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.spot.place_order",
                PendingRequest {
                    request_id: "req-1".to_string(),
                    performer: "acct-11".to_string(),
                    vm_kind: VmKind::RustVm,
                    capability: VmCapability::new("dex.spot.place_order"),
                    action_type: "order".to_string(),
                    payload_hash: "payload-sell".to_string(),
                },
            ))
            .unwrap();

        assert_eq!(sell_output.product_events.len(), 1);
        assert_eq!(sell_output.product_events[0].product_type, "Spot");
        assert_eq!(sell_output.product_events[0].event_type, "accepted:1:1:0");

        let handler = ExecuteTradingBatchHandler::new();
        let first_batch = handler
            .cmd_handle(
                vec![spot_place_order(1, 11, SpotSide::Sell, 100_000, 1)],
                |writes: &_, _| writes.clone(),
            )
            .unwrap();
        assert_eq!(first_batch.summary.trades_executed, 0);

        let second_batch = handler
            .cmd_handle(
                vec![spot_place_order(2, 22, SpotSide::Buy, 100_000, 1)],
                |writes: &_, _| writes.clone(),
            )
            .unwrap();

        assert_eq!(second_batch.summary.accepted_commands, 1);
        assert_eq!(second_batch.summary.orders_created, 1);
        assert_eq!(second_batch.summary.trades_executed, 1);
        assert_eq!(second_batch.summary.balance_updates, 4);
    }
}
