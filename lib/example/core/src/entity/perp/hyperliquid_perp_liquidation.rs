use common_entity::{Entity, EntityError, EntityFieldChange};

use crate::entity::{HyperliquidPerpMarginMode, HyperliquidPerpPositionSide};

const HYPERLIQUID_PERP_LIQUIDATION_ENTITY_TYPE: u8 = 14;

/// Hyperliquid perp 强平触发原因。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpLiquidationTriggerReason {
    /// 维持保证金条件被击穿。
    MaintenanceMarginBreach,
    /// 已经进入破产风险区间。
    BankruptcyRisk,
}

impl HyperliquidPerpLiquidationTriggerReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaintenanceMarginBreach => "maintenanceMarginBreach",
            Self::BankruptcyRisk => "bankruptcyRisk",
        }
    }
}

/// Hyperliquid perp 强平事实状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpLiquidationStatus {
    /// 已进入强平流程。
    Started,
    /// 已经发出至少一张强平单。
    OrderPlaced,
    /// 当前强平流程已经完成。
    Resolved,
    /// 当前强平流程需要升级到更高风险处置路径。
    Escalated,
}

impl HyperliquidPerpLiquidationStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::OrderPlaced => "order_placed",
            Self::Resolved => "resolved",
            Self::Escalated => "escalated",
        }
    }
}

/// 一次 Hyperliquid perp 强平启动产生的仓位级事实。
///
/// 该实体只表示“进入强平流程”，不承载自动减仓成交、余额结算或 ADL 结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpLiquidation {
    /// 本系统稳定强平记录 ID。
    pub liquidation_id: String,
    /// 强平批次 ID。
    pub liquidation_batch_id: String,
    /// 触发强平的业务主体。
    pub party_id: String,
    /// 被强平账户 ID。
    pub account_id: String,
    /// 被强平仓位 ID。
    pub position_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 被强平仓位方向。
    pub side: HyperliquidPerpPositionSide,
    /// 被强平仓位数量。
    pub qty: u64,
    /// 强平时的保证金模式。
    pub margin_mode: HyperliquidPerpMarginMode,
    /// 触发判定使用的标记价格。
    pub mark_price: u64,
    /// 本次判定使用的破产价格。
    pub bankruptcy_price: u64,
    /// 触发原因。
    pub trigger_reason: HyperliquidPerpLiquidationTriggerReason,
    /// 当前强平事实状态。
    pub status: HyperliquidPerpLiquidationStatus,
    /// 已经发出的强平单数量。
    pub placed_order_count: u32,
    /// 已经发出的强平单累计数量。
    pub placed_qty_total: u64,
    /// 当前尚未发单的剩余待处置数量。
    pub remaining_qty: u64,
    /// 最近一次发出的强平单 ID。
    pub last_order_id: Option<String>,
    /// 当前强平会话实体版本。
    pub version: u64,
}

impl HyperliquidPerpLiquidation {
    /// 从已校验的强平触发事实构造记录。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        liquidation_id: String,
        liquidation_batch_id: String,
        party_id: String,
        account_id: String,
        position_id: String,
        asset: u32,
        symbol: String,
        side: HyperliquidPerpPositionSide,
        qty: u64,
        margin_mode: HyperliquidPerpMarginMode,
        mark_price: u64,
        bankruptcy_price: u64,
        trigger_reason: HyperliquidPerpLiquidationTriggerReason,
        status: HyperliquidPerpLiquidationStatus,
    ) -> Self {
        Self {
            liquidation_id,
            liquidation_batch_id,
            party_id,
            account_id,
            position_id,
            asset,
            symbol,
            side,
            qty,
            margin_mode,
            mark_price,
            bankruptcy_price,
            trigger_reason,
            status,
            placed_order_count: 0,
            placed_qty_total: 0,
            remaining_qty: qty,
            last_order_id: None,
            version: 1,
        }
    }

    /// 返回当前强平会话是否允许继续发出订单簿强平单。
    pub fn can_place_order(&self) -> bool {
        matches!(
            self.status,
            HyperliquidPerpLiquidationStatus::Started
                | HyperliquidPerpLiquidationStatus::OrderPlaced
        ) && self.remaining_qty > 0
    }

    /// 返回下一张强平单应该发出的数量。
    pub fn next_order_qty(&self, current_position_qty: u64) -> Option<u64> {
        if !self.can_place_order() {
            return None;
        }

        let effective_qty = current_position_qty.min(self.remaining_qty);
        if effective_qty == 0 {
            return None;
        }

        let next_qty = if effective_qty >= 5 {
            effective_qty.checked_mul(20)?.checked_add(99)?.checked_div(100)?
        } else {
            effective_qty
        };

        Some(next_qty.max(1).min(effective_qty))
    }

    /// 应用一次强平单发出后的会话推进。
    pub fn apply_order_placed(
        &mut self,
        last_order_id: String,
        placed_qty: u64,
        version: u64,
    ) -> Option<()> {
        let next_count = self.placed_order_count.checked_add(1)?;
        let next_total = self.placed_qty_total.checked_add(placed_qty)?;
        let next_remaining = self.remaining_qty.checked_sub(placed_qty)?;

        self.status = HyperliquidPerpLiquidationStatus::OrderPlaced;
        self.placed_order_count = next_count;
        self.placed_qty_total = next_total;
        self.remaining_qty = next_remaining;
        self.last_order_id = Some(last_order_id);
        self.version = version;
        Some(())
    }
}

impl Entity for HyperliquidPerpLiquidation {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.liquidation_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_LIQUIDATION_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("liquidation_id", "", self.liquidation_id.clone()),
            EntityFieldChange::new("liquidation_batch_id", "", self.liquidation_batch_id.clone()),
            EntityFieldChange::new("party_id", "", self.party_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("position_id", "", self.position_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("margin_mode", "", self.margin_mode.as_str()),
            EntityFieldChange::new("mark_price", "", self.mark_price.to_string()),
            EntityFieldChange::new("bankruptcy_price", "", self.bankruptcy_price.to_string()),
            EntityFieldChange::new("trigger_reason", "", self.trigger_reason.as_str()),
            EntityFieldChange::new("status", "", self.status.as_str()),
            EntityFieldChange::new("placed_order_count", "", self.placed_order_count.to_string()),
            EntityFieldChange::new("placed_qty_total", "", self.placed_qty_total.to_string()),
            EntityFieldChange::new("remaining_qty", "", self.remaining_qty.to_string()),
            EntityFieldChange::new(
                "last_order_id",
                "",
                self.last_order_id.clone().unwrap_or_default(),
            ),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        push_change(
            &mut changes,
            "placed_order_count",
            self.placed_order_count.to_string(),
            other.placed_order_count.to_string(),
        );
        push_change(
            &mut changes,
            "placed_qty_total",
            self.placed_qty_total.to_string(),
            other.placed_qty_total.to_string(),
        );
        push_change(
            &mut changes,
            "remaining_qty",
            self.remaining_qty.to_string(),
            other.remaining_qty.to_string(),
        );
        push_change(
            &mut changes,
            "last_order_id",
            self.last_order_id.clone().unwrap_or_default(),
            other.last_order_id.clone().unwrap_or_default(),
        );
        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "liquidation_id"
            | "liquidation_batch_id"
            | "party_id"
            | "account_id"
            | "position_id"
            | "symbol"
            | "side"
            | "margin_mode"
            | "trigger_reason"
            | "status"
            | "last_order_id" => 0,
            "asset" | "qty" | "mark_price" | "bankruptcy_price" | "placed_order_count"
            | "placed_qty_total" | "remaining_qty" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.liquidation_id))
    }
}

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn push_change(
    changes: &mut Vec<EntityFieldChange>,
    field_name: &'static str,
    old_value: impl Into<String>,
    new_value: impl Into<String>,
) {
    let old_value = old_value.into();
    let new_value = new_value.into();
    if old_value != new_value {
        changes.push(EntityFieldChange::new(field_name, old_value, new_value));
    }
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    #[test]
    fn create_event_contains_liquidation_fields() {
        let liquidation = HyperliquidPerpLiquidation::new(
            "liq-1-position-1".to_string(),
            "liq-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            2,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            HyperliquidPerpLiquidationStatus::Started,
        );

        let event = liquidation.track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("margin_mode")
                && change.new_value_bytes() == b"cross"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("status")
                && change.new_value_bytes() == b"started"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("remaining_qty")
                && change.new_value_bytes() == b"2"
        }));
    }

    #[test]
    fn next_order_qty_supports_partial_liquidation() {
        let liquidation = HyperliquidPerpLiquidation::new(
            "liq-1-position-1".to_string(),
            "liq-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            10,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            HyperliquidPerpLiquidationStatus::Started,
        );

        assert_eq!(liquidation.next_order_qty(10), Some(2));
        assert!(liquidation.can_place_order());
    }

    #[test]
    fn apply_order_placed_advances_session() {
        let mut liquidation = HyperliquidPerpLiquidation::new(
            "liq-1-position-1".to_string(),
            "liq-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            10,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            HyperliquidPerpLiquidationStatus::Started,
        );

        liquidation.apply_order_placed("order-1".to_string(), 2, 2).unwrap();

        assert_eq!(liquidation.status, HyperliquidPerpLiquidationStatus::OrderPlaced);
        assert_eq!(liquidation.placed_order_count, 1);
        assert_eq!(liquidation.placed_qty_total, 2);
        assert_eq!(liquidation.remaining_qty, 8);
        assert_eq!(liquidation.last_order_id.as_deref(), Some("order-1"));
        assert_eq!(liquidation.version, 2);
    }
}
