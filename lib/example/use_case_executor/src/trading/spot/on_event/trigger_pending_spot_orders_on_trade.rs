use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{ExecutionError, ExecutionResult, StateSink, StateSource};
use common_entity::{Entity, EntityReplayableEventHandler, ReplayEventFieldParseError};
use example_core_use_case::{
    ActivateMatchSpotOrderV2Changes, ActivateMatchSpotOrderV2Cmd, ActivateMatchSpotOrderV2Error,
    ActivateMatchSpotOrderV2UseCase, PendingTriggerSpotOrderSource, SpotTrade,
};
use thiserror::Error;

use crate::trading::spot::activate_match_spot_order_v2_executor::execute_activate_match_spot_order_v2_with_outbound;

/// 成交 created event 中用于触发条件单的市场事实。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpotTradeTriggerInput {
    pub asset: u32,
    pub market_price: u64,
}

/// 一笔成交事件触发的条件单激活与撮合结果。
pub type TriggerPendingSpotOrdersOnTradeOutput =
    Vec<ExecutionResult<ActivateMatchSpotOrderV2Changes>>;

/// 从成交 created event 读取资产和成交价，并触发满足条件的 pending 订单。
#[derive(Debug)]
pub struct TriggerPendingSpotOrdersOnTradeHandler<OB> {
    outbound: OB,
}

impl<OB> TriggerPendingSpotOrdersOnTradeHandler<OB> {
    pub fn new(outbound: OB) -> Self {
        Self { outbound }
    }
}

impl<OB> EntityReplayableEventHandler for TriggerPendingSpotOrdersOnTradeHandler<OB>
where
    OB: PendingTriggerSpotOrderSource<
            Error = <OB as StateSink<ActivateMatchSpotOrderV2UseCase>>::Error,
        > + StateSource<
            ActivateMatchSpotOrderV2UseCase,
            Error = <OB as StateSink<ActivateMatchSpotOrderV2UseCase>>::Error,
        > + StateSink<ActivateMatchSpotOrderV2UseCase>,
{
    type Output = TriggerPendingSpotOrdersOnTradeOutput;
    type Error = TriggerPendingSpotOrdersOnTradeError<
        <OB as StateSink<ActivateMatchSpotOrderV2UseCase>>::Error,
    >;

    fn handle(
        &mut self,
        event: &EntityReplayableEvent,
    ) -> Result<Option<Self::Output>, Self::Error> {
        let Some(input) = spot_trade_trigger_input_from_event(event)
            .map_err(TriggerPendingSpotOrdersOnTradeError::Parse)?
        else {
            return Ok(None);
        };

        let orders = self
            .outbound
            .load_pending_trigger_orders(input.asset, input.market_price)
            .map_err(TriggerPendingSpotOrdersOnTradeError::Load)?;
        let mut results = Vec::with_capacity(orders.len());

        for order in orders {
            let order_id = order.order_id();
            let command = ActivateMatchSpotOrderV2Cmd {
                party_id: order.account_id().to_owned(),
                asset: order.asset(),
                order_id,
            };
            let result = execute_activate_match_spot_order_v2_with_outbound(
                &common_entity::LogEntrySequencer::from_metadata(
                    event.lsn,
                    event.logical_ts,
                    0,
                    order_id,
                    command,
                ),
                &self.outbound,
            )
            .map_err(|error| TriggerPendingSpotOrdersOnTradeError::Execute { order_id, error })?;
            results.push(result);
        }

        Ok(Some(results))
    }
}

/// 从成交 created event 读取条件单触发所需的市场事实。
pub fn spot_trade_trigger_input_from_event(
    event: &EntityReplayableEvent,
) -> Result<Option<SpotTradeTriggerInput>, SpotTradeTriggerInputParseError> {
    if event.entity_type != SpotTrade::entity_type() || !event.is_created() {
        return Ok(None);
    }

    Ok(Some(SpotTradeTriggerInput {
        asset: event.required_u32_field("asset")?,
        market_price: event.required_u64_field("price")?,
    }))
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SpotTradeTriggerInputParseError {
    #[error("spot trade event is missing field `{0}`")]
    MissingField(&'static str),
    #[error("spot trade event field `{field}` is not valid UTF-8")]
    InvalidUtf8 { field: &'static str },
    #[error("spot trade event field `{field}` is not a valid {expected}: {value}")]
    InvalidNumber {
        field: &'static str,
        expected: &'static str,
        value: String,
    },
}

impl From<ReplayEventFieldParseError> for SpotTradeTriggerInputParseError {
    fn from(error: ReplayEventFieldParseError) -> Self {
        match error {
            ReplayEventFieldParseError::MissingField(field) => Self::MissingField(field),
            ReplayEventFieldParseError::InvalidUtf8 { field } => Self::InvalidUtf8 { field },
            ReplayEventFieldParseError::InvalidNumber {
                field,
                expected,
                value,
            } => Self::InvalidNumber {
                field,
                expected,
                value,
            },
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TriggerPendingSpotOrdersOnTradeError<OutboundError> {
    #[error(transparent)]
    Parse(#[from] SpotTradeTriggerInputParseError),
    #[error("failed to load pending trigger spot orders: {0}")]
    Load(OutboundError),
    #[error("failed to activate and match triggered spot order {order_id}: {error:?}")]
    Execute {
        order_id: u64,
        error: ExecutionError<ActivateMatchSpotOrderV2Error, OutboundError>,
    },
}

#[cfg(test)]
mod tests {
    use cmd_handler::EntityReplayableEvent;
    use common_entity::{Entity, ReplayFieldChange, stable_string_entity_id};
    use example_core_use_case::{
        Balance, MarketRules, OrderSide, SpotOrderStatus, OrderTriggerRole, OrderType,
        SpotOrderV2,
    };
    use example_outbound_adapter::InMemoryPlaceOrderOutbound;

    use super::*;

    fn trade_event(asset: u32, price: u64) -> EntityReplayableEvent {
        let mut event = EntityReplayableEvent::new_created(1, 1, SpotTrade::entity_type());
        for (name, value) in [("asset", asset.to_string()), ("price", price.to_string())] {
            event.add_field_change(ReplayFieldChange::new(
                ReplayFieldChange::field_name_from_str(name),
                &[],
                value.as_bytes(),
                0,
            ));
        }
        event
    }

    #[test]
    fn trigger_input_uses_the_trade_price_field() {
        let input =
            spot_trade_trigger_input_from_event(&trade_event(10_001, 123)).expect("valid event");

        assert_eq!(
            input,
            Some(SpotTradeTriggerInput {
                asset: 10_001,
                market_price: 123
            })
        );
    }

    #[test]
    fn created_trade_activates_and_matches_triggered_pending_order() {
        let outbound = InMemoryPlaceOrderOutbound::new();
        outbound
            .seed_market_rules(MarketRules {
                symbol: "BTCUSDT".to_string(),
                min_qty: 1,
            })
            .expect("market rules should seed");
        outbound
            .seed_balance(Balance::new(
                stable_string_entity_id("buyer"),
                stable_string_entity_id("buyer"),
                stable_string_entity_id("BTC"),
                0,
                0,
                1,
            ))
            .expect("base balance should seed");
        outbound
            .seed_balance(Balance::new(
                stable_string_entity_id("buyer"),
                stable_string_entity_id("buyer"),
                stable_string_entity_id("USDT"),
                1_000,
                0,
                1,
            ))
            .expect("quote balance should seed");
        outbound
            .seed_order(SpotOrderV2::new_pending_trigger(
                7,
                10_001,
                "buyer".to_string(),
                "BTCUSDT".to_string(),
                OrderSide::Buy,
                1,
                100,
                OrderType::Trigger {
                    is_market: false,
                    trigger_price: 100,
                    tpsl: OrderTriggerRole::StopLoss,
                },
                None,
                0,
                1,
            ))
            .expect("pending trigger order should seed");

        let mut handler = TriggerPendingSpotOrdersOnTradeHandler::new(outbound.clone());
        let results = handler
            .handle(&trade_event(10_001, 100))
            .expect("trade event should be handled")
            .expect("spot trade should be a target event");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].events.len(), 5);

        let snapshot = outbound.snapshot().expect("snapshot should load");
        assert_eq!(
            snapshot.orders.get(&7).map(SpotOrderV2::status),
            Some(SpotOrderStatus::Open)
        );
        assert_eq!(snapshot.persisted_event_count, 5);
        assert!(
            outbound
                .load_pending_trigger_orders(10_001, 100)
                .expect("trigger query should load")
                .is_empty()
        );
    }

    #[test]
    fn non_spot_trade_events_are_ignored() {
        let mut handler =
            TriggerPendingSpotOrdersOnTradeHandler::new(InMemoryPlaceOrderOutbound::new());
        let event = EntityReplayableEvent::new_created(1, 1, 99);

        assert_eq!(
            handler
                .handle(&event)
                .expect("non-target event should be ignored"),
            None
        );
    }
}
