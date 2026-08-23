use cmd_handler::use_case_def2::{CommandUseCase2, IssuedByParty};
use cmd_handler::{EntityReplayableEvent, ReplayFieldChange};
use kani::Arbitrary;

const WITHDRAWAL_ENTITY_TYPE: u8 = 11;
const FIELD_TYPE_STRING: u8 = 0;
const FIELD_TYPE_INT: u8 = 1;

fn string_field(name: &str, value: &str) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.as_bytes(),
        FIELD_TYPE_STRING,
    )
}

fn int_field(name: &str, value: u64) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.to_string().as_bytes(),
        FIELD_TYPE_INT,
    )
}

fn event_field<'a>(event: &'a EntityReplayableEvent, name: &str) -> Option<&'a str> {
    event.field_changes.iter().find_map(|change| {
        let field_name = change.field_name_as_str().ok()?;
        if field_name != name {
            return None;
        }
        std::str::from_utf8(change.new_value_bytes()).ok()
    })
}

fn event_u64_field(event: &EntityReplayableEvent, name: &str) -> Option<u64> {
    event_field(event, name)?.parse().ok()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Arbitrary)]
struct WithdrawCmd {
    party_suffix: u8,
    amount: u64,
}

impl IssuedByParty for WithdrawCmd {
    fn party_id(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Arbitrary)]
struct WithdrawState {
    available: u64,
    account_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WithdrawError {
    ZeroAmount,
    AccountClosed,
    InsufficientFunds,
}

#[derive(Debug, Clone, Copy, Default)]
struct WithdrawUseCase;

impl CommandUseCase2 for WithdrawUseCase {
    type Command = WithdrawCmd;
    type GivenState = WithdrawState;
    type Error = WithdrawError;

    fn role(&self) -> &'static str {
        "WalletOwner"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.amount == 0 {
            return Err(WithdrawError::ZeroAmount);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if !state.account_open {
            return Err(WithdrawError::AccountClosed);
        }
        if cmd.amount > state.available {
            return Err(WithdrawError::InsufficientFunds);
        }
        Ok(())
    }

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        _state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let mut event = EntityReplayableEvent::new_created(
            0,
            0,
            i64::from(cmd.party_suffix) + 1,
            WITHDRAWAL_ENTITY_TYPE,
        );
        event.add_field_change(string_field("party_suffix", &cmd.party_suffix.to_string()));
        event.add_field_change(int_field("amount", cmd.amount));
        Ok(vec![event])
    }
}

#[kani::proof]
fn prove_withdraw_use_case_emits_amount_consistent_event() {
    let cmd: WithdrawCmd = kani::any();
    let state: WithdrawState = kani::any();
    let use_case = WithdrawUseCase;

    kani::assume(cmd.amount > 0);
    kani::assume(state.account_open);
    kani::assume(cmd.amount <= state.available);

    assert_eq!(use_case.pre_check_command(&cmd), Ok(()));
    assert_eq!(use_case.validate_against_state(&cmd, &state), Ok(()));

    let events = use_case.compute_replayable_events(&cmd, state).unwrap();
    assert_eq!(events.len(), 1);

    let event = &events[0];
    assert_eq!(event.entity_type, WITHDRAWAL_ENTITY_TYPE);
    assert_eq!(event_u64_field(event, "amount"), Some(cmd.amount));
    assert_eq!(event_u64_field(event, "party_suffix"), Some(u64::from(cmd.party_suffix)));
}

#[kani::proof]
fn prove_withdraw_use_case_rejects_zero_amount_before_state_matters() {
    let cmd = WithdrawCmd {
        party_suffix: kani::any(),
        amount: 0,
    };
    let state: WithdrawState = kani::any();
    let use_case = WithdrawUseCase;

    assert_eq!(use_case.pre_check_command(&cmd), Err(WithdrawError::ZeroAmount));
    let _ = state;
}
