use common_entity::{Entity, EntityError, EntityFieldChange};

const USDS_M_FUTURES_ORDER_ENTITY_TYPE: u8 = 8;

/// USDS-M 合约订单方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdsMFuturesOrderSide {
    /// 买入或做多方向。
    Buy,
    /// 卖出或做空方向。
    Sell,
}

impl UsdsMFuturesOrderSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// USDS-M 合约持仓方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdsMFuturesPositionSide {
    /// Binance 单向持仓模式。
    Both,
}

impl UsdsMFuturesPositionSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Both => "both",
        }
    }
}

/// USDS-M 合约订单执行方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdsMFuturesOrderExecution {
    /// 限价订单。
    Limit {
        /// 委托价格。
        price: u64,
    },
    /// 市价意图，使用激进价格估算保证金上限。
    Market {
        /// 市价单愿意接受的保证金估算价格。
        aggressive_price: u64,
    },
}

impl UsdsMFuturesOrderExecution {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Limit { .. } => "limit",
            Self::Market { .. } => "market",
        }
    }

    /// 返回用于下单和保证金估算的价格。
    pub const fn margin_price(self) -> u64 {
        match self {
            Self::Limit { price } => price,
            Self::Market { aggressive_price } => aggressive_price,
        }
    }
}

/// USDS-M 合约限价单有效方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdsMFuturesOrderTimeInForce {
    /// 一直有效，直到成交或取消。
    Gtc,
    /// 立即成交，剩余取消。
    Ioc,
    /// 全部立即成交，否则取消。
    Fok,
}

impl UsdsMFuturesOrderTimeInForce {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gtc => "gtc",
            Self::Ioc => "ioc",
            Self::Fok => "fok",
        }
    }
}

/// 已进入执行流程的 USDS-M 合约订单生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdsMFuturesOrderStatus {
    /// 订单已接受，等待执行。
    Open,
    /// 订单提交时被拒绝。
    Rejected,
    /// 订单已取消。
    Canceled,
    /// 订单已完全成交。
    Filled,
}

impl UsdsMFuturesOrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Rejected => "rejected",
            Self::Canceled => "canceled",
            Self::Filled => "filled",
        }
    }
}

/// 已接受的 USDS-M 合约普通订单快照。
///
/// 该实体表示 `POST /fapi/v1/order` 中已经通过 core 校验、可以进入执行流程的订单事实。
/// 构造器假设输入来自已校验命令或事件回放，不重复执行业务拒绝规则。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsdsMFuturesOrder {
    /// 本系统生成的稳定订单 ID。
    pub order_id: String,
    /// 订单所属账户 ID。
    pub account_id: String,
    /// 合约交易对，例如 `BTCUSDT`。
    pub symbol: String,
    /// 买卖方向。
    pub side: UsdsMFuturesOrderSide,
    /// 持仓方向；第一版仅支持单向持仓 `Both`。
    pub position_side: UsdsMFuturesPositionSide,
    /// 执行方式。
    pub execution: UsdsMFuturesOrderExecution,
    /// 限价单有效方式；市价单默认使用 `Ioc`。
    pub time_in_force: UsdsMFuturesOrderTimeInForce,
    /// 合约数量。
    pub qty: u64,
    /// 用于下单和保证金冻结的价格。
    pub price: u64,
    /// 本订单接受时需要冻结的 USDT 保证金。
    pub required_margin: u64,
    /// 是否只减仓；第一版只接受 `false`。
    pub reduce_only: bool,
    /// 客户端自定义订单 ID。
    pub client_order_id: Option<String>,
    /// 本地生命周期状态。
    pub status: UsdsMFuturesOrderStatus,
    /// 当前订单实体版本。
    pub version: u64,
}

impl UsdsMFuturesOrder {
    /// 从已经校验过的业务事实或回放事件构造 USDS-M 合约订单。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        order_id: String,
        account_id: String,
        symbol: String,
        side: UsdsMFuturesOrderSide,
        position_side: UsdsMFuturesPositionSide,
        execution: UsdsMFuturesOrderExecution,
        time_in_force: UsdsMFuturesOrderTimeInForce,
        qty: u64,
        price: u64,
        required_margin: u64,
        reduce_only: bool,
        client_order_id: Option<String>,
    ) -> Self {
        Self {
            order_id,
            account_id,
            symbol,
            side,
            position_side,
            execution,
            time_in_force,
            qty,
            price,
            required_margin,
            reduce_only,
            client_order_id,
            status: UsdsMFuturesOrderStatus::Open,
            version: 1,
        }
    }

    /// 返回订单是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回订单是否交易指定合约。
    pub fn trades_symbol(&self, symbol: &str) -> bool {
        self.symbol == symbol
    }

    /// 返回订单名义价值。
    pub fn notional_quote(&self) -> Option<u64> {
        self.qty.checked_mul(self.price)
    }

    /// 返回订单记录的保证金是否等于给定杠杆下的向上取整保证金。
    pub fn has_required_margin_for_leverage(&self, leverage: u64) -> bool {
        required_margin(self.qty, self.price, leverage) == Some(self.required_margin)
    }
}

impl Entity for UsdsMFuturesOrder {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.order_id.clone()
    }

    fn entity_type() -> u8 {
        USDS_M_FUTURES_ORDER_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("order_id", "", self.order_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("position_side", "", self.position_side.as_str()),
            EntityFieldChange::new("execution", "", self.execution.as_str()),
            EntityFieldChange::new("time_in_force", "", self.time_in_force.as_str()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("price", "", self.price.to_string()),
            EntityFieldChange::new("required_margin", "", self.required_margin.to_string()),
            EntityFieldChange::new("reduce_only", "", self.reduce_only.to_string()),
            EntityFieldChange::new(
                "client_order_id",
                "",
                self.client_order_id.clone().unwrap_or_default(),
            ),
            EntityFieldChange::new("status", "", self.status.as_str()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();

        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(&mut changes, "symbol", &self.symbol, &other.symbol);
        push_change(&mut changes, "side", self.side.as_str(), other.side.as_str());
        push_change(
            &mut changes,
            "position_side",
            self.position_side.as_str(),
            other.position_side.as_str(),
        );
        push_change(&mut changes, "execution", self.execution.as_str(), other.execution.as_str());
        push_change(
            &mut changes,
            "time_in_force",
            self.time_in_force.as_str(),
            other.time_in_force.as_str(),
        );
        push_change(&mut changes, "qty", self.qty.to_string(), other.qty.to_string());
        push_change(&mut changes, "price", self.price.to_string(), other.price.to_string());
        push_change(
            &mut changes,
            "required_margin",
            self.required_margin.to_string(),
            other.required_margin.to_string(),
        );
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
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());

        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "order_id" | "account_id" | "symbol" | "side" | "position_side" | "execution"
            | "time_in_force" | "reduce_only" | "client_order_id" | "status" => 0,
            "qty" | "price" | "required_margin" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_order_entity_id(&self.order_id))
    }
}

/// 返回 `ceil(qty * price / leverage)`，任一算术步骤溢出或杠杆为零时返回 `None`。
pub fn required_margin(qty: u64, price: u64, leverage: u64) -> Option<u64> {
    if leverage == 0 {
        return None;
    }
    let notional = qty.checked_mul(price)?;
    let quotient = notional / leverage;
    let remainder = notional % leverage;
    if remainder == 0 { Some(quotient) } else { quotient.checked_add(1) }
}

fn stable_order_entity_id(value: &str) -> i64 {
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

    fn order() -> UsdsMFuturesOrder {
        UsdsMFuturesOrder::new(
            "trader-1-BTCUSDT-1".to_string(),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            UsdsMFuturesOrderSide::Buy,
            UsdsMFuturesPositionSide::Both,
            UsdsMFuturesOrderExecution::Limit { price: 101 },
            UsdsMFuturesOrderTimeInForce::Gtc,
            3,
            101,
            61,
            false,
            Some("client-1".to_string()),
        )
    }

    #[test]
    fn order_stores_business_facts() {
        let order = order();

        assert!(order.belongs_to_account("trader-1"));
        assert!(!order.belongs_to_account("trader-2"));
        assert!(order.trades_symbol("BTCUSDT"));
        assert_eq!(order.notional_quote(), Some(303));
        assert!(order.has_required_margin_for_leverage(5));
        assert_eq!(order.status, UsdsMFuturesOrderStatus::Open);
    }

    #[test]
    fn required_margin_rounds_up() {
        assert_eq!(required_margin(3, 101, 5), Some(61));
        assert_eq!(required_margin(10, 20, 5), Some(40));
        assert_eq!(required_margin(1, 1, 0), None);
    }
}
