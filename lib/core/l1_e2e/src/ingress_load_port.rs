use cmd_handler::use_case_def::LoadState;
use l1_core::{ChainState, IngressDecision, PendingRequest, ReceiveAndAdmitTransactionsCmd, ReceiveAndAdmitTransactionsError, ReceiveAndAdmitTransactionsStateSnapshot, StateRoot};

pub struct IngressLoadPort {
    pending_requests: Vec<PendingRequest>,
}

impl IngressLoadPort {
    pub fn new(pending_requests: Vec<PendingRequest>) -> Self {
        Self { pending_requests }
    }
}

impl LoadState<ReceiveAndAdmitTransactionsCmd, ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError>
    for IngressLoadPort
{
    fn load_state(
        &self,
        _cmd: &ReceiveAndAdmitTransactionsCmd,
    ) -> Result<ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError> {
        Ok(ReceiveAndAdmitTransactionsStateSnapshot {
            chain_state: ChainState {
                height: 0,
                state_root: StateRoot("state-root-0".to_string()),
            },
            admitted_requests: self.pending_requests.clone(),
            ingress_decisions: self
                .pending_requests
                .iter()
                .map(|_| IngressDecision::Admit)
                .collect(),
        })
    }
}
