use cmd_handler::EntityReplayableEvent;
use common_entity::Entity;
use example_core_use_case::{
    KlineAggregationError, KlineUpdate, SpotKlineAggregator, SpotKlineTradeInput, SpotTrade,
};
use thiserror::Error;

/// 从成交 created event 解析 K 线聚合所需的最小成交事实。
pub fn spot_kline_trade_input_from_event(
    event: &EntityReplayableEvent,
) -> Result<Option<SpotKlineTradeInput>, SpotTradeCreatedEventError> {
    if event.entity_type != SpotTrade::entity_type() || !event.is_created() {
        return Ok(None);
    }

    Ok(Some(SpotKlineTradeInput {
        symbol: required_string_field(event, "symbol")?,
        price: required_u64_field(event, "price")?,
        qty: required_u64_field(event, "qty")?,
        executed_at_ms: required_u64_field(event, "executed_at_ms")?,
    }))
}

/// 处理一笔成交 created event，并返回所有周期产生的 K 线更新。
///
/// 非 `SpotTrade` created event 被视为当前投影不关注的事件，返回空更新而不是错误。
pub fn handle_trade_created_event(
    event: &EntityReplayableEvent,
    aggregator: &mut SpotKlineAggregator,
) -> Result<Vec<KlineUpdate>, SpotTradeCreatedEventError> {
    let Some(trade) = spot_kline_trade_input_from_event(event)? else {
        return Ok(Vec::new());
    };

    aggregator.update_trade(&trade).map_err(SpotTradeCreatedEventError::Aggregation)
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

/// `handle_trade_created_event` 的语义别名，便于事件消费代码表达投影动作。
pub fn process_trade_created_event(
    event: &EntityReplayableEvent,
    aggregator: &mut SpotKlineAggregator,
) -> Result<Vec<KlineUpdate>, SpotTradeCreatedEventError> {
    handle_trade_created_event(event, aggregator)
}

/// 成交 created event 到 K 线输入的解析或聚合错误。
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SpotTradeCreatedEventError {
    /// 事件缺少 K 线投影所需字段。
    #[error("spot trade event is missing field `{0}`")]
    MissingField(&'static str),
    /// 事件字段不是合法 UTF-8。
    #[error("spot trade event field `{field}` is not valid UTF-8")]
    InvalidUtf8 { field: &'static str },
    /// 数字字段无法解析为无符号定点整数。
    #[error("spot trade event field `{field}` is not a valid u64: {value}")]
    InvalidNumber { field: &'static str, value: String },
    /// K 线聚合失败。
    #[error(transparent)]
    Aggregation(#[from] KlineAggregationError),
}

fn required_string_field(
    event: &EntityReplayableEvent,
    field: &'static str,
) -> Result<String, SpotTradeCreatedEventError> {
    let value = field_value(event, field).ok_or(SpotTradeCreatedEventError::MissingField(field))?;
    let value = std::str::from_utf8(value)
        .map_err(|_| SpotTradeCreatedEventError::InvalidUtf8 { field })?;
    Ok(value.to_owned())
}

fn required_u64_field(
    event: &EntityReplayableEvent,
    field: &'static str,
) -> Result<u64, SpotTradeCreatedEventError> {
    let value = field_value(event, field).ok_or(SpotTradeCreatedEventError::MissingField(field))?;
    let value = std::str::from_utf8(value)
        .map_err(|_| SpotTradeCreatedEventError::InvalidUtf8 { field })?;
    value
        .parse::<u64>()
        .map_err(|_| SpotTradeCreatedEventError::InvalidNumber { field, value: value.to_owned() })
}

fn field_value<'a>(event: &'a EntityReplayableEvent, field: &str) -> Option<&'a [u8]> {
    event.field_changes.iter().find_map(|change| {
        (change.field_name_as_str().ok() == Some(field)).then(|| change.new_value_bytes())
    })
}

#[cfg(test)]
mod tests {
    use common_entity::{EntityChangeType, ReplayFieldChange};
    use example_core_use_case::{KlineCandle, KlineInterval};

    use super::*;

    fn created_trade_event(fields: &[(&str, &str)]) -> EntityReplayableEvent {
        let mut event =
            EntityReplayableEvent::new_created(999_999, 1, 42, SpotTrade::entity_type());
        for (name, value) in fields {
            event.add_field_change(ReplayFieldChange::new(
                ReplayFieldChange::field_name_from_str(name),
                &[],
                value.as_bytes(),
                1,
            ));
        }
        event
    }

    fn trade_fields() -> [(&'static str, &'static str); 4] {
        [("symbol", "BTCUSDT"), ("price", "100"), ("qty", "2"), ("executed_at_ms", "1717171717000")]
    }

    #[test]
    fn first_trade_is_projected_without_using_event_timestamp() {
        let event = created_trade_event(&trade_fields());
        let mut aggregator = SpotKlineAggregator::with_intervals([KlineInterval::OneMinute]);
        let mut updates = Vec::new();

        on_trade_created(&event, &mut aggregator, |update| updates.push(update)).unwrap();

        assert_eq!(
            updates,
            vec![KlineUpdate::Updated(KlineCandle {
                symbol: "BTCUSDT".to_string(),
                interval: KlineInterval::OneMinute,
                window_start_ms: 1_717_171_680_000,
                open: 100,
                high: 100,
                low: 100,
                close: 100,
                volume: 2,
                num_trades: 1,
            })]
        );
    }

    #[test]
    fn same_window_updates_ohlcv_and_cross_window_closes_before_update() {
        let mut aggregator = SpotKlineAggregator::with_intervals([KlineInterval::OneMinute]);
        let first = created_trade_event(&trade_fields());
        let second = created_trade_event(&[
            ("symbol", "BTCUSDT"),
            ("price", "110"),
            ("qty", "3"),
            ("executed_at_ms", "1717171717050"),
        ]);
        let third = created_trade_event(&[
            ("symbol", "BTCUSDT"),
            ("price", "120"),
            ("qty", "1"),
            ("executed_at_ms", "1717171777000"),
        ]);

        assert_eq!(handle_trade_created_event(&first, &mut aggregator).unwrap().len(), 1);
        let same_window = handle_trade_created_event(&second, &mut aggregator).unwrap();
        assert_eq!(same_window.len(), 1);
        assert!(matches!(same_window[0], KlineUpdate::Updated(_)));
        let crossed_window = handle_trade_created_event(&third, &mut aggregator).unwrap();
        assert!(matches!(crossed_window[0], KlineUpdate::Closed(_)));
        assert!(matches!(crossed_window[1], KlineUpdate::Updated(_)));
    }

    #[test]
    fn non_trade_events_are_ignored() {
        let mut event = created_trade_event(&trade_fields());
        event.entity_type = 3;
        let mut aggregator = SpotKlineAggregator::new();

        assert!(handle_trade_created_event(&event, &mut aggregator).unwrap().is_empty());

        event.entity_type = SpotTrade::entity_type();
        event.change_type = EntityChangeType::Updated.as_tag();
        assert!(handle_trade_created_event(&event, &mut aggregator).unwrap().is_empty());
        assert!(aggregator.is_empty());
    }

    #[test]
    fn missing_and_invalid_fields_return_explicit_errors() {
        let mut aggregator = SpotKlineAggregator::new();
        let missing = created_trade_event(&[("symbol", "BTCUSDT"), ("price", "100"), ("qty", "2")]);
        assert_eq!(
            handle_trade_created_event(&missing, &mut aggregator),
            Err(SpotTradeCreatedEventError::MissingField("executed_at_ms"))
        );

        let invalid = created_trade_event(&[
            ("symbol", "BTCUSDT"),
            ("price", "not-a-number"),
            ("qty", "2"),
            ("executed_at_ms", "1717171717000"),
        ]);
        assert!(matches!(
            handle_trade_created_event(&invalid, &mut aggregator),
            Err(SpotTradeCreatedEventError::InvalidNumber { field: "price", .. })
        ));
    }

    #[test]
    fn zero_execution_time_is_rejected() {
        let event = created_trade_event(&[
            ("symbol", "BTCUSDT"),
            ("price", "100"),
            ("qty", "2"),
            ("executed_at_ms", "0"),
        ]);
        let mut aggregator = SpotKlineAggregator::new();

        assert_eq!(
            handle_trade_created_event(&event, &mut aggregator),
            Err(SpotTradeCreatedEventError::Aggregation(KlineAggregationError::ZeroExecutedAt))
        );
    }
}
