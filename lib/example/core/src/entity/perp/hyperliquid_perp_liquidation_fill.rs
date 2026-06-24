use common_entity::{Entity, EntityError, EntityFieldChange};

use crate::entity::HyperliquidPerpPositionSide;

const HYPERLIQUID_PERP_LIQUIDATION_FILL_ENTITY_TYPE: u8 = 15;

/// 一次 Hyperliquid perp 强平执行成交形成的 append-only 事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpLiquidationFill {
    pub liquidation_fill_id: String,
    pub liquidation_id: String,
    pub order_id: String,
    pub trade_id: String,
    pub account_id: String,
    pub position_id: String,
    pub asset: u32,
    pub symbol: String,
    pub side: HyperliquidPerpPositionSide,
    pub fill_qty: u64,
    pub fill_price: u64,
    pub bankruptcy_price: u64,
    pub recovered_quote: u64,
}

impl HyperliquidPerpLiquidationFill {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        liquidation_fill_id: String,
        liquidation_id: String,
        order_id: String,
        trade_id: String,
        account_id: String,
        position_id: String,
        asset: u32,
        symbol: String,
        side: HyperliquidPerpPositionSide,
        fill_qty: u64,
        fill_price: u64,
        bankruptcy_price: u64,
        recovered_quote: u64,
    ) -> Self {
        Self {
            liquidation_fill_id,
            liquidation_id,
            order_id,
            trade_id,
            account_id,
            position_id,
            asset,
            symbol,
            side,
            fill_qty,
            fill_price,
            bankruptcy_price,
            recovered_quote,
        }
    }
}

impl Entity for HyperliquidPerpLiquidationFill {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.liquidation_fill_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_LIQUIDATION_FILL_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("liquidation_fill_id", "", self.liquidation_fill_id.clone()),
            EntityFieldChange::new("liquidation_id", "", self.liquidation_id.clone()),
            EntityFieldChange::new("order_id", "", self.order_id.clone()),
            EntityFieldChange::new("trade_id", "", self.trade_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("position_id", "", self.position_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("fill_qty", "", self.fill_qty.to_string()),
            EntityFieldChange::new("fill_price", "", self.fill_price.to_string()),
            EntityFieldChange::new("bankruptcy_price", "", self.bankruptcy_price.to_string()),
            EntityFieldChange::new("recovered_quote", "", self.recovered_quote.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "liquidation_fill_id"
            | "liquidation_id"
            | "order_id"
            | "trade_id"
            | "account_id"
            | "position_id"
            | "symbol"
            | "side" => 0,
            "asset" | "fill_qty" | "fill_price" | "bankruptcy_price" | "recovered_quote" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.liquidation_fill_id))
    }
}

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    #[test]
    fn create_event_contains_fill_fields() {
        let fill = HyperliquidPerpLiquidationFill::new(
            "liq-fill-1".to_string(),
            "liq-1".to_string(),
            "order-1".to_string(),
            "trade-1".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            2,
            48_500,
            47_500,
            97_000,
        );

        let event = fill.track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("liquidation_fill_id")
                && change.new_value_bytes() == b"liq-fill-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("recovered_quote")
                && change.new_value_bytes() == b"97000"
        }));
    }
}
