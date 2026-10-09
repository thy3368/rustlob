use cmd_handler::EntityReplayableEvent;
use example_core_use_case::{KlineUpdate, SpotKlineAggregator};
use use_case_executor::trading::spot::on_event::handle_trade_created_event as execute_trade_created_event;
pub use use_case_executor::trading::spot::on_event::{
    SpotTradeCreatedEventError, spot_kline_trade_input_from_event,
};

/// 处理一笔成交 created event，并返回所有周期产生的 K 线更新。
///
/// 业务解析与 K 线聚合由 use_case_executor 侧 handler 负责；adapter 只保留兼容入口。
pub fn handle_trade_created_event(
    event: &EntityReplayableEvent,
    aggregator: &mut SpotKlineAggregator,
) -> Result<Vec<KlineUpdate>, SpotTradeCreatedEventError> {
    execute_trade_created_event(event, aggregator)
}

/// 处理成交 created event，并通过回调把 K 线更新交给后续 outbound adapter。
pub fn on_trade_created<F>(
    event: &EntityReplayableEvent,
    aggregator: &mut SpotKlineAggregator,
    mut publish_update: F,
) -> Result<(), SpotTradeCreatedEventError>
where
    F: FnMut(KlineUpdate),
{
    for update in handle_trade_created_event(event, aggregator)? {
        publish_update(update);
    }
    Ok(())
}
