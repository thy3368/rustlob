use std::fmt;
use std::sync::Mutex;

use cmd_handler::HandlerLatencyMetrics;
use cmd_handler::command_use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCase5, CommandUseCaseExecutionError,
    CommandUseCaseExecutor5, CommandUseCaseOutbound, CommandUseCaseOutboundPhase,
    EventProjectError, IssuedByParty, MainMiAuthoritativeTruth, MainMiChanges,
    ObserveHandlerLatency, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::{Entity, EntityFieldChange, EntityReplayableEvent};

const TEST_ENTITY_TYPE: u8 = 17;

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
struct MainMiEntity {
    id: i64,
    value: String,
    version: u64,
}

impl Entity for MainMiEntity {
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
struct CreatedMainMiChanges {
    created_order: MainMiEntity,
}

impl ReplayableChanges for CreatedMainMiChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        Ok(vec![self.created_order.track_create_event()?])
    }
}

impl MainMiChanges for CreatedMainMiChanges {
    type MainMi = MainMiEntity;

    fn main_mi_truth(&self) -> Option<MainMiAuthoritativeTruth<'_, Self::MainMi>> {
        Some(MainMiAuthoritativeTruth::Created(&self.created_order))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UpdatedMainMiChanges {
    updated_order: UpdatedEntityPair<MainMiEntity>,
}

impl ReplayableChanges for UpdatedMainMiChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        Ok(vec![self.updated_order.after.track_update_event_from(&self.updated_order.before)?])
    }
}

impl MainMiChanges for UpdatedMainMiChanges {
    type MainMi = MainMiEntity;

    fn main_mi_truth(&self) -> Option<MainMiAuthoritativeTruth<'_, Self::MainMi>> {
        Some(MainMiAuthoritativeTruth::Updated(&self.updated_order))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MissingMainMiTruthChanges {
    projected_order: MainMiEntity,
}

impl ReplayableChanges for MissingMainMiTruthChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        Ok(vec![self.projected_order.track_create_event()?])
    }
}

impl MainMiChanges for MissingMainMiTruthChanges {
    type MainMi = MainMiEntity;

    fn main_mi_truth(&self) -> Option<MainMiAuthoritativeTruth<'_, Self::MainMi>> {
        None
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct StubCreatedMainMiUseCase;

impl CommandUseCase5 for StubCreatedMainMiUseCase {
    type Command = StubCommand;
    type GivenState = StubState;
    type Error = StubBusinessError;
    type Changes = CreatedMainMiChanges;

    fn main_mi_name(&self) -> &'static str {
        "OrderMi"
    }

    fn main_mi_identity_field(&self) -> &'static str {
        "order_id"
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
        Ok(CreatedMainMiChanges {
            created_order: MainMiEntity { id: 11, value: "created".to_string(), version: 1 },
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct StubUpdatedMainMiUseCase;

impl CommandUseCase5 for StubUpdatedMainMiUseCase {
    type Command = StubCommand;
    type GivenState = StubState;
    type Error = StubBusinessError;
    type Changes = UpdatedMainMiChanges;

    fn main_mi_name(&self) -> &'static str {
        "OrderMi"
    }

    fn main_mi_identity_field(&self) -> &'static str {
        "order_id"
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
        Ok(UpdatedMainMiChanges {
            updated_order: UpdatedEntityPair {
                before: MainMiEntity { id: 21, value: "before".to_string(), version: 3 },
                after: MainMiEntity { id: 21, value: "after".to_string(), version: 4 },
            },
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct StubMissingMainMiTruthUseCase;

impl CommandUseCase5 for StubMissingMainMiTruthUseCase {
    type Command = StubCommand;
    type GivenState = StubState;
    type Error = StubBusinessError;
    type Changes = MissingMainMiTruthChanges;

    fn main_mi_name(&self) -> &'static str {
        "OrderMi"
    }

    fn main_mi_identity_field(&self) -> &'static str {
        "order_id"
    }

    fn compute_changes(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        if state.reject_in_compute {
            return Err(StubBusinessError::RejectedInCompute);
        }
        Ok(MissingMainMiTruthChanges {
            projected_order: MainMiEntity { id: 31, value: "projected".to_string(), version: 1 },
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
            trace_id: Some("trace-5".to_string()),
            command_id: Some("cmd-5".to_string()),
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
fn use_case5_exposes_main_mi_metadata() {
    let use_case = StubCreatedMainMiUseCase;

    assert_eq!(use_case.main_mi_name(), "OrderMi");
    assert_eq!(use_case.main_mi_identity_field(), "order_id");
}

#[test]
fn pre_check_command_rejects_empty_party_id() {
    let use_case = StubCreatedMainMiUseCase;

    let result = use_case.pre_check_command(&StubCommand { party_id: String::new() });

    assert_eq!(result, Err(StubBusinessError::RejectedInPreCheck));
}

#[test]
fn validate_against_state_rejects_invalid_state() {
    let use_case = StubCreatedMainMiUseCase;

    let result = use_case.validate_against_state(
        &StubCommand { party_id: "trader-1".to_string() },
        &StubState { reject_in_validate: true, reject_in_compute: false },
    );

    assert_eq!(result, Err(StubBusinessError::RejectedInValidate));
}

#[test]
fn created_main_mi_truth_is_explicit_and_projects_to_events() {
    let use_case = StubCreatedMainMiUseCase;
    let changes = use_case
        .compute_changes(
            &StubCommand { party_id: "trader-1".to_string() },
            StubState { reject_in_validate: false, reject_in_compute: false },
        )
        .unwrap();

    assert_eq!(
        changes.main_mi_truth(),
        Some(MainMiAuthoritativeTruth::Created(&MainMiEntity {
            id: 11,
            value: "created".to_string(),
            version: 1,
        }))
    );

    let events = changes.to_replayable_events().unwrap();
    assert_eq!(events.len(), 1);
    assert!(events[0].is_created());
    assert_eq!(field_value(&events[0], "value").as_deref(), Some("created"));
}

#[test]
fn updated_main_mi_truth_uses_updated_entity_pair_and_projects_to_events() {
    let use_case = StubUpdatedMainMiUseCase;
    let changes = use_case
        .compute_changes(
            &StubCommand { party_id: "trader-1".to_string() },
            StubState { reject_in_validate: false, reject_in_compute: false },
        )
        .unwrap();

    match changes.main_mi_truth() {
        Some(MainMiAuthoritativeTruth::Updated(pair)) => {
            assert_eq!(pair.before.value, "before");
            assert_eq!(pair.after.value, "after");
            assert_eq!(pair.before.version, 3);
            assert_eq!(pair.after.version, 4);
        }
        other => panic!("unexpected main MI truth: {other:?}"),
    }

    let events = changes.to_replayable_events().unwrap();
    assert_eq!(events.len(), 1);
    assert!(events[0].is_updated());
    assert_eq!(events[0].old_version, 3);
    assert_eq!(events[0].new_version, 4);
    assert_eq!(field_value(&events[0], "value").as_deref(), Some("after"));
}

#[test]
fn executor5_execute_returns_changes_and_events_on_happy_path() {
    let executor = CommandUseCaseExecutor5;
    let use_case = StubCreatedMainMiUseCase;
    let outbound =
        StubOutbound::with_state(StubState { reject_in_validate: false, reject_in_compute: false });
    let observer = StubObserver::default();

    let result =
        executor.execute(&use_case, sample_envelope("trader-1"), &outbound, &observer).unwrap();

    assert_eq!(
        result.changes,
        CreatedMainMiChanges {
            created_order: MainMiEntity { id: 11, value: "created".to_string(), version: 1 },
        }
    );
    assert_eq!(result.events.len(), 1);
    assert!(result.events[0].is_created());
    assert_eq!(outbound.calls(), vec!["load_state", "persist", "replay", "publish"]);
    assert_eq!(observer.observed_count(), 1);
    assert_eq!(observer.last().map(|metrics| metrics.domain_event_count), Some(1));
}

#[test]
fn executor5_execute_rejects_in_pre_check_before_loading_state() {
    let executor = CommandUseCaseExecutor5;
    let use_case = StubCreatedMainMiUseCase;
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
fn executor5_wraps_outbound_load_failure() {
    let executor = CommandUseCaseExecutor5;
    let use_case = StubCreatedMainMiUseCase;
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
fn executor5_rejects_missing_main_mi_truth_even_if_events_can_be_projected() {
    let executor = CommandUseCaseExecutor5;
    let use_case = StubMissingMainMiTruthUseCase;
    let outbound =
        StubOutbound::with_state(StubState { reject_in_validate: false, reject_in_compute: false });
    let observer = StubObserver::default();

    let result = executor.execute(&use_case, sample_envelope("trader-1"), &outbound, &observer);

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::EventProject(EventProjectError::Custom(
            "missing authoritative main MI truth for OrderMi (order_id)".to_string(),
        )))
    );
    assert_eq!(outbound.calls(), vec!["load_state"]);
    assert_eq!(observer.observed_count(), 0);
}
