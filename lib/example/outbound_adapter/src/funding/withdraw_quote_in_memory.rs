use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCaseOutbound;
use example_core::{Balance, WithdrawQuoteCmd, WithdrawQuoteState};

use crate::shared::{
    InMemoryStore, WithdrawQuoteOutboundError, balance_key, event_string_field, event_u64_field,
};

#[derive(Debug, Clone, Default)]
pub struct InMemoryWithdrawQuoteOutbound {
    store: InMemoryStore,
}

impl InMemoryWithdrawQuoteOutbound {
    pub fn from_store(store: InMemoryStore) -> Self {
        Self { store }
    }
}

impl CommandUseCaseOutbound for InMemoryWithdrawQuoteOutbound {
    type Command = WithdrawQuoteCmd;
    type State = WithdrawQuoteState;
    type Error = WithdrawQuoteOutboundError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        let state = self.store.lock_state()?;
        let quote_balance = state
            .balances
            .get(&balance_key(cmd.party_id.as_str(), "USDT"))
            .cloned()
            .ok_or(WithdrawQuoteOutboundError::BalanceNotFound)?;

        Ok(WithdrawQuoteState { quote_balance })
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.persisted_events.extend(events.iter().cloned());
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;

        for event in events {
            if event.is_updated() {
                let account_id = event_string_field(event, "account_id")
                    .ok_or(WithdrawQuoteOutboundError::EventDecodeFailed)?;
                let asset_id = event_string_field(event, "asset_id")
                    .ok_or(WithdrawQuoteOutboundError::EventDecodeFailed)?;
                let key = balance_key(account_id.as_str(), asset_id.as_str());
                let balance = state
                    .balances
                    .entry(key)
                    .or_insert_with(|| Balance::new(account_id.clone(), asset_id.clone(), 0, 0, 0));
                if let Some(available) = event_u64_field(event, "available") {
                    balance.available = available;
                }
                if let Some(frozen) = event_u64_field(event, "frozen") {
                    balance.frozen = frozen;
                }
                balance.version = event.new_version;
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
