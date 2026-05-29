use l1_core::{
    ChainState, IngressDecision, PendingRequest, ReceiveAndAdmitTransactionsCmd,
    ReceiveAndAdmitTransactionsError, ReceiveAndAdmitTransactionsStateSnapshot, StateRoot,
};

pub struct IngressLoadPort {
    pending_requests: Vec<PendingRequest>,
}

impl IngressLoadPort {
    pub fn new(pending_requests: Vec<PendingRequest>) -> Self {
        Self { pending_requests }
    }

    pub fn load_state(
        &self,
        cmd: &ReceiveAndAdmitTransactionsCmd,
    ) -> Result<ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError> {
        use minstant::Instant;

        let started_at = Instant::now();
        let request_count = cmd.requests.len() as u64;
        let admitted_request_count = self.pending_requests.len() as u64;
        let ingress_decision_count = self.pending_requests.len() as u64;
        let trace_id =
            cmd.requests.first().and_then(|request| request.trace_id.as_deref()).unwrap_or("-");
        tracing::trace!(
            call_stack = true,
            layer = "outbound",
            component = "IngressLoadPort",
            operation = "load_state",
            trace_id = trace_id,
            request_request_count = request_count,
            response_admitted_request_count = admitted_request_count,
            response_ingress_decision_count = ingress_decision_count,
            adapter = "IngressLoadPort",
            action = "load_state",
            request_count = request_count,
            first_trace_id = trace_id,
            status = "ok",
            latency_ns = started_at.elapsed().as_nanos().min(u64::MAX as u128) as u64,
            "l1 ingress load_state completed"
        );
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
