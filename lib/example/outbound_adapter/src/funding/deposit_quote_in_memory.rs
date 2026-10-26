use common_entity::{EntityReplayableEvent, LogEntry, StateMachineV2Diff, StateSink, StateSource};
use example_core_use_case::{Balance, DepositQuoteCmd, DepositQuoteState, DepositQuoteUseCase};

use crate::shared::{
    DepositQuoteOutboundError, InMemoryStore, balance_key, event_identity_field, event_u64_field,
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

impl StateSource<DepositQuoteUseCase> for InMemoryDepositQuoteOutbound {
    type Error = DepositQuoteOutboundError;

    fn load_given_state(
        &self,
        cmd: &LogEntry<DepositQuoteCmd>,
    ) -> Result<DepositQuoteState, Self::Error> {
        let state = self.store.lock_state()?;
        let quote_balance = state
            .balances
            .get(&balance_key(cmd.party_id.as_str(), "USDT"))
            .cloned()
            .ok_or(DepositQuoteOutboundError::BalanceNotFound)?;

        Ok(DepositQuoteState { quote_balance })
    }
}

impl StateSink<DepositQuoteUseCase> for InMemoryDepositQuoteOutbound {
    type Error = DepositQuoteOutboundError;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.persisted_events.extend(events.iter().cloned());
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;

        for event in events {
            if event.is_updated() {
                let account_id = event_identity_field(event, "account_id")
                    .ok_or(DepositQuoteOutboundError::EventDecodeFailed)?;
                let asset_id = event_identity_field(event, "asset_id")
                    .ok_or(DepositQuoteOutboundError::EventDecodeFailed)?;
                let key = balance_key(account_id, asset_id);
                let balance = state
                    .balances
                    .entry(key)
                    .or_insert_with(|| Balance::new(account_id, account_id, asset_id, 0, 0, 0));
                if let Some(available) = event_u64_field(event, "available") {
                    balance.available = available;
                }
                if let Some(frozen) = event_u64_field(event, "frozen") {
                    balance.frozen = frozen;
                }
                balance.version = event.lsn;
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
