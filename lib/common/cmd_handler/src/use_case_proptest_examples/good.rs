use std::sync::atomic::{AtomicUsize, Ordering};

use proptest::prelude::*;
use thiserror::Error;

use crate::use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCase2, CommandUseCaseExecutionError,
    CommandUseCaseExecutor2, CommandUseCaseOutbound, IssuedByParty,
};
use crate::{EntityReplayableEvent, ReplayFieldChange};

const DEPOSIT_ENTITY_TYPE: u8 = 3;
const FIELD_TYPE_STRING: u8 = 0;
const FIELD_TYPE_INT: u8 = 1;

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

// Good proptest example:
// - properties express business invariants, not syntax trivia
// - generated input space covers both success and rejection paths
// - assertions check side effects and executor behavior, not just returned values
// - the test proves pipeline methods do not run when validation fails

#[derive(Debug, Clone, PartialEq, Eq)]
struct DepositCmd {
    party_id: String,
    amount: u64,
}

impl IssuedByParty for DepositCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
enum DepositError {
    #[error("amount must be greater than zero")]
    ZeroAmount,
    #[error("account is frozen")]
    AccountFrozen,
    #[error("amount exceeds current limit")]
    LimitExceeded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DepositState {
    account_open: bool,
    max_amount: u64,
}

#[derive(Debug, Clone, Copy, Default)]
struct DepositUseCase;

impl CommandUseCase2 for DepositUseCase {
    type Command = DepositCmd;
    type GivenState = DepositState;
    type Error = DepositError;

    fn role(&self) -> &'static str {
        "WalletOwner"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.amount == 0 {
            return Err(DepositError::ZeroAmount);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if !state.account_open {
            return Err(DepositError::AccountFrozen);
        }
        if cmd.amount > state.max_amount {
            return Err(DepositError::LimitExceeded);
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
            stable_entity_id(&cmd.party_id),
            DEPOSIT_ENTITY_TYPE,
        );
        event.add_field_change(string_field("party_id", &cmd.party_id));
        event.add_field_change(int_field("amount", cmd.amount));
        Ok(vec![event])
    }
}

#[derive(Debug)]
struct CountingOutbound {
    state: DepositState,
    persist_calls: AtomicUsize,
    replay_calls: AtomicUsize,
    publish_calls: AtomicUsize,
}

impl CommandUseCaseOutbound for CountingOutbound {
    type Command = DepositCmd;
    type State = DepositState;
    type Error = DepositError;

    fn load_state(&self, _cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        Ok(self.state.clone())
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

fn deposit_case_strategy() -> impl Strategy<Value = (DepositCmd, DepositState)> {
    (any::<u16>(), any::<u64>(), any::<bool>(), any::<u64>()).prop_map(
        |(party_suffix, amount, account_open, max_amount)| {
            (
                DepositCmd { party_id: format!("acct-{party_suffix}"), amount },
                DepositState { account_open, max_amount },
            )
        },
    )
}

proptest! {
    #[test]
    fn property_executor_respects_business_invariants(
        (cmd, state) in deposit_case_strategy(),
    ) {
        let executor = CommandUseCaseExecutor2;
        let use_case = DepositUseCase;
        let outbound = CountingOutbound {
            state: state.clone(),
            persist_calls: AtomicUsize::new(0),
            replay_calls: AtomicUsize::new(0),
            publish_calls: AtomicUsize::new(0),
        };

        let result = executor.execute(
            &use_case,
            CommandEnvelope {
                meta: CommandMeta::default(),
                command: cmd.clone(),
            },
            &outbound,
            &(),
        );

        if cmd.amount == 0 {
            prop_assert_eq!(result, Err(CommandUseCaseExecutionError::Business(DepositError::ZeroAmount)));
            prop_assert_eq!(outbound.persist_calls.load(Ordering::Relaxed), 0);
            prop_assert_eq!(outbound.replay_calls.load(Ordering::Relaxed), 0);
            prop_assert_eq!(outbound.publish_calls.load(Ordering::Relaxed), 0);
        } else if !state.account_open {
            prop_assert_eq!(result, Err(CommandUseCaseExecutionError::Business(DepositError::AccountFrozen)));
            prop_assert_eq!(outbound.persist_calls.load(Ordering::Relaxed), 0);
            prop_assert_eq!(outbound.replay_calls.load(Ordering::Relaxed), 0);
            prop_assert_eq!(outbound.publish_calls.load(Ordering::Relaxed), 0);
        } else if cmd.amount > state.max_amount {
            prop_assert_eq!(result, Err(CommandUseCaseExecutionError::Business(DepositError::LimitExceeded)));
            prop_assert_eq!(outbound.persist_calls.load(Ordering::Relaxed), 0);
            prop_assert_eq!(outbound.replay_calls.load(Ordering::Relaxed), 0);
            prop_assert_eq!(outbound.publish_calls.load(Ordering::Relaxed), 0);
        } else {
            let events = result.unwrap();
            let expected_amount = cmd.amount.to_string();
            prop_assert_eq!(events.len(), 1);
            prop_assert_eq!(event_field(&events[0], "party_id"), Some(cmd.party_id.as_str()));
            prop_assert_eq!(event_field(&events[0], "amount"), Some(expected_amount.as_str()));
            prop_assert_eq!(outbound.persist_calls.load(Ordering::Relaxed), 1);
            prop_assert_eq!(outbound.replay_calls.load(Ordering::Relaxed), 1);
            prop_assert_eq!(outbound.publish_calls.load(Ordering::Relaxed), 1);
        }
    }
}
