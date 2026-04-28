use cmd_handler::{
    use_case_def::{CommandUseCase, CommandUseCaseExecutor, DomainEventPipeline, UseCaseReplyMapper},
    DomainEventSet,
};

use crate::entity::{ChainState, IngressDecision, PendingRequest, SignedTransactionRequest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiveAndAdmitTransactionsError {
    EmptyRequests,
    LoadStateFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiveAndAdmitTransactionsCmd {
    pub requests: Vec<SignedTransactionRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiveAndAdmitTransactionsStateSnapshot {
    pub chain_state: ChainState,
    pub admitted_requests: Vec<PendingRequest>,
    pub ingress_decisions: Vec<IngressDecision>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiveAndAdmitTransactionsEvents {
    pub admitted_requests: Vec<PendingRequest>,
    pub ingress_decisions: Vec<IngressDecision>,
}

impl DomainEventSet for ReceiveAndAdmitTransactionsEvents {
    fn domain_event_count(&self) -> usize {
        self.ingress_decisions.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiveAndAdmitTransactionsReply {
    pub admitted_count: usize,
    pub rejected_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ReceiveAndAdmitTransactionsReplyMapper;

impl UseCaseReplyMapper<ReceiveAndAdmitTransactionsEvents>
    for ReceiveAndAdmitTransactionsReplyMapper
{
    type Reply = ReceiveAndAdmitTransactionsReply;

    fn map(&self, events: ReceiveAndAdmitTransactionsEvents) -> Self::Reply {
        let admitted_count = events.admitted_requests.len();
        let rejected_count = events
            .ingress_decisions
            .iter()
            .filter(|decision| matches!(decision, IngressDecision::Reject))
            .count();

        ReceiveAndAdmitTransactionsReply {
            admitted_count,
            rejected_count,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ReceiveAndAdmitTransactionsUseCase;

impl CommandUseCase for ReceiveAndAdmitTransactionsUseCase {
    type Command = ReceiveAndAdmitTransactionsCmd;
    type GivenState = ReceiveAndAdmitTransactionsStateSnapshot;
    type Events = ReceiveAndAdmitTransactionsEvents;
    type Error = ReceiveAndAdmitTransactionsError;
    type LoadPort = dyn cmd_handler::use_case_def::LoadState<
        ReceiveAndAdmitTransactionsCmd,
        ReceiveAndAdmitTransactionsStateSnapshot,
        ReceiveAndAdmitTransactionsError,
    >;

    fn actor(&self) -> &'static str {
        "IngressGateway"
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

    fn then_event_4_new_state(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Events, Self::Error> {
        Ok(ReceiveAndAdmitTransactionsEvents {
            admitted_requests: state.admitted_requests,
            ingress_decisions: state.ingress_decisions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::StateRoot;

    struct StubLoadPort;

    impl
        cmd_handler::use_case_def::LoadState<
            ReceiveAndAdmitTransactionsCmd,
            ReceiveAndAdmitTransactionsStateSnapshot,
            ReceiveAndAdmitTransactionsError,
        > for StubLoadPort
    {
        fn load_state(
            &self,
            _cmd: &ReceiveAndAdmitTransactionsCmd,
        ) -> Result<ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError>
        {
            Ok(ReceiveAndAdmitTransactionsStateSnapshot {
                chain_state: ChainState {
                    height: 1,
                    state_root: StateRoot("root-1".to_string()),
                },
                admitted_requests: vec![PendingRequest {
                    request_id: "req-1".to_string(),
                    performer: "acct-1".to_string(),
                    action_type: "order".to_string(),
                    payload_hash: "payload-1".to_string(),
                }],
                ingress_decisions: vec![IngressDecision::Admit],
            })
        }
    }

    #[test]
    fn actor_is_ingress_gateway() {
        assert_eq!(ReceiveAndAdmitTransactionsUseCase.actor(), "IngressGateway");
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
        let events = ReceiveAndAdmitTransactionsEvents {
            admitted_requests: vec![PendingRequest {
                request_id: "req-1".to_string(),
                performer: "acct-1".to_string(),
                action_type: "order".to_string(),
                payload_hash: "payload-1".to_string(),
            }],
            ingress_decisions: vec![IngressDecision::Admit, IngressDecision::Reject],
        };

        let reply = ReceiveAndAdmitTransactionsReplyMapper.map(events);

        assert_eq!(reply.admitted_count, 1);
        assert_eq!(reply.rejected_count, 1);
    }

    #[test]
    fn completes_minimal_command_path() {
        let cmd = ReceiveAndAdmitTransactionsCmd {
            requests: vec![SignedTransactionRequest {
                request_id: "req-1".to_string(),
                account: "acct-1".to_string(),
                nonce: "1".to_string(),
                expires_at: "2026-04-25T00:00:00Z".to_string(),
                action_type: "order".to_string(),
                payload_hash: "payload-1".to_string(),
                signature_hash: "sig-1".to_string(),
            }],
        };

        let state = ReceiveAndAdmitTransactionsUseCase
            .load_state(&cmd, &StubLoadPort)
            .unwrap();
        let events = ReceiveAndAdmitTransactionsUseCase.then_event_4_new_state(&cmd, state).unwrap();

        assert_eq!(events.domain_event_count(), 1);
        assert_eq!(events.admitted_requests.len(), 1);
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct NoopReceiveAndAdmitPipeline;

    impl DomainEventPipeline<ReceiveAndAdmitTransactionsEvents, ReceiveAndAdmitTransactionsError>
        for NoopReceiveAndAdmitPipeline
    {
        fn persist(&self, _events: &ReceiveAndAdmitTransactionsEvents) -> Result<(), ReceiveAndAdmitTransactionsError> {
            Ok(())
        }

        fn replay(&self, _events: &ReceiveAndAdmitTransactionsEvents) -> Result<(), ReceiveAndAdmitTransactionsError> {
            Ok(())
        }

        fn publish(&self, _events: &ReceiveAndAdmitTransactionsEvents) -> Result<(), ReceiveAndAdmitTransactionsError> {
            Ok(())
        }
    }

    #[test]
    fn execute_with_command_use_case_executor() {
        let executor = CommandUseCaseExecutor;
        let use_case = ReceiveAndAdmitTransactionsUseCase;
        let load_port = StubLoadPort;
        let pipeline = NoopReceiveAndAdmitPipeline;

        let cmd = ReceiveAndAdmitTransactionsCmd {
            requests: vec![SignedTransactionRequest {
                request_id: "req-1".to_string(),
                account: "acct-1".to_string(),
                nonce: "1".to_string(),
                expires_at: "2026-04-25T00:00:00Z".to_string(),
                action_type: "order".to_string(),
                payload_hash: "payload-1".to_string(),
                signature_hash: "sig-1".to_string(),
            }],
        };

        let events = executor.execute(&use_case, cmd, &load_port, &pipeline).unwrap();

        assert_eq!(events.domain_event_count(), 1);
        assert_eq!(events.admitted_requests.len(), 1);
        assert_eq!(events.admitted_requests[0].request_id, "req-1");
    }

    #[test]
    fn executor_rejects_empty_requests() {
        let executor = CommandUseCaseExecutor;
        let use_case = ReceiveAndAdmitTransactionsUseCase;
        let load_port = StubLoadPort;
        let pipeline = NoopReceiveAndAdmitPipeline;

        let cmd = ReceiveAndAdmitTransactionsCmd { requests: vec![] };

        let error = executor.execute(&use_case, cmd, &load_port, &pipeline).unwrap_err();

        assert_eq!(error, ReceiveAndAdmitTransactionsError::EmptyRequests);
    }
}
