use cmd_handler::use_case_def2::{CommandUseCase2, IssuedByParty};
use cmd_handler::{EntityReplayableEvent, ReplayFieldChange};
use kani::Arbitrary;

const SUBMIT_ENTITY_TYPE: u8 = 12;
const FIELD_TYPE_BOOL: u8 = 3;

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

// Bad kani example:
// - role is technical, not a business-game role
// - command carries trace_id instead of business identity
// - output is copied from state, so the proof is close to tautological
// - the proof only shows a prepared answer stays the same after copying

#[derive(Debug, Clone, Copy, PartialEq, Eq, Arbitrary)]
struct SubmitCmd {
    trace_id: u64,
}

impl IssuedByParty for SubmitCmd {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Arbitrary)]
struct SubmitState {
    accepted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SubmitError {
    Rejected,
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
            i64::from(state.accepted as u8) + 1,
            SUBMIT_ENTITY_TYPE,
        );
        event.add_field_change(bool_field("accepted", state.accepted));
        Ok(vec![event])
    }
}

#[kani::proof]
fn weak_proof_only_repeats_precomputed_state_answer() {
    let cmd: SubmitCmd = kani::any();
    let state: SubmitState = kani::any();
    let use_case = OrderCheckingEngineUseCase;

    if state.accepted {
        let events = use_case.compute_replayable_events(&cmd, state).unwrap();
        assert_eq!(event_accepted(&events), Some(true));
    }
}
