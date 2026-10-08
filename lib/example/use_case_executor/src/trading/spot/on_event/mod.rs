mod match_spot_order_created;
mod spot_trade_created;

use common_entity::EntityReplayableEvent;
pub use match_spot_order_created::{
    DefaultMatchSpotOrderCreatedEventError, DefaultMatchSpotOrderCreatedEventOutput,
    MatchSpotOrderCreatedEventError, MatchSpotOrderCreatedEventHandler,
    MatchSpotOrderCreatedEventParseError, MatchSpotOrderExecutorError,
    match_spot_order_command_from_event,
};
pub use spot_trade_created::{
    SpotTradeCreatedEventError, SpotTradeCreatedEventHandler, handle_trade_created_event,
    process_trade_created_event, spot_kline_trade_input_from_event,
};

/// 统一处理可回放实体事件的静态 handler 抽象。
///
/// 非目标实体或非目标变更类型返回 `Ok(None)`；目标事件解析失败或业务执行失败返回
/// `Err(...)`。
pub trait EntityReplayableEventHandler {
    type Output;
    type Error;

    fn handle(
        &mut self,
        event: &EntityReplayableEvent,
    ) -> Result<Option<Self::Output>, Self::Error>;
}
