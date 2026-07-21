use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff};
use thiserror::Error;

const HYPERLIQUID_PERP_POSITION_ENTITY_TYPE: u8 = 11;

/// Hyperliquid perp 单向净仓位方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpPositionSide {
    /// 无持仓。
    Flat,
    /// 多头净仓位。
    Long,
    /// 空头净仓位。
    Short,
}

impl HyperliquidPerpPositionSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Long => "long",
            Self::Short => "short",
        }
    }
}

/// Hyperliquid perp 仓位槽位的持久化生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpPositionStatus {
    /// 尚未由真实仓位创建事件落库的空槽位。
    EmptySlot,
    /// 当前存在非零净仓位。
    Open,
    /// 曾经存在仓位，但当前已经完全平仓。
    Closed,
}

impl HyperliquidPerpPositionStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySlot => "empty_slot",
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

/// 当前仓位在一次资金费结算中的收付方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpFundingDirection {
    /// 当前仓位需要支付资金费。
    Pay,
    /// 当前仓位会收到资金费。
    Receive,
}

/// Hyperliquid perp 保证金模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpMarginMode {
    /// 账户级共享保证金池。
    Cross,
    /// 仓位级独立保证金池。
    Isolated,
}

impl HyperliquidPerpMarginMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cross => "cross",
            Self::Isolated => "isolated",
        }
    }
}

/// Hyperliquid perp 仓位实体行为可能产生的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HyperliquidPerpPositionError {
    /// 成交数量必须大于 0。
    #[error("trade qty must be greater than zero")]
    InvalidTradeQty,
    /// 成交价格必须大于 0。
    #[error("trade price must be greater than zero")]
    InvalidTradePrice,
    /// 杠杆必须大于等于 1。
    #[error("leverage must be greater than or equal to 1")]
    InvalidLeverage,
    /// 仓位字段之间不自洽。
    #[error("position state is inconsistent")]
    InconsistentState,
    /// 仓位和杠杆配置的保证金模式不一致。
    #[error("margin mode does not match leverage setting")]
    MarginModeMismatch,
    /// 推进仓位状态时发生整数溢出。
    #[error("arithmetic overflow while updating perp position")]
    ArithmeticOverflow,
}

/// 一次成交落仓位后的业务结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HyperliquidPerpPositionTradeOutcome {
    /// 本次成交新增的已实现 PnL。
    pub realized_pnl_delta: i64,
    /// 本次成交导致的保证金占用变化；正数表示追加占用，负数表示释放占用。
    pub margin_delta: i128,
}

/// 一次仓位应用杠杆配置后的业务结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HyperliquidPerpPositionLeverageOutcome {
    /// 杠杆调整导致的保证金占用变化；正数表示追加占用，负数表示释放占用。
    pub margin_delta: i128,
}

/// Hyperliquid perp 账户在单个合约上的单向净仓位快照。
///
/// `version == 0` 可表示 adapter 加载到的未创建空仓位槽位；结算后若产生非空仓位，
/// use case 会发出 create event。构造器假设输入来自已校验命令或事件回放。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpPosition {
    /// 本系统稳定仓位 ID，建议由 account + asset/symbol 生成。
    pub position_id: String,
    /// 仓位所属账户 ID。
    pub account_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 单向净仓位方向。
    pub side: HyperliquidPerpPositionSide,
    /// 当前净仓位数量。
    pub qty: u64,
    /// 当前仓位均价；空仓时为 0。
    pub entry_price: u64,
    /// 当前仓位保证金计算使用的杠杆。
    pub leverage: u64,
    /// 当前仓位保证金模式事实。
    pub margin_mode: HyperliquidPerpMarginMode,
    /// 当前仓位占用保证金。
    pub margin: u64,
    /// 当前仓位强平价；未知或上游未提供时为 `None`。
    ///
    /// 这是上游或 adapter 提供的风险快照事实，不是主动平仓成交价。
    pub liquidation_price: Option<u64>,
    /// 当前仓位未实现 PnL，允许为负。
    pub unrealized_pnl: i64,
    /// 累计已实现 PnL，允许为负。
    pub realized_pnl: i64,
    /// 当前仓位槽位的持久化生命周期状态。
    pub status: HyperliquidPerpPositionStatus,
    /// 当前仓位实体版本。
    pub version: u64,
}

impl HyperliquidPerpPosition {
    /// 从已经校验过的业务事实或回放事件构造 Hyperliquid perp 仓位。
    ///
    /// 该构造器只装配已知事实，不在 entity 内推导强平价。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        position_id: String,
        account_id: String,
        asset: u32,
        symbol: String,
        side: HyperliquidPerpPositionSide,
        qty: u64,
        entry_price: u64,
        leverage: u64,
        margin_mode: HyperliquidPerpMarginMode,
        margin: u64,
        liquidation_price: Option<u64>,
        unrealized_pnl: i64,
        realized_pnl: i64,
        version: u64,
    ) -> Self {
        let status = derive_position_status(side, qty, version);
        Self {
            position_id,
            account_id,
            asset,
            symbol,
            side,
            qty,
            entry_price,
            leverage,
            margin_mode,
            margin,
            liquidation_price,
            unrealized_pnl,
            realized_pnl,
            status,
            version,
        }
    }

    /// 返回尚未创建的空仓位槽位。
    pub fn empty_slot(
        position_id: String,
        account_id: String,
        asset: u32,
        symbol: String,
        leverage: u64,
    ) -> Self {
        Self::new(
            position_id,
            account_id,
            asset,
            symbol,
            HyperliquidPerpPositionSide::Flat,
            0,
            0,
            leverage,
            HyperliquidPerpMarginMode::Cross,
            0,
            None,
            0,
            0,
            0,
        )
    }

    /// 返回仓位是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回仓位是否交易指定 Hyperliquid asset。
    pub fn trades_asset(&self, asset: u32) -> bool {
        self.asset == asset
    }

    /// 返回仓位是否交易指定展示合约。
    pub fn trades_symbol(&self, symbol: &str) -> bool {
        self.symbol == symbol
    }

    /// 返回仓位是否为空。
    pub fn is_flat(&self) -> bool {
        self.side == HyperliquidPerpPositionSide::Flat && self.qty == 0
    }

    /// 返回仓位状态是否和数量、均价、保证金一致。
    pub fn has_consistent_state(&self) -> bool {
        match self.side {
            HyperliquidPerpPositionSide::Flat => {
                self.qty == 0
                    && self.entry_price == 0
                    && self.margin == 0
                    && matches!(
                        self.status,
                        HyperliquidPerpPositionStatus::EmptySlot
                            | HyperliquidPerpPositionStatus::Closed
                    )
            }
            HyperliquidPerpPositionSide::Long | HyperliquidPerpPositionSide::Short => {
                self.qty > 0
                    && self.entry_price > 0
                    && self.status == HyperliquidPerpPositionStatus::Open
            }
        }
    }

    /// 返回当前仓位名义价值；乘法溢出时返回 `None`。
    pub fn notional(&self) -> Option<u64> {
        self.qty.checked_mul(self.entry_price)
    }

    /// 返回当前仓位是否应参与资金费结算。
    pub fn is_funding_eligible(&self) -> bool {
        !self.is_flat() && self.has_consistent_state()
    }

    /// 返回按 oracle 价格计算的资金费名义价值；乘法溢出时返回 `None`。
    pub fn funding_notional(&self, oracle_price: u64) -> Option<u64> {
        self.qty.checked_mul(oracle_price)
    }

    /// 返回当前仓位在指定资金费率下是付款方还是收款方。
    pub fn funding_direction(
        &self,
        funding_rate_e8: i64,
    ) -> Option<HyperliquidPerpFundingDirection> {
        if self.is_flat() || funding_rate_e8 == 0 {
            return None;
        }

        match (self.side, funding_rate_e8.is_positive()) {
            (HyperliquidPerpPositionSide::Long, true)
            | (HyperliquidPerpPositionSide::Short, false) => {
                Some(HyperliquidPerpFundingDirection::Pay)
            }
            (HyperliquidPerpPositionSide::Long, false)
            | (HyperliquidPerpPositionSide::Short, true) => {
                Some(HyperliquidPerpFundingDirection::Receive)
            }
            (HyperliquidPerpPositionSide::Flat, _) => None,
        }
    }

    /// 返回当前仓位在指定 oracle 价格和资金费率下的资金费绝对金额。
    pub fn funding_fee(&self, oracle_price: u64, funding_rate_e8: i64) -> Option<u64> {
        if funding_rate_e8 == 0 {
            return Some(0);
        }
        let notional = self.funding_notional(oracle_price)? as u128;
        let rate = funding_rate_e8.unsigned_abs() as u128;
        let fee = notional.checked_mul(rate)?.checked_div(100_000_000)?;
        u64::try_from(fee).ok()
    }

    /// 返回当前仓位是否满足强平触发条件。
    pub fn liquidation_triggered_by_mark_price(
        &self,
        mark_price: u64,
        bankruptcy_price: u64,
    ) -> bool {
        if self.is_flat() || !self.has_consistent_state() {
            return false;
        }

        match self.side {
            HyperliquidPerpPositionSide::Long => mark_price <= bankruptcy_price,
            HyperliquidPerpPositionSide::Short => mark_price >= bankruptcy_price,
            HyperliquidPerpPositionSide::Flat => false,
        }
    }

    /// 返回当前仓位是否可进入强平流程。
    pub fn is_liquidatable(&self) -> bool {
        !self.is_flat() && self.has_consistent_state()
    }

    /// 返回当前仓位强平价事实；未知或上游未提供时返回 `None`。
    ///
    /// 该值来自上游风险快照或 adapter，不在 entity 内推导。
    pub fn liquidation_price(&self) -> Option<u64> {
        self.liquidation_price
    }

    /// 返回 `ceil(qty * entry_price / leverage)`；空仓返回 0。
    pub fn required_margin(&self) -> Option<u64> {
        required_position_margin(self.qty, self.entry_price, self.leverage)
    }

    /// 可 BDD 规格化的聚合根行为：应用一笔成交到当前单向净仓位。
    ///
    /// 该行为会重算均价、保证金占用、已实现 PnL 增量和生命周期状态；强平价和未实现 PnL
    /// 在成交后清空为等待下一次风险快照刷新。
    pub fn settle_trade(
        &mut self,
        incoming_side: HyperliquidPerpPositionSide,
        trade_qty: u64,
        trade_price: u64,
    ) -> Result<HyperliquidPerpPositionTradeOutcome, HyperliquidPerpPositionError> {
        if trade_qty == 0 {
            return Err(HyperliquidPerpPositionError::InvalidTradeQty);
        }
        if trade_price == 0 {
            return Err(HyperliquidPerpPositionError::InvalidTradePrice);
        }
        if incoming_side == HyperliquidPerpPositionSide::Flat || !self.has_consistent_state() {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }
        if self.leverage == 0 {
            return Err(HyperliquidPerpPositionError::InvalidLeverage);
        }
        if self.required_margin() != Some(self.margin) {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }

        let before_margin = self.margin;
        let (side, qty, entry_price, realized_pnl_delta) =
            self.position_after_trade(incoming_side, trade_qty, trade_price)?;
        let margin = required_position_margin(qty, entry_price, self.leverage)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let realized_pnl = self
            .realized_pnl
            .checked_add(realized_pnl_delta)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let version = next_entity_version(self.version)?;

        self.side = side;
        self.qty = qty;
        self.entry_price = entry_price;
        self.margin = margin;
        self.liquidation_price = None;
        self.unrealized_pnl = 0;
        self.realized_pnl = realized_pnl;
        self.version = version;
        self.status = derive_position_status(self.side, self.qty, self.version);

        Ok(HyperliquidPerpPositionTradeOutcome {
            realized_pnl_delta,
            margin_delta: i128::from(self.margin) - i128::from(before_margin),
        })
    }

    /// 可 BDD 规格化的聚合根行为：将权威杠杆配置应用到当前仓位快照。
    ///
    /// `account_id + asset + margin_mode + leverage` 是上游配置真相同步到仓位所需的业务事实投影。
    pub fn apply_leverage_setting(
        &mut self,
        account_id: &str,
        asset: u32,
        margin_mode: HyperliquidPerpMarginMode,
        leverage: u64,
    ) -> Result<HyperliquidPerpPositionLeverageOutcome, HyperliquidPerpPositionError> {
        if leverage == 0 {
            return Err(HyperliquidPerpPositionError::InvalidLeverage);
        }
        if self.account_id != account_id || self.asset != asset {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }
        if self.margin_mode != margin_mode {
            return Err(HyperliquidPerpPositionError::MarginModeMismatch);
        }
        if !self.has_consistent_state() {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }

        let before_margin = self.margin;
        self.leverage = leverage;
        self.margin = match self.status {
            HyperliquidPerpPositionStatus::Open => {
                required_position_margin(self.qty, self.entry_price, self.leverage)
                    .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?
            }
            HyperliquidPerpPositionStatus::EmptySlot | HyperliquidPerpPositionStatus::Closed => 0,
        };
        self.version = next_entity_version(self.version)?;

        Ok(HyperliquidPerpPositionLeverageOutcome {
            margin_delta: i128::from(self.margin) - i128::from(before_margin),
        })
    }

    fn position_after_trade(
        &self,
        incoming_side: HyperliquidPerpPositionSide,
        trade_qty: u64,
        trade_price: u64,
    ) -> Result<(HyperliquidPerpPositionSide, u64, u64, i64), HyperliquidPerpPositionError> {
        if self.is_flat() || self.side == incoming_side {
            let next_qty = self
                .qty
                .checked_add(trade_qty)
                .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
            let old_notional = self
                .qty
                .checked_mul(self.entry_price)
                .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
            let add_notional = trade_qty
                .checked_mul(trade_price)
                .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
            let total_notional = old_notional
                .checked_add(add_notional)
                .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
            return Ok((incoming_side, next_qty, total_notional / next_qty, 0));
        }

        let close_qty = self.qty.min(trade_qty);
        let pnl_per_unit = match self.side {
            HyperliquidPerpPositionSide::Long => {
                i128::from(trade_price) - i128::from(self.entry_price)
            }
            HyperliquidPerpPositionSide::Short => {
                i128::from(self.entry_price) - i128::from(trade_price)
            }
            HyperliquidPerpPositionSide::Flat => 0,
        };
        let realized_pnl_delta = checked_i128_to_i64(pnl_per_unit * i128::from(close_qty))?;

        if trade_qty < self.qty {
            Ok((self.side, self.qty - trade_qty, self.entry_price, realized_pnl_delta))
        } else if trade_qty == self.qty {
            Ok((HyperliquidPerpPositionSide::Flat, 0, 0, realized_pnl_delta))
        } else {
            Ok((incoming_side, trade_qty - self.qty, trade_price, realized_pnl_delta))
        }
    }
}

impl FieldDiff for HyperliquidPerpPosition {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("position_id", "", self.position_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("entry_price", "", self.entry_price.to_string()),
            EntityFieldChange::new("leverage", "", self.leverage.to_string()),
            EntityFieldChange::new("margin_mode", "", self.margin_mode.as_str()),
            EntityFieldChange::new("margin", "", self.margin.to_string()),
            EntityFieldChange::new(
                "liquidation_price",
                "",
                option_u64_value(self.liquidation_price),
            ),
            EntityFieldChange::new("unrealized_pnl", "", self.unrealized_pnl.to_string()),
            EntityFieldChange::new("realized_pnl", "", self.realized_pnl.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();

        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(&mut changes, "asset", self.asset.to_string(), other.asset.to_string());
        push_change(&mut changes, "symbol", &self.symbol, &other.symbol);
        push_change(&mut changes, "side", self.side.as_str(), other.side.as_str());
        push_change(&mut changes, "qty", self.qty.to_string(), other.qty.to_string());
        push_change(
            &mut changes,
            "entry_price",
            self.entry_price.to_string(),
            other.entry_price.to_string(),
        );
        push_change(
            &mut changes,
            "leverage",
            self.leverage.to_string(),
            other.leverage.to_string(),
        );
        push_change(
            &mut changes,
            "margin_mode",
            self.margin_mode.as_str(),
            other.margin_mode.as_str(),
        );
        push_change(&mut changes, "margin", self.margin.to_string(), other.margin.to_string());
        push_change(
            &mut changes,
            "liquidation_price",
            option_u64_value(self.liquidation_price),
            option_u64_value(other.liquidation_price),
        );
        push_change(
            &mut changes,
            "unrealized_pnl",
            self.unrealized_pnl.to_string(),
            other.unrealized_pnl.to_string(),
        );
        push_change(
            &mut changes,
            "realized_pnl",
            self.realized_pnl.to_string(),
            other.realized_pnl.to_string(),
        );
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());

        changes
    }
}

impl Entity for HyperliquidPerpPosition {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.position_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_POSITION_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "position_id" | "account_id" | "symbol" | "side" | "margin_mode" | "status" => 0,
            "asset" | "qty" | "entry_price" | "leverage" | "margin" | "liquidation_price"
            | "unrealized_pnl" | "realized_pnl" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.position_id))
    }
}

/// 返回 `ceil(qty * price / leverage)`，任一算术步骤溢出或杠杆为零时返回 `None`。
pub fn required_position_margin(qty: u64, price: u64, leverage: u64) -> Option<u64> {
    if qty == 0 {
        return Some(0);
    }
    if leverage == 0 {
        return None;
    }
    let notional = qty.checked_mul(price)?;
    let quotient = notional / leverage;
    let remainder = notional % leverage;
    if remainder == 0 { Some(quotient) } else { quotient.checked_add(1) }
}

fn derive_position_status(
    side: HyperliquidPerpPositionSide,
    qty: u64,
    version: u64,
) -> HyperliquidPerpPositionStatus {
    if side == HyperliquidPerpPositionSide::Flat && qty == 0 {
        if version == 0 {
            HyperliquidPerpPositionStatus::EmptySlot
        } else {
            HyperliquidPerpPositionStatus::Closed
        }
    } else {
        HyperliquidPerpPositionStatus::Open
    }
}

fn next_entity_version(version: u64) -> Result<u64, HyperliquidPerpPositionError> {
    if version == 0 {
        Ok(1)
    } else {
        version.checked_add(1).ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)
    }
}

fn checked_i128_to_i64(value: i128) -> Result<i64, HyperliquidPerpPositionError> {
    i64::try_from(value).map_err(|_| HyperliquidPerpPositionError::ArithmeticOverflow)
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

fn option_u64_value(value: Option<u64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_slot_carries_leverage_for_future_position_creation() {
        let position = HyperliquidPerpPosition::empty_slot(
            "buyer:BTC-PERP".to_string(),
            "buyer".to_string(),
            0,
            "BTC-PERP".to_string(),
            10,
        );

        assert!(position.belongs_to_account("buyer"));
        assert!(position.trades_asset(0));
        assert!(position.trades_symbol("BTC-PERP"));
        assert!(position.is_flat());
        assert_eq!(position.status, HyperliquidPerpPositionStatus::EmptySlot);
        assert!(!position.is_funding_eligible());
        assert_eq!(position.required_margin(), Some(0));
        assert_eq!(position.margin_mode, HyperliquidPerpMarginMode::Cross);
        assert_eq!(position.margin, 0);
        assert_eq!(position.liquidation_price(), None);
        assert_eq!(position.unrealized_pnl, 0);
        assert_eq!(position.realized_pnl, 0);
        assert_eq!(position.version, 0);
    }

    fn open_position(
        side: HyperliquidPerpPositionSide,
        qty: u64,
        entry_price: u64,
        leverage: u64,
    ) -> HyperliquidPerpPosition {
        let margin = required_position_margin(qty, entry_price, leverage).unwrap();
        HyperliquidPerpPosition::new(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            side,
            qty,
            entry_price,
            leverage,
            HyperliquidPerpMarginMode::Cross,
            margin,
            Some(45_000),
            123,
            0,
            2,
        )
    }

    #[test]
    fn required_margin_rounds_up() {
        assert_eq!(required_position_margin(3, 101, 5), Some(61));
        assert_eq!(required_position_margin(10, 20, 5), Some(40));
        assert_eq!(required_position_margin(0, 20, 0), Some(0));
        assert_eq!(required_position_margin(1, 1, 0), None);
    }

    #[test]
    fn funding_helpers_follow_hyperliquid_direction_rule() {
        let long = HyperliquidPerpPosition::new(
            "long-1".to_string(),
            "trader-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            2,
            50_000,
            10,
            HyperliquidPerpMarginMode::Cross,
            10_000,
            Some(45_000),
            321,
            0,
            3,
        );
        let short = HyperliquidPerpPosition::new(
            "short-1".to_string(),
            "trader-2".to_string(),
            0,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Short,
            2,
            50_000,
            10,
            HyperliquidPerpMarginMode::Cross,
            10_000,
            Some(55_000),
            -123,
            0,
            3,
        );

        assert!(long.is_funding_eligible());
        assert_eq!(long.funding_notional(60_000), Some(120_000));
        assert_eq!(long.funding_direction(10_000), Some(HyperliquidPerpFundingDirection::Pay));
        assert_eq!(short.funding_direction(10_000), Some(HyperliquidPerpFundingDirection::Receive));
        assert_eq!(long.funding_direction(-10_000), Some(HyperliquidPerpFundingDirection::Receive));
        assert_eq!(long.funding_fee(60_000, 10_000), Some(12));
        assert_eq!(long.funding_fee(60_000, 0), Some(0));
        assert_eq!(long.liquidation_price(), Some(45_000));
        assert_eq!(long.margin_mode, HyperliquidPerpMarginMode::Cross);
        assert_eq!(long.unrealized_pnl, 321);
        assert!(short.has_consistent_state());
    }

    #[test]
    fn liquidation_trigger_uses_mark_and_bankruptcy_price() {
        let long = HyperliquidPerpPosition::new(
            "position-long".to_string(),
            "trader-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            2,
            60_000,
            5,
            HyperliquidPerpMarginMode::Cross,
            24_000,
            None,
            0,
            0,
            1,
        );
        let short = HyperliquidPerpPosition::new(
            "position-short".to_string(),
            "trader-2".to_string(),
            0,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Short,
            2,
            60_000,
            5,
            HyperliquidPerpMarginMode::Isolated,
            24_000,
            None,
            1_000,
            0,
            1,
        );

        assert!(long.liquidation_triggered_by_mark_price(50_000, 50_000));
        assert!(!long.liquidation_triggered_by_mark_price(50_001, 50_000));
        assert!(short.liquidation_triggered_by_mark_price(50_000, 50_000));
        assert!(!short.liquidation_triggered_by_mark_price(49_999, 50_000));
        assert!(long.is_liquidatable());
    }

    #[test]
    fn settle_trade_opens_empty_slot() {
        let mut position = HyperliquidPerpPosition::empty_slot(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            10,
        );

        let outcome = position.settle_trade(HyperliquidPerpPositionSide::Long, 3, 100).unwrap();

        assert_eq!(position.side, HyperliquidPerpPositionSide::Long);
        assert_eq!(position.qty, 3);
        assert_eq!(position.entry_price, 100);
        assert_eq!(position.margin, 30);
        assert_eq!(position.status, HyperliquidPerpPositionStatus::Open);
        assert_eq!(position.version, 1);
        assert_eq!(outcome.realized_pnl_delta, 0);
        assert_eq!(outcome.margin_delta, 30);
    }

    #[test]
    fn settle_trade_increases_same_side_with_weighted_entry() {
        let mut position = open_position(HyperliquidPerpPositionSide::Long, 2, 100, 10);

        let outcome = position.settle_trade(HyperliquidPerpPositionSide::Long, 2, 120).unwrap();

        assert_eq!(position.qty, 4);
        assert_eq!(position.entry_price, 110);
        assert_eq!(position.margin, 44);
        assert_eq!(position.liquidation_price(), None);
        assert_eq!(position.unrealized_pnl, 0);
        assert_eq!(position.version, 3);
        assert_eq!(outcome.realized_pnl_delta, 0);
        assert_eq!(outcome.margin_delta, 24);
    }

    #[test]
    fn settle_trade_reduces_closes_and_flips_position() {
        let mut reduced = open_position(HyperliquidPerpPositionSide::Long, 3, 100, 10);
        let reduce = reduced.settle_trade(HyperliquidPerpPositionSide::Short, 1, 130).unwrap();
        assert_eq!(reduced.side, HyperliquidPerpPositionSide::Long);
        assert_eq!(reduced.qty, 2);
        assert_eq!(reduced.entry_price, 100);
        assert_eq!(reduced.margin, 20);
        assert_eq!(reduced.realized_pnl, 30);
        assert_eq!(reduce.realized_pnl_delta, 30);
        assert_eq!(reduce.margin_delta, -10);

        let mut closed = open_position(HyperliquidPerpPositionSide::Long, 2, 100, 10);
        let close = closed.settle_trade(HyperliquidPerpPositionSide::Short, 2, 90).unwrap();
        assert_eq!(closed.side, HyperliquidPerpPositionSide::Flat);
        assert_eq!(closed.qty, 0);
        assert_eq!(closed.entry_price, 0);
        assert_eq!(closed.margin, 0);
        assert_eq!(closed.status, HyperliquidPerpPositionStatus::Closed);
        assert_eq!(closed.realized_pnl, -20);
        assert_eq!(close.margin_delta, -20);

        let mut flipped = open_position(HyperliquidPerpPositionSide::Long, 2, 100, 10);
        let flip = flipped.settle_trade(HyperliquidPerpPositionSide::Short, 3, 90).unwrap();
        assert_eq!(flipped.side, HyperliquidPerpPositionSide::Short);
        assert_eq!(flipped.qty, 1);
        assert_eq!(flipped.entry_price, 90);
        assert_eq!(flipped.margin, 9);
        assert_eq!(flipped.status, HyperliquidPerpPositionStatus::Open);
        assert_eq!(flipped.realized_pnl, -20);
        assert_eq!(flip.margin_delta, -11);
    }

    #[test]
    fn apply_leverage_setting_recomputes_open_margin() {
        let mut position = open_position(HyperliquidPerpPositionSide::Long, 3, 100, 5);

        let outcome = position
            .apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 10)
            .unwrap();

        assert_eq!(position.leverage, 10);
        assert_eq!(position.margin, 30);
        assert_eq!(position.status, HyperliquidPerpPositionStatus::Open);
        assert_eq!(position.version, 3);
        assert_eq!(outcome.margin_delta, -30);
    }

    #[test]
    fn apply_leverage_setting_rejects_mismatch_and_zero_leverage() {
        let mut position = open_position(HyperliquidPerpPositionSide::Long, 3, 100, 5);
        assert_eq!(
            position.apply_leverage_setting("other", 7, HyperliquidPerpMarginMode::Cross, 10),
            Err(HyperliquidPerpPositionError::InconsistentState)
        );

        assert_eq!(
            position.apply_leverage_setting("trader-1", 8, HyperliquidPerpMarginMode::Cross, 10),
            Err(HyperliquidPerpPositionError::InconsistentState)
        );

        assert_eq!(
            position.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Isolated, 10),
            Err(HyperliquidPerpPositionError::MarginModeMismatch)
        );

        assert_eq!(
            position.apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 0),
            Err(HyperliquidPerpPositionError::InvalidLeverage)
        );
    }

    #[test]
    fn apply_leverage_setting_keeps_empty_and_closed_margin_zero() {
        let mut empty = HyperliquidPerpPosition::empty_slot(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            5,
        );
        let empty_outcome = empty
            .apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 20)
            .unwrap();
        assert_eq!(empty.status, HyperliquidPerpPositionStatus::EmptySlot);
        assert_eq!(empty.margin, 0);
        assert_eq!(empty_outcome.margin_delta, 0);

        let mut closed = HyperliquidPerpPosition::new(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Flat,
            0,
            0,
            5,
            HyperliquidPerpMarginMode::Cross,
            0,
            None,
            0,
            12,
            3,
        );
        let closed_outcome = closed
            .apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 20)
            .unwrap();
        assert_eq!(closed.status, HyperliquidPerpPositionStatus::Closed);
        assert_eq!(closed.margin, 0);
        assert_eq!(closed_outcome.margin_delta, 0);
    }

    #[test]
    fn created_field_changes_and_diff_include_status_and_liquidation_price() {
        let position = HyperliquidPerpPosition::new(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            3,
            55_000,
            8,
            HyperliquidPerpMarginMode::Isolated,
            20_625,
            Some(48_000),
            -456,
            123,
            4,
        );

        let changes = position.created_field_changes();

        assert!(changes.iter().any(|change| {
            change.field_name.as_ref() == "margin_mode" && change.new_value == "isolated"
        }));
        assert!(changes.iter().any(|change| {
            change.field_name.as_ref() == "liquidation_price" && change.new_value == "48000"
        }));
        assert!(changes.iter().any(|change| {
            change.field_name.as_ref() == "unrealized_pnl" && change.new_value == "-456"
        }));
        assert!(changes.iter().any(|change| {
            change.field_name.as_ref() == "status" && change.new_value == "open"
        }));

        let mut after = position.clone();
        after.side = HyperliquidPerpPositionSide::Flat;
        after.qty = 0;
        after.entry_price = 0;
        after.margin = 0;
        after.liquidation_price = None;
        after.status = HyperliquidPerpPositionStatus::Closed;

        let diff = position.diff(&after);
        assert!(diff.iter().any(|change| {
            change.field_name.as_ref() == "liquidation_price"
                && change.old_value == "48000"
                && change.new_value.is_empty()
        }));
        assert!(diff.iter().any(|change| {
            change.field_name.as_ref() == "status"
                && change.old_value == "open"
                && change.new_value == "closed"
        }));
    }
}
