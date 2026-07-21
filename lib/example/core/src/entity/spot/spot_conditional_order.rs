use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff};

use super::spot_order_primitives::{
    SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
    SpotOrderTimeInForce, option_status_reason_value, option_u64_value, push_change,
    stable_order_entity_id,
};
use super::spot_order_v2::SpotOrderV2;
use crate::entity::ReservationError;

const SPOT_CONDITIONAL_ORDER_ENTITY_TYPE: u8 = 4;

/// 条件单触发角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SpotOrderTriggerRole {
    /// 止盈触发单。
    TakeProfit,
    /// 止损触发单。
    StopLoss,
}

impl SpotOrderTriggerRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TakeProfit => "take_profit",
            Self::StopLoss => "stop_loss",
        }
    }
}

/// 未触发条件单生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotConditionalOrderStatus {
    /// 条件单已接受，等待触发。
    Open,
    /// 条件单已触发，并转换为 active `SpotOrderV2`。
    Triggered,
    /// 条件单触发前被取消。
    Canceled,
    /// 条件单提交时被交易所拒绝。
    Rejected,
    /// 条件单过期。
    Expired,
}

impl SpotConditionalOrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Triggered => "triggered",
            Self::Canceled => "canceled",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub const fn is_cancelable(self) -> bool {
        matches!(self, Self::Open)
    }
}

/// 未触发的 Hyperliquid 条件现货订单。
///
/// 条件单只保存触发规则和触发后的执行意图；它不表示已进入订单簿的订单，也不冻结余额。
/// 条件满足后，用 `triggered_order` 转换成 active `SpotOrderV2`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotConditionalOrder {
    /// 本系统生成的稳定条件单 ID。
    pub trigger_order_id: String,
    /// Hyperliquid 现货资产编号，现货通常为 `10000 + spot index`。
    pub asset: u32,
    /// Hyperliquid 返回的 numeric `oid`；尚未确认时可以为空。
    pub exchange_oid: Option<u64>,
    /// 条件单所属账户。
    pub account_id: String,
    /// 交易对展示名，例如 `BTCUSDT`。
    pub symbol: String,
    /// 买卖方向。
    pub side: SpotOrderSide,
    /// 触发价格，使用 core fixed-point 整数价格。
    pub trigger_price: u64,
    /// 触发单业务角色。
    pub trigger_role: SpotOrderTriggerRole,
    /// 触发后的执行意图。
    pub execution: SpotOrderExecution,
    /// 触发后 active 订单使用的 TIF。市价意图通常为 `Ioc`。
    pub time_in_force: SpotOrderTimeInForce,
    /// 以 base asset 计价的下单数量。
    pub qty: u64,
    /// 条件单生命周期状态。
    pub status: SpotConditionalOrderStatus,
    /// Hyperliquid 细分状态原因。
    pub status_reason: Option<SpotOrderStatusReason>,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub client_order_id: Option<String>,
}

impl SpotConditionalOrder {
    /// 从已经校验过的业务事实或回放事件构造条件单快照。
    pub fn new(
        trigger_order_id: String,
        asset: u32,
        exchange_oid: Option<u64>,
        account_id: String,
        symbol: String,
        side: SpotOrderSide,
        trigger_price: u64,
        trigger_role: SpotOrderTriggerRole,
        execution: SpotOrderExecution,
        time_in_force: SpotOrderTimeInForce,
        qty: u64,
        client_order_id: Option<String>,
    ) -> Self {
        Self {
            trigger_order_id,
            asset,
            exchange_oid,
            account_id,
            symbol,
            side,
            trigger_price,
            trigger_role,
            execution,
            time_in_force,
            qty,
            status: SpotConditionalOrderStatus::Open,
            status_reason: None,
            client_order_id,
        }
    }

    /// 返回带交易所 `oid` 的条件单快照。
    pub fn with_exchange_oid(mut self, exchange_oid: u64) -> Self {
        self.exchange_oid = Some(exchange_oid);
        self
    }

    /// 返回带指定条件单状态的快照。
    pub fn with_status(mut self, status: SpotConditionalOrderStatus) -> Self {
        self.status = status;
        self
    }

    /// 返回带 Hyperliquid 细分状态原因的条件单快照。
    pub fn with_status_reason(mut self, status_reason: SpotOrderStatusReason) -> Self {
        self.status_reason = Some(status_reason);
        self
    }

    /// 返回条件单是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回条件单是否交易指定 Hyperliquid asset。
    pub fn trades_asset(&self, asset: u32) -> bool {
        self.asset == asset
    }

    /// 返回条件单是否允许撤销。
    pub fn can_be_cancelled(&self) -> bool {
        self.status.is_cancelable()
    }

    /// 条件满足后生成 active 现货订单。
    pub fn triggered_order(
        &self,
        order_id: String,
        reserved_base: u64,
        reserved_quote: u64,
        base_asset_id: &str,
        quote_asset_id: &str,
    ) -> Result<SpotOrderV2, ReservationError> {
        let reservation = SpotOrderV2::principal_reservation(
            order_id.as_str(),
            self.account_id.as_str(),
            self.side,
            self.qty,
            self.execution.order_price(),
            base_asset_id,
            quote_asset_id,
        )?;
        Ok(SpotOrderV2::new(
            order_id,
            self.asset,
            self.exchange_oid,
            self.account_id.clone(),
            self.symbol.clone(),
            self.side,
            self.execution,
            self.time_in_force,
            self.qty,
            0,
            SpotOrderStatus::Open,
            None,
            reserved_base,
            reserved_quote,
            reservation,
            self.client_order_id.clone(),
            1,
        ))
    }
}

impl FieldDiff for SpotConditionalOrder {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("trigger_order_id", "", self.trigger_order_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("exchange_oid", "", option_u64_value(self.exchange_oid)),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("trigger_price", "", self.trigger_price.to_string()),
            EntityFieldChange::new("trigger_role", "", self.trigger_role.as_str()),
            EntityFieldChange::new("execution", "", self.execution.as_str()),
            EntityFieldChange::new("time_in_force", "", self.time_in_force.as_str()),
            EntityFieldChange::new("price", "", self.execution.order_price().to_string()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
            EntityFieldChange::new(
                "status_reason",
                "",
                option_status_reason_value(self.status_reason),
            ),
            EntityFieldChange::new(
                "client_order_id",
                "",
                self.client_order_id.clone().unwrap_or_default(),
            ),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();

        push_change(&mut changes, "asset", self.asset.to_string(), other.asset.to_string());
        push_change(
            &mut changes,
            "exchange_oid",
            option_u64_value(self.exchange_oid),
            option_u64_value(other.exchange_oid),
        );
        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(&mut changes, "symbol", &self.symbol, &other.symbol);
        push_change(&mut changes, "side", self.side.as_str(), other.side.as_str());
        push_change(
            &mut changes,
            "trigger_price",
            self.trigger_price.to_string(),
            other.trigger_price.to_string(),
        );
        push_change(
            &mut changes,
            "trigger_role",
            self.trigger_role.as_str(),
            other.trigger_role.as_str(),
        );
        push_change(&mut changes, "execution", self.execution.as_str(), other.execution.as_str());
        push_change(
            &mut changes,
            "time_in_force",
            self.time_in_force.as_str(),
            other.time_in_force.as_str(),
        );
        push_change(
            &mut changes,
            "price",
            self.execution.order_price().to_string(),
            other.execution.order_price().to_string(),
        );
        push_change(&mut changes, "qty", self.qty.to_string(), other.qty.to_string());
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        push_change(
            &mut changes,
            "status_reason",
            option_status_reason_value(self.status_reason),
            option_status_reason_value(other.status_reason),
        );
        push_change(
            &mut changes,
            "client_order_id",
            self.client_order_id.clone().unwrap_or_default(),
            other.client_order_id.clone().unwrap_or_default(),
        );

        changes
    }
}

impl Entity for SpotConditionalOrder {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.trigger_order_id.clone()
    }

    fn entity_type() -> u8 {
        SPOT_CONDITIONAL_ORDER_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "trigger_order_id" | "account_id" | "symbol" | "side" | "trigger_role"
            | "execution" | "time_in_force" | "status" | "status_reason" | "client_order_id" => 0,
            "asset" | "exchange_oid" | "trigger_price" | "qty" | "price" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_order_entity_id(&self.trigger_order_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conditional_sell_order() -> SpotConditionalOrder {
        SpotConditionalOrder::new(
            "trigger-1".to_string(),
            10_001,
            None,
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            90,
            SpotOrderTriggerRole::StopLoss,
            SpotOrderExecution::Market { aggressive_price: 95 },
            SpotOrderTimeInForce::Ioc,
            2,
            None,
        )
    }

    #[test]
    fn conditional_order_does_not_freeze_before_trigger() {
        let order = conditional_sell_order();

        assert_eq!(order.status, SpotConditionalOrderStatus::Open);
        assert!(order.can_be_cancelled());
        assert_eq!(order.trigger_price, 90);
        assert_eq!(order.execution.limit_price(), None);
        assert_eq!(order.execution.order_price(), 95);
    }

    #[test]
    fn conditional_order_triggers_active_order() -> Result<(), ReservationError> {
        let conditional = conditional_sell_order().with_exchange_oid(77);
        let active = conditional.triggered_order("order-2".to_string(), 2, 0, "BTC", "USDT")?;

        assert_eq!(active.order_id, "order-2");
        assert_eq!(active.asset, conditional.asset);
        assert_eq!(active.exchange_oid, Some(77));
        assert_eq!(active.side, SpotOrderSide::Sell);
        assert_eq!(active.execution, conditional.execution);
        assert_eq!(active.time_in_force, SpotOrderTimeInForce::Ioc);
        assert_eq!(active.reserved_base, 2);
        assert_eq!(active.reserved_quote, 0);
        assert_eq!(active.reservation.caused_by_order_id, "order-2");
        assert_eq!(active.reservation.asset_id, "BTC");
        assert!(active.has_consistent_reserved_base());
        assert!(active.has_consistent_reserved_quote());
        Ok(())
    }

    #[test]
    fn created_events_contain_conditional_order_fields() {
        let conditional = conditional_sell_order()
            .with_status(SpotConditionalOrderStatus::Triggered)
            .with_status_reason(SpotOrderStatusReason::Triggered);
        let conditional_changes = conditional.created_field_changes();

        assert!(
            conditional_changes
                .iter()
                .any(|change| { change.field_name == "trigger_price" && change.new_value == "90" })
        );
        assert!(
            conditional_changes
                .iter()
                .any(|change| { change.field_name == "status" && change.new_value == "triggered" })
        );
    }
}
