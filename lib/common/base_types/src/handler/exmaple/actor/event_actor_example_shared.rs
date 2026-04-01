use crate::handler::exmaple::cmd_handler::place_order_handler::{
    OrderSide, OrderType, PlaceOrderAcceptedEvent, PlaceOrderCmd, PlaceOrderEvent,
    PlaceOrderHandler, PlaceOrderOutput, PlaceOrderResult,
};
use crate::handler::exmaple::event_handler::event_template::{
    emit_place_order_event, EventHandlerError,
};
use crate::handler::handler_update::CmdHandlerForUpdate;

pub fn build_first_place_order_event() -> Result<PlaceOrderAcceptedEvent, EventHandlerError> {
    let place_order_handler = PlaceOrderHandler::new();
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

    let output = place_order_handler
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

    emit_place_order_event(&output)
        .ok_or_else(|| EventHandlerError("missing place order event".into()))
}
