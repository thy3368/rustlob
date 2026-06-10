use thiserror::Error;

use crate::command_use_case_def2::{CommandUseCase2, IssuedByParty};
use crate::{EntityReplayableEvent, ReplayFieldChange};

const SUBMIT_ENTITY_TYPE: u8 = 2;
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

// Shared bad example for both writing and reviewing use cases:
// - `role()` uses a technical component name, so four-color Role clarity is weak.
// - `party_id` is missing even though a business party is clearly issuing the command.
// - `trace_id` is placed on the business command, which encourages identity confusion.
// - `GivenState` already contains the decision result and final event, so the use case
//   does very little business work and mostly copies a precomputed answer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitCmd {
    pub trace_id: String,
    pub symbol: String,
}

impl IssuedByParty for SubmitCmd {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SubmitError {
    #[error("rejected")]
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitState {
    pub accepted: bool,
    pub generated_status: &'static str,
}

pub struct OrderCheckingEngineUseCase;

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
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let mut event = EntityReplayableEvent::new_created(
            0,
            0,
            stable_entity_id(&cmd.trace_id),
            SUBMIT_ENTITY_TYPE,
        );
        event.add_field_change(string_field("status", state.generated_status));
        Ok(vec![event])
    }
}
