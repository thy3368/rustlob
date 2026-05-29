use std::marker::PhantomData;

use cmd_handler::EntityReplayableEvent;
use l1_core::{
    MempoolPort, PendingRequest, ReceiveAndAdmitTransactionsError, VmCapability, VmKind,
};

const INGRESS_DECISION_ENTITY_TYPE: u8 = 1;
const PENDING_REQUEST_ENTITY_TYPE: u8 = 2;

pub struct MempoolWritingPipeline<E = ()> {
    mempool: Box<dyn MempoolPort>,
    _events: PhantomData<E>,
}

impl<E> MempoolWritingPipeline<E> {
    pub fn new(mempool: Box<dyn MempoolPort>) -> Self {
        Self { mempool, _events: PhantomData }
    }
}

fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
    event.field_changes.iter().find_map(|change| {
        let current_name = change.field_name_as_str().ok()?;
        if current_name != field_name {
            return None;
        }
        std::str::from_utf8(change.new_value_bytes()).ok()
    })
}

fn parse_vm_kind(value: &str) -> Option<VmKind> {
    match value {
        "Evm" => Some(VmKind::Evm),
        "RustVm" => Some(VmKind::RustVm),
        _ => None,
    }
}

fn admitted_requests_from_events(events: &[EntityReplayableEvent]) -> Vec<PendingRequest> {
    events
        .iter()
        .filter(|event| event.entity_type == PENDING_REQUEST_ENTITY_TYPE)
        .filter_map(|event| {
            Some(PendingRequest {
                trace_id: event_field(event, "trace_id").map(str::to_string),
                request_id: event_field(event, "request_id")?.to_string(),
                performer: event_field(event, "performer")?.to_string(),
                vm_kind: parse_vm_kind(event_field(event, "vm_kind")?)?,
                capability: VmCapability::new(event_field(event, "capability")?.to_string()),
                action_type: event_field(event, "action_type")?.to_string(),
                payload_hash: event_field(event, "payload_hash")?.to_string(),
                payload: event_field(event, "payload").map(str::to_string),
            })
        })
        .collect()
}

impl<E> MempoolWritingPipeline<E>
where
    E: Send + Sync,
{
    pub fn persist(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        tracing::trace!(
            adapter = "MempoolWritingPipeline",
            adapter_kind = "inbound",
            port = "MempoolPort",
            action = "persist",
            admitted_request_count = events
                .iter()
                .filter(|event| event.entity_type == PENDING_REQUEST_ENTITY_TYPE)
                .count() as u64,
            ingress_decision_count = events
                .iter()
                .filter(|event| event.entity_type == INGRESS_DECISION_ENTITY_TYPE)
                .count() as u64,
            status = "ok",
            "l1 adapter persist completed"
        );
        Ok(())
    }

    pub fn replay(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        use minstant::Instant;

        let started_at = Instant::now();
        let admitted_requests = admitted_requests_from_events(events);
        let first_request = admitted_requests.first();
        tracing::trace!(
            adapter = "MempoolWritingPipeline",
            adapter_kind = "inbound",
            port = "MempoolPort",
            action = "replay",
            admitted_request_count = admitted_requests.len() as u64,
            first_trace_id =
                first_request.and_then(|request| request.trace_id.as_deref()).unwrap_or("-"),
            status = "start",
            "l1 adapter replay started"
        );

        self.mempool.add_requests(admitted_requests.clone()).map_err(|error| {
            let error_message = format!("{error:?}");
            tracing::trace!(
                call_stack = true,
                layer = "outbound",
                component = "MempoolWritingPipeline",
                operation = "replay",
                trace_id =
                    first_request.and_then(|request| request.trace_id.as_deref()).unwrap_or("-"),
                request_admitted_request_count = admitted_requests.len() as u64,
                response_result = "err",
                adapter = "MempoolWritingPipeline",
                adapter_kind = "inbound",
                port = "MempoolPort",
                action = "replay",
                admitted_request_count = admitted_requests.len() as u64,
                status = "err",
                latency_ns = started_at.elapsed().as_nanos().min(u64::MAX as u128) as u64,
                error_message = %error_message,
                "l1 adapter replay failed"
            );
            ReceiveAndAdmitTransactionsError::LoadStateFailed(error_message)
        })?;

        tracing::trace!(
            call_stack = true,
            layer = "outbound",
            component = "MempoolWritingPipeline",
            operation = "replay",
            trace_id = first_request.and_then(|request| request.trace_id.as_deref()).unwrap_or("-"),
            request_admitted_request_count = admitted_requests.len() as u64,
            response_enqueued_request_count = admitted_requests.len() as u64,
            adapter = "MempoolWritingPipeline",
            adapter_kind = "inbound",
            port = "MempoolPort",
            action = "replay",
            admitted_request_count = admitted_requests.len() as u64,
            status = "ok",
            latency_ns = started_at.elapsed().as_nanos().min(u64::MAX as u128) as u64,
            "l1 adapter replay completed"
        );
        Ok(())
    }

    pub fn publish(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        tracing::trace!(
            adapter = "MempoolWritingPipeline",
            adapter_kind = "inbound",
            port = "MempoolPort",
            action = "publish",
            admitted_request_count = events
                .iter()
                .filter(|event| event.entity_type == PENDING_REQUEST_ENTITY_TYPE)
                .count() as u64,
            status = "ok",
            "l1 adapter publish completed"
        );
        Ok(())
    }
}
