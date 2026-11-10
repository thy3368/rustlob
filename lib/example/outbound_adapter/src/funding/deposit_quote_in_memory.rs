use common_entity::{EntityReplayableEvent, LogEntry, StateDiff, StateSink, StateSource};
use example_core_use_case::{BalanceV2, DepositQuoteCmd, DepositQuoteState, DepositQuoteUseCase};

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
                let coin_idx = event_identity_field(event, "coin_idx")
                    .ok_or(DepositQuoteOutboundError::EventDecodeFailed)?;
                let key = balance_key(account_id, coin_idx);
                let balance = state.balances.entry(key).or_insert_with(|| {
                    BalanceV2::new(account_id, account_id, coin_idx, 0, 0, 0, 0)
                });
                let available = event_u64_field(event, "available")
                    .map(rust_decimal::Decimal::from)
                    .unwrap_or_else(|| balance.available());
                let frozen = event_u64_field(event, "frozen")
                    .map(rust_decimal::Decimal::from)
                    .unwrap_or_else(|| balance.frozen());
                *balance = BalanceV2::new(
                    balance.party_id(),
                    balance.account_id(),
                    balance.coin_idx(),
                    available,
                    frozen,
                    event.lsn,
                    balance.created_at(),
                );
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
