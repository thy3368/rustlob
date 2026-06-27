use std::fmt;
use std::sync::Mutex;

use cmd_handler::HandlerLatencyMetrics;
use cmd_handler::command_use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCase6, CommandUseCaseExecutionError,
    CommandUseCaseExecutor6, CommandUseCaseOutbound, CommandUseCaseOutboundPhase,
    CommandWithGivenState, EventProjectError, IssuedByParty, MainMiAuthoritativeTruth,
    MainMiChanges, MainMiStatefulChanges, ObserveHandlerLatency, ReplayableChanges,
    UpdatedEntityPair,
};
use common_entity::{Entity, EntityFieldChange, EntityReplayableEvent};

const TEST_ENTITY_TYPE: u8 = 29;

#[derive(Debug, Clone, PartialEq, Eq)]
enum StubCommand {
    Place { party_id: String, order_id: i64 },
    Cancel { party_id: String, order_id: i64 },
}

impl IssuedByParty for StubCommand {
    fn party_id(&self) -> Option<&str> {
        match self {
            Self::Place { party_id, .. } | Self::Cancel { party_id, .. } => Some(party_id.as_str()),
        }
    }
}

impl CommandWithGivenState for StubCommand {
    type GivenState = StubGivenState;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StubGivenState {
    Missing,
    Existing { order: OrderMi, reject_in_validate: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StubBusinessError {
    RejectedInPreCheck,
    RejectedInValidate,
    BranchMismatch { command_kind: &'static str, state_kind: &'static str },
}

impl fmt::Display for StubBusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RejectedInPreCheck => f.write_str("rejected in pre_check"),
            Self::RejectedInValidate => f.write_str("rejected in validate"),
            Self::BranchMismatch { command_kind, state_kind } => {
                write!(f, "branch mismatch: command={command_kind}, state={state_kind}")
            }
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
struct OrderMi {
    id: i64,
    status: String,
    version: u64,
}

impl Entity for OrderMi {
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
        if self.status != other.status {
            changes.push(EntityFieldChange::new("status", &self.status, &other.status));
        }
        changes
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![EntityFieldChange::new("status", "", &self.status)]
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "status" => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StubChanges {
    PlaceCreated { created_order: OrderMi },
    Cancelled { updated_order: UpdatedEntityPair<OrderMi> },
    MissingTruth { projected_order: OrderMi, command_kind: &'static str },
    MissingState { created_order: OrderMi, command_kind: &'static str },
    MissingCommandKind { created_order: OrderMi },
}

impl ReplayableChanges for StubChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        match self {
            Self::PlaceCreated { created_order }
            | Self::MissingState { created_order, .. }
            | Self::MissingCommandKind { created_order } => {
                Ok(vec![created_order.track_create_event()?])
            }
            Self::Cancelled { updated_order } => {
                Ok(vec![updated_order.after.track_update_event_from(&updated_order.before)?])
            }
            Self::MissingTruth { projected_order, .. } => {
                Ok(vec![projected_order.track_create_event()?])
            }
        }
    }
}

impl MainMiChanges for StubChanges {
    type MainMi = OrderMi;

    fn main_mi_truth(&self) -> Option<MainMiAuthoritativeTruth<'_, Self::MainMi>> {
        match self {
            Self::PlaceCreated { created_order }
            | Self::MissingState { created_order, .. }
            | Self::MissingCommandKind { created_order } => {
                Some(MainMiAuthoritativeTruth::Created(created_order))
            }
            Self::Cancelled { updated_order } => {
                Some(MainMiAuthoritativeTruth::Updated(updated_order))
            }
            Self::MissingTruth { .. } => None,
        }
    }
}

impl MainMiStatefulChanges for StubChanges {
    fn command_kind(&self) -> &'static str {
        match self {
            Self::PlaceCreated { .. } => "place",
            Self::Cancelled { .. } => "cancel",
            Self::MissingTruth { command_kind, .. } | Self::MissingState { command_kind, .. } => {
                command_kind
            }
            Self::MissingCommandKind { .. } => "",
        }
    }

    fn main_mi_current_state(&self) -> Option<&str> {
        match self {
            Self::PlaceCreated { created_order } | Self::MissingCommandKind { created_order } => {
                Some(created_order.status.as_str())
            }
            Self::Cancelled { updated_order } => Some(updated_order.after.status.as_str()),
            Self::MissingTruth { projected_order, .. } => Some(projected_order.status.as_str()),
            Self::MissingState { .. } => None,
        }
    }
}

#[derive(Debug, Clone)]
struct StubUseCase6 {
    main_mi_name: &'static str,
    main_mi_identity_field: &'static str,
    main_mi_state_field: &'static str,
    output_mode: StubOutputMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StubOutputMode {
    Normal,
    MissingTruth,
    MissingState,
    MissingCommandKind,
}

impl Default for StubUseCase6 {
    fn default() -> Self {
        Self {
            main_mi_name: "OrderMi",
            main_mi_identity_field: "order_id",
            main_mi_state_field: "status",
            output_mode: StubOutputMode::Normal,
        }
    }
}

impl CommandUseCase6 for StubUseCase6 {
    type Command = StubCommand;
    type Error = StubBusinessError;
    type Changes = StubChanges;

    fn main_mi_name(&self) -> &'static str {
        self.main_mi_name
    }

    fn main_mi_identity_field(&self) -> &'static str {
        self.main_mi_identity_field
    }

    fn main_mi_state_field(&self) -> &'static str {
        self.main_mi_state_field
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id().is_some_and(str::is_empty) {
            return Err(StubBusinessError::RejectedInPreCheck);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        state: &<Self::Command as CommandWithGivenState>::GivenState,
    ) -> Result<(), Self::Error> {
        if matches!(state, StubGivenState::Existing { reject_in_validate: true, .. }) {
            return Err(StubBusinessError::RejectedInValidate);
        }
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: <Self::Command as CommandWithGivenState>::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        match (cmd, state) {
            (StubCommand::Place { order_id, .. }, StubGivenState::Missing) => {
                let created_order =
                    OrderMi { id: *order_id, status: "open".to_string(), version: 1 };
                Ok(match self.output_mode {
                    StubOutputMode::Normal => StubChanges::PlaceCreated { created_order },
                    StubOutputMode::MissingTruth => StubChanges::MissingTruth {
                        projected_order: created_order,
                        command_kind: "place",
                    },
                    StubOutputMode::MissingState => {
                        StubChanges::MissingState { created_order, command_kind: "place" }
                    }
                    StubOutputMode::MissingCommandKind => {
                        StubChanges::MissingCommandKind { created_order }
                    }
                })
            }
            (StubCommand::Cancel { order_id, .. }, StubGivenState::Existing { order, .. }) => {
                let after = OrderMi {
                    id: *order_id,
                    status: "cancelled".to_string(),
                    version: order.version + 1,
                };
                Ok(match self.output_mode {
                    StubOutputMode::Normal => StubChanges::Cancelled {
                        updated_order: UpdatedEntityPair { before: order, after },
                    },
                    StubOutputMode::MissingTruth => {
                        StubChanges::MissingTruth { projected_order: after, command_kind: "cancel" }
                    }
                    StubOutputMode::MissingState => {
                        StubChanges::MissingState { created_order: after, command_kind: "cancel" }
                    }
                    StubOutputMode::MissingCommandKind => {
                        StubChanges::MissingCommandKind { created_order: after }
                    }
                })
            }
            (StubCommand::Place { .. }, StubGivenState::Existing { .. }) => {
                Err(StubBusinessError::BranchMismatch {
                    command_kind: "place",
                    state_kind: "existing",
                })
            }
            (StubCommand::Cancel { .. }, StubGivenState::Missing) => {
                Err(StubBusinessError::BranchMismatch {
                    command_kind: "cancel",
                    state_kind: "missing",
                })
            }
        }
    }
}

#[derive(Debug)]
struct StubOutbound {
    state: Mutex<Result<StubGivenState, StubOutboundError>>,
    calls: Mutex<Vec<&'static str>>,
}

impl StubOutbound {
    fn with_state(state: StubGivenState) -> Self {
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
    type State = StubGivenState;
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

fn sample_envelope(command: StubCommand) -> CommandEnvelope<StubCommand> {
    CommandEnvelope {
        meta: CommandMeta {
            trace_id: Some("trace-6".to_string()),
            command_id: Some("cmd-6".to_string()),
        },
        command,
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

fn sample_existing_state(order_id: i64) -> StubGivenState {
    StubGivenState::Existing {
        order: OrderMi { id: order_id, status: "open".to_string(), version: 4 },
        reject_in_validate: false,
    }
}

#[test]
fn use_case6_exposes_main_mi_metadata() {
    let use_case = StubUseCase6::default();

    assert_eq!(use_case.main_mi_name(), "OrderMi");
    assert_eq!(use_case.main_mi_identity_field(), "order_id");
    assert_eq!(use_case.main_mi_state_field(), "status");
}

#[test]
fn pre_check_command_rejects_empty_party_id() {
    let use_case = StubUseCase6::default();

    let result =
        use_case.pre_check_command(&StubCommand::Place { party_id: String::new(), order_id: 11 });

    assert_eq!(result, Err(StubBusinessError::RejectedInPreCheck));
}

#[test]
fn validate_against_state_rejects_invalid_state() {
    let use_case = StubUseCase6::default();

    let result = use_case.validate_against_state(
        &StubCommand::Cancel { party_id: "trader-1".to_string(), order_id: 11 },
        &StubGivenState::Existing {
            order: OrderMi { id: 11, status: "open".to_string(), version: 4 },
            reject_in_validate: true,
        },
    );

    assert_eq!(result, Err(StubBusinessError::RejectedInValidate));
}

#[test]
fn command_and_state_branches_match_on_place_happy_path() {
    let use_case = StubUseCase6::default();

    let changes = use_case
        .compute_changes(
            &StubCommand::Place { party_id: "trader-1".to_string(), order_id: 11 },
            StubGivenState::Missing,
        )
        .unwrap();

    assert_eq!(
        changes,
        StubChanges::PlaceCreated {
            created_order: OrderMi { id: 11, status: "open".to_string(), version: 1 }
        }
    );
}

#[test]
fn command_and_state_branches_mismatch_returns_business_error() {
    let use_case = StubUseCase6::default();

    let result = use_case.compute_changes(
        &StubCommand::Cancel { party_id: "trader-1".to_string(), order_id: 11 },
        StubGivenState::Missing,
    );

    assert_eq!(
        result,
        Err(StubBusinessError::BranchMismatch { command_kind: "cancel", state_kind: "missing" })
    );
}

#[test]
fn create_branch_exposes_truth_state_and_projects_events() {
    let use_case = StubUseCase6::default();
    let changes = use_case
        .compute_changes(
            &StubCommand::Place { party_id: "trader-1".to_string(), order_id: 11 },
            StubGivenState::Missing,
        )
        .unwrap();

    assert_eq!(changes.command_kind(), "place");
    assert_eq!(changes.main_mi_current_state(), Some("open"));
    assert_eq!(
        changes.main_mi_truth(),
        Some(MainMiAuthoritativeTruth::Created(&OrderMi {
            id: 11,
            status: "open".to_string(),
            version: 1,
        }))
    );

    let events = changes.to_replayable_events().unwrap();
    assert_eq!(events.len(), 1);
    assert!(events[0].is_created());
    assert_eq!(field_value(&events[0], "status").as_deref(), Some("open"));
}

#[test]
fn update_branch_exposes_truth_state_and_projects_events() {
    let use_case = StubUseCase6::default();
    let changes = use_case
        .compute_changes(
            &StubCommand::Cancel { party_id: "trader-1".to_string(), order_id: 11 },
            sample_existing_state(11),
        )
        .unwrap();

    assert_eq!(changes.command_kind(), "cancel");
    assert_eq!(changes.main_mi_current_state(), Some("cancelled"));
    match changes.main_mi_truth() {
        Some(MainMiAuthoritativeTruth::Updated(pair)) => {
            assert_eq!(pair.before.status, "open");
            assert_eq!(pair.after.status, "cancelled");
            assert_eq!(pair.before.version, 4);
            assert_eq!(pair.after.version, 5);
        }
        other => panic!("unexpected main MI truth: {other:?}"),
    }

    let events = changes.to_replayable_events().unwrap();
    assert_eq!(events.len(), 1);
    assert!(events[0].is_updated());
    assert_eq!(events[0].old_version, 4);
    assert_eq!(events[0].new_version, 5);
    assert_eq!(field_value(&events[0], "status").as_deref(), Some("cancelled"));
}

#[test]
fn executor6_execute_returns_changes_and_events_on_happy_path() {
    let executor = CommandUseCaseExecutor6;
    let use_case = StubUseCase6::default();
    let outbound = StubOutbound::with_state(StubGivenState::Missing);
    let observer = StubObserver::default();

    let result = executor
        .execute(
            &use_case,
            sample_envelope(StubCommand::Place { party_id: "trader-1".to_string(), order_id: 11 }),
            &outbound,
            &observer,
        )
        .unwrap();

    assert_eq!(
        result.changes,
        StubChanges::PlaceCreated {
            created_order: OrderMi { id: 11, status: "open".to_string(), version: 1 }
        }
    );
    assert_eq!(result.events.len(), 1);
    assert!(result.events[0].is_created());
    assert_eq!(outbound.calls(), vec!["load_state", "persist", "replay", "publish"]);
    assert_eq!(observer.observed_count(), 1);
    assert_eq!(observer.last().map(|metrics| metrics.domain_event_count), Some(1));
}

#[test]
fn executor6_execute_rejects_in_pre_check_before_loading_state() {
    let executor = CommandUseCaseExecutor6;
    let use_case = StubUseCase6::default();
    let outbound = StubOutbound::with_state(StubGivenState::Missing);
    let observer = StubObserver::default();

    let result = executor.execute(
        &use_case,
        sample_envelope(StubCommand::Place { party_id: String::new(), order_id: 11 }),
        &outbound,
        &observer,
    );

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::Business(StubBusinessError::RejectedInPreCheck))
    );
    assert!(outbound.calls().is_empty());
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn executor6_wraps_outbound_load_failure() {
    let executor = CommandUseCaseExecutor6;
    let use_case = StubUseCase6::default();
    let outbound = StubOutbound::with_error(StubOutboundError::LoadFailed);
    let observer = StubObserver::default();

    let result = executor.execute(
        &use_case,
        sample_envelope(StubCommand::Place { party_id: "trader-1".to_string(), order_id: 11 }),
        &outbound,
        &observer,
    );

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
fn executor6_rejects_missing_main_mi_truth_even_if_events_can_be_projected() {
    let executor = CommandUseCaseExecutor6;
    let use_case =
        StubUseCase6 { output_mode: StubOutputMode::MissingTruth, ..StubUseCase6::default() };
    let outbound = StubOutbound::with_state(StubGivenState::Missing);
    let observer = StubObserver::default();

    let result = executor.execute(
        &use_case,
        sample_envelope(StubCommand::Place { party_id: "trader-1".to_string(), order_id: 11 }),
        &outbound,
        &observer,
    );

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::EventProject(EventProjectError::Custom(
            "missing authoritative main MI truth for OrderMi (order_id)".to_string(),
        )))
    );
    assert_eq!(outbound.calls(), vec!["load_state"]);
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn executor6_rejects_missing_state_field_metadata() {
    let executor = CommandUseCaseExecutor6;
    let use_case = StubUseCase6 { main_mi_state_field: "", ..StubUseCase6::default() };
    let outbound = StubOutbound::with_state(StubGivenState::Missing);
    let observer = StubObserver::default();

    let result = executor.execute(
        &use_case,
        sample_envelope(StubCommand::Place { party_id: "trader-1".to_string(), order_id: 11 }),
        &outbound,
        &observer,
    );

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::EventProject(EventProjectError::Custom(
            "CommandUseCase6 main_mi_state_field must not be empty".to_string(),
        )))
    );
    assert_eq!(outbound.calls(), vec!["load_state"]);
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn executor6_rejects_missing_current_state() {
    let executor = CommandUseCaseExecutor6;
    let use_case =
        StubUseCase6 { output_mode: StubOutputMode::MissingState, ..StubUseCase6::default() };
    let outbound = StubOutbound::with_state(StubGivenState::Missing);
    let observer = StubObserver::default();

    let result = executor.execute(
        &use_case,
        sample_envelope(StubCommand::Place { party_id: "trader-1".to_string(), order_id: 11 }),
        &outbound,
        &observer,
    );

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::EventProject(EventProjectError::Custom(
            "missing main MI current state for OrderMi.status on command place".to_string(),
        )))
    );
    assert_eq!(outbound.calls(), vec!["load_state"]);
    assert_eq!(observer.observed_count(), 0);
}

#[test]
fn executor6_rejects_missing_command_kind() {
    let executor = CommandUseCaseExecutor6;
    let use_case =
        StubUseCase6 { output_mode: StubOutputMode::MissingCommandKind, ..StubUseCase6::default() };
    let outbound = StubOutbound::with_state(StubGivenState::Missing);
    let observer = StubObserver::default();

    let result = executor.execute(
        &use_case,
        sample_envelope(StubCommand::Place { party_id: "trader-1".to_string(), order_id: 11 }),
        &outbound,
        &observer,
    );

    assert_eq!(
        result,
        Err(CommandUseCaseExecutionError::EventProject(EventProjectError::Custom(
            "missing command_kind for OrderMi (status)".to_string(),
        )))
    );
    assert_eq!(outbound.calls(), vec!["load_state"]);
    assert_eq!(observer.observed_count(), 0);
}
