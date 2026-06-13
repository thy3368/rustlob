use std::fmt;
use std::sync::Mutex;

use cmd_handler::HandlerLatencyMetrics;
use cmd_handler::command_use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCase3, CommandUseCaseExecutionError,
    CommandUseCaseExecutor3, CommandUseCaseOutbound, CommandUseCaseOutboundPhase, IssuedByParty,
    ObserveHandlerLatency, UseCaseOutput, UseCaseReplyMapper3,
};
use common_entity::EntityReplayableEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
struct StubCommand {
    party_id: String,
    event_count: u64,
}

impl IssuedByParty for StubCommand {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StubOutput {
    event_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StubBusinessError {
    RejectedInPreCheck,
    RejectedInValidate,
    RejectedInCompute,
}

impl fmt::Display for StubBusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RejectedInPreCheck => f.write_str("rejected in pre_check"),
            Self::RejectedInValidate => f.write_str("rejected in validate"),
            Self::RejectedInCompute => f.write_str("rejected in compute"),
        }
    }
}

impl std::error::Error for StubBusinessError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StubOutboundError {
    LoadFailed,
}

impl fmt::Display for StubOutboundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFailed => f.write_str("load failed"),
        }
    }
}

impl std::error::Error for StubOutboundError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StubState {
    loaded_event_count: u64,
    reject_in_validate: bool,
    reject_in_compute: bool,
}

fn stub_event(sequence: u64) -> EntityReplayableEvent {
    EntityReplayableEvent::new_created(0, sequence, sequence as i64 + 1, 1)
}

#[derive(Debug, Clone, Copy, Default)]
struct StubUseCase3;

impl CommandUseCase3 for StubUseCase3 {
    type Command = StubCommand;
    type GivenState = StubState;
    type Error = StubBusinessError;
    type Output = StubOutput;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(StubBusinessError::RejectedInPreCheck);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.reject_in_validate {
            return Err(StubBusinessError::RejectedInValidate);
        }
        Ok(())
    }

    fn compute_output_and_events(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<UseCaseOutput<Self::Output>, Self::Error> {
        if state.reject_in_compute {
            return Err(StubBusinessError::RejectedInCompute);
        }
        let output = StubOutput { event_count: state.loaded_event_count };
        let events = (0..output.event_count).map(stub_event).collect();
        Ok(UseCaseOutput { output, events })
    }
}

#[derive(Debug)]
struct StubOutbound {
    state: Mutex<Result<StubState, StubOutboundError>>,
    calls: Mutex<Vec<&'static str>>,
}

impl StubOutbound {
    fn with_state(state: StubState) -> Self {
        Self { state: Mutex::new(Ok(state)), calls: Mutex::new(Vec::new()) }
    }

    fn with_error(error: StubOutboundError) -> Self {
        Self { state: Mutex::new(Err(error)), calls: Mutex::new(Vec::new()) }
    }

    fn calls(&self) -> Vec<&'static str> {
        self.calls.lock().unwrap().clone()
    }

    fn record(&self, phase: &'static str) {
        self.calls.lock().unwrap().push(phase);
    }
}

impl CommandUseCaseOutbound for StubOutbound {
    type Command = StubCommand;
    type State = StubState;
    type Error = StubOutboundError;

    fn load_state(&self, _cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        self.record("load_state");
        self.state.lock().unwrap().clone()
    }

    fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.record("persist");
        Ok(())
    }

    fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.record("replay");
        Ok(())
    }

    fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.record("publish");
        Ok(())
    }
}

#[derive(Debug, Default)]
struct StubObserver {
    observed: Mutex<Vec<HandlerLatencyMetrics>>,
}

impl StubObserver {
    fn observed_count(&self) -> usize {
        self.observed.lock().unwrap().len()
    }

    fn last(&self) -> Option<HandlerLatencyMetrics> {
        self.observed.lock().unwrap().last().copied()
    }
}

impl ObserveHandlerLatency for StubObserver {
    fn observe_latency(&self, metrics: &HandlerLatencyMetrics) {
        self.observed.lock().unwrap().push(*metrics);
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct StubReplyMapper3;

impl UseCaseReplyMapper3 for StubReplyMapper3 {
    type Output = StubOutput;
    type Reply = usize;

    fn map(&self, result: UseCaseOutput<Self::Output>) -> Self::Reply {
        result.output.event_count as usize + result.events.len()
    }
}

fn sample_envelope(party_id: &str, event_count: u64) -> CommandEnvelope<StubCommand> {
    CommandEnvelope {
        meta: CommandMeta {
            trace_id: Some("trace-1".to_string()),
            command_id: Some("cmd-1".to_string()),
        },
        command: StubCommand { party_id: party_id.to_string(), event_count },
    }
}

#[test]
fn execute_returns_typed_output_and_events_on_happy_path() {
    let executor = CommandUseCaseExecutor3;
    let use_case = StubUseCase3;
    let outbound = StubOutbound::with_state(StubState {
        loaded_event_count: 2,
        reject_in_validate: false,
        reject_in_compute: false,
    });
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("trader-1", 2), &outbound, &observer);

    assert_eq!(
        result,
        Ok(UseCaseOutput {
            output: StubOutput { event_count: 2 },
            events: vec![stub_event(0), stub_event(1)],
        })
    );
    assert_eq!(outbound.calls(), vec!["load_state", "persist", "replay", "publish"]);
    assert_eq!(observer.observed_count(), 1);
    assert_eq!(observer.last().map(|metrics| metrics.domain_event_count), Some(2));
}

#[test]
fn execute_rejects_in_pre_check_before_loading_state() {
    let executor = CommandUseCaseExecutor3;
    let use_case = StubUseCase3;
    let outbound = StubOutbound::with_state(StubState {
        loaded_event_count: 2,
        reject_in_validate: false,
        reject_in_compute: false,
    });
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("", 2), &outbound, &observer);

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::Business(StubBusinessError::RejectedInPreCheck))
    );
    assert!(outbound.calls().is_empty());
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn execute_stops_after_validate_failure() {
    let executor = CommandUseCaseExecutor3;
    let use_case = StubUseCase3;
    let outbound = StubOutbound::with_state(StubState {
        loaded_event_count: 2,
        reject_in_validate: true,
        reject_in_compute: false,
    });
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("trader-1", 2), &outbound, &observer);

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::Business(StubBusinessError::RejectedInValidate))
    );
    assert_eq!(outbound.calls(), vec!["load_state"]);
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn execute_stops_after_compute_failure() {
    let executor = CommandUseCaseExecutor3;
    let use_case = StubUseCase3;
    let outbound = StubOutbound::with_state(StubState {
        loaded_event_count: 2,
        reject_in_validate: false,
        reject_in_compute: true,
    });
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("trader-1", 2), &outbound, &observer);

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::Business(StubBusinessError::RejectedInCompute))
    );
    assert_eq!(outbound.calls(), vec!["load_state"]);
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn execute_and_map_reply_consumes_typed_output() {
    let executor = CommandUseCaseExecutor3;
    let use_case = StubUseCase3;
    let outbound = StubOutbound::with_state(StubState {
        loaded_event_count: 3,
        reject_in_validate: false,
        reject_in_compute: false,
    });
    let observer = StubObserver::default();

    let reply = executor.execute_and_map_reply(
        &use_case,
        sample_envelope("trader-1", 3),
        &outbound,
        &observer,
        &StubReplyMapper3,
    );

    assert_eq!(reply, Ok(6));
    assert_eq!(observer.last().map(|metrics| metrics.domain_event_count), Some(3));
}

#[test]
fn execute_wraps_outbound_load_failure() {
    let executor = CommandUseCaseExecutor3;
    let use_case = StubUseCase3;
    let outbound = StubOutbound::with_error(StubOutboundError::LoadFailed);
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("trader-1", 2), &outbound, &observer);

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::Outbound {
            phase: CommandUseCaseOutboundPhase::LoadState,
            source: StubOutboundError::LoadFailed,
        })
    );
    assert_eq!(outbound.calls(), vec!["load_state"]);
    assert_eq!(observer.observed_count(), 0);
}
