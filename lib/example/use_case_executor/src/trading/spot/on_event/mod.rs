mod match_spot_order_created;
mod spot_trade_created;
mod trigger_pending_spot_orders_on_trade;

pub use match_spot_order_created::{
    DefaultMatchSpotOrderCreatedEventError, DefaultMatchSpotOrderCreatedEventOutput,
    MatchSpotOrderCreatedEventError, MatchSpotOrderCreatedEventHandler,
    MatchSpotOrderCreatedEventParseError, MatchSpotOrderExecutorError,
    match_spot_order_command_from_event,
};
pub use spot_trade_created::{
    SpotTradeCreatedEventError, SpotTradeCreatedEventHandler, handle_trade_created_event,
    spot_kline_trade_input_from_event,
};
pub use trigger_pending_spot_orders_on_trade::{
    SpotTradeTriggerInput, SpotTradeTriggerInputParseError, TriggerPendingSpotOrdersOnTradeError,
    TriggerPendingSpotOrdersOnTradeHandler, TriggerPendingSpotOrdersOnTradeOutput,
    spot_trade_trigger_input_from_event,
};
