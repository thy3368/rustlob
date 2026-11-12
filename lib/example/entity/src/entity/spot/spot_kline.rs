use base_types_v2::Symbol;
use common_entity::{
    Entity, EntityFieldChange, EntityMutationModel, FieldDiff, FourColorArchetype,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

const SPOT_KLINE_ENTITY_TYPE: u8 = 6;

/// 由历史现货成交事实派生出的固定周期 K 线读模型。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotKline {
    pub id: u64,
    pub symbol: Symbol,
    pub interval_ns: u64,
    pub open_time_ns: u64,
    pub close_time_ns: u64,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub close_price: Decimal,
    pub volume_base: Decimal,
    pub volume_quote: Decimal,
    pub trade_count: u64,
    pub first_trade_id: u64,
    pub last_trade_id: u64,
}

impl SpotKline {
    /// 从已校验的聚合事实构造完整周期快照。
    #[expect(clippy::too_many_arguments, reason = "K 线需要完整保留 OHLCV 与来源成交事实")]
    pub fn new(
        symbol: Symbol,
        interval_ns: u64,
        open_time_ns: u64,
        close_time_ns: u64,
        open_price: Decimal,
        high_price: Decimal,
        low_price: Decimal,
        close_price: Decimal,
        volume_base: Decimal,
        volume_quote: Decimal,
        trade_count: u64,
        first_trade_id: u64,
        last_trade_id: u64,
    ) -> Self {
        let id = crate::support::derived_entity_id(
            SPOT_KLINE_ENTITY_TYPE,
            &[&symbol.id().to_le_bytes(), &interval_ns.to_le_bytes(), &open_time_ns.to_le_bytes()],
        );
        Self {
            id,
            symbol,
            interval_ns,
            open_time_ns,
            close_time_ns,
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
}

impl FieldDiff for SpotKline {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("symbol", "", self.symbol.id().to_string()),
            EntityFieldChange::new("interval_ns", "", self.interval_ns.to_string()),
            EntityFieldChange::new("open_time_ns", "", self.open_time_ns.to_string()),
            EntityFieldChange::new("close_time_ns", "", self.close_time_ns.to_string()),
            EntityFieldChange::new("open_price", "", self.open_price.to_string()),
            EntityFieldChange::new("high_price", "", self.high_price.to_string()),
            EntityFieldChange::new("low_price", "", self.low_price.to_string()),
            EntityFieldChange::new("close_price", "", self.close_price.to_string()),
            EntityFieldChange::new("volume_base", "", self.volume_base.to_string()),
            EntityFieldChange::new("volume_quote", "", self.volume_quote.to_string()),
            EntityFieldChange::new("trade_count", "", self.trade_count.to_string()),
            EntityFieldChange::new("first_trade_id", "", self.first_trade_id.to_string()),
            EntityFieldChange::new("last_trade_id", "", self.last_trade_id.to_string()),
        ]
    }
    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl Entity for SpotKline {
    fn entity_id(&self) -> u64 {
        self.id
    }
    fn is_derived() -> bool {
        true
    }
    fn entity_type() -> u8 {
        SPOT_KLINE_ENTITY_TYPE
    }
    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::MomentInterval
    }
    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::DerivedReadModel
    }
    fn lsn(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "symbol" | "interval_ns" | "open_time_ns" | "close_time_ns" | "trade_count"
            | "first_trade_id" | "last_trade_id" => 1,
            _ => 0,
        }
    }
}

impl common_entity::Shard for SpotKline {
    fn partition(&self) -> u8 {
        self.symbol.id() as u8
    }
}

#[cfg(test)]
mod tests {
    use base_types_v2::Coin;
    use common_entity::{Entity, Shard};

    use super::*;

    fn kline() -> SpotKline {
        SpotKline::new(
            Symbol::new(10_001, Coin::BTC, Coin::USDT),
            60,
            120,
            179,
            Decimal::new(1005, 1),
            Decimal::new(1105, 1),
            Decimal::new(905, 1),
            Decimal::new(1055, 1),
            Decimal::new(55, 1),
            Decimal::new(52025, 2),
            3,
            1,
            3,
        )
    }

    #[test]
    fn structured_identity_replay_and_partition_are_stable() {
        let kline = kline();
        assert_eq!(kline.entity_id(), self::kline().entity_id());
        assert_eq!(kline.partition(), 10_001_u32 as u8);
        let event = kline.track_create_event().expect("create event");
        assert!(
            event
                .field_changes
                .iter()
                .any(|change| change.field_name_as_str().ok() == Some("volume_quote")
                    && change.new_value_bytes() == b"520.25")
        );
        assert!(
            event
                .field_changes
                .iter()
                .any(|change| change.field_name_as_str().ok() == Some("open_time_ns")
                    && change.new_value_bytes() == b"120")
        );
    }
}
