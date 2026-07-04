use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCaseOutbound;
use example_core::{MatchSpotOrderCmd, MatchSpotOrderState, SpotOrderStatus, SpotTrade};

use crate::shared::{
    InMemorySpotPipelineBroker, InMemoryStore, MatchSpotOrderOutboundError, SpotPipelineBroker,
    SpotPipelineMessage, SpotTradeMatchedMessage, event_string_field, event_u64_field,
};

#[derive(Debug, Clone)]
pub struct InMemoryMatchSpotOrderOutbound {
    store: InMemoryStore,
    broker: InMemorySpotPipelineBroker,
}

impl InMemoryMatchSpotOrderOutbound {
    pub fn new(store: InMemoryStore, broker: InMemorySpotPipelineBroker) -> Self {
        Self { store, broker }
    }
}

impl CommandUseCaseOutbound for InMemoryMatchSpotOrderOutbound {
    type Command = MatchSpotOrderCmd;
    type State = MatchSpotOrderState;
    type Error = MatchSpotOrderOutboundError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        let state = self.store.lock_state()?;
        let taker_order = state
            .orders
            .get(cmd.taker_order_id.as_str())
            .cloned()
            .ok_or(MatchSpotOrderOutboundError::OrderNotFound)?;
        let mut maker_orders = state
            .orders
            .values()
            .filter(|order| {
                order.order_id != taker_order.order_id
                    && order.side != taker_order.side
                    && order.asset == taker_order.asset
                    && order.symbol == taker_order.symbol
                    && matches!(
                        order.status,
                        SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
                    )
            })
            .cloned()
            .collect::<Vec<_>>();
        maker_orders.sort_by(|left, right| left.order_id.cmp(&right.order_id));

        Ok(MatchSpotOrderState { taker_order, maker_orders })
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.persisted_events.extend(events.iter().cloned());
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;

        for event in events {
            if event.is_updated() && event_string_field(event, "order_id").is_some() {
                let order_id = event_string_field(event, "order_id")
                    .ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?;
                let order = state
                    .orders
                    .get_mut(order_id.as_str())
                    .ok_or(MatchSpotOrderOutboundError::OrderNotFound)?;
                if let Some(filled_qty) = event_u64_field(event, "filled_qty") {
                    order.filled_qty = filled_qty;
                }
                if let Some(status) = event_string_field(event, "status") {
                    order.status = decode_status(status.as_str())?;
                }
                order.version = event.new_version;
                continue;
            }

            if event.is_created() && event_string_field(event, "trade_id").is_some() {
                let trade = decode_trade(event)?;
                state.trades.insert(trade.trade_id.clone(), trade);
            }
        }

        Ok(())
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.published_events.extend(events.iter().cloned());
        drop(state);

        let trades = events
            .iter()
            .filter(|event| event.is_created() && event_string_field(event, "trade_id").is_some())
            .map(|event| decode_trade(event))
            .collect::<Result<Vec<_>, _>>()?;
        if trades.is_empty() {
            return Ok(());
        }

        let first = &trades[0];
        self.broker
            .publish(SpotPipelineMessage::SpotTradeMatched(SpotTradeMatchedMessage {
                trade_ids: trades.iter().map(|trade| trade.trade_id.clone()).collect(),
                party_id: first.taker_account_id.clone(),
                match_id: first.match_id.clone(),
                settlement_batch_id: first.match_id.clone(),
                trace_id: None,
                command_id: None,
            }))
            .map_err(|_| MatchSpotOrderOutboundError::BrokerPublishFailed)?;
        Ok(())
    }
}

fn decode_status(value: &str) -> Result<SpotOrderStatus, MatchSpotOrderOutboundError> {
    match value {
        "open" => Ok(SpotOrderStatus::Open),
        "partially_filled" => Ok(SpotOrderStatus::PartiallyFilled),
        "filled" => Ok(SpotOrderStatus::Filled),
        "canceled" => Ok(SpotOrderStatus::Canceled),
        "rejected" => Ok(SpotOrderStatus::Rejected),
        _ => Err(MatchSpotOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_trade(event: &EntityReplayableEvent) -> Result<SpotTrade, MatchSpotOrderOutboundError> {
    let taker_side = match event_string_field(event, "taker_side").as_deref() {
        Some("buy") => example_core::SpotOrderSide::Buy,
        Some("sell") => example_core::SpotOrderSide::Sell,
        _ => return Err(MatchSpotOrderOutboundError::EventDecodeFailed),
    };
    Ok(SpotTrade::new(
        event_string_field(event, "trade_id")
            .ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "match_id")
            .ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "asset").ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?
            as u32,
        event_string_field(event, "symbol")
            .ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "taker_order_id")
            .ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "maker_order_id")
            .ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "taker_account_id")
            .ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "maker_account_id")
            .ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?,
        taker_side,
        event_u64_field(event, "price").ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "qty").ok_or(MatchSpotOrderOutboundError::EventDecodeFailed)?,
    ))
}
