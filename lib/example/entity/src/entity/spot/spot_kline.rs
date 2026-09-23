use common_entity::{
    Entity, EntityError, EntityFieldChange, EntityMutationModel, FieldDiff, FourColorArchetype,
};
use serde::{Deserialize, Serialize};

const SPOT_KLINE_ENTITY_TYPE: u8 = 6;

/// 由历史现货成交事实派生出的固定周期 K 线快照。
///
/// `SpotKline` 不是成交事实来源，而是可按成交事实重新投影的行情读模型。
/// 同一交易对、周期和开盘时间只对应一个稳定实体 ID。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotKline {
    /// Hyperliquid 现货资产编号。
    pub asset: u32,
    /// 交易对。
    pub symbol: String,
    /// K 线周期，单位毫秒。
    pub interval_ms: u64,
    /// K 线开盘时间，单位毫秒。
    pub open_time_ms: u64,
    /// K 线收盘时间，单位毫秒，包含在当前周期内。
    pub close_time_ms: u64,
    /// 周期内首笔成交价格。
    pub open_price: u64,
    /// 周期内最高成交价格。
    pub high_price: u64,
    /// 周期内最低成交价格。
    pub low_price: u64,
    /// 周期内末笔成交价格。
    pub close_price: u64,
    /// base asset 成交量。
    pub volume_base: u64,
    /// quote asset 成交量。
    pub volume_quote: u64,
    /// 周期内成交笔数。
    pub trade_count: u64,
    /// 按成交时间和成交 ID 排序后的首笔成交 ID。
    pub first_trade_id: String,
    /// 按成交时间和成交 ID 排序后的末笔成交 ID。
    pub last_trade_id: String,
}

impl SpotKline {
    /// 从已经完成校验的聚合事实构造 K 线快照。
    #[expect(clippy::too_many_arguments, reason = "K 线快照需要完整保留 OHLCV 与来源成交事实")]
    pub fn new(
        asset: u32,
        symbol: String,
        interval_ms: u64,
        open_time_ms: u64,
        close_time_ms: u64,
        open_price: u64,
        high_price: u64,
        low_price: u64,
        close_price: u64,
        volume_base: u64,
        volume_quote: u64,
        trade_count: u64,
        first_trade_id: String,
        last_trade_id: String,
    ) -> Self {
        Self {
            asset,
            symbol,
            interval_ms,
            open_time_ms,
            close_time_ms,
            open_price,
            high_price,
            low_price,
            close_price,
            volume_base,
            volume_quote,
            trade_count,
            first_trade_id,
            last_trade_id,
        }
    }

    /// 返回 K 线快照的确定性业务身份。
    pub fn kline_key(&self) -> String {
        kline_key(self.symbol.as_str(), self.interval_ms, self.open_time_ms)
    }
}

impl FieldDiff for SpotKline {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("interval_ms", "", self.interval_ms.to_string()),
            EntityFieldChange::new("open_time_ms", "", self.open_time_ms.to_string()),
            EntityFieldChange::new("close_time_ms", "", self.close_time_ms.to_string()),
            EntityFieldChange::new("open_price", "", self.open_price.to_string()),
            EntityFieldChange::new("high_price", "", self.high_price.to_string()),
            EntityFieldChange::new("low_price", "", self.low_price.to_string()),
            EntityFieldChange::new("close_price", "", self.close_price.to_string()),
            EntityFieldChange::new("volume_base", "", self.volume_base.to_string()),
            EntityFieldChange::new("volume_quote", "", self.volume_quote.to_string()),
            EntityFieldChange::new("trade_count", "", self.trade_count.to_string()),
            EntityFieldChange::new("first_trade_id", "", self.first_trade_id.clone()),
            EntityFieldChange::new("last_trade_id", "", self.last_trade_id.clone()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::with_capacity(0)
    }
}

impl Entity for SpotKline {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.kline_key()
    }

    fn entity_type() -> u8 {
        SPOT_KLINE_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel
    where
        Self: Sized,
    {
        EntityMutationModel::DerivedReadModel
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "symbol" | "first_trade_id" | "last_trade_id" => 0,
            "asset" | "interval_ms" | "open_time_ms" | "close_time_ms" | "open_price"
            | "high_price" | "low_price" | "close_price" | "volume_base" | "volume_quote"
            | "trade_count" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(self.entity_id().as_str()))
    }
}

fn kline_key(symbol: &str, interval_ms: u64, open_time_ms: u64) -> String {
    format!("spot-kline:{symbol}:{interval_ms}:{open_time_ms}")
}

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, EntityMutationModel, FourColorArchetype};

    use super::*;

    fn kline() -> SpotKline {
        SpotKline::new(
            10_001,
            "BTCUSDT".to_string(),
            60_000,
            1_717_171_680_000,
            1_717_171_739_999,
            100,
            110,
            90,
            105,
            5,
            520,
            3,
            "trade-1".to_string(),
            "trade-3".to_string(),
        )
    }

    #[test]
    fn identity_is_deterministic_from_symbol_interval_and_open_time() {
        let kline = kline();

        assert_eq!(kline.entity_id(), "spot-kline:BTCUSDT:60000:1717171680000");
        assert_eq!(kline.kline_key(), kline.entity_id());
    }

    #[test]
    fn derived_snapshot_replays_all_business_fields() {
        let kline = kline();

        assert_eq!(SpotKline::four_color_archetype(), FourColorArchetype::MomentInterval);
        assert_eq!(SpotKline::mutation_model(), EntityMutationModel::DerivedReadModel);

        let event = kline.track_create_event().expect("kline create event");
        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("volume_quote")
                && change.new_value_bytes() == b"520"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("first_trade_id")
                && change.new_value_bytes() == b"trade-1"
        }));
    }
}
