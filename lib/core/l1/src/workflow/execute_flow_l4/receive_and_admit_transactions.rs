use cmd_handler::command_use_case_def2::{CommandUseCase2, IssuedByParty, UseCaseReplyMapper};
use cmd_handler::{EntityReplayableEvent, ReplayFieldChange};
use thiserror::Error;

use crate::{
    ChainState, IngressDecision, PendingRequest, SignedTransactionRequest, VmCapability, VmKind,
};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReceiveAndAdmitTransactionsError {
    #[error("requests must not be empty")]
    EmptyRequests,
    #[error("failed to load ingress state: {0}")]
    LoadStateFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiveAndAdmitTransactionsCmd {
    pub requests: Vec<SignedTransactionRequest>,
}

impl IssuedByParty for ReceiveAndAdmitTransactionsCmd {
    fn party_id(&self) -> Option<&str> {
        self.requests.first().map(|request| request.account.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiveAndAdmitTransactionsStateSnapshot {
    pub chain_state: ChainState,
    pub admitted_requests: Vec<PendingRequest>,
    pub ingress_decisions: Vec<IngressDecision>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiveAndAdmitTransactionsReply {
    pub admitted_count: usize,
    pub rejected_count: usize,
}

const INGRESS_DECISION_ENTITY_TYPE: u8 = 1;
const PENDING_REQUEST_ENTITY_TYPE: u8 = 2;
const FIELD_TYPE_STRING: u8 = 0;

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn string_field(name: &str, value: &str) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.as_bytes(),
        FIELD_TYPE_STRING,
    )
}

fn add_optional_string_field(
    event: &mut EntityReplayableEvent,
    field_name: &str,
    value: Option<&str>,
) {
    if let Some(value) = value {
        event.add_field_change(string_field(field_name, value));
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

fn decision_label(decision: &IngressDecision) -> &'static str {
    match decision {
        IngressDecision::Admit => "Admit",
        IngressDecision::Reject => "Reject",
    }
}

#[cfg(test)]
fn parse_decision(value: &str) -> Option<IngressDecision> {
    match value {
        "Admit" => Some(IngressDecision::Admit),
        "Reject" => Some(IngressDecision::Reject),
        _ => None,
    }
}

fn vm_kind_label(vm_kind: VmKind) -> &'static str {
    match vm_kind {
        VmKind::Evm => "Evm",
        VmKind::RustVm => "RustVm",
    }
}

#[cfg(test)]
fn parse_vm_kind(value: &str) -> Option<VmKind> {
    match value {
        "Evm" => Some(VmKind::Evm),
        "RustVm" => Some(VmKind::RustVm),
        _ => None,
    }
}

fn ingress_decision_event(
    request: &SignedTransactionRequest,
    decision: &IngressDecision,
    sequence: u64,
) -> EntityReplayableEvent {
    let mut event = EntityReplayableEvent::new_created(
        0,
        sequence,
        stable_entity_id(&request.request_id),
        INGRESS_DECISION_ENTITY_TYPE,
    );
    add_optional_string_field(&mut event, "trace_id", request.trace_id.as_deref());
    event.add_field_change(string_field("request_id", &request.request_id));
    event.add_field_change(string_field("account", &request.account));
    event.add_field_change(string_field("decision", decision_label(decision)));
    event
}

fn pending_request_event(request: &PendingRequest, sequence: u64) -> EntityReplayableEvent {
    let mut event = EntityReplayableEvent::new_created(
        0,
        sequence,
        stable_entity_id(&request.request_id),
        PENDING_REQUEST_ENTITY_TYPE,
    );
    add_optional_string_field(&mut event, "trace_id", request.trace_id.as_deref());
    event.add_field_change(string_field("request_id", &request.request_id));
    event.add_field_change(string_field("performer", &request.performer));
    event.add_field_change(string_field("vm_kind", vm_kind_label(request.vm_kind)));
    event.add_field_change(string_field("capability", &request.capability.0));
    event.add_field_change(string_field("action_type", &request.action_type));
    event.add_field_change(string_field("payload_hash", &request.payload_hash));
    add_optional_string_field(&mut event, "payload", request.payload.as_deref());
    event
}

fn is_pending_request_event(event: &EntityReplayableEvent) -> bool {
    event.entity_type == PENDING_REQUEST_ENTITY_TYPE
}

fn is_rejected_ingress_decision_event(event: &EntityReplayableEvent) -> bool {
    event.entity_type == INGRESS_DECISION_ENTITY_TYPE
        && event_field(event, "decision") == Some("Reject")
}

#[cfg(test)]
fn admitted_requests_from_events(events: &[EntityReplayableEvent]) -> Vec<PendingRequest> {
    events
        .iter()
        .filter(|event| is_pending_request_event(event))
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

#[cfg(test)]
fn ingress_decisions_from_events(events: &[EntityReplayableEvent]) -> Vec<IngressDecision> {
    events
        .iter()
        .filter(|event| event.entity_type == INGRESS_DECISION_ENTITY_TYPE)
        .filter_map(|event| parse_decision(event_field(event, "decision")?))
        .collect()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ReceiveAndAdmitTransactionsReplyMapper;

impl UseCaseReplyMapper for ReceiveAndAdmitTransactionsReplyMapper {
    type Reply = ReceiveAndAdmitTransactionsReply;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        let admitted_count = events.iter().filter(|event| is_pending_request_event(event)).count();
        let rejected_count =
            events.iter().filter(|event| is_rejected_ingress_decision_event(event)).count();

        ReceiveAndAdmitTransactionsReply { admitted_count, rejected_count }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ReceiveAndAdmitTransactionsUseCase;

impl CommandUseCase2 for ReceiveAndAdmitTransactionsUseCase {
    type Command = ReceiveAndAdmitTransactionsCmd;
    type GivenState = ReceiveAndAdmitTransactionsStateSnapshot;
    type Error = ReceiveAndAdmitTransactionsError;

    fn role(&self) -> &'static str {
        "IngressGateway"
    }

    fn format_error(&self, error: &Self::Error) -> Option<String> {
        match error {
            ReceiveAndAdmitTransactionsError::EmptyRequests => {
                Some("receive_and_admit_transactions: empty requests".to_string())
            }
            ReceiveAndAdmitTransactionsError::LoadStateFailed(message) => {
                Some(format!("receive_and_admit_transactions load_state_failed: {message}"))
            }
        }
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.requests.is_empty() {
            return Err(ReceiveAndAdmitTransactionsError::EmptyRequests);
        }

        Ok(())
    }

    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        _state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let mut events =
            Vec::with_capacity(state.ingress_decisions.len() + state.admitted_requests.len());

        for (index, decision) in state.ingress_decisions.iter().enumerate() {
            if let Some(request) = cmd.requests.get(index) {
                events.push(ingress_decision_event(request, decision, index as u64));
            }
        }

        let offset = events.len() as u64;
        for (index, request) in state.admitted_requests.iter().enumerate() {
            events.push(pending_request_event(request, offset + index as u64));
        }

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use cmd_handler::command_use_case_def2::{
        CommandEnvelope, CommandMeta, CommandUseCaseExecutionError, CommandUseCaseExecutor2,
        CommandUseCaseOutbound,
    };
    use proptest::prelude::*;

    use super::*;
    use crate::{StateRoot, VmCapability, VmKind};

    struct StubLoadPort;

    impl StubLoadPort {
        fn load_state(
            &self,
            _cmd: &ReceiveAndAdmitTransactionsCmd,
        ) -> Result<ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError>
        {
            Ok(ReceiveAndAdmitTransactionsStateSnapshot {
                chain_state: ChainState { height: 1, state_root: StateRoot("root-1".to_string()) },
                admitted_requests: vec![PendingRequest {
                    trace_id: Some("trace-1".to_string()),
                    request_id: "req-1".to_string(),
                    performer: "acct-1".to_string(),
                    vm_kind: VmKind::RustVm,
                    capability: VmCapability::new("dex.prep.place_order"),
                    action_type: "order".to_string(),
                    payload_hash: "payload-1".to_string(),
                    payload: None,
                }],
                ingress_decisions: vec![IngressDecision::Admit],
            })
        }
    }

    #[test]
    fn role_is_ingress_gateway() {
        assert_eq!(ReceiveAndAdmitTransactionsUseCase.role(), "IngressGateway");
    }

    #[test]
    fn rejects_empty_requests() {
        let cmd = ReceiveAndAdmitTransactionsCmd { requests: vec![] };

        assert_eq!(
            ReceiveAndAdmitTransactionsUseCase.pre_check_command(&cmd),
            Err(ReceiveAndAdmitTransactionsError::EmptyRequests)
        );
    }

    #[test]
    fn maps_events_to_reply() {
        let admitted_request = PendingRequest {
            trace_id: Some("trace-1".to_string()),
            request_id: "req-1".to_string(),
            performer: "acct-1".to_string(),
            vm_kind: VmKind::RustVm,
            capability: VmCapability::new("dex.prep.place_order"),
            action_type: "order".to_string(),
            payload_hash: "payload-1".to_string(),
            payload: None,
        };
        let events = vec![
            ingress_decision_event(
                &SignedTransactionRequest {
                    trace_id: Some("trace-1".to_string()),
                    request_id: "req-1".to_string(),
                    account: "acct-1".to_string(),
                    nonce: "1".to_string(),
                    expires_at: "2026-04-25T00:00:00Z".to_string(),
                    action_type: "order".to_string(),
                    payload_hash: "payload-1".to_string(),
                    signature_hash: "sig-1".to_string(),
                },
                &IngressDecision::Admit,
                0,
            ),
            ingress_decision_event(
                &SignedTransactionRequest {
                    trace_id: Some("trace-2".to_string()),
                    request_id: "req-2".to_string(),
                    account: "acct-2".to_string(),
                    nonce: "2".to_string(),
                    expires_at: "2026-04-25T00:00:00Z".to_string(),
                    action_type: "order".to_string(),
                    payload_hash: "payload-2".to_string(),
                    signature_hash: "sig-2".to_string(),
                },
                &IngressDecision::Reject,
                1,
            ),
            pending_request_event(&admitted_request, 2),
        ];

        let reply = ReceiveAndAdmitTransactionsReplyMapper.map(events);

        assert_eq!(reply.admitted_count, 1);
        assert_eq!(reply.rejected_count, 1);
    }

    #[test]
    fn completes_minimal_command_path() {
        let cmd = ReceiveAndAdmitTransactionsCmd {
            requests: vec![SignedTransactionRequest {
                trace_id: Some("trace-1".to_string()),
                request_id: "req-1".to_string(),
                account: "acct-1".to_string(),
                nonce: "1".to_string(),
                expires_at: "2026-04-25T00:00:00Z".to_string(),
                action_type: "order".to_string(),
                payload_hash: "payload-1".to_string(),
                signature_hash: "sig-1".to_string(),
            }],
        };

        let state = StubLoadPort.load_state(&cmd).unwrap();
        let events =
            ReceiveAndAdmitTransactionsUseCase.compute_replayable_events(&cmd, state).unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(ingress_decisions_from_events(&events), vec![IngressDecision::Admit]);
        assert_eq!(admitted_requests_from_events(&events).len(), 1);
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct NoopReceiveAndAdmitOutbound;

    impl CommandUseCaseOutbound for NoopReceiveAndAdmitOutbound {
        type Command = ReceiveAndAdmitTransactionsCmd;
        type State = ReceiveAndAdmitTransactionsStateSnapshot;
        type Error = ReceiveAndAdmitTransactionsError;

        fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
            StubLoadPort.load_state(cmd)
        }

        fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[test]
    fn execute_with_command_use_case_executor() {
        let executor = CommandUseCaseExecutor2;
        let use_case = ReceiveAndAdmitTransactionsUseCase;
        let outbound = NoopReceiveAndAdmitOutbound;

        let cmd = ReceiveAndAdmitTransactionsCmd {
            requests: vec![SignedTransactionRequest {
                trace_id: Some("trace-1".to_string()),
                request_id: "req-1".to_string(),
                account: "acct-1".to_string(),
                nonce: "1".to_string(),
                expires_at: "2026-04-25T00:00:00Z".to_string(),
                action_type: "order".to_string(),
                payload_hash: "payload-1".to_string(),
                signature_hash: "sig-1".to_string(),
            }],
        };

        let events = executor
            .execute(
                &use_case,
                CommandEnvelope { meta: CommandMeta::default(), command: cmd },
                &outbound,
                &(),
            )
            .unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(admitted_requests_from_events(&events)[0].request_id, "req-1");
    }

    #[test]
    fn executor_rejects_empty_requests() {
        let executor = CommandUseCaseExecutor2;
        let use_case = ReceiveAndAdmitTransactionsUseCase;
        let outbound = NoopReceiveAndAdmitOutbound;

        let cmd = ReceiveAndAdmitTransactionsCmd { requests: vec![] };

        let error = executor
            .execute(
                &use_case,
                CommandEnvelope { meta: CommandMeta::default(), command: cmd },
                &outbound,
                &(),
            )
            .unwrap_err();

        assert_eq!(
            error,
            CommandUseCaseExecutionError::Business(ReceiveAndAdmitTransactionsError::EmptyRequests)
        );
    }

    fn signed_transaction_request_strategy() -> impl Strategy<Value = SignedTransactionRequest> {
        (
            any::<u64>(),
            any::<u64>(),
            any::<u64>(),
            any::<u8>(),
            any::<u8>(),
            any::<u16>(),
            any::<u64>(),
            any::<u64>(),
        )
            .prop_map(
                |(
                    request_id,
                    account,
                    nonce,
                    expires_day,
                    expires_hour,
                    action_type,
                    payload_hash,
                    signature_hash,
                )| SignedTransactionRequest {
                    trace_id: Some(format!("trace-{request_id}")),
                    request_id: format!("req-{request_id}"),
                    account: format!("acct-{account}"),
                    nonce: nonce.to_string(),
                    expires_at: format!(
                        "2026-04-{:02}T{:02}:00:00Z",
                        (expires_day % 28) + 1,
                        expires_hour % 24
                    ),
                    action_type: format!("action-{action_type}"),
                    payload_hash: format!("payload-{payload_hash}"),
                    signature_hash: format!("sig-{signature_hash}"),
                },
            )
    }

    fn request_is_admitted(request: &SignedTransactionRequest) -> bool {
        request
            .nonce
            .chars()
            .last()
            .is_some_and(|digit| matches!(digit, '0' | '2' | '4' | '6' | '8'))
    }

    fn ingress_decision_from_request(request: &SignedTransactionRequest) -> IngressDecision {
        if request_is_admitted(request) { IngressDecision::Admit } else { IngressDecision::Reject }
    }

    fn pending_request_from_request(request: &SignedTransactionRequest) -> PendingRequest {
        PendingRequest {
            trace_id: request.trace_id.clone(),
            request_id: request.request_id.clone(),
            performer: request.account.clone(),
            vm_kind: if request.signature_hash.len() % 2 == 0 {
                VmKind::RustVm
            } else {
                VmKind::Evm
            },
            capability: VmCapability::new(format!("ingress.{}", request.expires_at)),
            action_type: request.action_type.clone(),
            payload_hash: request.payload_hash.clone(),
            payload: Some(format!("nonce:{}|sig:{}", request.nonce, request.signature_hash)),
        }
    }

    #[derive(Debug, Default)]
    struct CountingOutbound {
        load_calls: AtomicUsize,
        persist_calls: AtomicUsize,
        replay_calls: AtomicUsize,
        publish_calls: AtomicUsize,
    }

    impl CommandUseCaseOutbound for CountingOutbound {
        type Command = ReceiveAndAdmitTransactionsCmd;
        type State = ReceiveAndAdmitTransactionsStateSnapshot;
        type Error = ReceiveAndAdmitTransactionsError;

        fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
            self.load_calls.fetch_add(1, Ordering::Relaxed);

            let ingress_decisions =
                cmd.requests.iter().map(ingress_decision_from_request).collect::<Vec<_>>();
            let admitted_requests = cmd
                .requests
                .iter()
                .filter(|request| request_is_admitted(request))
                .map(pending_request_from_request)
                .collect::<Vec<_>>();

            Ok(ReceiveAndAdmitTransactionsStateSnapshot {
                chain_state: ChainState {
                    height: cmd.requests.len() as u64,
                    state_root: StateRoot(format!("root-{}", cmd.requests.len())),
                },
                admitted_requests,
                ingress_decisions,
            })
        }

        fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.persist_calls.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.replay_calls.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.publish_calls.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    proptest! {
        #[test]
        fn property_executor_execute_covers_empty_and_non_empty_commands(
            requests in proptest::collection::vec(signed_transaction_request_strategy(), 0..8),
        ) {
            let executor = CommandUseCaseExecutor2;
            let use_case = ReceiveAndAdmitTransactionsUseCase;
            let outbound = CountingOutbound::default();

            let result = executor.execute(
                &use_case,
                CommandEnvelope {
                    meta: CommandMeta::default(),
                    command: ReceiveAndAdmitTransactionsCmd { requests: requests.clone() },
                },
                &outbound,
                &(),
            );

            if requests.is_empty() {
                prop_assert_eq!(
                    result,
                    Err(CommandUseCaseExecutionError::Business(
                        ReceiveAndAdmitTransactionsError::EmptyRequests,
                    ))
                );
                prop_assert_eq!(outbound.load_calls.load(Ordering::Relaxed), 0);
                prop_assert_eq!(outbound.persist_calls.load(Ordering::Relaxed), 0);
                prop_assert_eq!(outbound.replay_calls.load(Ordering::Relaxed), 0);
                prop_assert_eq!(outbound.publish_calls.load(Ordering::Relaxed), 0);
            } else {
                let expected_ingress_decisions = requests
                    .iter()
                    .map(ingress_decision_from_request)
                    .collect::<Vec<_>>();
                let expected_admitted_requests = requests
                    .iter()
                    .filter(|request| request_is_admitted(request))
                    .map(pending_request_from_request)
                    .collect::<Vec<_>>();

                let events = match result {
                    Ok(events) => events,
                    Err(error) => {
                        prop_assert!(false, "unexpected executor error: {:?}", error);
                        unreachable!();
                    }
                };

                prop_assert_eq!(outbound.load_calls.load(Ordering::Relaxed), 1);
                prop_assert_eq!(outbound.persist_calls.load(Ordering::Relaxed), 1);
                prop_assert_eq!(outbound.replay_calls.load(Ordering::Relaxed), 1);
                prop_assert_eq!(outbound.publish_calls.load(Ordering::Relaxed), 1);
                let expected_event_count =
                    expected_ingress_decisions.len() + expected_admitted_requests.len();
                prop_assert_eq!(ingress_decisions_from_events(&events), expected_ingress_decisions);
                prop_assert_eq!(admitted_requests_from_events(&events), expected_admitted_requests);
                prop_assert_eq!(events.len(), expected_event_count);
            }
        }
    }
}
