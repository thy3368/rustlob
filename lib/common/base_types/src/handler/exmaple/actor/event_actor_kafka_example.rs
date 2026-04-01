//! EventActor Kafka-style 示例。

use std::collections::VecDeque;
use std::thread;

use crate::handler::event_actor::EventActor;
use crate::handler::event_handler::EventHandler;
use crate::handler::exmaple::cmd_handler::match_handler::{MatchHandler, MatchOutput, TradeCreatedEvent};
use crate::handler::exmaple::cmd_handler::place_order_handler::PlaceOrderAcceptedEvent;
use crate::handler::exmaple::cmd_handler::settlement_handler::{SettlementHandler, SettlementResult};
use crate::handler::exmaple::actor::event_actor_example_shared::build_first_place_order_event;
use crate::handler::exmaple::event_handler::event_template::{
    emit_trade_created_event, EventHandlerError, PlaceOrderEventHandler, TradeEventHandler,
};

pub enum KafkaPayload {
    PlaceOrderAccepted(PlaceOrderAcceptedEvent),
    TradeCreated(TradeCreatedEvent),
}

pub struct KafkaRecord {
    pub topic: &'static str,
    pub key: String,
    pub payload: KafkaPayload,
}

pub const KAFKA_TOPIC_ORDER_ACCEPTED: &str = "orders.accepted";
pub const KAFKA_TOPIC_TRADE_CREATED: &str = "trades.created";

pub struct KafkaDispatcher {
    place_order_event_handler: PlaceOrderEventHandler,
    trade_event_handler: TradeEventHandler,
    pending_records: std::sync::Mutex<VecDeque<KafkaRecord>>,
    settlement_result: std::sync::Mutex<Option<SettlementResult>>,
}

impl KafkaDispatcher {
    pub fn new() -> Self {
        Self {
            place_order_event_handler: PlaceOrderEventHandler::new(MatchHandler::new()),
            trade_event_handler: TradeEventHandler::new(SettlementHandler::new()),
            pending_records: std::sync::Mutex::new(VecDeque::new()),
            settlement_result: std::sync::Mutex::new(None),
        }
    }

    pub fn take_pending_record(&self) -> Option<KafkaRecord> {
        self.pending_records.lock().ok()?.pop_front()
    }

    pub fn take_settlement_result(&self) -> Option<SettlementResult> {
        self.settlement_result.lock().ok()?.take()
    }
}

impl EventHandler<KafkaRecord, (), EventHandlerError> for KafkaDispatcher {
    fn event_handle(&self, record: KafkaRecord) -> Result<(), EventHandlerError> {
        match record.payload {
            KafkaPayload::PlaceOrderAccepted(event) => {
                let match_output: MatchOutput = self.place_order_event_handler.event_handle(event)?;
                if let Some(trade_event) = emit_trade_created_event(&match_output) {
                    self.pending_records
                        .lock()
                        .map_err(|_| EventHandlerError("pending kafka queue poisoned".into()))?
                        .push_back(KafkaRecord {
                            topic: KAFKA_TOPIC_TRADE_CREATED,
                            key: trade_event.trade_id.clone(),
                            payload: KafkaPayload::TradeCreated(trade_event),
                        });
                }
                Ok(())
            }
            KafkaPayload::TradeCreated(event) => {
                let settlement_result = self.trade_event_handler.event_handle(event)?;
                *self
                    .settlement_result
                    .lock()
                    .map_err(|_| EventHandlerError("kafka settlement result lock poisoned".into()))? =
                    Some(settlement_result);
                Ok(())
            }
        }
    }
}

pub struct KafkaEventActor {
    records: VecDeque<KafkaRecord>,
    dispatcher: KafkaDispatcher,
}

impl KafkaEventActor {
    pub fn new(records: VecDeque<KafkaRecord>) -> Self {
        Self { records, dispatcher: KafkaDispatcher::new() }
    }

    pub fn into_result(self) -> Option<SettlementResult> {
        self.dispatcher.take_settlement_result()
    }
}

impl EventActor<KafkaRecord, EventHandlerError> for KafkaEventActor {
    fn handle_event(&self, record: KafkaRecord) -> Result<(), EventHandlerError> {
        self.dispatcher.event_handle(record)
    }

    fn recv_event(&mut self) -> Result<Option<KafkaRecord>, EventHandlerError> {
        if let Some(record) = self.dispatcher.take_pending_record() {
            return Ok(Some(record));
        }

        Ok(self.records.pop_front())
    }
}

pub fn run_event_actor_with_kafka_queue() -> Result<SettlementResult, EventHandlerError> {
    let first_event = build_first_place_order_event()?;

    let mut records = VecDeque::new();
    records.push_back(KafkaRecord {
        topic: KAFKA_TOPIC_ORDER_ACCEPTED,
        key: first_event.order_id.clone(),
        payload: KafkaPayload::PlaceOrderAccepted(first_event),
    });

    let join_handle = thread::spawn(move || {
        let mut actor = KafkaEventActor::new(records);
        actor.run()?;
        actor.into_result()
            .ok_or_else(|| EventHandlerError("kafka actor ended without settlement result".into()))
    });

    join_handle
        .join()
        .map_err(|_| EventHandlerError("kafka actor thread panicked".into()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kafka_actor_chain() {
        let result = run_event_actor_with_kafka_queue();
        assert!(result.is_ok());
    }
}
