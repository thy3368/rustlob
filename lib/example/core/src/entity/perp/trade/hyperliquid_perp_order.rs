use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, FieldDiff, FinancialClassification,
    FourColorArchetype,
};
use thiserror::Error;

use crate::entity::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error, BalanceLedgerReason, Reservation,
    ReservationError, ReservationKind, ReservationMarketKind,
};

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

/// 创建 Hyperliquid perp 订单所需的已校验业务输入。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderInput {
    /// 本系统生成的稳定订单 ID。
    pub order_id: String,
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
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub client_order_id: Option<String>,
    /// 若订单来源于强平流程，则携带强平会话 ID。
    pub liquidation_id: Option<String>,
    /// 本次普通下单的仓位业务意图。
    pub intent: PlaceHyperliquidPerpOrderIntent,
}

/// Hyperliquid perp 普通下单的仓位业务意图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceHyperliquidPerpOrderIntent {
    /// 当前无仓位，本次建立新仓位，需要新增保证金冻结。
    OpenPosition {
        /// 保证金资产 ID。
        margin_asset_id: String,
        /// 保证金余额实体 ID。
        margin_balance_entity_id: String,
        /// 本次下单需要冻结的保证金金额。
        margin_amount: u64,
    },
    /// 已有同方向仓位，本次加仓，需要新增保证金冻结。
    IncreasePosition {
        /// 保证金资产 ID。
        margin_asset_id: String,
        /// 保证金余额实体 ID。
        margin_balance_entity_id: String,
        /// 本次下单需要冻结的保证金金额。
        margin_amount: u64,
    },
    /// 反方向订单减少仓位，不创建新的保证金冻结。
    ReducePosition,
    /// 反方向订单刚好完全平仓，不创建新的保证金冻结。
    ClosePosition,
}

/// Hyperliquid perp 下单创建的聚合结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderOutcome {
    /// 已创建的订单聚合根。
    pub order: HyperliquidPerpOrder,
    /// 由订单创建直接派生的冻结余额流水。
    pub freeze_ledger_entry: Option<BalanceLedgerEntryV2>,
}

/// Hyperliquid perp 订单行为错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HyperliquidPerpOrderBehaviorError {
    /// 下单数量必须大于零。
    #[error("perp order quantity must be greater than zero")]
    InvalidQuantity,
    /// 下单价格必须大于零。
    #[error("perp order price must be greater than zero")]
    InvalidPrice,
    /// 保证金冻结金额必须大于零。
    #[error("perp order margin amount must be greater than zero")]
    InvalidMarginAmount,
    /// 派生余额流水失败。
    #[error("failed to derive balance ledger entry: {0}")]
    BalanceLedger(#[from] BalanceLedgerEntryV2Error),
    /// 创建保证金 reservation 失败。
    #[error("failed to create reservation: {0}")]
    Reservation(#[from] ReservationError),
}

/// 已接受并可由撮合层读取的 Hyperliquid perp 订单快照。
///
/// 该实体表示订单执行状态，并保存下单时的保证金 reservation 快照；
/// 它不表达仓位、手续费、PnL 或资金费。
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
    /// 订单内保存的保证金冻结 MI 事实；不新增净敞口的订单没有新冻结。
    pub reservation: Option<Reservation>,
    /// 已成交数量。
    pub filled_qty: u64,
    /// 订单生命周期状态。
    pub status: HyperliquidPerpOrderStatus,
    /// 是否只减仓。
    pub reduce_only: bool,
    /// 是否是强平流程发出的风险订单。
    pub is_liquidation: bool,
    /// 若是强平订单，则记录来源 liquidation id。
    pub liquidation_id: Option<String>,
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
        reservation: Option<Reservation>,
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
            reservation,
            filled_qty: 0,
            status: HyperliquidPerpOrderStatus::Open,
            reduce_only,
            is_liquidation: false,
            liquidation_id: None,
            client_order_id,
            version: 1,
        }
    }

    /// 可 BDD 规格化的聚合根行为：按四态仓位意图创建 Hyperliquid perp 订单。
    ///
    /// 该方法只创建订单聚合和开仓 / 加仓意图直接派生的冻结余额流水单据，
    /// 不执行余额落账，也不检查仓位开平仓资格。
    pub(crate) fn place(
        input: PlaceHyperliquidPerpOrderInput,
    ) -> Result<PlaceHyperliquidPerpOrderOutcome, HyperliquidPerpOrderBehaviorError> {
        if input.qty == 0 {
            return Err(HyperliquidPerpOrderBehaviorError::InvalidQuantity);
        }
        if input.execution.order_price() == 0 {
            return Err(HyperliquidPerpOrderBehaviorError::InvalidPrice);
        }
        let (reduce_only, reservation, freeze_ledger_entry) = match input.intent {
            PlaceHyperliquidPerpOrderIntent::ReducePosition
            | PlaceHyperliquidPerpOrderIntent::ClosePosition => (true, None, None),
            PlaceHyperliquidPerpOrderIntent::OpenPosition {
                margin_asset_id,
                margin_balance_entity_id,
                margin_amount,
            }
            | PlaceHyperliquidPerpOrderIntent::IncreasePosition {
                margin_asset_id,
                margin_balance_entity_id,
                margin_amount,
            } => {
                if margin_amount == 0 {
                    return Err(HyperliquidPerpOrderBehaviorError::InvalidMarginAmount);
                }

                let reservation = Reservation::new(
                    format!("reservation:{}", input.order_id),
                    input.account_id.clone(),
                    input.order_id.clone(),
                    ReservationMarketKind::Perp,
                    ReservationKind::PerpOpenMargin,
                    margin_asset_id,
                    margin_amount,
                )?;

                let freeze_ledger_entry = BalanceLedgerEntryV2::freeze(
                    format!("balance-ledger:freeze:{}", input.order_id),
                    input.account_id.clone(),
                    reservation.asset_id.clone(),
                    margin_balance_entity_id,
                    reservation.original_amount,
                    BalanceLedgerReason::FreezeForOrder { order_id: input.order_id.clone() },
                )?;
                (false, Some(reservation), Some(freeze_ledger_entry))
            }
        };

        let order = Self::new(
            input.order_id,
            None,
            input.asset,
            input.account_id,
            input.symbol,
            input.side,
            input.execution,
            input.time_in_force,
            input.qty,
            reduce_only,
            input.client_order_id,
            reservation,
        );
        let order = if let Some(liquidation_id) = input.liquidation_id {
            order.with_liquidation(liquidation_id)
        } else {
            order
        };

        Ok(PlaceHyperliquidPerpOrderOutcome { order, freeze_ledger_entry })
    }

    /// 返回带强平来源标记的订单快照。
    pub fn with_liquidation(mut self, liquidation_id: String) -> Self {
        self.is_liquidation = true;
        self.liquidation_id = Some(liquidation_id);
        self
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

impl FieldDiff for HyperliquidPerpOrder {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        let mut changes = vec![
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
            EntityFieldChange::new("is_liquidation", "", self.is_liquidation.to_string()),
            EntityFieldChange::new(
                "liquidation_id",
                "",
                self.liquidation_id.clone().unwrap_or_default(),
            ),
            EntityFieldChange::new(
                "client_order_id",
                "",
                self.client_order_id.clone().unwrap_or_default(),
            ),
        ];
        if let Some(reservation) = &self.reservation {
            changes.extend([
                EntityFieldChange::new("reservation_id", "", reservation.reservation_id.clone()),
                EntityFieldChange::new(
                    "reservation_owner_account_id",
                    "",
                    reservation.owner_account_id.clone(),
                ),
                EntityFieldChange::new(
                    "reservation_caused_by_order_id",
                    "",
                    reservation.caused_by_order_id.clone(),
                ),
                EntityFieldChange::new(
                    "reservation_market_kind",
                    "",
                    reservation.market_kind.as_str(),
                ),
                EntityFieldChange::new(
                    "reservation_kind",
                    "",
                    reservation.reservation_kind.as_str(),
                ),
                EntityFieldChange::new("reservation_asset_id", "", reservation.asset_id.clone()),
                EntityFieldChange::new(
                    "reservation_original_amount",
                    "",
                    reservation.original_amount.to_string(),
                ),
                EntityFieldChange::new(
                    "reservation_remaining_amount",
                    "",
                    reservation.remaining_amount.to_string(),
                ),
                EntityFieldChange::new("reservation_status", "", reservation.status.as_str()),
            ]);
        }
        changes
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
            "reservation_id",
            reservation_string(&self.reservation, |reservation| &reservation.reservation_id),
            reservation_string(&other.reservation, |reservation| &reservation.reservation_id),
        );
        push_change(
            &mut changes,
            "reservation_owner_account_id",
            reservation_string(&self.reservation, |reservation| &reservation.owner_account_id),
            reservation_string(&other.reservation, |reservation| &reservation.owner_account_id),
        );
        push_change(
            &mut changes,
            "reservation_caused_by_order_id",
            reservation_string(&self.reservation, |reservation| &reservation.caused_by_order_id),
            reservation_string(&other.reservation, |reservation| &reservation.caused_by_order_id),
        );
        push_change(
            &mut changes,
            "reservation_market_kind",
            self.reservation
                .as_ref()
                .map(|reservation| reservation.market_kind.as_str())
                .unwrap_or_default(),
            other
                .reservation
                .as_ref()
                .map(|reservation| reservation.market_kind.as_str())
                .unwrap_or_default(),
        );
        push_change(
            &mut changes,
            "reservation_kind",
            self.reservation
                .as_ref()
                .map(|reservation| reservation.reservation_kind.as_str())
                .unwrap_or_default(),
            other
                .reservation
                .as_ref()
                .map(|reservation| reservation.reservation_kind.as_str())
                .unwrap_or_default(),
        );
        push_change(
            &mut changes,
            "reservation_asset_id",
            reservation_string(&self.reservation, |reservation| &reservation.asset_id),
            reservation_string(&other.reservation, |reservation| &reservation.asset_id),
        );
        push_change(
            &mut changes,
            "reservation_original_amount",
            reservation_amount(&self.reservation, |reservation| reservation.original_amount),
            reservation_amount(&other.reservation, |reservation| reservation.original_amount),
        );
        push_change(
            &mut changes,
            "reservation_remaining_amount",
            reservation_amount(&self.reservation, |reservation| reservation.remaining_amount),
            reservation_amount(&other.reservation, |reservation| reservation.remaining_amount),
        );
        push_change(
            &mut changes,
            "reservation_status",
            self.reservation
                .as_ref()
                .map(|reservation| reservation.status.as_str())
                .unwrap_or_default(),
            other
                .reservation
                .as_ref()
                .map(|reservation| reservation.status.as_str())
                .unwrap_or_default(),
        );
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
            "is_liquidation",
            self.is_liquidation.to_string(),
            other.is_liquidation.to_string(),
        );
        push_change(
            &mut changes,
            "liquidation_id",
            self.liquidation_id.clone().unwrap_or_default(),
            other.liquidation_id.clone().unwrap_or_default(),
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

impl Entity for HyperliquidPerpOrder {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.order_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_ORDER_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::MomentInterval
    }

    fn aggregate_role() -> AggregateRole
    where
        Self: Sized,
    {
        AggregateRole::AggregateRoot
    }

    fn financial_classification() -> FinancialClassification
    where
        Self: Sized,
    {
        FinancialClassification::BusinessVoucher
    }

    fn entity_version(&self) -> u64 {
        self.version
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "order_id"
            | "account_id"
            | "symbol"
            | "side"
            | "execution"
            | "time_in_force"
            | "status"
            | "reduce_only"
            | "is_liquidation"
            | "liquidation_id"
            | "client_order_id"
            | "reservation_id"
            | "reservation_owner_account_id"
            | "reservation_caused_by_order_id"
            | "reservation_market_kind"
            | "reservation_asset_id"
            | "reservation_kind"
            | "reservation_status" => 0,
            "exchange_oid"
            | "asset"
            | "price"
            | "qty"
            | "filled_qty"
            | "reservation_original_amount"
            | "reservation_remaining_amount" => 1,
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

fn reservation_string(
    reservation: &Option<Reservation>,
    field: impl FnOnce(&Reservation) -> &String,
) -> String {
    reservation.as_ref().map(field).cloned().unwrap_or_default()
}

fn reservation_amount(
    reservation: &Option<Reservation>,
    field: impl FnOnce(&Reservation) -> u64,
) -> String {
    reservation.as_ref().map(field).map(|value| value.to_string()).unwrap_or_default()
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
