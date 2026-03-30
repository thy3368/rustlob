//! Kafka 风格 topic 事件传递示例。
//!
//! 展示：
//! - PlaceOrderHandler 产出收单事件
//! - producer 将事件发布到 topic
//! - consumer 按 topic 消费并调用 PlaceOrderEventHandler
//! - 再把成交事件发布到 trade topic
//! - consumer 调用 TradeEventHandler 完成结算
//!
//! 这里使用 mock broker / mock record 表达 Kafka publish-consume 语义，
//! 不引入真实 Kafka 依赖。

use std::collections::VecDeque;

use crate::handler::event_handler::EventHandler;
use crate::handler::exmaple::event_handler::event_template::{
    emit_place_order_event, emit_trade_created_event, EventHandlerError, PlaceOrderEventHandler,
    TradeEventHandler,
};
use crate::handler::exmaple::cmd_handler::match_handler::MatchOutput;
use crate::handler::exmaple::cmd_handler::place_order_handler::{
    OrderSide, OrderType, PlaceOrderAcceptedEvent, PlaceOrderCmd, PlaceOrderEvent,
    PlaceOrderHandler, PlaceOrderOutput, PlaceOrderResult,
};
use crate::handler::exmaple::cmd_handler::settlement_handler::{SettlementHandler, SettlementResult};
use crate::handler::handler_update::CmdHandlerForUpdate;

pub const TOPIC_ORDER_ACCEPTED: &str = "orders.accepted";
pub const TOPIC_TRADE_CREATED: &str = "trades.created";

pub enum KafkaPayload {
    PlaceOrderAccepted(PlaceOrderAcceptedEvent),
    TradeCreated(crate::handler::exmaple::cmd_handler::match_handler::TradeCreatedEvent),
}

pub struct KafkaRecord {
    pub topic: &'static str,
    pub key: String,
    pub payload: KafkaPayload,
}

pub struct MockKafkaBroker {
    records: VecDeque<KafkaRecord>,
}

impl MockKafkaBroker {
    pub fn new() -> Self {
        Self { records: VecDeque::new() }
    }

    pub fn publish(&mut self, record: KafkaRecord) {
        self.records.push_back(record);
    }

    pub fn consume(&mut self) -> Option<KafkaRecord> {
        self.records.pop_front()
    }
}

pub fn run_full_event_chain_via_kafka_queue() -> Result<SettlementResult, EventHandlerError> {
    let place_order_handler = PlaceOrderHandler::new();
    let place_order_event_handler =
        PlaceOrderEventHandler::new(crate::handler::exmaple::cmd_handler::match_handler::MatchHandler::new());
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
                status: crate::handler::exmaple::cmd_handler::example_types::OrderStatus::Open,
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

    let mut broker = MockKafkaBroker::new();
    if let Some(event) = emit_place_order_event(&place_order_output) {
        broker.publish(KafkaRecord {
            topic: TOPIC_ORDER_ACCEPTED,
            key: event.order_id.clone(),
            payload: KafkaPayload::PlaceOrderAccepted(event),
        });
    }

    while let Some(record) = broker.consume() {
        match record.payload {
            KafkaPayload::PlaceOrderAccepted(event) => {
                let match_output: MatchOutput = place_order_event_handler.event_handle(event)?;
                if let Some(trade_event) = emit_trade_created_event(&match_output) {
                    broker.publish(KafkaRecord {
                        topic: TOPIC_TRADE_CREATED,
                        key: trade_event.trade_id.clone(),
                        payload: KafkaPayload::TradeCreated(trade_event),
                    });
                }
            }
            KafkaPayload::TradeCreated(event) => {
                return trade_event_handler.event_handle(event);
            }
        }
    }

    Err(EventHandlerError("kafka queue ended without settlement result".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kafka_queue_event_chain() {
        let result = run_full_event_chain_via_kafka_queue();
        assert!(result.is_ok());
    }
}
