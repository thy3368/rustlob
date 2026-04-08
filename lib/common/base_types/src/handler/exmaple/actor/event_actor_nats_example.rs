//! EventActor NATS-style 示例。

use std::collections::VecDeque;
use std::thread;

use crate::handler::event_actor::EventRecvActor;
use crate::handler::event_handler::EventHandler;
use crate::handler::exmaple::cmd_handler::match_handler::{MatchHandler, MatchOutput, TradeCreatedEvent};
use crate::handler::exmaple::cmd_handler::place_order_handler::PlaceOrderAcceptedEvent;
use crate::handler::exmaple::cmd_handler::settlement_handler::{SettlementHandler, SettlementResult};
use crate::handler::exmaple::actor::event_actor_example_shared::build_first_place_order_event;
use crate::handler::exmaple::event_handler::event_template::{
    emit_trade_created_event, EventHandlerError, PlaceOrderEventHandler, TradeEventHandler,
};

pub enum NatsPayload {
    PlaceOrderAccepted(PlaceOrderAcceptedEvent),
    TradeCreated(TradeCreatedEvent),
}

pub struct NatsMessage {
    pub subject: &'static str,
    pub reply_to: Option<String>,
    pub payload: NatsPayload,
}

pub const NATS_SUBJECT_ORDER_ACCEPTED: &str = "orders.accepted";
pub const NATS_SUBJECT_TRADE_CREATED: &str = "trades.created";

pub struct NatsDispatcher {
    place_order_event_handler: PlaceOrderEventHandler,
    trade_event_handler: TradeEventHandler,
    pending_messages: std::sync::Mutex<VecDeque<NatsMessage>>,
    settlement_result: std::sync::Mutex<Option<SettlementResult>>,
}

impl NatsDispatcher {
    pub fn new() -> Self {
        Self {
            place_order_event_handler: PlaceOrderEventHandler::new(MatchHandler::new()),
            trade_event_handler: TradeEventHandler::new(SettlementHandler::new()),
            pending_messages: std::sync::Mutex::new(VecDeque::new()),
            settlement_result: std::sync::Mutex::new(None),
        }
    }

    pub fn take_pending_message(&self) -> Option<NatsMessage> {
        self.pending_messages.lock().ok()?.pop_front()
    }

    pub fn take_settlement_result(&self) -> Option<SettlementResult> {
        self.settlement_result.lock().ok()?.take()
    }
}

impl EventHandler<NatsMessage, (), EventHandlerError> for NatsDispatcher {
    fn event_handle(&self, message: NatsMessage) -> Result<(), EventHandlerError> {
        match message.payload {
            NatsPayload::PlaceOrderAccepted(event) => {
                let match_output: MatchOutput = self.place_order_event_handler.event_handle(event)?;
                if let Some(trade_event) = emit_trade_created_event(&match_output) {
                    self.pending_messages
                        .lock()
                        .map_err(|_| EventHandlerError("pending nats queue poisoned".into()))?
                        .push_back(NatsMessage {
                            subject: NATS_SUBJECT_TRADE_CREATED,
                            reply_to: None,
                            payload: NatsPayload::TradeCreated(trade_event),
                        });
                }
                Ok(())
            }
            NatsPayload::TradeCreated(event) => {
                let settlement_result = self.trade_event_handler.event_handle(event)?;
                *self
                    .settlement_result
                    .lock()
                    .map_err(|_| EventHandlerError("nats settlement result lock poisoned".into()))? =
                    Some(settlement_result);
                Ok(())
            }
        }
    }
}

pub struct NatsEventActor {
    messages: VecDeque<NatsMessage>,
    dispatcher: NatsDispatcher,
}

impl NatsEventActor {
    pub fn new(messages: VecDeque<NatsMessage>) -> Self {
        Self { messages, dispatcher: NatsDispatcher::new() }
    }

    pub fn into_result(self) -> Option<SettlementResult> {
        self.dispatcher.take_settlement_result()
    }
}

impl EventRecvActor<NatsMessage, EventHandlerError> for NatsEventActor {
    fn handle_event(&self, message: NatsMessage) -> Result<(), EventHandlerError> {
        self.dispatcher.event_handle(message)
    }

    fn recv_event(&mut self) -> Result<Option<NatsMessage>, EventHandlerError> {
        if let Some(message) = self.dispatcher.take_pending_message() {
            return Ok(Some(message));
        }

        Ok(self.messages.pop_front())
    }
}

pub fn run_event_actor_with_nats_queue() -> Result<SettlementResult, EventHandlerError> {
    let first_event = build_first_place_order_event()?;

    let mut messages = VecDeque::new();
    messages.push_back(NatsMessage {
        subject: NATS_SUBJECT_ORDER_ACCEPTED,
        reply_to: None,
        payload: NatsPayload::PlaceOrderAccepted(first_event),
    });

    let join_handle = thread::spawn(move || {
        let mut actor = NatsEventActor::new(messages);
        actor.run()?;
        actor.into_result()
            .ok_or_else(|| EventHandlerError("nats actor ended without settlement result".into()))
    });

    join_handle
        .join()
        .map_err(|_| EventHandlerError("nats actor thread panicked".into()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nats_actor_chain() {
        let result = run_event_actor_with_nats_queue();
        assert!(result.is_ok());
    }
}
