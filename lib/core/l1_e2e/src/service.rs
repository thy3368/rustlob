use std::sync::{Arc, Mutex};

use cmd_handler::use_case_def::CommandUseCaseExecutor;
use l1_adapter::{ExecuteAndCommitBlockStatePipeline, InMemoryMempool, MdbxStateStore, MempoolReadingLoadPort, MempoolWritingPipeline};
use l1_core::{ExecuteAndCommitBlockCmd, ExecuteAndCommitBlockError, ExecuteAndCommitBlockReply, ExecuteAndCommitBlockReplyMapper, ExecuteAndCommitBlockUseCase, PendingRequest, ReceiveAndAdmitTransactionsCmd, ReceiveAndAdmitTransactionsError, ReceiveAndAdmitTransactionsEvents, ReceiveAndAdmitTransactionsReply, ReceiveAndAdmitTransactionsReplyMapper, ReceiveAndAdmitTransactionsUseCase, SignedTransactionRequest, VmRegistry};

use crate::http::dto::{ExecuteBlockRequest, SubmitTransactionItem, SubmitTransactionsRequest};
use crate::ingress_load_port::IngressLoadPort;

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<L1E2eService>,
}

pub struct L1E2eService {
    mempool: InMemoryMempool,
    state_store: Arc<Mutex<MdbxStateStore>>,
    executor: CommandUseCaseExecutor,
    execute_use_case: ExecuteAndCommitBlockUseCase,
    execute_batch_size: usize,
}

impl L1E2eService {
    pub fn new(
        mempool: InMemoryMempool,
        state_store: Arc<Mutex<MdbxStateStore>>,
        vm_registry: VmRegistry<PendingRequest>,
        execute_batch_size: usize,
    ) -> Self {
        Self {
            mempool,
            state_store,
            executor: CommandUseCaseExecutor,
            execute_use_case: ExecuteAndCommitBlockUseCase::with_vm_registry(vm_registry),
            execute_batch_size,
        }
    }

    pub fn health(&self) -> HealthResponse {
        HealthResponse {
            ok: true,
            mempool_len: l1_core::MempoolPort::len(&self.mempool),
        }
    }

    pub fn submit_transactions(
        &self,
        request: SubmitTransactionsRequest,
    ) -> Result<ReceiveAndAdmitTransactionsReply, ReceiveAndAdmitTransactionsError> {
        let signed_requests = request
            .requests
            .iter()
            .map(to_signed_request)
            .collect::<Vec<SignedTransactionRequest>>();
        let pending_requests = request
            .requests
            .iter()
            .map(to_pending_request)
            .collect::<Vec<PendingRequest>>();

        let cmd = ReceiveAndAdmitTransactionsCmd {
            requests: signed_requests,
        };
        let load_port = IngressLoadPort::new(pending_requests);
        let pipeline = MempoolWritingPipeline::<ReceiveAndAdmitTransactionsEvents>::new(Box::new(self.mempool.clone()));

        self.executor.execute_and_map_reply(
            &ReceiveAndAdmitTransactionsUseCase,
            cmd,
            &load_port,
            &pipeline,
            &ReceiveAndAdmitTransactionsReplyMapper,
        )
    }

    pub fn execute_block(
        &self,
        request: ExecuteBlockRequest,
    ) -> Result<ExecuteAndCommitBlockReply, ExecuteAndCommitBlockError> {
        let load_port = MempoolReadingLoadPort::new(Box::new(self.mempool.clone()), request.batch_size.unwrap_or(self.execute_batch_size));
        let pipeline = ExecuteAndCommitBlockStatePipeline {
            state_store: self.state_store.clone(),
        };
        let cmd = ExecuteAndCommitBlockCmd {
            block_height: request.block_height,
            pending_requests: vec![],
        };

        self.executor.execute_and_map_reply(
            &self.execute_use_case,
            cmd,
            &load_port,
            &pipeline,
            &ExecuteAndCommitBlockReplyMapper,
        )
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HealthResponse {
    pub ok: bool,
    pub mempool_len: usize,
}

fn to_signed_request(item: &SubmitTransactionItem) -> SignedTransactionRequest {
    SignedTransactionRequest {
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
        request_id: item.request_id.clone(),
        performer: item.account.clone(),
        vm_kind: item.vm_kind.into(),
        capability: l1_core::VmCapability::new(&item.capability),
        action_type: item.action_type.clone(),
        payload_hash: item.payload_hash.clone(),
    }
}
