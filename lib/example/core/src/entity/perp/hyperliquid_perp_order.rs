use common_entity::{Entity, EntityError, EntityFieldChange};

const HYPERLIQUID_PERP_ORDER_ENTITY_TYPE: u8 = 9;

/// Hyperliquid perp 订单方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpOrderSide {
    /// 买入或做多方向。
    Buy,
    /// 卖出或做空方向。
    Sell,
}

impl HyperliquidPerpOrderSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// Hyperliquid perp 订单执行方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpOrderExecution {
    /// 市价意图。adapter 可映射为 IOC + 激进限价。
    Market {
        /// 市价意图使用的激进价格，用于撮合价格比较。
        aggressive_price: u64,
    },
    /// 限价意图。
    Limit {
        /// 委托价格。
        price: u64,
    },
}

impl HyperliquidPerpOrderExecution {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Market { .. } => "market",
            Self::Limit { .. } => "limit",
        }
    }

    /// 返回限价价格；市价意图没有稳定 maker 限价。
    pub const fn limit_price(self) -> Option<u64> {
        match self {
            Self::Market { .. } => None,
            Self::Limit { price } => Some(price),
        }
    }

    /// 返回撮合时可比较的订单价格。
    pub const fn order_price(self) -> u64 {
        match self {
            Self::Market { aggressive_price } => aggressive_price,
            Self::Limit { price } => price,
        }
    }
}

/// Hyperliquid perp 限价订单有效方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpOrderTimeInForce {
    /// 一直有效，直到成交或取消。
    Gtc,
    /// 立即成交，剩余取消。
    Ioc,
    /// 只做 Maker，若会立即吃单则拒绝。
    Alo,
}

impl HyperliquidPerpOrderTimeInForce {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gtc => "gtc",
            Self::Ioc => "ioc",
            Self::Alo => "alo",
        }
    }
}

/// 已进入执行流程的 Hyperliquid perp 订单生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpOrderStatus {
    /// 订单已进入执行流程，尚未成交。
    Open,
    /// 订单已部分成交。
    PartiallyFilled,
    /// 订单已完全成交。
    Filled,
    /// 订单已取消。
    Canceled,
    /// 订单提交时被拒绝。
    Rejected,
}

impl HyperliquidPerpOrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::PartiallyFilled => "partially_filled",
            Self::Filled => "filled",
            Self::Canceled => "canceled",
            Self::Rejected => "rejected",
        }
    }
}

/// 已接受并可由撮合层读取的 Hyperliquid perp 订单快照。
///
/// 该实体只表示订单执行状态，不表达仓位、保证金、手续费、PnL 或资金费。
/// 构造器假设输入来自已校验命令或事件回放，不重复执行业务拒绝规则。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpOrder {
    /// 本系统生成的稳定订单 ID。
    pub order_id: String,
    /// Hyperliquid 返回的 numeric `oid`；尚未确认时可以为空。
    pub exchange_oid: Option<u64>,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 订单所属账户 ID。
    pub account_id: String,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 买卖方向。
    pub side: HyperliquidPerpOrderSide,
    /// 执行方式。
    pub execution: HyperliquidPerpOrderExecution,
    /// 限价订单有效方式。
    pub time_in_force: HyperliquidPerpOrderTimeInForce,
    /// 合约数量。
    pub qty: u64,
    /// 已成交数量。
    pub filled_qty: u64,
    /// 订单生命周期状态。
    pub status: HyperliquidPerpOrderStatus,
    /// 是否只减仓。
    pub reduce_only: bool,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub client_order_id: Option<String>,
    /// 当前订单实体版本。
    pub version: u64,
}

impl HyperliquidPerpOrder {
    /// 从已经校验过的业务事实或回放事件构造 Hyperliquid perp 订单。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        order_id: String,
        exchange_oid: Option<u64>,
        asset: u32,
        account_id: String,
        symbol: String,
        side: HyperliquidPerpOrderSide,
        execution: HyperliquidPerpOrderExecution,
        time_in_force: HyperliquidPerpOrderTimeInForce,
        qty: u64,
        reduce_only: bool,
        client_order_id: Option<String>,
    ) -> Self {
        Self {
            order_id,
            exchange_oid,
            asset,
            account_id,
            symbol,
            side,
            execution,
            time_in_force,
            qty,
            filled_qty: 0,
            status: HyperliquidPerpOrderStatus::Open,
            reduce_only,
            client_order_id,
            version: 1,
        }
    }

    /// 返回带指定执行状态的订单快照。
    pub fn with_execution_state(
        mut self,
        status: HyperliquidPerpOrderStatus,
        filled_qty: u64,
    ) -> Self {
        self.status = status;
        self.filled_qty = filled_qty;
        self
    }

    /// 返回订单是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回订单剩余可成交数量；已成交数量大于订单数量时返回 `None`。
    pub fn remaining_qty(&self) -> Option<u64> {
        self.qty.checked_sub(self.filled_qty)
    }

    /// 返回订单当前是否可进入撮合。
    pub fn is_matchable(&self) -> bool {
        matches!(
            self.status,
            HyperliquidPerpOrderStatus::Open | HyperliquidPerpOrderStatus::PartiallyFilled
        ) && self.remaining_qty().is_some_and(|qty| qty > 0)
    }

    /// 返回生命周期状态和成交数量是否一致。
    pub fn has_consistent_execution_state(&self) -> bool {
        match self.status {
            HyperliquidPerpOrderStatus::Open => self.filled_qty == 0,
            HyperliquidPerpOrderStatus::PartiallyFilled => {
                0 < self.filled_qty && self.filled_qty < self.qty
            }
            HyperliquidPerpOrderStatus::Filled => self.filled_qty == self.qty,
            HyperliquidPerpOrderStatus::Canceled => self.filled_qty <= self.qty,
            HyperliquidPerpOrderStatus::Rejected => self.filled_qty == 0,
        }
    }

    /// 返回订单是否交易指定 Hyperliquid asset。
    pub fn trades_asset(&self, asset: u32) -> bool {
        self.asset == asset
    }

    /// 返回订单是否交易指定展示合约。
    pub fn trades_symbol(&self, symbol: &str) -> bool {
        self.symbol == symbol
    }

    /// 返回撮合时可比较的订单价格。
    pub fn order_price(&self) -> u64 {
        self.execution.order_price()
    }

    /// 返回订单限价价格。
    pub fn limit_price(&self) -> Option<u64> {
        self.execution.limit_price()
    }
}

impl Entity for HyperliquidPerpOrder {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.order_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_ORDER_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("order_id", "", self.order_id.clone()),
            EntityFieldChange::new("exchange_oid", "", option_u64_value(self.exchange_oid)),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("execution", "", self.execution.as_str()),
            EntityFieldChange::new("time_in_force", "", self.time_in_force.as_str()),
            EntityFieldChange::new("price", "", self.order_price().to_string()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("filled_qty", "", self.filled_qty.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
            EntityFieldChange::new("reduce_only", "", self.reduce_only.to_string()),
            EntityFieldChange::new(
                "client_order_id",
                "",
                self.client_order_id.clone().unwrap_or_default(),
            ),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();

        push_change(
            &mut changes,
            "exchange_oid",
            option_u64_value(self.exchange_oid),
            option_u64_value(other.exchange_oid),
        );
        push_change(&mut changes, "asset", self.asset.to_string(), other.asset.to_string());
        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(&mut changes, "symbol", &self.symbol, &other.symbol);
        push_change(&mut changes, "side", self.side.as_str(), other.side.as_str());
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
            self.order_price().to_string(),
            other.order_price().to_string(),
        );
        push_change(&mut changes, "qty", self.qty.to_string(), other.qty.to_string());
        push_change(
            &mut changes,
            "filled_qty",
            self.filled_qty.to_string(),
            other.filled_qty.to_string(),
        );
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        push_change(
            &mut changes,
            "reduce_only",
            self.reduce_only.to_string(),
            other.reduce_only.to_string(),
        );
        push_change(
            &mut changes,
            "client_order_id",
            self.client_order_id.clone().unwrap_or_default(),
            other.client_order_id.clone().unwrap_or_default(),
        );

        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "order_id" | "account_id" | "symbol" | "side" | "execution" | "time_in_force"
            | "status" | "reduce_only" | "client_order_id" => 0,
            "exchange_oid" | "asset" | "price" | "qty" | "filled_qty" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.order_id))
    }
}

fn option_u64_value(value: Option<u64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
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
    use super::*;

    fn order() -> HyperliquidPerpOrder {
        HyperliquidPerpOrder::new(
            "order-1".to_string(),
            Some(42),
            0,
            "trader-1".to_string(),
            "BTC-PERP".to_string(),
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Limit { price: 100 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            3,
            false,
            Some("client-1".to_string()),
        )
    }

    #[test]
    fn order_exposes_matching_facts() {
        let order = order();

        assert!(order.belongs_to_account("trader-1"));
        assert!(order.trades_asset(0));
        assert!(order.trades_symbol("BTC-PERP"));
        assert_eq!(order.remaining_qty(), Some(3));
        assert_eq!(order.order_price(), 100);
        assert_eq!(order.limit_price(), Some(100));
        assert!(order.is_matchable());
        assert!(order.has_consistent_execution_state());
    }

    #[test]
    fn execution_state_detects_inconsistent_fills() {
        let order = order().with_execution_state(HyperliquidPerpOrderStatus::PartiallyFilled, 3);

        assert!(!order.has_consistent_execution_state());
        assert!(!order.is_matchable());
    }
}
