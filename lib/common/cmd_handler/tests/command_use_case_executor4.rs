use std::fmt;
use std::sync::Mutex;

use cmd_handler::HandlerLatencyMetrics;
use cmd_handler::command_use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCase4, CommandUseCaseExecutionError,
    CommandUseCaseExecutor4, CommandUseCaseOutbound, CommandUseCaseOutboundPhase,
    EventProjectError, IssuedByParty, ObserveHandlerLatency, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::{Entity, EntityFieldChange, EntityReplayableEvent};

const TEST_ENTITY_TYPE: u8 = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
struct StubCommand {
    party_id: String,
}

impl IssuedByParty for StubCommand {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
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
    reject_in_validate: bool,
    reject_in_compute: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestEntity {
    id: i64,
    value: String,
    version: u64,
}

impl Entity for TestEntity {
    type Id = i64;

    fn entity_id(&self) -> Self::Id {
        self.id
    }

    fn entity_type() -> u8 {
        TEST_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        if self.value != other.value {
            changes.push(EntityFieldChange::new("value", &self.value, &other.value));
        }
        changes
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![EntityFieldChange::new("value", "", &self.value)]
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "value" => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CreatedOnlyChanges {
    created_orders: Vec<TestEntity>,
}

impl ReplayableChanges for CreatedOnlyChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        self.created_orders.iter().map(Entity::track_create_event).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CreateAndUpdateChanges {
    created_orders: Vec<TestEntity>,
    updated_orders: Vec<UpdatedEntityPair<TestEntity>>,
}

impl ReplayableChanges for CreateAndUpdateChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::new();
        for created in &self.created_orders {
            events.push(created.track_create_event()?);
        }
        for updated in &self.updated_orders {
            events.push(updated.after.track_update_event_from(&updated.before)?);
        }
        Ok(events)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct StubCreatedOnlyUseCase;

impl CommandUseCase4 for StubCreatedOnlyUseCase {
    type Command = StubCommand;
    type GivenState = StubState;
    type Error = StubBusinessError;
    type Changes = CreatedOnlyChanges;

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

    fn compute_changes(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        if state.reject_in_compute {
            return Err(StubBusinessError::RejectedInCompute);
        }
        Ok(CreatedOnlyChanges {
            created_orders: vec![TestEntity { id: 11, value: "created".to_string(), version: 1 }],
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct StubCreateAndUpdateUseCase;

impl CommandUseCase4 for StubCreateAndUpdateUseCase {
    type Command = StubCommand;
    type GivenState = StubState;
    type Error = StubBusinessError;
    type Changes = CreateAndUpdateChanges;

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

    fn compute_changes(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        if state.reject_in_compute {
            return Err(StubBusinessError::RejectedInCompute);
        }

        Ok(CreateAndUpdateChanges {
            created_orders: vec![TestEntity { id: 21, value: "trade-1".to_string(), version: 1 }],
            updated_orders: vec![
                UpdatedEntityPair {
                    before: TestEntity { id: 31, value: "maker-before".to_string(), version: 1 },
                    after: TestEntity { id: 31, value: "maker-after".to_string(), version: 2 },
                },
                UpdatedEntityPair {
                    before: TestEntity { id: 41, value: "taker-before".to_string(), version: 4 },
                    after: TestEntity { id: 41, value: "taker-after".to_string(), version: 5 },
                },
            ],
        })
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

fn sample_envelope(party_id: &str) -> CommandEnvelope<StubCommand> {
    CommandEnvelope {
        meta: CommandMeta {
            trace_id: Some("trace-1".to_string()),
            command_id: Some("cmd-1".to_string()),
        },
        command: StubCommand { party_id: party_id.to_string() },
    }
}

fn field_value(event: &EntityReplayableEvent, field_name: &str) -> Option<String> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }
        std::str::from_utf8(change.new_value_bytes()).ok().map(ToString::to_string)
    })
}

#[test]
fn execute_returns_created_only_changes_and_events_on_happy_path() {
    let executor = CommandUseCaseExecutor4;
    let use_case = StubCreatedOnlyUseCase;
    let outbound =
        StubOutbound::with_state(StubState { reject_in_validate: false, reject_in_compute: false });
    let observer = StubObserver::default();

    let result =
        executor.execute(&use_case, sample_envelope("trader-1"), &outbound, &observer).unwrap();

    assert_eq!(
        result.changes,
        CreatedOnlyChanges {
            created_orders: vec![TestEntity { id: 11, value: "created".to_string(), version: 1 }],
        }
    );
    assert_eq!(result.events.len(), 1);
    assert!(result.events[0].is_created());
    assert_eq!(field_value(&result.events[0], "value").as_deref(), Some("created"));
    assert_eq!(outbound.calls(), vec!["load_state", "persist", "replay", "publish"]);
    assert_eq!(observer.observed_count(), 1);
    assert_eq!(observer.last().map(|metrics| metrics.domain_event_count), Some(1));
}

#[test]
fn execute_projects_create_and_update_events_in_stable_order() {
    let executor = CommandUseCaseExecutor4;
    let use_case = StubCreateAndUpdateUseCase;
    let outbound =
        StubOutbound::with_state(StubState { reject_in_validate: false, reject_in_compute: false });
    let observer = StubObserver::default();

    let result =
        executor.execute(&use_case, sample_envelope("trader-1"), &outbound, &observer).unwrap();

    assert_eq!(result.changes.created_orders.len(), 1);
    assert_eq!(result.changes.updated_orders.len(), 2);
    assert_eq!(result.changes.updated_orders[0].before.value, "maker-before");
    assert_eq!(result.changes.updated_orders[0].after.value, "maker-after");
    assert_eq!(result.changes.updated_orders[1].before.value, "taker-before");
    assert_eq!(result.changes.updated_orders[1].after.value, "taker-after");

    assert_eq!(result.events.len(), 3);
    assert!(result.events[0].is_created());
    assert_eq!(field_value(&result.events[0], "value").as_deref(), Some("trade-1"));
    assert!(result.events[1].is_updated());
    assert_eq!(result.events[1].old_version, 1);
    assert_eq!(result.events[1].new_version, 2);
    assert_eq!(field_value(&result.events[1], "value").as_deref(), Some("maker-after"));
    assert!(result.events[2].is_updated());
    assert_eq!(result.events[2].old_version, 4);
    assert_eq!(result.events[2].new_version, 5);
    assert_eq!(field_value(&result.events[2], "value").as_deref(), Some("taker-after"));
}

#[test]
fn execute_rejects_in_pre_check_before_loading_state() {
    let executor = CommandUseCaseExecutor4;
    let use_case = StubCreatedOnlyUseCase;
    let outbound =
        StubOutbound::with_state(StubState { reject_in_validate: false, reject_in_compute: false });
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope(""), &outbound, &observer);

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::Business(StubBusinessError::RejectedInPreCheck))
    );
    assert!(outbound.calls().is_empty());
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn execute_wraps_outbound_load_failure() {
    let executor = CommandUseCaseExecutor4;
    let use_case = StubCreatedOnlyUseCase;
    let outbound = StubOutbound::with_error(StubOutboundError::LoadFailed);
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("trader-1"), &outbound, &observer);

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

#[test]
fn updated_entity_pair_projects_before_after_diff() {
    let pair = UpdatedEntityPair {
        before: TestEntity { id: 91, value: "before".to_string(), version: 9 },
        after: TestEntity { id: 91, value: "after".to_string(), version: 10 },
    };

    let event = pair.after.track_update_event_from(&pair.before).unwrap();

    assert!(event.is_updated());
    assert_eq!(event.old_version, 9);
    assert_eq!(event.new_version, 10);
    assert_eq!(field_value(&event, "value").as_deref(), Some("after"));
}
