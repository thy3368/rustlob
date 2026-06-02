use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::CommandUseCaseOutbound;
use example_core::{ACCOUNT_ENTITY_TYPE, DepositQuoteCmd, DepositQuoteState};

use crate::shared::{
    DepositQuoteOutboundError, InMemoryStore, event_string_field, event_u64_field,
};

#[derive(Debug, Clone, Default)]
pub struct InMemoryDepositQuoteOutbound {
    store: InMemoryStore,
}

impl InMemoryDepositQuoteOutbound {
    pub fn from_store(store: InMemoryStore) -> Self {
        Self { store }
    }
}

impl CommandUseCaseOutbound for InMemoryDepositQuoteOutbound {
    type Command = DepositQuoteCmd;
    type State = DepositQuoteState;
    type Error = DepositQuoteOutboundError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        let state = self.store.lock_state()?;
        let account = state
            .accounts
            .get(cmd.party_id.as_str())
            .cloned()
            .ok_or(DepositQuoteOutboundError::AccountNotFound)?;

        Ok(DepositQuoteState { account })
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.persisted_events.extend(events.iter().cloned());
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;

        for event in events {
            if event.entity_type == ACCOUNT_ENTITY_TYPE && event.is_updated() {
                let account_id = event_string_field(event, "account_id")
                    .ok_or(DepositQuoteOutboundError::EventDecodeFailed)?;
                let account = state
                    .accounts
                    .get_mut(account_id.as_str())
                    .ok_or(DepositQuoteOutboundError::AccountNotFound)?;
                account.available_quote = event_u64_field(event, "available_quote")
                    .ok_or(DepositQuoteOutboundError::EventDecodeFailed)?;
                account.frozen_quote = event_u64_field(event, "frozen_quote")
                    .ok_or(DepositQuoteOutboundError::EventDecodeFailed)?;
                account.version = event.new_version;
            }
        }

        Ok(())
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.published_events.extend(events.iter().cloned());
        Ok(())
    }
}
