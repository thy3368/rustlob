use common_entity::{
    Entity, EntityReplayableEvent, EntityReplayableEventHandler, ReplayEventFieldParseError,
};
use example_core_use_case::{
    KlineAggregationError, KlineUpdate, SpotKlineAggregator, SpotKlineTradeInput, SpotTrade,
};
use thiserror::Error;

/// 成交 created event 到 K 线聚合的业务 handler。
#[derive(Debug, Default)]
pub struct SpotTradeCreatedEventHandler {
    aggregator: SpotKlineAggregator,
}

impl SpotTradeCreatedEventHandler {
    pub fn new(aggregator: SpotKlineAggregator) -> Self {
        Self { aggregator }
    }
}

impl EntityReplayableEventHandler for SpotTradeCreatedEventHandler {
    type Output = Vec<KlineUpdate>;
    type Error = SpotTradeCreatedEventError;

    fn handle(
        &mut self,
        event: &EntityReplayableEvent,
    ) -> Result<Option<Self::Output>, Self::Error> {
        handle_trade_created_event_optional(event, &mut self.aggregator)
    }
}

/// 从成交 created event 解析 K 线聚合所需的最小成交事实。
pub fn spot_kline_trade_input_from_event(
    event: &EntityReplayableEvent,
) -> Result<Option<SpotKlineTradeInput>, SpotTradeCreatedEventError> {
    if event.entity_type != SpotTrade::entity_type() || !event.is_created() {
        return Ok(None);
    }

    Ok(Some(SpotKlineTradeInput {
        symbol: event.required_string_field("symbol")?,
        price: event.required_u64_field("price")?,
        qty: event.required_u64_field("qty")?,
        executed_at_ms: event.required_u64_field("executed_at_ms")?,
    }))
}

/// 处理一笔成交 created event，并返回所有周期产生的 K 线更新。
///
/// 非 `SpotTrade` created event 被视为当前投影不关注的事件，返回空更新而不是错误。
pub fn handle_trade_created_event(
    event: &EntityReplayableEvent,
    aggregator: &mut SpotKlineAggregator,
) -> Result<Vec<KlineUpdate>, SpotTradeCreatedEventError> {
    handle_trade_created_event_optional(event, aggregator)
        .map(|updates| updates.unwrap_or_default())
}

fn handle_trade_created_event_optional(
    event: &EntityReplayableEvent,
    aggregator: &mut SpotKlineAggregator,
) -> Result<Option<Vec<KlineUpdate>>, SpotTradeCreatedEventError> {
    let Some(trade) = spot_kline_trade_input_from_event(event)? else {
        return Ok(None);
    };

    aggregator.update_trade(&trade).map(Some).map_err(SpotTradeCreatedEventError::Aggregation)
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

impl From<ReplayEventFieldParseError> for SpotTradeCreatedEventError {
    fn from(error: ReplayEventFieldParseError) -> Self {
        match error {
            ReplayEventFieldParseError::MissingField(field) => Self::MissingField(field),
            ReplayEventFieldParseError::InvalidUtf8 { field } => Self::InvalidUtf8 { field },
            ReplayEventFieldParseError::InvalidNumber { field, value, .. } => {
                Self::InvalidNumber { field, value }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{EntityChangeType, ReplayFieldChange};
    use example_core_use_case::{KlineCandle, KlineInterval};

    use super::*;

    fn created_trade_event(fields: &[(&str, &[u8])]) -> EntityReplayableEvent {
        let mut event =
            EntityReplayableEvent::new_created(999_999, 1, 42, SpotTrade::entity_type());
        for (name, value) in fields {
            event.add_field_change(ReplayFieldChange::new(
                ReplayFieldChange::field_name_from_str(name),
                &[],
                value,
                1,
            ));
        }
        event
    }

    fn trade_fields() -> [(&'static str, &'static [u8]); 4] {
        [
            ("symbol", b"BTCUSDT"),
            ("price", b"100"),
            ("qty", b"2"),
            ("executed_at_ms", b"1717171717000"),
        ]
    }

    #[test]
    fn non_trade_events_are_ignored() {
        let mut event = created_trade_event(&trade_fields());
        event.entity_type = 3;
        let mut handler = SpotTradeCreatedEventHandler::default();
        let mut aggregator = SpotKlineAggregator::default();

        assert_eq!(handler.handle(&event).unwrap(), None);
        assert!(handle_trade_created_event(&event, &mut aggregator).unwrap().is_empty());

        event.entity_type = SpotTrade::entity_type();
        event.change_type = EntityChangeType::Updated.as_tag();
        assert_eq!(handler.handle(&event).unwrap(), None);
    }

    #[test]
    fn first_trade_is_projected_without_using_event_timestamp() {
        let event = created_trade_event(&trade_fields());
        let mut handler = SpotTradeCreatedEventHandler::new(SpotKlineAggregator::with_intervals([
            KlineInterval::OneMinute,
        ]));

        let updates = handler.handle(&event).unwrap().unwrap();

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
            ("symbol", b"BTCUSDT"),
            ("price", b"110"),
            ("qty", b"3"),
            ("executed_at_ms", b"1717171717050"),
        ]);
        let third = created_trade_event(&[
            ("symbol", b"BTCUSDT"),
            ("price", b"120"),
            ("qty", b"1"),
            ("executed_at_ms", b"1717171777000"),
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
    fn missing_and_invalid_fields_return_explicit_errors() {
        let mut aggregator = SpotKlineAggregator::new();
        let missing =
            created_trade_event(&[("symbol", b"BTCUSDT"), ("price", b"100"), ("qty", b"2")]);
        assert_eq!(
            handle_trade_created_event(&missing, &mut aggregator),
            Err(SpotTradeCreatedEventError::MissingField("executed_at_ms"))
        );

        let invalid_number = created_trade_event(&[
            ("symbol", b"BTCUSDT"),
            ("price", b"not-a-number"),
            ("qty", b"2"),
            ("executed_at_ms", b"1717171717000"),
        ]);
        assert!(matches!(
            handle_trade_created_event(&invalid_number, &mut aggregator),
            Err(SpotTradeCreatedEventError::InvalidNumber { field: "price", .. })
        ));

        let invalid_utf8 = created_trade_event(&[
            ("symbol", &[0xff]),
            ("price", b"100"),
            ("qty", b"2"),
            ("executed_at_ms", b"1717171717000"),
        ]);
        assert_eq!(
            handle_trade_created_event(&invalid_utf8, &mut aggregator),
            Err(SpotTradeCreatedEventError::InvalidUtf8 { field: "symbol" })
        );
    }

    #[test]
    fn zero_execution_time_is_rejected() {
        let event = created_trade_event(&[
            ("symbol", b"BTCUSDT"),
            ("price", b"100"),
            ("qty", b"2"),
            ("executed_at_ms", b"0"),
        ]);
        let mut aggregator = SpotKlineAggregator::new();

        assert_eq!(
            handle_trade_created_event(&event, &mut aggregator),
            Err(SpotTradeCreatedEventError::Aggregation(KlineAggregationError::ZeroExecutedAt))
        );
    }
}
