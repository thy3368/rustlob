use common_entity::{Entity, EntityError, EntityFieldChange};

const HYPERLIQUID_PERP_SETTLEMENT_ENTITY_TYPE: u8 = 12;

/// 一笔 Hyperliquid perp 成交完成清结算后的事实记录。
///
/// 该实体记录成交落仓位、保证金重算、手续费和已实现 PnL 的结果摘要。
/// 构造器假定输入已经由清结算 use case 校验。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpSettlement {
    /// 本系统稳定清算记录 ID。
    pub settlement_id: String,
    /// 被清结算的成交 ID。
    pub trade_id: String,
    /// 成交所属撮合批次 ID。
    pub match_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 成交后多头方向账户 ID。
    pub long_account_id: String,
    /// 成交后空头方向账户 ID。
    pub short_account_id: String,
    /// 成交价格。
    pub price: u64,
    /// 成交数量。
    pub qty: u64,
    /// 成交名义价值。
    pub notional: u64,
    /// taker 支付手续费。
    pub taker_fee: u64,
    /// maker 支付手续费。
    pub maker_fee: u64,
    /// taker 本笔实现 PnL。
    pub taker_realized_pnl: i64,
    /// maker 本笔实现 PnL。
    pub maker_realized_pnl: i64,
}

impl HyperliquidPerpSettlement {
    /// 从已经校验过的清结算事实构造记录。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        settlement_id: String,
        trade_id: String,
        match_id: String,
        asset: u32,
        symbol: String,
        long_account_id: String,
        short_account_id: String,
        price: u64,
        qty: u64,
        notional: u64,
        taker_fee: u64,
        maker_fee: u64,
        taker_realized_pnl: i64,
        maker_realized_pnl: i64,
    ) -> Self {
        Self {
            settlement_id,
            trade_id,
            match_id,
            asset,
            symbol,
            long_account_id,
            short_account_id,
            price,
            qty,
            notional,
            taker_fee,
            maker_fee,
            taker_realized_pnl,
            maker_realized_pnl,
        }
    }
}

impl Entity for HyperliquidPerpSettlement {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.settlement_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_SETTLEMENT_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("settlement_id", "", self.settlement_id.clone()),
            EntityFieldChange::new("trade_id", "", self.trade_id.clone()),
            EntityFieldChange::new("match_id", "", self.match_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("long_account_id", "", self.long_account_id.clone()),
            EntityFieldChange::new("short_account_id", "", self.short_account_id.clone()),
            EntityFieldChange::new("price", "", self.price.to_string()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("notional", "", self.notional.to_string()),
            EntityFieldChange::new("taker_fee", "", self.taker_fee.to_string()),
            EntityFieldChange::new("maker_fee", "", self.maker_fee.to_string()),
            EntityFieldChange::new("taker_realized_pnl", "", self.taker_realized_pnl.to_string()),
            EntityFieldChange::new("maker_realized_pnl", "", self.maker_realized_pnl.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "settlement_id" | "trade_id" | "match_id" | "symbol" | "long_account_id"
            | "short_account_id" => 0,
            "asset" | "price" | "qty" | "notional" | "taker_fee" | "maker_fee"
            | "taker_realized_pnl" | "maker_realized_pnl" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.settlement_id))
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
    fn create_event_contains_settlement_fields() {
        let settlement = HyperliquidPerpSettlement::new(
            "settle-1-1".to_string(),
            "trade-1".to_string(),
            "match-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            "buyer".to_string(),
            "seller".to_string(),
            100,
            2,
            200,
            1,
            1,
            0,
            0,
        );

        let event = settlement.track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("settlement_id")
                && change.new_value_bytes() == b"settle-1-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("notional")
                && change.new_value_bytes() == b"200"
        }));
    }
}
