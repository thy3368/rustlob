use proptest::prelude::*;
use thiserror::Error;

use crate::command_use_case_def2::{CommandUseCase2, IssuedByParty};
use crate::{EntityReplayableEvent, ReplayFieldChange};

const SUBMIT_ENTITY_TYPE: u8 = 4;
const FIELD_TYPE_BOOL: u8 = 3;

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn bool_field(name: &str, value: bool) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.to_string().as_bytes(),
        FIELD_TYPE_BOOL,
    )
}

fn event_accepted(events: &[EntityReplayableEvent]) -> Option<bool> {
    let event = events.first()?;
    let change = event
        .field_changes
        .iter()
        .find(|change| change.field_name_as_str().ok() == Some("accepted"))?;
    Some(std::str::from_utf8(change.new_value_bytes()).ok()? == "true")
}

// Bad proptest example:
// - properties restate precomputed answers instead of testing business invariants
// - command identity is confused with trace identity
// - the use case copies output from state, so the property test proves almost nothing
// - assertions are tautological and do not guard behavior regressions

#[derive(Debug, Clone, PartialEq, Eq)]
struct SubmitCmd {
    trace_id: String,
}

impl IssuedByParty for SubmitCmd {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
enum SubmitError {
    #[error("rejected")]
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SubmitState {
    accepted: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct OrderCheckingEngineUseCase;

impl CommandUseCase2 for OrderCheckingEngineUseCase {
    type Command = SubmitCmd;
    type GivenState = SubmitState;
    type Error = SubmitError;

    fn role(&self) -> &'static str {
        "OrderCheckingEngine"
    }

    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.accepted { Ok(()) } else { Err(SubmitError::Rejected) }
    }

    fn compute_replayable_events(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let mut event = EntityReplayableEvent::new_created(
            0,
            0,
            stable_entity_id(&state.accepted.to_string()),
            SUBMIT_ENTITY_TYPE,
        );
        event.add_field_change(bool_field("accepted", state.accepted));
        Ok(vec![event])
    }
}

fn bad_case_strategy() -> impl Strategy<Value = (SubmitCmd, SubmitState)> {
    any::<bool>().prop_map(|accepted| {
        (SubmitCmd { trace_id: format!("trace-{accepted}") }, SubmitState { accepted })
    })
}

proptest! {
    #[test]
    fn property_only_repeats_precomputed_state_answers(
        (cmd, state) in bad_case_strategy(),
    ) {
        let use_case = OrderCheckingEngineUseCase;

        if state.accepted {
            let events = use_case.compute_replayable_events(&cmd, state.clone()).unwrap();
            prop_assert_eq!(event_accepted(&events), Some(state.accepted));
        } else {
            prop_assert_eq!(use_case.validate_against_state(&cmd, &state), Err(SubmitError::Rejected));
        }
    }
}
