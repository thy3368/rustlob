use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::CommandUseCaseOutbound;
use example_core::{
    ACCOUNT_ENTITY_TYPE, MarketRules, ORDER_ENTITY_TYPE, PlaceOrderCmd, PlaceOrderState,
    StoredOrder, TradingAccount,
};

use crate::shared::{
    InMemoryStore, PlaceOrderOutboundError, StoreSnapshot, event_order_sequence,
    event_string_field, event_u64_field,
};

#[derive(Debug, Clone, Default)]
pub struct InMemoryPlaceOrderOutbound {
    store: InMemoryStore,
}

impl InMemoryPlaceOrderOutbound {
    pub fn new() -> Self {
        Self::from_store(InMemoryStore::new())
    }

    pub fn from_store(store: InMemoryStore) -> Self {
        Self { store }
    }

    pub fn seeded(
        account: TradingAccount,
        market_rules: MarketRules,
    ) -> Result<Self, crate::StoreError> {
        Ok(Self::from_store(InMemoryStore::seeded(account, market_rules)?))
    }

    pub fn seed_account(&self, account: TradingAccount) -> Result<(), crate::StoreError> {
        self.store.seed_account(account)
    }

    pub fn seed_market_rules(&self, market_rules: MarketRules) -> Result<(), crate::StoreError> {
        self.store.seed_market_rules(market_rules)
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, crate::StoreError> {
        self.store.snapshot()
    }
}

impl CommandUseCaseOutbound for InMemoryPlaceOrderOutbound {
    type Command = PlaceOrderCmd;
    type State = PlaceOrderState;
    type Error = PlaceOrderOutboundError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        let state = self.store.lock_state()?;
        let account = state
            .accounts
            .get(cmd.party_id.as_str())
            .cloned()
            .ok_or(PlaceOrderOutboundError::AccountNotFound)?;
        let market_rules = state
            .market_rules_by_symbol
            .get(cmd.symbol.as_str())
            .cloned()
            .ok_or(PlaceOrderOutboundError::MarketRulesNotFound)?;

        Ok(PlaceOrderState {
            trading_enabled: true,
            next_order_sequence: state.next_order_sequence,
            account,
            market_rules,
        })
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.persisted_events.extend(events.iter().cloned());
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;

        for event in events {
            if event.entity_type == ORDER_ENTITY_TYPE && event.is_created() {
                let order_sequence = event_order_sequence(event)
                    .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
                let stored_order = StoredOrder {
                    order_id: event_string_field(event, "order_id")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    account_id: event_string_field(event, "account_id")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    symbol: event_string_field(event, "symbol")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    qty: event_u64_field(event, "qty")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    price: event_u64_field(event, "price")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    reserved_quote: event_u64_field(event, "reserved_quote")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                };
                state.orders.insert(stored_order.order_id.clone(), stored_order);
                state.next_order_sequence = order_sequence
                    .checked_add(1)
                    .map(|next| state.next_order_sequence.max(next))
                    .ok_or(PlaceOrderOutboundError::SequenceOverflow)?;
                continue;
            }

            if event.entity_type == ACCOUNT_ENTITY_TYPE && event.is_updated() {
                let account_id = event_string_field(event, "account_id")
                    .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
                let account = state
                    .accounts
                    .get_mut(account_id.as_str())
                    .ok_or(PlaceOrderOutboundError::AccountNotFound)?;
                account.available_quote = event_u64_field(event, "available_quote")
                    .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
                account.frozen_quote = event_u64_field(event, "frozen_quote")
                    .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
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
