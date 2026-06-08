use common_entity::{Entity, EntityError, EntityFieldChange};
use crate::HyperliquidPerpOrderSide;

const HYPERLIQUID_PERP_TRADE_ENTITY_TYPE: u8 = 10;

/// 已完成撮合的一笔 Hyperliquid perp 成交事实。
///
/// 该实体只记录订单撮合结果，不表达仓位、保证金、手续费、PnL 或资金费。
/// 构造器假定输入已经由撮合 use case 校验。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpTrade {
    /// 本系统稳定成交 ID。
    pub trade_id: String,
    /// 一次撮合批次 ID。
    pub match_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 主动吃单订单 ID。
    pub taker_order_id: String,
    /// 被动挂单订单 ID。
    pub maker_order_id: String,
    /// taker 所属账户 ID。
    pub taker_account_id: String,
    /// maker 所属账户 ID。
    pub maker_account_id: String,
    /// taker 买卖方向。
    pub taker_side: HyperliquidPerpOrderSide,
    /// 成交价格，取 maker 限价。
    pub price: u64,
    /// 成交数量。
    pub qty: u64,
}

impl HyperliquidPerpTrade {
    /// 从已经校验过的撮合事实构造成交实体。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trade_id: String,
        match_id: String,
        asset: u32,
        symbol: String,
        taker_order_id: String,
        maker_order_id: String,
        taker_account_id: String,
        maker_account_id: String,
        taker_side: HyperliquidPerpOrderSide,
        price: u64,
        qty: u64,
    ) -> Self {
        Self {
            trade_id,
            match_id,
            asset,
            symbol,
            taker_order_id,
            maker_order_id,
            taker_account_id,
            maker_account_id,
            taker_side,
            price,
            qty,
        }
    }

    /// 返回成交 quote 名义价值；乘法溢出时返回 `None`。
    pub fn notional_quote(&self) -> Option<u64> {
        self.price.checked_mul(self.qty)
    }
}

impl Entity for HyperliquidPerpTrade {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.trade_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_TRADE_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("trade_id", "", self.trade_id.clone()),
            EntityFieldChange::new("match_id", "", self.match_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("taker_order_id", "", self.taker_order_id.clone()),
            EntityFieldChange::new("maker_order_id", "", self.maker_order_id.clone()),
            EntityFieldChange::new("taker_account_id", "", self.taker_account_id.clone()),
            EntityFieldChange::new("maker_account_id", "", self.maker_account_id.clone()),
            EntityFieldChange::new("taker_side", "", self.taker_side.as_str()),
            EntityFieldChange::new("price", "", self.price.to_string()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "trade_id" | "match_id" | "symbol" | "taker_order_id" | "maker_order_id"
            | "taker_account_id" | "maker_account_id" | "taker_side" => 0,
            "asset" | "price" | "qty" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.trade_id))
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
    fn create_event_contains_trade_fields() {
        let trade = HyperliquidPerpTrade::new(
            "match-1-1".to_string(),
            "match-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            "taker-1".to_string(),
            "maker-1".to_string(),
            "buyer".to_string(),
            "seller".to_string(),
            HyperliquidPerpOrderSide::Buy,
            100,
            2,
        );

        let event = trade.track_create_event().unwrap();

        assert!(event.is_created());
        assert_eq!(trade.notional_quote(), Some(200));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("trade_id")
                && change.new_value_bytes() == b"match-1-1"
        }));
    }
}
