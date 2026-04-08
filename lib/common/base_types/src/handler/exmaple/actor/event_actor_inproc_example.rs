//! EventActor in-process channel 示例。

use std::sync::mpsc::{self, Receiver, Sender};
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

pub struct InProcMatchStageDispatcher {
    place_order_event_handler: PlaceOrderEventHandler,
    trade_sender: Sender<TradeCreatedEvent>,
}

impl InProcMatchStageDispatcher {
    pub fn new(trade_sender: Sender<TradeCreatedEvent>) -> Self {
        Self {
            place_order_event_handler: PlaceOrderEventHandler::new(MatchHandler::new()),
            trade_sender,
        }
    }
}

impl EventHandler<PlaceOrderAcceptedEvent, (), EventHandlerError> for InProcMatchStageDispatcher {
    fn event_handle(&self, event: PlaceOrderAcceptedEvent) -> Result<(), EventHandlerError> {
        let match_output: MatchOutput = self.place_order_event_handler.event_handle(event)?;
        if let Some(trade_event) = emit_trade_created_event(&match_output) {
            self.trade_sender
                .send(trade_event)
                .map_err(|err| EventHandlerError(err.to_string()))?;
        }
        Ok(())
    }
}

pub struct InProcSettlementStageDispatcher {
    trade_event_handler: TradeEventHandler,
    settlement_result: std::sync::Mutex<Option<SettlementResult>>,
}

impl InProcSettlementStageDispatcher {
    pub fn new() -> Self {
        Self {
            trade_event_handler: TradeEventHandler::new(SettlementHandler::new()),
            settlement_result: std::sync::Mutex::new(None),
        }
    }

    pub fn take_settlement_result(&self) -> Option<SettlementResult> {
        self.settlement_result.lock().ok()?.take()
    }
}

impl EventHandler<TradeCreatedEvent, (), EventHandlerError> for InProcSettlementStageDispatcher {
    fn event_handle(&self, event: TradeCreatedEvent) -> Result<(), EventHandlerError> {
        let settlement_result = self.trade_event_handler.event_handle(event)?;
        *self
            .settlement_result
            .lock()
            .map_err(|_| EventHandlerError("settlement result lock poisoned".into()))? =
            Some(settlement_result);
        Ok(())
    }
}

// 撮合阶段
pub struct InProcMatchStageActor {
    receiver: Receiver<PlaceOrderAcceptedEvent>,
    dispatcher: InProcMatchStageDispatcher,
}

impl InProcMatchStageActor {
    pub fn new(
        receiver: Receiver<PlaceOrderAcceptedEvent>,
        trade_sender: Sender<TradeCreatedEvent>,
    ) -> Self {
        Self {
            receiver,
            dispatcher: InProcMatchStageDispatcher::new(trade_sender),
        }
    }
}

impl EventRecvActor<PlaceOrderAcceptedEvent, EventHandlerError> for InProcMatchStageActor {
    fn handle_event(&self, event: PlaceOrderAcceptedEvent) -> Result<(), EventHandlerError> {
        self.dispatcher.event_handle(event)
    }

    fn recv_event(&mut self) -> Result<Option<PlaceOrderAcceptedEvent>, EventHandlerError> {
        match self.receiver.recv() {
            Ok(event) => Ok(Some(event)),
            Err(_) => Ok(None),
        }
    }
}


// 结算阶段
pub struct InProcSettlementStageActor {
    receiver: Receiver<TradeCreatedEvent>,
    dispatcher: InProcSettlementStageDispatcher,
}

impl InProcSettlementStageActor {
    pub fn new(receiver: Receiver<TradeCreatedEvent>) -> Self {
        Self { receiver, dispatcher: InProcSettlementStageDispatcher::new() }
    }

    pub fn into_result(self) -> Option<SettlementResult> {
        self.dispatcher.take_settlement_result()
    }
}

impl EventRecvActor<TradeCreatedEvent, EventHandlerError> for InProcSettlementStageActor {
    fn handle_event(&self, event: TradeCreatedEvent) -> Result<(), EventHandlerError> {
        self.dispatcher.event_handle(event)
    }

    fn recv_event(&mut self) -> Result<Option<TradeCreatedEvent>, EventHandlerError> {
        match self.receiver.recv() {
            Ok(event) => Ok(Some(event)),
            Err(_) => Ok(None),
        }
    }
}

pub fn run_event_actor_with_inproc_channel() -> Result<SettlementResult, EventHandlerError> {
    let first_event = build_first_place_order_event()?;

    let (match_stage_sender, match_stage_receiver) = mpsc::channel();
    let (settlement_stage_sender, settlement_stage_receiver) = mpsc::channel();

    let match_stage_join_handle = thread::spawn(move || {
        let mut actor = InProcMatchStageActor::new(match_stage_receiver, settlement_stage_sender);
        actor.run()
    });

    let settlement_stage_join_handle = thread::spawn(move || {
        let mut actor = InProcSettlementStageActor::new(settlement_stage_receiver);
        actor.run()?;
        actor.into_result()
            .ok_or_else(|| EventHandlerError("settlement-stage actor ended without settlement result".into()))
    });

    match_stage_sender
        .send(first_event)
        .map_err(|err| EventHandlerError(err.to_string()))?;
    drop(match_stage_sender);

    match_stage_join_handle
        .join()
        .map_err(|_| EventHandlerError("match-stage actor thread panicked".into()))??;

    settlement_stage_join_handle
        .join()
        .map_err(|_| EventHandlerError("settlement-stage actor thread panicked".into()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inproc_channel_actor_chain() {
        let result = run_event_actor_with_inproc_channel();
        assert!(result.is_ok());
    }
}
