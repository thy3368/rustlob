//! EventHandler 示例模板。
//!
//! 展示完整的事件驱动链路：
//! - PlaceOrderHandler 产出收单事件
//! - EventHandler 接收收单事件并调用 MatchHandler
//! - MatchHandler 产出成交事件
//! - EventHandler 接收成交事件并调用 SettlementHandler

use crate::handler::event_handler::EventHandler;
use crate::handler::exmaple::match_handler::{
    MatchCmd, MatchEvent, MatchHandler, MatchOutput, MatchResult, TradeCreatedEvent,
};
use crate::handler::exmaple::place_order_handler::{
    PlaceOrderAcceptedEvent, PlaceOrderCmd, PlaceOrderEvent, PlaceOrderHandler, PlaceOrderOutput,
    PlaceOrderResult,
};
use crate::handler::exmaple::settlement_handler::{
    SettlementCmd, SettlementHandler, SettlementResult,
};
use crate::handler::handler_update::CmdHandlerForUpdate;

#[derive(Debug)]
pub struct EventHandlerError(pub String);

pub struct PlaceOrderEventHandler {
    pub match_handler: MatchHandler,
}

impl PlaceOrderEventHandler {
    pub fn new(match_handler: MatchHandler) -> Self {
        Self { match_handler }
    }
}

impl EventHandler<PlaceOrderAcceptedEvent, MatchOutput, EventHandlerError> for PlaceOrderEventHandler {
    fn event_handle(&self, event: PlaceOrderAcceptedEvent) -> Result<MatchOutput, EventHandlerError> {
        let cmd = MatchCmd {
            match_id: format!("match_{}", event.order_id),
            taker_order_id: event.order_id,
        };

        self.match_handler
            .cmd_handle(cmd, |writes, _| MatchOutput {
                result: MatchResult {
                    trades: writes
                        .result
                        .trades
                        .iter()
                        .map(|trade| crate::handler::exmaple::example_types::Trade {
                            trade_id: trade.trade_id.clone(),
                            taker_order_id: trade.taker_order_id.clone(),
                            maker_order_id: trade.maker_order_id.clone(),
                            price: trade.price,
                            quantity: trade.quantity,
                        })
                        .collect(),
                    order_updates: vec![],
                },
                events: writes
                    .events
                    .iter()
                    .map(|event| match event {
                        MatchEvent::TradeCreated(event) => MatchEvent::TradeCreated(TradeCreatedEvent {
                            trade_id: event.trade_id.clone(),
                        }),
                    })
                    .collect(),
            })
            .map_err(|err| EventHandlerError(err.0))
    }
}

pub struct TradeEventHandler {
    pub settlement_handler: SettlementHandler,
}

impl TradeEventHandler {
    pub fn new(settlement_handler: SettlementHandler) -> Self {
        Self { settlement_handler }
    }
}

impl EventHandler<TradeCreatedEvent, SettlementResult, EventHandlerError> for TradeEventHandler {
    fn event_handle(&self, event: TradeCreatedEvent) -> Result<SettlementResult, EventHandlerError> {
        let cmd = SettlementCmd {
            settlement_id: format!("settlement_{}", event.trade_id),
            trade_ids: vec![event.trade_id],
        };

        self.settlement_handler
            .cmd_handle(cmd, |writes, _| SettlementResult {
                balance_changes: writes
                    .balance_changes
                    .iter()
                    .map(|change| crate::handler::exmaple::example_types::BalanceChange {
                        user_id: change.user_id.clone(),
                        asset: change.asset.clone(),
                        change: change.change,
                    })
                    .collect(),
            })
            .map_err(|err| EventHandlerError(err.0))
    }
}

pub fn emit_place_order_event(output: &PlaceOrderOutput) -> Option<PlaceOrderAcceptedEvent> {
    output.events.iter().find_map(|event| match event {
        PlaceOrderEvent::Accepted(event) => Some(PlaceOrderAcceptedEvent {
            order_id: event.order_id.clone(),
        }),
    })
}

pub fn emit_trade_created_event(output: &MatchOutput) -> Option<TradeCreatedEvent> {
    output.events.iter().find_map(|event| match event {
        MatchEvent::TradeCreated(event) => Some(TradeCreatedEvent {
            trade_id: event.trade_id.clone(),
        }),
    })
}

pub fn run_full_event_chain(
    place_order_handler: &PlaceOrderHandler,
    place_order_event_handler: &PlaceOrderEventHandler,
    trade_event_handler: &TradeEventHandler,
    cmd: PlaceOrderCmd,
) -> Result<SettlementResult, EventHandlerError> {
    let place_order_output = place_order_handler
        .cmd_handle(cmd, |writes, _| PlaceOrderOutput {
            result: PlaceOrderResult {
                order_id: writes.result.order_id.clone(),
                status: crate::handler::exmaple::example_types::OrderStatus::Open,
                balance_change: None,
            },
            events: writes
                .events
                .iter()
                .map(|event| match event {
                    PlaceOrderEvent::Accepted(event) => {
                        PlaceOrderEvent::Accepted(PlaceOrderAcceptedEvent {
                            order_id: event.order_id.clone(),
                        })
                    }
                })
                .collect(),
        })
        .map_err(|err| EventHandlerError(err.0))?;

    let place_order_event = emit_place_order_event(&place_order_output)
        .ok_or_else(|| EventHandlerError("missing place order event".into()))?;
    let match_output = place_order_event_handler.event_handle(place_order_event)?;

    let trade_event = emit_trade_created_event(&match_output)
        .ok_or_else(|| EventHandlerError("missing trade created event".into()))?;
    trade_event_handler.event_handle(trade_event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::exmaple::match_handler::MatchHandler;
    use crate::handler::exmaple::place_order_handler::{OrderSide, OrderType};
    use crate::handler::exmaple::settlement_handler::SettlementHandler;

    #[test]
    fn test_place_order_event_handler_invokes_match_handler() {
        let handler = PlaceOrderEventHandler::new(MatchHandler::new());
        let event = PlaceOrderAcceptedEvent { order_id: "order_1".into() };

        let result = handler.event_handle(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_trade_event_handler_invokes_settlement_handler() {
        let handler = TradeEventHandler::new(SettlementHandler::new());
        let event = TradeCreatedEvent { trade_id: "trade_1".into() };

        let result = handler.event_handle(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_full_event_chain() {
        let place_order_handler = PlaceOrderHandler::new();
        let place_order_event_handler = PlaceOrderEventHandler::new(MatchHandler::new());
        let trade_event_handler = TradeEventHandler::new(SettlementHandler::new());
        let cmd = PlaceOrderCmd {
            cmd_id: "1".into(),
            user_id: "u1".into(),
            symbol: "BTCUSDT".into(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Some(50000),
            quantity: 10,
            timestamp_ms: 1234567890,
        };

        let result = run_full_event_chain(
            &place_order_handler,
            &place_order_event_handler,
            &trade_event_handler,
            cmd,
        );
        assert!(result.is_ok());
    }
}
