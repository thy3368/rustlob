use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, FieldDiff, FinancialClassification,
    FourColorArchetype,
};
use thiserror::Error;

pub use crate::entity::option::cex::cex_option_instrument::{
    CexOptionInstrument, CexOptionInstrumentStatus, CexOptionType,
};

const CEX_OPTION_ORDER_ENTITY_TYPE: u8 = 41;

/// CEX option 订单方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CexOptionOrderSide {
    /// 买入 option，支付权利金。
    Buy,
    /// 卖出 option，收取权利金并可能占用卖方保证金。
    Sell,
}

impl CexOptionOrderSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// CEX option 订单执行方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CexOptionOrderExecution {
    /// 市价意图。adapter 可映射为 IOC + 激进权利金价格。
    Market {
        /// 市价意图使用的激进权利金价格。
        aggressive_price: u64,
    },
    /// 限价意图。
    Limit {
        /// 权利金限价，不是行权价。
        price: u64,
    },
}

impl CexOptionOrderExecution {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Market { .. } => "market",
            Self::Limit { .. } => "limit",
        }
    }

    /// 返回限价权利金；市价意图没有稳定 maker 限价。
    pub const fn limit_price(self) -> Option<u64> {
        match self {
            Self::Market { .. } => None,
            Self::Limit { price } => Some(price),
        }
    }

    /// 返回撮合或资金占用推导可使用的权利金价格。
    pub const fn order_price(self) -> u64 {
        match self {
            Self::Market { aggressive_price } => aggressive_price,
            Self::Limit { price } => price,
        }
    }
}

/// CEX option 订单有效方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CexOptionOrderTimeInForce {
    /// 一直有效，直到成交或取消。
    Gtc,
    /// 立即成交，剩余取消。
    Ioc,
    /// 只做 Maker，若会立即吃单则拒绝。
    Alo,
}

impl CexOptionOrderTimeInForce {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gtc => "gtc",
            Self::Ioc => "ioc",
            Self::Alo => "alo",
        }
    }
}

/// CEX option 订单生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CexOptionOrderStatus {
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

impl CexOptionOrderStatus {
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

/// CEX option 普通下单的仓位业务意图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceCexOptionOrderIntent {
    /// 建立买方多头，需要占用权利金。
    OpenLong,
    /// 增加买方多头，需要占用权利金。
    IncreaseLong,
    /// 买回平空，不在订单实体内修改仓位。
    CloseShort,
    /// 建立卖方空头，需要占用卖方保证金。
    OpenShort,
    /// 增加卖方空头，需要占用卖方保证金。
    IncreaseShort,
    /// 卖出平多，不在订单实体内修改仓位。
    CloseLong,
}

impl PlaceCexOptionOrderIntent {
    pub const fn reduce_only(&self) -> bool {
        matches!(self, Self::CloseShort | Self::CloseLong)
    }

    pub const fn needs_premium_hold(&self) -> bool {
        matches!(self, Self::OpenLong | Self::IncreaseLong)
    }

    pub const fn needs_short_margin_hold(&self) -> bool {
        matches!(self, Self::OpenShort | Self::IncreaseShort)
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::OpenLong => "open_long",
            Self::IncreaseLong => "increase_long",
            Self::CloseShort => "close_short",
            Self::OpenShort => "open_short",
            Self::IncreaseShort => "increase_short",
            Self::CloseLong => "close_long",
        }
    }
}

/// 买方开多 / 加多下单需要占用的权利金要求。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CexOptionPremiumHoldRequirement {
    /// 被占用账户。
    pub account_id: String,
    /// 来源订单 ID。
    pub order_id: String,
    /// 交易合约 ID。
    pub instrument_id: String,
    /// 权利金资产，通常等于合约 `quote_asset`。
    pub asset_id: String,
    /// 权利金价格。
    pub premium_price: u64,
    /// 订单数量。
    pub quantity: u64,
    /// 本次应占用权利金金额。
    pub premium_amount: u64,
}

/// 下单手续费预占用要求。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CexOptionFeeHoldRequirement {
    /// 被占用账户。
    pub account_id: String,
    /// 来源订单 ID。
    pub order_id: String,
    /// 手续费资产。
    pub asset_id: String,
    /// 手续费金额。
    pub fee_amount: u64,
}

/// 卖方开空 / 加空下单需要占用的保证金要求。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CexOptionShortMarginHoldRequirement {
    /// 被占用账户。
    pub account_id: String,
    /// 来源订单 ID。
    pub order_id: String,
    /// 交易合约 ID。
    pub instrument_id: String,
    /// 保证金资产，通常等于合约 `settle_asset`。
    pub asset_id: String,
    /// 本次应占用卖方保证金金额。
    pub margin_amount: u64,
}

/// 创建 CEX option 订单所需的已校验业务输入。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceCexOptionOrderInput {
    /// 本系统生成的稳定订单 ID。
    pub order_id: String,
    /// 订单所属账户 ID。
    pub account_id: String,
    /// 订单交易的 option 合约 ID。
    pub instrument_id: String,
    /// 下单时已加载的合约规格快照，用于校验挂牌状态并推导资产。
    pub instrument: CexOptionInstrument,
    /// 买卖方向。
    pub order_side: CexOptionOrderSide,
    /// 执行方式，价格字段表达权利金报价。
    pub execution: CexOptionOrderExecution,
    /// 订单有效方式。
    pub time_in_force: CexOptionOrderTimeInForce,
    /// 合约数量。
    pub quantity: u64,
    /// 下单业务意图。
    pub intent: PlaceCexOptionOrderIntent,
    /// 买方开多 / 加多需要占用的权利金金额。
    pub premium_amount: Option<u64>,
    /// 卖方开空 / 加空需要占用的保证金金额。
    pub short_margin_amount: Option<u64>,
    /// 可选手续费预占用金额。
    pub fee_amount: Option<u64>,
    /// 客户端自定义订单 ID。
    pub client_order_id: Option<String>,
}

/// CEX option 下单创建的聚合结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceCexOptionOrderOutcome {
    /// 已创建的订单聚合根。
    pub order: CexOptionOrder,
    /// 买方开多 / 加多时的权利金占用要求。
    pub premium_hold: Option<CexOptionPremiumHoldRequirement>,
    /// 可选手续费预占用要求。
    pub fee_hold: Option<CexOptionFeeHoldRequirement>,
    /// 卖方开空 / 加空时的保证金占用要求。
    pub short_margin_hold: Option<CexOptionShortMarginHoldRequirement>,
}

/// CEX option 订单行为错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CexOptionOrderBehaviorError {
    /// 下单数量必须大于零。
    #[error("option order quantity must be greater than zero")]
    InvalidQuantity,
    /// 权利金报价必须大于零。
    #[error("option order premium price must be greater than zero")]
    InvalidPrice,
    /// 行权价必须大于零。
    #[error("option strike price must be greater than zero")]
    InvalidStrikePrice,
    /// 权利金占用金额必须大于零。
    #[error("option premium hold amount must be greater than zero")]
    InvalidPremiumAmount,
    /// 卖方保证金占用金额必须大于零。
    #[error("option short margin hold amount must be greater than zero")]
    InvalidMarginAmount,
    /// 合约当前不可交易。
    #[error("option instrument is not tradable")]
    InstrumentNotTradable,
    /// 输入合约 ID 与合约规格快照不匹配。
    #[error("option instrument id does not match order input")]
    InstrumentMismatch,
}

/// 已接受并可由撮合层读取的 CEX option 订单快照。
///
/// 这是 `MomentInterval + AggregateRoot + BusinessVoucher`。它只表达订单自身执行事实和
/// 下单资金占用需求，不修改余额、仓位、订单簿，也不处理到期行权结算。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CexOptionOrder {
    /// 本系统生成的稳定订单 ID。
    pub order_id: String,
    /// 订单所属账户 ID。
    pub account_id: String,
    /// 订单交易的 option 合约 ID。
    pub instrument_id: String,
    /// 买卖方向。
    pub order_side: CexOptionOrderSide,
    /// 执行方式，价格字段表达权利金报价。
    pub execution: CexOptionOrderExecution,
    /// 订单有效方式。
    pub time_in_force: CexOptionOrderTimeInForce,
    /// 合约数量。
    pub quantity: u64,
    /// 已成交数量。
    pub filled_quantity: u64,
    /// 是否只减仓。
    pub reduce_only: bool,
    /// 生命周期状态。
    pub status: CexOptionOrderStatus,
    /// 客户端自定义订单 ID。
    pub client_order_id: Option<String>,
    /// 当前订单实体版本。
    pub version: u64,
}

impl CexOptionOrder {
    /// 从已经校验过的业务事实或回放事件构造 CEX option 订单。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        order_id: String,
        account_id: String,
        instrument_id: String,
        order_side: CexOptionOrderSide,
        execution: CexOptionOrderExecution,
        time_in_force: CexOptionOrderTimeInForce,
        quantity: u64,
        reduce_only: bool,
        client_order_id: Option<String>,
    ) -> Self {
        Self {
            order_id,
            account_id,
            instrument_id,
            order_side,
            execution,
            time_in_force,
            quantity,
            filled_quantity: 0,
            reduce_only,
            status: CexOptionOrderStatus::Open,
            client_order_id,
            version: 1,
        }
    }

    /// 可 BDD 规格化的聚合根行为：按 option 仓位意图创建 CEX option 订单。
    ///
    /// 该方法只创建订单聚合和资金占用需求，不执行余额冻结、仓位变更、撮合或到期结算。
    pub fn place(
        input: PlaceCexOptionOrderInput,
    ) -> Result<PlaceCexOptionOrderOutcome, CexOptionOrderBehaviorError> {
        if input.instrument_id != input.instrument.instrument_id {
            return Err(CexOptionOrderBehaviorError::InstrumentMismatch);
        }
        if !input.instrument.is_tradable() {
            return Err(CexOptionOrderBehaviorError::InstrumentNotTradable);
        }
        if input.instrument.strike_price == 0 {
            return Err(CexOptionOrderBehaviorError::InvalidStrikePrice);
        }
        if input.quantity == 0 {
            return Err(CexOptionOrderBehaviorError::InvalidQuantity);
        }
        if input.execution.order_price() == 0 {
            return Err(CexOptionOrderBehaviorError::InvalidPrice);
        }

        let premium_hold = if input.intent.needs_premium_hold() {
            let premium_amount = input
                .premium_amount
                .filter(|amount| *amount > 0)
                .ok_or(CexOptionOrderBehaviorError::InvalidPremiumAmount)?;
            Some(CexOptionPremiumHoldRequirement {
                account_id: input.account_id.clone(),
                order_id: input.order_id.clone(),
                instrument_id: input.instrument_id.clone(),
                asset_id: input.instrument.quote_asset.clone(),
                premium_price: input.execution.order_price(),
                quantity: input.quantity,
                premium_amount,
            })
        } else {
            None
        };

        let short_margin_hold = if input.intent.needs_short_margin_hold() {
            let margin_amount = input
                .short_margin_amount
                .filter(|amount| *amount > 0)
                .ok_or(CexOptionOrderBehaviorError::InvalidMarginAmount)?;
            Some(CexOptionShortMarginHoldRequirement {
                account_id: input.account_id.clone(),
                order_id: input.order_id.clone(),
                instrument_id: input.instrument_id.clone(),
                asset_id: input.instrument.settle_asset.clone(),
                margin_amount,
            })
        } else {
            None
        };

        let fee_hold = input.fee_amount.filter(|amount| *amount > 0).map(|fee_amount| {
            CexOptionFeeHoldRequirement {
                account_id: input.account_id.clone(),
                order_id: input.order_id.clone(),
                asset_id: input.instrument.quote_asset.clone(),
                fee_amount,
            }
        });

        let order = Self::new(
            input.order_id,
            input.account_id,
            input.instrument_id,
            input.order_side,
            input.execution,
            input.time_in_force,
            input.quantity,
            input.intent.reduce_only(),
            input.client_order_id,
        );

        Ok(PlaceCexOptionOrderOutcome { order, premium_hold, fee_hold, short_margin_hold })
    }

    /// 返回带指定执行状态的订单快照。
    pub fn with_execution_state(
        mut self,
        status: CexOptionOrderStatus,
        filled_quantity: u64,
    ) -> Self {
        self.status = status;
        self.filled_quantity = filled_quantity;
        self
    }

    /// 返回订单剩余可成交数量；已成交数量大于订单数量时返回 `None`。
    pub fn remaining_quantity(&self) -> Option<u64> {
        self.quantity.checked_sub(self.filled_quantity)
    }

    /// 返回订单是否仍在开放生命周期中。
    pub fn is_open(&self) -> bool {
        matches!(self.status, CexOptionOrderStatus::Open | CexOptionOrderStatus::PartiallyFilled)
    }

    /// 返回订单是否处于终态。
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            CexOptionOrderStatus::Filled
                | CexOptionOrderStatus::Canceled
                | CexOptionOrderStatus::Rejected
        )
    }

    /// 返回订单当前是否允许撤销。
    pub fn is_cancelable(&self) -> bool {
        self.is_open() && self.remaining_quantity().is_some_and(|quantity| quantity > 0)
    }

    /// 返回订单当前是否可进入撮合。
    pub fn is_matchable(&self) -> bool {
        self.is_open() && self.remaining_quantity().is_some_and(|quantity| quantity > 0)
    }

    /// 返回生命周期状态和成交数量是否一致。
    pub fn has_consistent_execution_state(&self) -> bool {
        match self.status {
            CexOptionOrderStatus::Open => self.filled_quantity == 0,
            CexOptionOrderStatus::PartiallyFilled => {
                0 < self.filled_quantity && self.filled_quantity < self.quantity
            }
            CexOptionOrderStatus::Filled => self.filled_quantity == self.quantity,
            CexOptionOrderStatus::Canceled => self.filled_quantity <= self.quantity,
            CexOptionOrderStatus::Rejected => self.filled_quantity == 0,
        }
    }

    /// 返回订单是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回订单是否交易指定 option 合约。
    pub fn trades_instrument(&self, instrument_id: &str) -> bool {
        self.instrument_id == instrument_id
    }

    /// 返回撮合或资金占用推导可使用的权利金价格。
    pub fn order_price(&self) -> u64 {
        self.execution.order_price()
    }

    /// 返回订单限价权利金价格。
    pub fn limit_price(&self) -> Option<u64> {
        self.execution.limit_price()
    }
}

impl FieldDiff for CexOptionOrder {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("order_id", "", self.order_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("instrument_id", "", self.instrument_id.clone()),
            EntityFieldChange::new("order_side", "", self.order_side.as_str()),
            EntityFieldChange::new("execution", "", self.execution.as_str()),
            EntityFieldChange::new("time_in_force", "", self.time_in_force.as_str()),
            EntityFieldChange::new("price", "", self.order_price().to_string()),
            EntityFieldChange::new("quantity", "", self.quantity.to_string()),
            EntityFieldChange::new("filled_quantity", "", self.filled_quantity.to_string()),
            EntityFieldChange::new("reduce_only", "", self.reduce_only.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
            EntityFieldChange::new(
                "client_order_id",
                "",
                self.client_order_id.clone().unwrap_or_default(),
            ),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(&mut changes, "instrument_id", &self.instrument_id, &other.instrument_id);
        push_change(
            &mut changes,
            "order_side",
            self.order_side.as_str(),
            other.order_side.as_str(),
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
            self.order_price().to_string(),
            other.order_price().to_string(),
        );
        push_change(
            &mut changes,
            "quantity",
            self.quantity.to_string(),
            other.quantity.to_string(),
        );
        push_change(
            &mut changes,
            "filled_quantity",
            self.filled_quantity.to_string(),
            other.filled_quantity.to_string(),
        );
        push_change(
            &mut changes,
            "reduce_only",
            self.reduce_only.to_string(),
            other.reduce_only.to_string(),
        );
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        push_change(
            &mut changes,
            "client_order_id",
            self.client_order_id.clone().unwrap_or_default(),
            other.client_order_id.clone().unwrap_or_default(),
        );
        changes
    }
}

impl Entity for CexOptionOrder {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.order_id.clone()
    }

    fn entity_type() -> u8 {
        CEX_OPTION_ORDER_ENTITY_TYPE
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
            "price" | "quantity" | "filled_quantity" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.order_id))
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
