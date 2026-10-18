use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{StateSink, StateSource};
use common_entity::Entity;
use example_core_use_case::{
    Balance, PlaceMatchSpotOrderV2State, PlaceMatchSpotOrderV2UseCase, PlaceOnlySpotOrderV2Cmd,
    SpotOrderSide, SpotOrderV2, SpotTrade,
};
use lob_repo_v2::lob::AssetLobV2;

use super::in_memory_place_order::{
    DEFAULT_FEE_ACCOUNT_ID, apply_order_update_event, decode_created_reservation,
    decode_created_trade, decode_order_from_event,
};
pub use super::in_memory_place_order::{
    InMemoryPlaceOrderOutbound, base_asset_id_for, quote_asset_id_for, symbol_for_asset,
};
use crate::shared::{
    PlaceOrderOutboundError, SpotOrderPlacedMessage, SpotPipelineBroker, SpotPipelineMessage,
    balance_key, event_string_field, event_u64_field,
};

impl StateSource<PlaceMatchSpotOrderV2UseCase> for InMemoryPlaceOrderOutbound {
    type Error = PlaceOrderOutboundError;

    fn load_given_state(
        &self,
        cmd: &PlaceOnlySpotOrderV2Cmd,
    ) -> Result<PlaceMatchSpotOrderV2State, Self::Error> {
        let order_cmd = match cmd {
            PlaceOnlySpotOrderV2Cmd::Single(order)
            | PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent: order, .. } => order,
        };
        let state = self.store.lock_state()?;
        let market_rules = state
            .market_rules_by_symbol
            .get(symbol_for_asset(order_cmd.asset))
            .cloned()
            .ok_or(PlaceOrderOutboundError::MarketRulesNotFound)?;
        let base_asset_id = base_asset_id_for(&market_rules.symbol).to_string();
        let quote_asset_id = quote_asset_id_for(&market_rules.symbol).to_string();
        let mut settlement_balances = state.balances.values().cloned().collect::<Vec<_>>();
        if !state.balances.contains_key(&balance_key(DEFAULT_FEE_ACCOUNT_ID, &quote_asset_id)) {
            settlement_balances.push(example_core_use_case::Balance::new(
                DEFAULT_FEE_ACCOUNT_ID.to_string(),
                quote_asset_id.clone(),
                0,
                0,
                1,
            ));
        }
        if !state.balances.contains_key(&balance_key(&order_cmd.party_id, &base_asset_id))
            || !state.balances.contains_key(&balance_key(&order_cmd.party_id, &quote_asset_id))
        {
            return Err(PlaceOrderOutboundError::BalanceNotFound);
        }
        let side = if order_cmd.is_buy { SpotOrderSide::Buy } else { SpotOrderSide::Sell };
        let maker_orders = state
            .lob
            .match_orders(u64::MAX, u64::MAX, side)
            .0
            .unwrap_or_default()
            .into_iter()
            .cloned()
            .collect();
        Ok(PlaceMatchSpotOrderV2State {
            maker_orders,
            settlement_balances,
            fee_account_id: DEFAULT_FEE_ACCOUNT_ID.to_string(),
        })
    }
}

impl StateSink<PlaceMatchSpotOrderV2UseCase> for InMemoryPlaceOrderOutbound {
    type Error = PlaceOrderOutboundError;
    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.persisted_events.extend(events.iter().cloned());
        Ok(())
    }
    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        replay_events(self, events)
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.published_events.extend(events.iter().cloned());
        drop(state);
        let Some(broker) = &self.broker else {
            return Ok(());
        };
        for event in events {
            if !event.is_created() || event_string_field(event, "order_id").is_none() {
                continue;
            }
            let order_id = event_string_field(event, "order_id")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
            broker
                .publish(SpotPipelineMessage::SpotOrderPlaced(SpotOrderPlacedMessage {
                    match_id: order_id.clone(),
                    order_id,
                    party_id: event_string_field(event, "account_id")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    symbol: event_string_field(event, "symbol")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    asset: event_u64_field(event, "asset")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?
                        as u32,
                    trace_id: None,
                    command_id: None,
                }))
                .map_err(|_| PlaceOrderOutboundError::BrokerPublishFailed)?;
        }
        Ok(())
    }
}

pub(crate) fn replay_events(
    outbound: &InMemoryPlaceOrderOutbound,
    events: &[EntityReplayableEvent],
) -> Result<(), PlaceOrderOutboundError> {
    let mut state = outbound.store.lock_state()?;

    //todo SpotOrderV2 相关的event, 在单独函数处理， 直接调用 replay_events+ event数组

    for event in events {
        if event.is_created()
            && event_string_field(event, "reservation_id").is_some()
            && event_string_field(event, "owner_account_id").is_some()
            && event_string_field(event, "caused_by_order_id").is_some()
        {
            let reservation = decode_created_reservation(event)?;
            state.reservations.insert(reservation.reservation_id.clone(), reservation);
            continue;
        }
        if event.entity_type == SpotOrderV2::entity_type() && event.is_created() {
            let order = decode_order_from_event(event)?;
            state.orders.insert(order.order_id, order);
            let mut lob_event = event.clone();
            lob_event.seq = state
                .lob
                .sequence()
                .checked_add(1)
                .ok_or(PlaceOrderOutboundError::SequenceOverflow)?;
            state
                .lob
                .replay_events(&[lob_event])
                .map_err(|_| PlaceOrderOutboundError::EventDecodeFailed)?;
            state.next_order_sequence = state
                .next_order_sequence
                .checked_add(1)
                .ok_or(PlaceOrderOutboundError::SequenceOverflow)?;
            continue;
        }
        if event.entity_type == SpotTrade::entity_type() && event.is_created() {
            let trade = decode_created_trade(event)?;
            state.trades.insert(trade.trade_id.clone(), trade);
            continue;
        }
        if event.entity_type == SpotOrderV2::entity_type() && event.is_updated() {
            apply_order_update_event(&mut state.orders, event)?;
            let mut lob_event = event.clone();
            lob_event.seq = state
                .lob
                .sequence()
                .checked_add(1)
                .ok_or(PlaceOrderOutboundError::SequenceOverflow)?;
            state
                .lob
                .replay_events(&[lob_event])
                .map_err(|_| PlaceOrderOutboundError::EventDecodeFailed)?;
            state.next_order_sequence = state
                .next_order_sequence
                .checked_add(1)
                .ok_or(PlaceOrderOutboundError::SequenceOverflow)?;
            continue;
        }
        if event.is_updated() && event_string_field(event, "asset_id").is_some() {
            let account_id = event_string_field(event, "account_id")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
            let asset_id = event_string_field(event, "asset_id")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
            let balance = state
                .balances
                .entry(balance_key(&account_id, &asset_id))
                .or_insert_with(|| Balance::new(account_id.clone(), asset_id.clone(), 0, 0, 0));
            if let Some(value) = event_u64_field(event, "available") {
                balance.available = value;
            }
            if let Some(value) = event_u64_field(event, "frozen") {
                balance.frozen = value;
            }
            balance.version = event.new_version;
        }
    }
    Ok(())
}
