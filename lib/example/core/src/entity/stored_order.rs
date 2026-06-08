use common_entity::{Entity, EntityError, EntityFieldChange};

const STORED_ORDER_ENTITY_TYPE: u8 = 2;

/// 订单方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredOrderSide {
    /// 买入订单。
    Buy,
    /// 卖出订单。
    Sell,
}

impl StoredOrderSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// Hyperliquid 风格的订单执行时机。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredOrderKind {
    /// 立即订单，对应 Hyperliquid `t.limit`。
    Immediate(StoredImmediateOrderSpec),
    /// 条件订单，对应 Hyperliquid `t.trigger`。
    Conditional(StoredConditionalOrderSpec),
}

impl StoredOrderKind {
    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::Immediate(_) => "immediate",
            Self::Conditional(_) => "conditional",
        }
    }

    pub const fn execution(self) -> StoredOrderExecution {
        match self {
            Self::Immediate(spec) => spec.execution,
            Self::Conditional(spec) => spec.execution,
        }
    }

    pub const fn time_in_force(self) -> Option<StoredOrderTimeInForce> {
        match self {
            Self::Immediate(spec) => Some(spec.time_in_force),
            Self::Conditional(_) => None,
        }
    }

    pub const fn trigger_price(self) -> Option<u64> {
        match self {
            Self::Immediate(_) => None,
            Self::Conditional(spec) => Some(spec.trigger_price),
        }
    }

    pub const fn trigger_role(self) -> Option<StoredOrderTriggerRole> {
        match self {
            Self::Immediate(_) => None,
            Self::Conditional(spec) => Some(spec.trigger_role),
        }
    }

    pub const fn limit_price(self) -> Option<u64> {
        match self.execution() {
            StoredOrderExecution::Market => None,
            StoredOrderExecution::Limit { price } => Some(price),
        }
    }
}

/// 立即订单参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoredImmediateOrderSpec {
    /// 执行方式，市价意图或限价意图。
    pub execution: StoredOrderExecution,
    /// Hyperliquid limit order 使用的有效方式：`Gtc` / `Ioc` / `Alo`。
    pub time_in_force: StoredOrderTimeInForce,
}

/// 条件订单参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoredConditionalOrderSpec {
    /// 满足该触发价格后再下发执行。
    pub trigger_price: u64,
    /// 触发单的业务角色，止盈或止损。
    pub trigger_role: StoredOrderTriggerRole,
    /// 触发后的执行方式，市价意图或限价意图。
    pub execution: StoredOrderExecution,
}

/// 订单执行方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredOrderExecution {
    /// 市价意图。Hyperliquid adapter 可映射为 IOC + 激进价格或 `trigger.isMarket = true`。
    Market,
    /// 限价意图，携带 quote 计价价格。
    Limit {
        /// quote 计价价格。
        price: u64,
    },
}

impl StoredOrderExecution {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Market => "market",
            Self::Limit { .. } => "limit",
        }
    }
}

/// 订单有效方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredOrderTimeInForce {
    /// 一直有效，直到成交或取消。
    Gtc,
    /// 立即成交剩余取消。
    Ioc,
    /// 只做 Maker，若会立即吃单则拒绝。
    Alo,
}

impl StoredOrderTimeInForce {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gtc => "gtc",
            Self::Ioc => "ioc",
            Self::Alo => "alo",
        }
    }
}

/// 触发单的业务角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredOrderTriggerRole {
    /// 止盈触发单。
    TakeProfit,
    /// 止损触发单。
    StopLoss,
}

impl StoredOrderTriggerRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TakeProfit => "take_profit",
            Self::StopLoss => "stop_loss",
        }
    }
}

/// 下单响应类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredOrderRespType {
    /// 只返回确认信息。
    Ack,
    /// 返回订单结果。
    Result,
    /// 返回完整订单信息。
    Full,
}

impl StoredOrderRespType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ack => "ack",
            Self::Result => "result",
            Self::Full => "full",
        }
    }
}

/// 自成交保护模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredOrderSelfTradePreventionMode {
    /// 不启用自成交保护。
    None,
    /// 过期接受方订单。
    ExpireTaker,
    /// 过期挂单方订单。
    ExpireMaker,
    /// 双方订单都过期。
    ExpireBoth,
}

impl StoredOrderSelfTradePreventionMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ExpireTaker => "expire_taker",
            Self::ExpireMaker => "expire_maker",
            Self::ExpireBoth => "expire_both",
        }
    }
}

/// 价格钉住类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredOrderPegPriceType {
    /// 主价格钉住，通常钉住同侧最优价。
    PrimaryPeg,
    /// 市场价格钉住，通常钉住对侧最优价。
    MarketPeg,
}

impl StoredOrderPegPriceType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryPeg => "primary_peg",
            Self::MarketPeg => "market_peg",
        }
    }
}

/// 价格钉住偏移单位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredOrderPegOffsetType {
    /// 以价格档位作为偏移单位。
    PriceLevel,
}

impl StoredOrderPegOffsetType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PriceLevel => "price_level",
        }
    }
}

/// 现货订单被接受后存储的订单快照。
///
/// 该实体保存后续撤单、撮合、账户冻结余额核对等流程需要复用的业务事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredOrder {
    /// 下单用例生成的稳定订单 ID。
    pub order_id: String,
    /// 拥有该订单及其冻结 quote 余额的交易账户 ID。
    pub account_id: String,
    /// 订单所属交易对，例如 `BTCUSDT`。
    pub symbol: String,
    /// 交易流程接受的订单方向。
    pub side: StoredOrderSide,
    /// Hyperliquid 风格的订单时机和执行方式组合。
    pub kind: StoredOrderKind,
    /// 以 base asset 计价的订单数量。
    pub qty: u64,
    /// 订单接受时从账户冻结的 quote 余额。
    pub reserved_quote: u64,
    /// 客户端自定义订单 ID，可由 adapter 映射为 Hyperliquid `cloid`。
    pub client_order_id: Option<String>,
    /// 客户端附带的策略 ID。
    pub strategy_id: Option<i64>,
    /// 客户端附带的策略类型。
    pub strategy_type: Option<i32>,
    /// 冰山订单可见数量。
    pub iceberg_qty: Option<u64>,
    /// 下单时请求的响应类型。
    pub new_order_resp_type: Option<StoredOrderRespType>,
    /// 下单时请求的自成交保护模式。
    pub self_trade_prevention_mode: Option<StoredOrderSelfTradePreventionMode>,
    /// 价格钉住来源。
    pub peg_price_type: Option<StoredOrderPegPriceType>,
    /// 价格钉住偏移值。
    pub peg_offset_value: Option<i32>,
    /// 价格钉住偏移单位。
    pub peg_offset_type: Option<StoredOrderPegOffsetType>,
}

impl StoredOrder {
    /// 从已经校验过的业务事实构造订单快照。
    ///
    /// 校验职责属于接受订单的用例。该构造器刻意不拒绝不一致字段，
    /// 这样 adapter 可以从历史持久化事件重建快照。
    pub fn new(
        order_id: String,
        account_id: String,
        symbol: String,
        side: StoredOrderSide,
        kind: StoredOrderKind,
        qty: u64,
        reserved_quote: u64,
        client_order_id: Option<String>,
        strategy_id: Option<i64>,
        strategy_type: Option<i32>,
        iceberg_qty: Option<u64>,
        new_order_resp_type: Option<StoredOrderRespType>,
        self_trade_prevention_mode: Option<StoredOrderSelfTradePreventionMode>,
        peg_price_type: Option<StoredOrderPegPriceType>,
        peg_offset_value: Option<i32>,
        peg_offset_type: Option<StoredOrderPegOffsetType>,
    ) -> Self {
        Self {
            order_id,
            account_id,
            symbol,
            side,
            kind,
            qty,
            reserved_quote,
            client_order_id,
            strategy_id,
            strategy_type,
            iceberg_qty,
            new_order_resp_type,
            self_trade_prevention_mode,
            peg_price_type,
            peg_offset_value,
            peg_offset_type,
        }
    }

    /// 返回订单是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回订单是否交易指定交易对。
    pub fn trades_symbol(&self, symbol: &str) -> bool {
        self.symbol == symbol
    }

    /// 返回订单限价价格。
    pub fn limit_price(&self) -> Option<u64> {
        self.kind.limit_price()
    }

    /// 计算订单 quote 名义价值，即 `qty * price`。
    ///
    /// 市价意图没有稳定限价价格，或乘法溢出时返回 `None`。
    pub fn notional_quote(&self) -> Option<u64> {
        self.qty.checked_mul(self.limit_price()?)
    }

    /// 返回冻结 quote 余额是否与订单名义价值一致。
    pub fn has_consistent_reserved_quote(&self) -> bool {
        self.notional_quote() == Some(self.reserved_quote)
    }

    /// 返回撤单时应释放的 quote 余额。
    pub fn quote_to_release_on_cancel(&self) -> u64 {
        self.reserved_quote
    }

    /// 返回该订单是否作为立即限价单被接受。
    pub fn is_immediate_limit_order(&self) -> bool {
        matches!(
            self.kind,
            StoredOrderKind::Immediate(StoredImmediateOrderSpec {
                execution: StoredOrderExecution::Limit { .. },
                ..
            })
        )
    }
}

impl Entity for StoredOrder {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.order_id.clone()
    }

    fn entity_type() -> u8 {
        STORED_ORDER_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        let mut changes = vec![
            EntityFieldChange::new("order_id", "", self.order_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("order_kind", "", self.kind.kind_name()),
            EntityFieldChange::new("execution", "", self.kind.execution().as_str()),
            EntityFieldChange::new(
                "time_in_force",
                "",
                option_tif_value(self.kind.time_in_force()),
            ),
            EntityFieldChange::new("price", "", option_u64_value(self.kind.limit_price())),
            EntityFieldChange::new(
                "trigger_price",
                "",
                option_u64_value(self.kind.trigger_price()),
            ),
            EntityFieldChange::new(
                "trigger_role",
                "",
                option_trigger_role_value(self.kind.trigger_role()),
            ),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("reserved_quote", "", self.reserved_quote.to_string()),
            EntityFieldChange::new(
                "client_order_id",
                "",
                self.client_order_id.clone().unwrap_or_default(),
            ),
            EntityFieldChange::new("strategy_id", "", option_i64_value(self.strategy_id)),
            EntityFieldChange::new("strategy_type", "", option_i32_value(self.strategy_type)),
            EntityFieldChange::new("iceberg_qty", "", option_u64_value(self.iceberg_qty)),
            EntityFieldChange::new(
                "new_order_resp_type",
                "",
                option_resp_type_value(self.new_order_resp_type),
            ),
            EntityFieldChange::new(
                "self_trade_prevention_mode",
                "",
                option_self_trade_prevention_value(self.self_trade_prevention_mode),
            ),
            EntityFieldChange::new(
                "peg_price_type",
                "",
                option_peg_price_value(self.peg_price_type),
            ),
            EntityFieldChange::new("peg_offset_value", "", option_i32_value(self.peg_offset_value)),
            EntityFieldChange::new(
                "peg_offset_type",
                "",
                option_peg_offset_value(self.peg_offset_type),
            ),
        ];

        if let Some(order_sequence) = self.order_sequence_from_id() {
            changes.push(EntityFieldChange::new("order_sequence", "", order_sequence.to_string()));
        }

        changes
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();

        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(&mut changes, "symbol", &self.symbol, &other.symbol);
        push_change(&mut changes, "side", self.side.as_str(), other.side.as_str());
        push_change(&mut changes, "order_kind", self.kind.kind_name(), other.kind.kind_name());
        push_change(
            &mut changes,
            "execution",
            self.kind.execution().as_str(),
            other.kind.execution().as_str(),
        );
        push_change(
            &mut changes,
            "time_in_force",
            option_tif_value(self.kind.time_in_force()),
            option_tif_value(other.kind.time_in_force()),
        );
        push_change(
            &mut changes,
            "price",
            option_u64_value(self.kind.limit_price()),
            option_u64_value(other.kind.limit_price()),
        );
        push_change(
            &mut changes,
            "trigger_price",
            option_u64_value(self.kind.trigger_price()),
            option_u64_value(other.kind.trigger_price()),
        );
        push_change(
            &mut changes,
            "trigger_role",
            option_trigger_role_value(self.kind.trigger_role()),
            option_trigger_role_value(other.kind.trigger_role()),
        );
        push_change(&mut changes, "qty", self.qty.to_string(), other.qty.to_string());
        push_change(
            &mut changes,
            "reserved_quote",
            self.reserved_quote.to_string(),
            other.reserved_quote.to_string(),
        );
        push_change(
            &mut changes,
            "client_order_id",
            self.client_order_id.clone().unwrap_or_default(),
            other.client_order_id.clone().unwrap_or_default(),
        );
        push_change(
            &mut changes,
            "strategy_id",
            option_i64_value(self.strategy_id),
            option_i64_value(other.strategy_id),
        );
        push_change(
            &mut changes,
            "strategy_type",
            option_i32_value(self.strategy_type),
            option_i32_value(other.strategy_type),
        );
        push_change(
            &mut changes,
            "iceberg_qty",
            option_u64_value(self.iceberg_qty),
            option_u64_value(other.iceberg_qty),
        );
        push_change(
            &mut changes,
            "new_order_resp_type",
            option_resp_type_value(self.new_order_resp_type),
            option_resp_type_value(other.new_order_resp_type),
        );
        push_change(
            &mut changes,
            "self_trade_prevention_mode",
            option_self_trade_prevention_value(self.self_trade_prevention_mode),
            option_self_trade_prevention_value(other.self_trade_prevention_mode),
        );
        push_change(
            &mut changes,
            "peg_price_type",
            option_peg_price_value(self.peg_price_type),
            option_peg_price_value(other.peg_price_type),
        );
        push_change(
            &mut changes,
            "peg_offset_value",
            option_i32_value(self.peg_offset_value),
            option_i32_value(other.peg_offset_value),
        );
        push_change(
            &mut changes,
            "peg_offset_type",
            option_peg_offset_value(self.peg_offset_type),
            option_peg_offset_value(other.peg_offset_type),
        );

        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "order_id"
            | "account_id"
            | "symbol"
            | "side"
            | "order_kind"
            | "execution"
            | "time_in_force"
            | "trigger_role"
            | "client_order_id"
            | "new_order_resp_type"
            | "self_trade_prevention_mode"
            | "peg_price_type"
            | "peg_offset_type" => 0,
            "order_sequence" | "qty" | "price" | "reserved_quote" | "trigger_price"
            | "strategy_id" | "strategy_type" | "iceberg_qty" | "peg_offset_value" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_order_entity_id(&self.order_id))
    }
}

impl StoredOrder {
    fn order_sequence_from_id(&self) -> Option<u64> {
        self.order_id.rsplit('-').next()?.parse::<u64>().ok()
    }
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

fn option_u64_value(value: Option<u64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

fn option_i64_value(value: Option<i64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

fn option_i32_value(value: Option<i32>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

fn option_tif_value(value: Option<StoredOrderTimeInForce>) -> &'static str {
    value.map(StoredOrderTimeInForce::as_str).unwrap_or_default()
}

fn option_trigger_role_value(value: Option<StoredOrderTriggerRole>) -> &'static str {
    value.map(StoredOrderTriggerRole::as_str).unwrap_or_default()
}

fn option_resp_type_value(value: Option<StoredOrderRespType>) -> &'static str {
    value.map(StoredOrderRespType::as_str).unwrap_or_default()
}

fn option_self_trade_prevention_value(
    value: Option<StoredOrderSelfTradePreventionMode>,
) -> &'static str {
    value.map(StoredOrderSelfTradePreventionMode::as_str).unwrap_or_default()
}

fn option_peg_price_value(value: Option<StoredOrderPegPriceType>) -> &'static str {
    value.map(StoredOrderPegPriceType::as_str).unwrap_or_default()
}

fn option_peg_offset_value(value: Option<StoredOrderPegOffsetType>) -> &'static str {
    value.map(StoredOrderPegOffsetType::as_str).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kind() -> StoredOrderKind {
        StoredOrderKind::Immediate(StoredImmediateOrderSpec {
            execution: StoredOrderExecution::Limit { price: 100 },
            time_in_force: StoredOrderTimeInForce::Gtc,
        })
    }

    fn sample_order() -> StoredOrder {
        StoredOrder::new(
            "order-1".to_string(),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            StoredOrderSide::Buy,
            sample_kind(),
            2,
            200,
            Some("client-order-1".to_string()),
            Some(10),
            Some(1_000_000),
            None,
            Some(StoredOrderRespType::Ack),
            Some(StoredOrderSelfTradePreventionMode::None),
            None,
            None,
            None,
        )
    }

    #[test]
    fn new_stores_order_fields() {
        let order = sample_order();

        assert_eq!(order.order_id, "order-1");
        assert_eq!(order.account_id, "trader-1");
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.side, StoredOrderSide::Buy);
        assert_eq!(order.kind, sample_kind());
        assert_eq!(order.qty, 2);
        assert_eq!(order.limit_price(), Some(100));
        assert_eq!(order.reserved_quote, 200);
        assert_eq!(order.client_order_id.as_deref(), Some("client-order-1"));
        assert_eq!(order.strategy_id, Some(10));
        assert_eq!(order.strategy_type, Some(1_000_000));
        assert_eq!(order.new_order_resp_type, Some(StoredOrderRespType::Ack));
        assert_eq!(
            order.self_trade_prevention_mode,
            Some(StoredOrderSelfTradePreventionMode::None)
        );
    }

    #[test]
    fn checks_order_owner_and_symbol() {
        let order = sample_order();

        assert!(order.belongs_to_account("trader-1"));
        assert!(!order.belongs_to_account("trader-2"));
        assert!(order.trades_symbol("BTCUSDT"));
        assert!(!order.trades_symbol("ETHUSDT"));
    }

    #[test]
    fn derives_notional_quote() {
        let order = sample_order();

        assert_eq!(order.notional_quote(), Some(200));
        assert!(order.has_consistent_reserved_quote());
    }

    #[test]
    fn detects_inconsistent_reserved_quote() {
        let order = StoredOrder::new(
            "order-1".to_string(),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            StoredOrderSide::Buy,
            sample_kind(),
            2,
            199,
            Some("client-order-1".to_string()),
            Some(10),
            Some(1_000_000),
            None,
            Some(StoredOrderRespType::Ack),
            Some(StoredOrderSelfTradePreventionMode::None),
            None,
            None,
            None,
        );

        assert!(!order.has_consistent_reserved_quote());
    }

    #[test]
    fn derives_quote_to_release_on_cancel() {
        let order = sample_order();

        assert_eq!(order.quote_to_release_on_cancel(), 200);
    }

    #[test]
    fn identifies_immediate_limit_orders() {
        let order = sample_order();

        assert!(order.is_immediate_limit_order());
    }

    #[test]
    fn market_order_has_no_notional_without_pricing_state() {
        let order = StoredOrder::new(
            "order-1".to_string(),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            StoredOrderSide::Buy,
            StoredOrderKind::Immediate(StoredImmediateOrderSpec {
                execution: StoredOrderExecution::Market,
                time_in_force: StoredOrderTimeInForce::Ioc,
            }),
            2,
            0,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert_eq!(order.limit_price(), None);
        assert_eq!(order.notional_quote(), None);
    }

    #[test]
    fn created_event_contains_order_api_fields() {
        let order = sample_order();
        let changes = order.created_field_changes();

        assert!(changes.iter().any(|change| {
            change.field_name == "side" && change.new_value == StoredOrderSide::Buy.as_str()
        }));
        assert!(changes.iter().any(|change| {
            change.field_name == "order_kind" && change.new_value == sample_kind().kind_name()
        }));
        assert!(changes.iter().any(|change| {
            change.field_name == "execution"
                && change.new_value == StoredOrderExecution::Limit { price: 100 }.as_str()
        }));
        assert!(changes.iter().any(|change| {
            change.field_name == "client_order_id" && change.new_value == "client-order-1"
        }));
        assert!(changes.iter().any(|change| {
            change.field_name == "strategy_type" && change.new_value == "1000000"
        }));
    }
}
