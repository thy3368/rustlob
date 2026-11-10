use common_entity::{
    EntityReplayableEvent, LogEntry, StateSink, StateSource, stable_string_entity_id,
};
use example_core_use_case::common::identifiers::OrderId;
use example_core_use_case::{
    ModifySpotOrderV2Cmd, ModifySpotOrderV2State, ModifySpotOrderV2UseCase,
};

use super::place_order_in_memory::{
    InMemoryPlaceOrderOutbound, base_asset_id_for, quote_asset_id_for,
};
use crate::shared::{PlaceOrderOutboundError, balance_key};

impl StateSource<ModifySpotOrderV2UseCase> for InMemoryPlaceOrderOutbound {
    type Error = PlaceOrderOutboundError;

    fn load_given_state(
        &self,
        cmd: &LogEntry<ModifySpotOrderV2Cmd>,
    ) -> Result<ModifySpotOrderV2State, Self::Error> {
        let state = self.store.lock_state()?;
        let order = state
            .orders
            .values()
            .find(|order| {
                order.account_id() == cmd.party_id
                    && order.asset() == cmd.asset
                    && match &cmd.order_id {
                        OrderId::Oid(order_id) => order.order_id() == *order_id,
                        OrderId::Cloid(cloid) => order.cloid.as_deref() == Some(cloid.as_str()),
                    }
            })
            .cloned()
            .ok_or(PlaceOrderOutboundError::OrderNotFound)?;
        let base_asset_id = base_asset_id_for(order.symbol()).to_string();
        let quote_asset_id = quote_asset_id_for(order.symbol()).to_string();
        let account_id = stable_string_entity_id(&cmd.party_id);
        let balances = [base_asset_id.as_str(), quote_asset_id.as_str()]
            .into_iter()
            .map(|asset_id| {
                state
                    .balances
                    .get(&balance_key(account_id, stable_string_entity_id(asset_id)))
                    .cloned()
                    .ok_or(PlaceOrderOutboundError::BalanceNotFound)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ModifySpotOrderV2State {
            order,
            balances,
            base_asset_id,
            quote_asset_id,
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        })
    }
}

impl StateSink<ModifySpotOrderV2UseCase> for InMemoryPlaceOrderOutbound {
    type Error = PlaceOrderOutboundError;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.store.lock_state()?.persisted_events.extend(events.iter().cloned());
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        super::place_order_in_memory::replay_events(self, events)
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.store.lock_state()?.published_events.extend(events.iter().cloned());
        Ok(())
    }
}
