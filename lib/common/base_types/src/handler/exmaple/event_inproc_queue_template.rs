//! 进程内队列 / channel 事件传递示例。
//!
//! 展示：
//! - PlaceOrderHandler 产出收单事件
//! - 事件进入进程内 memory queue
//! - consumer 消费后调用 PlaceOrderEventHandler
//! - 再产生成交事件并继续入队
//! - consumer 调用 TradeEventHandler 完成结算

use std::collections::VecDeque;

use crate::handler::exmaple::event_template::{
    emit_place_order_event, emit_trade_created_event, EventHandlerError, PlaceOrderEventHandler,
    TradeEventHandler,
};
use crate::handler::exmaple::match_handler::MatchOutput;
use crate::handler::exmaple::place_order_handler::{
    OrderSide, OrderType, PlaceOrderAcceptedEvent, PlaceOrderCmd, PlaceOrderEvent,
    PlaceOrderHandler, PlaceOrderOutput, PlaceOrderResult,
};
use crate::handler::exmaple::settlement_handler::{SettlementHandler, SettlementResult};
use crate::handler::event_handler::EventHandler;
use crate::handler::handler_update::CmdHandlerForUpdate;

pub enum InProcEventEnvelope {
    PlaceOrderAccepted(PlaceOrderAcceptedEvent),
    TradeCreated(crate::handler::exmaple::match_handler::TradeCreatedEvent),
}

pub struct InProcEventQueue {
    queue: VecDeque<InProcEventEnvelope>,
}

impl InProcEventQueue {
    pub fn new() -> Self {
        Self { queue: VecDeque::new() }
    }

    pub fn push(&mut self, event: InProcEventEnvelope) {
        self.queue.push_back(event);
    }

    pub fn pop(&mut self) -> Option<InProcEventEnvelope> {
        self.queue.pop_front()
    }
}

pub fn run_full_event_chain_via_inproc_queue() -> Result<SettlementResult, EventHandlerError> {
    let place_order_handler = PlaceOrderHandler::new();
    let place_order_event_handler =
        PlaceOrderEventHandler::new(crate::handler::exmaple::match_handler::MatchHandler::new());
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

    let mut queue = InProcEventQueue::new();
    if let Some(event) = emit_place_order_event(&place_order_output) {
        queue.push(InProcEventEnvelope::PlaceOrderAccepted(event));
    }

    while let Some(event) = queue.pop() {
        match event {
            InProcEventEnvelope::PlaceOrderAccepted(event) => {
                let match_output: MatchOutput = place_order_event_handler.event_handle(event)?;
                if let Some(trade_event) = emit_trade_created_event(&match_output) {
                    queue.push(InProcEventEnvelope::TradeCreated(trade_event));
                }
            }
            InProcEventEnvelope::TradeCreated(event) => {
                return trade_event_handler.event_handle(event);
            }
        }
    }

    Err(EventHandlerError("in-process queue ended without settlement result".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inproc_queue_event_chain() {
        let result = run_full_event_chain_via_inproc_queue();
        assert!(result.is_ok());
    }
}
