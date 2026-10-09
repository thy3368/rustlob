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
