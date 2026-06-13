use std::sync::{Arc, Mutex};

use cmd_handler::command_use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCaseExecutionError, CommandUseCaseExecutor2,
};
use dex::adapter::rust_vm_runtime::{RustVmRuntimeAdapter, SpotBookOrderView};
use l1_adapter::{
    ExecuteAndCommitBlockStatePipeline, InMemoryMempool, MdbxStateStore, MempoolReadingLoadPort,
    MempoolWritingPipeline,
};
use l1_core::{
    ExecuteAndCommitBlockCmd, ExecuteAndCommitBlockError, ExecuteAndCommitBlockReply,
    ExecuteAndCommitBlockReplyMapper, ExecuteAndCommitBlockUseCase, PendingRequest,
    ReceiveAndAdmitTransactionsCmd, ReceiveAndAdmitTransactionsError,
    ReceiveAndAdmitTransactionsReply, ReceiveAndAdmitTransactionsReplyMapper,
    ReceiveAndAdmitTransactionsUseCase, SignedTransactionRequest, VmRegistry,
};

use crate::http::dto::{
    ExecuteBlockRequest, SpotBookResponse, SubmitTransactionItem, SubmitTransactionsRequest,
};
use crate::ingress_load_port::IngressLoadPort;
use crate::outbound::{ExecuteAndCommitBlockOutbound, ReceiveAndAdmitTransactionsOutbound};

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<L1E2eService>,
}

pub struct L1E2eService {
    mempool: InMemoryMempool,
    state_store: Arc<Mutex<MdbxStateStore>>,
    rust_vm_runtime: Arc<RustVmRuntimeAdapter>,
    executor: CommandUseCaseExecutor2,
    execute_use_case: ExecuteAndCommitBlockUseCase,
    execute_batch_size: usize,
}

#[derive(Debug, Clone, Default)]
struct ExecuteBlockTraceHints {
    first_trace_id: Option<String>,
}

fn flatten_receive_and_admit_error(
    error: CommandUseCaseExecutionError<
        ReceiveAndAdmitTransactionsError,
        ReceiveAndAdmitTransactionsError,
    >,
) -> ReceiveAndAdmitTransactionsError {
    // TODO: change L1E2eService::submit_transactions to return CommandUseCaseExecutionError
    // so inbound can preserve business vs outbound phase information end-to-end.
    match error {
        CommandUseCaseExecutionError::Business(error) => error,
        CommandUseCaseExecutionError::Outbound { source, .. } => source,
    }
}

fn flatten_execute_and_commit_error(
    error: CommandUseCaseExecutionError<ExecuteAndCommitBlockError, ExecuteAndCommitBlockError>,
) -> ExecuteAndCommitBlockError {
    // TODO: change L1E2eService::execute_block to return CommandUseCaseExecutionError
    // so inbound can preserve business vs outbound phase information end-to-end.
    match error {
        CommandUseCaseExecutionError::Business(error) => error,
        CommandUseCaseExecutionError::Outbound { source, .. } => source,
    }
}

impl L1E2eService {
    pub fn new(
        mempool: InMemoryMempool,
        state_store: Arc<Mutex<MdbxStateStore>>,
        vm_registry: VmRegistry<PendingRequest>,
        rust_vm_runtime: Arc<RustVmRuntimeAdapter>,
        execute_batch_size: usize,
    ) -> Self {
        Self {
            mempool,
            state_store,
            rust_vm_runtime,
            executor: CommandUseCaseExecutor2,
            execute_use_case: ExecuteAndCommitBlockUseCase::with_vm_registry(vm_registry),
            execute_batch_size,
        }
    }

    pub fn health(&self) -> HealthResponse {
        HealthResponse { ok: true, mempool_len: l1_core::MempoolPort::len(&self.mempool) }
    }

    pub fn spot_book(&self, market: &str) -> Result<SpotBookResponse, ExecuteAndCommitBlockError> {
        let orders = self
            .rust_vm_runtime
            .spot_book_orders(market)
            .map_err(|error| ExecuteAndCommitBlockError::VmExecutionFailed(format!("{error:?}")))?;

        Ok(SpotBookResponse {
            market: market.to_string(),
            orders: orders.into_iter().map(Into::into).collect(),
        })
    }

    pub fn submit_transactions(
        &self,
        request: SubmitTransactionsRequest,
    ) -> Result<ReceiveAndAdmitTransactionsReply, ReceiveAndAdmitTransactionsError> {
        use minstant::Instant;

        let dispatch_start = Instant::now();
        let request_count = request.requests.len();
        let first_trace_id = request.requests.first().map(submit_trace_id);
        let first_trace_id_for_log = first_trace_id.clone();
        let _dispatch_guard = tracing::info_span!(
            "l1_submit_transactions",
            inbound_adapter = "http",
            endpoint = "POST /api/l1/transactions/submit",
            request_count = request_count as u64,
            trace_id = first_trace_id.as_deref().unwrap_or("-")
        )
        .entered();
        let signed_requests = request
            .requests
            .iter()
            .map(to_signed_request)
            .collect::<Vec<SignedTransactionRequest>>();
        let pending_requests =
            request.requests.iter().map(to_pending_request).collect::<Vec<PendingRequest>>();

        let cmd = ReceiveAndAdmitTransactionsCmd { requests: signed_requests };
        let load_port = IngressLoadPort::new(pending_requests);
        let pipeline = MempoolWritingPipeline::<()>::new(Box::new(self.mempool.clone()));
        let outbound = ReceiveAndAdmitTransactionsOutbound { load_state: load_port, pipeline };

        tracing::trace!("l1 submit_transactions dispatch started");
        let reply = self.executor.execute_and_map_reply(
            &ReceiveAndAdmitTransactionsUseCase,
            CommandEnvelope {
                meta: CommandMeta { trace_id: first_trace_id, command_id: None },
                command: cmd,
            },
            &outbound,
            &(),
            &ReceiveAndAdmitTransactionsReplyMapper,
        );

        match reply {
            Ok(reply) => {
                tracing::trace!(
                    call_stack = true,
                    layer = "inbound",
                    component = "l1_submit_transactions",
                    operation = "handle",
                    trace_id = first_trace_id_for_log.as_deref().unwrap_or("-"),
                    request_method = "POST",
                    request_endpoint = "POST /api/l1/transactions/submit",
                    request_request_count = request_count as u64,
                    response_http_status = 200u64,
                    response_admitted_count = reply.admitted_count as u64,
                    response_rejected_count = reply.rejected_count as u64,
                    status = "ok",
                    latency_ns = dispatch_start.elapsed().as_nanos().min(u64::MAX as u128) as u64,
                    "l1 submit_transactions dispatch completed"
                );
                Ok(reply)
            }
            Err(error) => {
                tracing::trace!(
                    call_stack = true,
                    layer = "inbound",
                    component = "l1_submit_transactions",
                    operation = "handle",
                    trace_id = first_trace_id_for_log.as_deref().unwrap_or("-"),
                    request_method = "POST",
                    request_endpoint = "POST /api/l1/transactions/submit",
                    request_request_count = request_count as u64,
                    response_result = "err",
                    status = "err",
                    latency_ns = dispatch_start.elapsed().as_nanos().min(u64::MAX as u128) as u64,
                    error_message = format!("{error:?}"),
                    "l1 submit_transactions dispatch failed"
                );
                Err(flatten_receive_and_admit_error(error))
            }
        }
    }

    pub fn execute_block(
        &self,
        request: ExecuteBlockRequest,
    ) -> Result<ExecuteAndCommitBlockReply, ExecuteAndCommitBlockError> {
        use minstant::Instant;

        let dispatch_start = Instant::now();
        let execute_batch_size = request.batch_size.unwrap_or(self.execute_batch_size);
        let trace_hints = self.execute_block_trace_hints()?;
        let trace_id = request
            .trace_id
            .clone()
            .or_else(|| trace_hints.first_trace_id.clone())
            .unwrap_or_else(|| format!("l1-block-{}", request.block_height));
        let block_command_id = request
            .block_command_id
            .clone()
            .unwrap_or_else(|| format!("execute-block-{}", request.block_height));
        let _dispatch_guard = tracing::info_span!(
            "l1_execute_block",
            inbound_adapter = "http",
            endpoint = "POST /api/l1/blocks/execute",
            block_height = request.block_height,
            batch_size = execute_batch_size as u64,
            trace_id = trace_id.as_str(),
            block_command_id = block_command_id.as_str()
        )
        .entered();
        let load_port =
            MempoolReadingLoadPort::new(Box::new(self.mempool.clone()), execute_batch_size);
        let pipeline = ExecuteAndCommitBlockStatePipeline { state_store: self.state_store.clone() };
        let outbound = ExecuteAndCommitBlockOutbound { load_state: load_port, pipeline };
        let cmd = ExecuteAndCommitBlockCmd {
            block_height: request.block_height,
            trace_id: Some(trace_id),
            block_command_id: Some(block_command_id),
            pending_requests: vec![],
        };
        let command_trace_id_for_log = cmd.trace_id.clone();
        let command_id_for_log = cmd.block_command_id.clone();

        tracing::trace!("l1 execute_block dispatch started");
        let reply = self.executor.execute_and_map_reply(
            &self.execute_use_case,
            CommandEnvelope {
                meta: CommandMeta {
                    trace_id: cmd.trace_id.clone(),
                    command_id: cmd.block_command_id.clone(),
                },
                command: cmd,
            },
            &outbound,
            &(),
            &ExecuteAndCommitBlockReplyMapper,
        );

        match reply {
            Ok(reply) => {
                tracing::trace!(
                    call_stack = true,
                    layer = "inbound",
                    component = "l1_execute_block",
                    operation = "handle",
                    trace_id = command_trace_id_for_log.as_deref().unwrap_or("-"),
                    command_id = command_id_for_log.as_deref().unwrap_or("-"),
                    request_method = "POST",
                    request_endpoint = "POST /api/l1/blocks/execute",
                    request_block_height = request.block_height,
                    request_block_command_id = command_id_for_log.as_deref().unwrap_or("-"),
                    request_batch_size = execute_batch_size as u64,
                    response_http_status = 200u64,
                    response_block_height = reply.block_height,
                    response_block_event_count = reply.block_event_count as u64,
                    response_node_state_update_count = reply.node_state_update_count as u64,
                    response_matched_trade_count = reply.matched_trade_count as u64,
                    status = "ok",
                    latency_ns = dispatch_start.elapsed().as_nanos().min(u64::MAX as u128) as u64,
                    "l1 execute_block dispatch completed"
                );
                Ok(reply)
            }
            Err(error) => {
                tracing::trace!(
                    call_stack = true,
                    layer = "inbound",
                    component = "l1_execute_block",
                    operation = "handle",
                    trace_id = command_trace_id_for_log.as_deref().unwrap_or("-"),
                    command_id = command_id_for_log.as_deref().unwrap_or("-"),
                    request_method = "POST",
                    request_endpoint = "POST /api/l1/blocks/execute",
                    request_block_height = request.block_height,
                    request_block_command_id = command_id_for_log.as_deref().unwrap_or("-"),
                    request_batch_size = execute_batch_size as u64,
                    response_result = "err",
                    status = "err",
                    latency_ns = dispatch_start.elapsed().as_nanos().min(u64::MAX as u128) as u64,
                    error_message = format!("{error:?}"),
                    "l1 execute_block dispatch failed"
                );
                Err(flatten_execute_and_commit_error(error))
            }
        }
    }

    fn execute_block_trace_hints(
        &self,
    ) -> Result<ExecuteBlockTraceHints, ExecuteAndCommitBlockError> {
        let first_request = self
            .mempool
            .peek_first_request()
            .map_err(|error| ExecuteAndCommitBlockError::LoadStateFailed(format!("{error:?}")))?;

        Ok(ExecuteBlockTraceHints {
            first_trace_id: first_request.as_ref().and_then(|request| request.trace_id.clone()),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HealthResponse {
    pub ok: bool,
    pub mempool_len: usize,
}

impl From<SpotBookOrderView> for crate::http::dto::SpotBookOrderResponse {
    fn from(value: SpotBookOrderView) -> Self {
        Self {
            order_id: value.order_id,
            trader_id: value.trader_id,
            side: value.side,
            price: value.price,
            original_quantity: value.original_quantity,
            remaining_quantity: value.remaining_quantity,
        }
    }
}

fn to_signed_request(item: &SubmitTransactionItem) -> SignedTransactionRequest {
    SignedTransactionRequest {
        trace_id: Some(submit_trace_id(item)),
        request_id: item.request_id.clone(),
        account: item.account.clone(),
        nonce: item.nonce.clone(),
        expires_at: item.expires_at.clone(),
        action_type: item.action_type.clone(),
        payload_hash: item.payload_hash.clone(),
        signature_hash: item.signature_hash.clone(),
    }
}

fn to_pending_request(item: &SubmitTransactionItem) -> PendingRequest {
    PendingRequest {
        trace_id: Some(submit_trace_id(item)),
        request_id: item.request_id.clone(),
        performer: item.account.clone(),
        vm_kind: item.vm_kind.into(),
        capability: l1_core::VmCapability::new(&item.capability),
        action_type: item.action_type.clone(),
        payload_hash: item.payload_hash.clone(),
        payload: item.payload.clone().map(|value| value.to_string()),
    }
}

fn submit_trace_id(item: &SubmitTransactionItem) -> String {
    item.trace_id.clone().unwrap_or_else(|| item.request_id.clone())
}
