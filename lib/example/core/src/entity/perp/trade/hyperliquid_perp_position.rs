use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff};
use thiserror::Error;

use crate::entity::perp::fund::hyperliquid_perp_funding_settlement::HyperliquidPerpFundingSettlement;

const HYPERLIQUID_PERP_POSITION_ENTITY_TYPE: u8 = 11;

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
    /// 本次成交导致的本地仓位保证金要求变化；正数表示追加占用，负数表示释放占用。
    pub required_margin_delta: i128,
}

/// 一次仓位应用杠杆配置后的业务结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HyperliquidPerpPositionLeverageOutcome {
    /// 杠杆调整导致的本地仓位保证金要求变化；正数表示追加占用，负数表示释放占用。
    pub required_margin_delta: i128,
}

/// Hyperliquid perp 账户在单个合约上的核心仓位事实。
///
/// `version == 0` 可表示 adapter 装载到的空仓位槽位；构造器只装配已校验事实或回放事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpPosition {
    /// 本系统稳定仓位主键，建议由 account + asset/symbol 生成。
    pub position_key: String,
    /// 仓位所属账户 ID。
    pub account_id: String,
    /// Hyperliquid perp asset 编号。
    pub perp_asset_id: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub coin: String,
    /// 带符号的净仓位数量，正数表示多头，负数表示空头。
    pub signed_size: i64,
    /// 当前仓位均价；空仓时为 0。
    pub entry_price: u64,
    /// 当前仓位保证金计算使用的杠杆。
    pub leverage_value: u64,
    /// 当前仓位保证金模式事实。
    pub margin_mode: HyperliquidPerpMarginMode,
    /// 当前仓位生命周期内累计已实现 PnL，允许为负；手续费和资金费不进入该字段。
    pub cumulative_realized_pnl: i64,
    /// 当前仓位实体版本。
    pub version: u64,
}

impl HyperliquidPerpPosition {
    /// 从已经校验过的持久化业务事实或回放事件构造 Hyperliquid perp 仓位。
    ///
    /// 该构造器只装配已知事实，不校验仓位自洽性，不接收或持久化风险快照派生字段。
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        position_key: String,
        account_id: String,
        perp_asset_id: u32,
        coin: String,
        signed_size: i64,
        entry_price: u64,
        leverage_value: u64,
        margin_mode: HyperliquidPerpMarginMode,
        cumulative_realized_pnl: i64,
        version: u64,
    ) -> Self {
        Self {
            position_key,
            account_id,
            perp_asset_id,
            coin,
            signed_size,
            entry_price,
            leverage_value,
            margin_mode,
            cumulative_realized_pnl,
            version,
        }
    }

    /// 返回尚未创建的空仓位槽位。
    pub fn empty_slot(
        position_key: String,
        account_id: String,
        perp_asset_id: u32,
        coin: String,
        leverage_value: u64,
    ) -> Self {
        Self::new(
            position_key,
            account_id,
            perp_asset_id,
            coin,
            0,
            0,
            leverage_value,
            HyperliquidPerpMarginMode::Cross,
            0,
            0,
        )
    }

    /// 返回仓位的稳定主键。
    pub fn position_key(&self) -> &str {
        &self.position_key
    }

    /// 返回仓位是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回仓位是否交易指定 Hyperliquid asset。
    pub fn trades_asset(&self, asset: u32) -> bool {
        self.perp_asset_id == asset
    }

    /// 返回仓位是否交易指定展示合约。
    pub fn trades_symbol(&self, symbol: &str) -> bool {
        self.coin == symbol
    }

    /// 返回仓位是否为空。
    pub fn is_flat(&self) -> bool {
        self.signed_size == 0
    }

    /// 返回仓位是否为多头。
    pub fn is_long(&self) -> bool {
        self.signed_size > 0
    }

    /// 返回仓位是否为空头。
    pub fn is_short(&self) -> bool {
        self.signed_size < 0
    }

    /// 返回仓位方向标签，仅用于查询展示与 replay 字段值。
    pub fn side_label(&self) -> &'static str {
        side_label_for_signed_size(self.signed_size)
    }

    /// 返回仓位绝对数量。
    pub fn qty(&self) -> u64 {
        self.signed_size.unsigned_abs()
    }

    /// 返回仓位状态是否和数量、均价、本地保证金要求一致。
    pub fn has_consistent_state(&self) -> bool {
        match self.signed_size.cmp(&0) {
            std::cmp::Ordering::Equal => {
                self.signed_size == 0
                    && self.entry_price == 0
                    && self.local_initial_margin_required() == Some(0)
                    && matches!(
                        self.lifecycle_status(),
                        HyperliquidPerpPositionStatus::EmptySlot
                            | HyperliquidPerpPositionStatus::Closed
                    )
            }
            std::cmp::Ordering::Less | std::cmp::Ordering::Greater => {
                self.signed_size != 0
                    && self.entry_price > 0
                    && self.lifecycle_status() == HyperliquidPerpPositionStatus::Open
            }
        }
    }

    /// 返回当前仓位名义价值；乘法溢出时返回 `None`。
    pub fn notional_at(&self, price: u64) -> Option<u64> {
        self.qty().checked_mul(price)
    }

    /// 返回当前仓位的本地初始保证金要求。
    pub fn local_initial_margin_required(&self) -> Option<u64> {
        required_position_margin(self.qty(), self.entry_price, self.leverage_value)
    }

    /// 返回当前仓位是否应参与资金费结算。
    pub fn is_funding_eligible(&self) -> bool {
        !self.is_flat() && self.has_consistent_state()
    }

    /// 返回按 oracle 价格计算的资金费名义价值；乘法溢出时返回 `None`。
    pub fn funding_notional(&self, oracle_price: u64) -> Option<u64> {
        self.qty().checked_mul(oracle_price)
    }

    /// 返回当前仓位在指定资金费率下是付款方还是收款方。
    pub fn funding_direction(
        &self,
        funding_rate_e8: i64,
    ) -> Option<HyperliquidPerpFundingDirection> {
        if self.is_flat() || funding_rate_e8 == 0 {
            return None;
        }

        match (self.is_long(), funding_rate_e8.is_positive()) {
            (true, true) | (false, false) => Some(HyperliquidPerpFundingDirection::Pay),
            (true, false) | (false, true) => Some(HyperliquidPerpFundingDirection::Receive),
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

    /// 可 BDD 规格化的聚合根行为：从当前仓位派生单仓位资金费结算单据。
    ///
    /// 该方法只表达 `Position -> FundingSettlement` 的直接下游单据链：
    /// funding 不合格仓位或资金费率为零时不产生结算单据；乘法或金额转换溢出时返回
    /// `HyperliquidPerpPositionError::ArithmeticOverflow`。它不聚合账户级 net delta，
    /// 不派生余额流水，也不应用余额。
    pub fn derive_funding_settlement(
        &self,
        funding_batch_id: &str,
        funding_time: u64,
        oracle_price: u64,
        funding_rate_e8: i64,
    ) -> Result<Option<HyperliquidPerpFundingSettlement>, HyperliquidPerpPositionError> {
        if !self.is_funding_eligible() || funding_rate_e8 == 0 {
            return Ok(None);
        }

        let funding_fee = self
            .funding_fee(oracle_price, funding_rate_e8)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let direction = self
            .funding_direction(funding_rate_e8)
            .ok_or(HyperliquidPerpPositionError::InconsistentState)?;
        let signed_fee = i128::from(funding_fee);
        let signed_usdc_delta = match direction {
            HyperliquidPerpFundingDirection::Pay => {
                signed_fee.checked_neg().ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?
            }
            HyperliquidPerpFundingDirection::Receive => signed_fee,
        };

        Ok(Some(HyperliquidPerpFundingSettlement::new(
            format!("{}-{}", funding_batch_id, self.position_key),
            funding_batch_id.to_string(),
            self.account_id.clone(),
            self.position_key.clone(),
            self.coin.clone(),
            funding_time,
            i128::from(self.signed_size),
            oracle_price,
            funding_rate_e8,
            signed_usdc_delta,
            None,
            None,
        )))
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

        if self.is_long() {
            mark_price <= bankruptcy_price
        } else if self.is_short() {
            mark_price >= bankruptcy_price
        } else {
            false
        }
    }

    /// 返回当前仓位是否可进入强平流程。
    pub fn is_liquidatable(&self) -> bool {
        !self.is_flat() && self.has_consistent_state()
    }

    /// 返回当前仓位在指定 mark 价格下的未实现 PnL。
    pub fn unrealized_pnl_at(&self, mark_price: u64) -> Option<i64> {
        let signed_size = i128::from(self.signed_size);
        let price_delta = i128::from(mark_price) - i128::from(self.entry_price);
        let pnl = signed_size.checked_mul(price_delta)?;
        checked_i128_to_i64(pnl).ok()
    }

    /// 返回 `ceil(qty * entry_price / leverage)`；空仓返回 0。
    pub fn required_margin(&self) -> Option<u64> {
        self.local_initial_margin_required()
    }

    /// 返回当前仓位的生命周期状态。
    pub fn lifecycle_status(&self) -> HyperliquidPerpPositionStatus {
        derive_position_status(self.signed_size, self.version)
    }

    /// 可 BDD 规格化的聚合根行为：以一笔成交创建新的单向净仓位。
    ///
    /// 该行为不修改传入的空仓位槽位事实；返回的新仓位版本固定为 1。
    #[allow(clippy::too_many_arguments)]
    pub fn open_position(
        position_key: String,
        account_id: String,
        perp_asset_id: u32,
        coin: String,
        signed_size_delta: i64,
        trade_price: u64,
        leverage_value: u64,
        margin_mode: HyperliquidPerpMarginMode,
    ) -> Result<
        (HyperliquidPerpPosition, HyperliquidPerpPositionTradeOutcome),
        HyperliquidPerpPositionError,
    > {
        validate_trade_input(signed_size_delta, trade_price)?;
        if leverage_value == 0 {
            return Err(HyperliquidPerpPositionError::InvalidLeverage);
        }

        let required_margin =
            required_position_margin(signed_size_delta.unsigned_abs(), trade_price, leverage_value)
                .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let position = Self::new(
            position_key,
            account_id,
            perp_asset_id,
            coin,
            signed_size_delta,
            trade_price,
            leverage_value,
            margin_mode,
            0,
            1,
        );

        Ok((
            position,
            HyperliquidPerpPositionTradeOutcome {
                realized_pnl_delta: 0,
                required_margin_delta: i128::from(required_margin),
            },
        ))
    }

    /// 可 BDD 规格化的聚合根行为：同向加仓并重算加权均价。
    pub fn increase_position(
        &mut self,
        signed_size_delta: i64,
        trade_price: u64,
    ) -> Result<HyperliquidPerpPositionTradeOutcome, HyperliquidPerpPositionError> {
        validate_existing_trade_transition(self, signed_size_delta, trade_price)?;
        if self.signed_size.signum() != signed_size_delta.signum() {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }

        let before_margin = self.current_required_margin()?;
        let trade_qty = signed_size_delta.unsigned_abs();
        let next_qty = self
            .qty()
            .checked_add(trade_qty)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let old_notional = self
            .qty()
            .checked_mul(self.entry_price)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let add_notional = trade_qty
            .checked_mul(trade_price)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let total_notional = old_notional
            .checked_add(add_notional)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let signed_size = signed_qty(next_qty, self.signed_size.signum())?;
        self.apply_trade_state(signed_size, total_notional / next_qty, 0, before_margin)
    }

    /// 可 BDD 规格化的聚合根行为：反向部分减仓，成交数量必须小于当前仓位数量。
    pub fn reduce_position(
        &mut self,
        signed_size_delta: i64,
        trade_price: u64,
    ) -> Result<HyperliquidPerpPositionTradeOutcome, HyperliquidPerpPositionError> {
        validate_existing_trade_transition(self, signed_size_delta, trade_price)?;
        if self.signed_size.signum() == signed_size_delta.signum()
            || signed_size_delta.unsigned_abs() >= self.qty()
        {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }

        let before_margin = self.current_required_margin()?;
        let realized_pnl_delta =
            self.realized_pnl_for_close_qty(signed_size_delta.unsigned_abs(), trade_price)?;
        let next_qty = self.qty() - signed_size_delta.unsigned_abs();
        let signed_size = signed_qty(next_qty, self.signed_size.signum())?;
        self.apply_trade_state(signed_size, self.entry_price, realized_pnl_delta, before_margin)
    }

    /// 可 BDD 规格化的聚合根行为：反向全平，成交数量必须刚好等于当前仓位数量。
    pub fn close_position(
        &mut self,
        signed_size_delta: i64,
        trade_price: u64,
    ) -> Result<HyperliquidPerpPositionTradeOutcome, HyperliquidPerpPositionError> {
        validate_existing_trade_transition(self, signed_size_delta, trade_price)?;
        if self.signed_size.signum() == signed_size_delta.signum()
            || signed_size_delta.unsigned_abs() != self.qty()
        {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }

        let before_margin = self.current_required_margin()?;
        let realized_pnl_delta =
            self.realized_pnl_for_close_qty(signed_size_delta.unsigned_abs(), trade_price)?;
        self.apply_trade_state(0, 0, realized_pnl_delta, before_margin)
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
        if self.account_id != account_id || self.perp_asset_id != asset {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }
        if self.margin_mode != margin_mode {
            return Err(HyperliquidPerpPositionError::MarginModeMismatch);
        }
        if !self.has_consistent_state() {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }

        let before_margin = self.local_initial_margin_required().unwrap_or(0);
        self.leverage_value = leverage;
        let next_margin = match self.lifecycle_status() {
            HyperliquidPerpPositionStatus::Open => self
                .local_initial_margin_required()
                .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?,
            HyperliquidPerpPositionStatus::EmptySlot | HyperliquidPerpPositionStatus::Closed => 0,
        };
        self.version = next_entity_version(self.version)?;

        Ok(HyperliquidPerpPositionLeverageOutcome {
            required_margin_delta: i128::from(next_margin) - i128::from(before_margin),
        })
    }

    fn current_required_margin(&self) -> Result<u64, HyperliquidPerpPositionError> {
        self.local_initial_margin_required().ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)
    }

    fn realized_pnl_for_close_qty(
        &self,
        close_qty: u64,
        trade_price: u64,
    ) -> Result<i64, HyperliquidPerpPositionError> {
        let pnl_per_unit = if self.is_long() {
            i128::from(trade_price) - i128::from(self.entry_price)
        } else if self.is_short() {
            i128::from(self.entry_price) - i128::from(trade_price)
        } else {
            0
        };
        checked_i128_to_i64(pnl_per_unit * i128::from(close_qty))
    }

    fn apply_trade_state(
        &mut self,
        signed_size: i64,
        entry_price: u64,
        realized_pnl_delta: i64,
        before_margin: u64,
    ) -> Result<HyperliquidPerpPositionTradeOutcome, HyperliquidPerpPositionError> {
        let required_margin =
            required_position_margin(signed_size.unsigned_abs(), entry_price, self.leverage_value)
                .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let cumulative_realized_pnl = self
            .cumulative_realized_pnl
            .checked_add(realized_pnl_delta)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let version = next_entity_version(self.version)?;

        self.signed_size = signed_size;
        self.entry_price = entry_price;
        self.cumulative_realized_pnl = cumulative_realized_pnl;
        self.version = version;

        Ok(HyperliquidPerpPositionTradeOutcome {
            realized_pnl_delta,
            required_margin_delta: i128::from(required_margin) - i128::from(before_margin),
        })
    }
}

impl FieldDiff for HyperliquidPerpPosition {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("position_key", "", self.position_key.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("perp_asset_id", "", self.perp_asset_id.to_string()),
            EntityFieldChange::new("coin", "", self.coin.clone()),
            EntityFieldChange::new("signed_size", "", self.signed_size.to_string()),
            EntityFieldChange::new("entry_price", "", self.entry_price.to_string()),
            EntityFieldChange::new("leverage_value", "", self.leverage_value.to_string()),
            EntityFieldChange::new("margin_mode", "", self.margin_mode.as_str()),
            EntityFieldChange::new(
                "cumulative_realized_pnl",
                "",
                self.cumulative_realized_pnl.to_string(),
            ),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();

        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(
            &mut changes,
            "perp_asset_id",
            self.perp_asset_id.to_string(),
            other.perp_asset_id.to_string(),
        );
        push_change(&mut changes, "coin", &self.coin, &other.coin);
        push_change(
            &mut changes,
            "signed_size",
            self.signed_size.to_string(),
            other.signed_size.to_string(),
        );
        push_change(
            &mut changes,
            "entry_price",
            self.entry_price.to_string(),
            other.entry_price.to_string(),
        );
        push_change(
            &mut changes,
            "leverage_value",
            self.leverage_value.to_string(),
            other.leverage_value.to_string(),
        );
        push_change(
            &mut changes,
            "margin_mode",
            self.margin_mode.as_str(),
            other.margin_mode.as_str(),
        );
        push_change(
            &mut changes,
            "cumulative_realized_pnl",
            self.cumulative_realized_pnl.to_string(),
            other.cumulative_realized_pnl.to_string(),
        );

        changes
    }
}

impl Entity for HyperliquidPerpPosition {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.position_key.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_POSITION_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "position_key" | "account_id" | "coin" | "margin_mode" => 0,
            "perp_asset_id"
            | "signed_size"
            | "entry_price"
            | "leverage_value"
            | "cumulative_realized_pnl" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.position_key))
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

fn derive_position_status(signed_size: i64, version: u64) -> HyperliquidPerpPositionStatus {
    if signed_size == 0 {
        if version == 0 {
            HyperliquidPerpPositionStatus::EmptySlot
        } else {
            HyperliquidPerpPositionStatus::Closed
        }
    } else {
        HyperliquidPerpPositionStatus::Open
    }
}

fn signed_qty(qty: u64, sign: i64) -> Result<i64, HyperliquidPerpPositionError> {
    let qty = i64::try_from(qty).map_err(|_| HyperliquidPerpPositionError::ArithmeticOverflow)?;
    match sign.cmp(&0) {
        std::cmp::Ordering::Greater => Ok(qty),
        std::cmp::Ordering::Less => {
            qty.checked_neg().ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)
        }
        std::cmp::Ordering::Equal => Ok(0),
    }
}

fn side_label_for_signed_size(signed_size: i64) -> &'static str {
    if signed_size > 0 {
        "long"
    } else if signed_size < 0 {
        "short"
    } else {
        "flat"
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

fn validate_trade_input(
    signed_size_delta: i64,
    trade_price: u64,
) -> Result<(), HyperliquidPerpPositionError> {
    if signed_size_delta == 0 {
        return Err(HyperliquidPerpPositionError::InvalidTradeQty);
    }
    if trade_price == 0 {
        return Err(HyperliquidPerpPositionError::InvalidTradePrice);
    }
    Ok(())
}

fn validate_existing_trade_transition(
    position: &HyperliquidPerpPosition,
    signed_size_delta: i64,
    trade_price: u64,
) -> Result<(), HyperliquidPerpPositionError> {
    validate_trade_input(signed_size_delta, trade_price)?;
    if position.lifecycle_status() != HyperliquidPerpPositionStatus::Open {
        return Err(HyperliquidPerpPositionError::InconsistentState);
    }
    if !position.has_consistent_state() {
        return Err(HyperliquidPerpPositionError::InconsistentState);
    }
    if position.leverage_value == 0 {
        return Err(HyperliquidPerpPositionError::InvalidLeverage);
    }
    Ok(())
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
        assert_eq!(position.lifecycle_status(), HyperliquidPerpPositionStatus::EmptySlot);
        assert!(!position.is_funding_eligible());
        assert_eq!(position.required_margin(), Some(0));
        assert_eq!(position.margin_mode, HyperliquidPerpMarginMode::Cross);
        assert_eq!(position.signed_size, 0);
        assert_eq!(position.cumulative_realized_pnl, 0);
        assert_eq!(position.version, 0);
    }

    #[test]
    fn new_assembles_only_persisted_position_facts() {
        let position = HyperliquidPerpPosition::new(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            -3,
            50_000,
            10,
            HyperliquidPerpMarginMode::Cross,
            -25,
            4,
        );

        assert_eq!(position.position_key, "position-1");
        assert_eq!(position.account_id, "trader-1");
        assert_eq!(position.perp_asset_id, 7);
        assert_eq!(position.coin, "BTC-PERP");
        assert!(position.is_short());
        assert_eq!(position.side_label(), "short");
        assert_eq!(position.qty(), 3);
        assert_eq!(position.entry_price, 50_000);
        assert_eq!(position.leverage_value, 10);
        assert_eq!(position.margin_mode, HyperliquidPerpMarginMode::Cross);
        assert_eq!(position.cumulative_realized_pnl, -25);
        assert_eq!(position.version, 4);
    }

    fn open_position(signed_size: i64, entry_price: u64, leverage: u64) -> HyperliquidPerpPosition {
        position_with_facts(
            "position-1",
            "trader-1",
            7,
            "BTC-PERP",
            signed_size,
            entry_price,
            leverage,
            HyperliquidPerpMarginMode::Cross,
            0,
            2,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn position_with_facts(
        position_key: &str,
        account_id: &str,
        perp_asset_id: u32,
        coin: &str,
        signed_size: i64,
        entry_price: u64,
        leverage_value: u64,
        margin_mode: HyperliquidPerpMarginMode,
        cumulative_realized_pnl: i64,
        version: u64,
    ) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::new(
            position_key.to_string(),
            account_id.to_string(),
            perp_asset_id,
            coin.to_string(),
            signed_size,
            entry_price,
            leverage_value,
            margin_mode,
            cumulative_realized_pnl,
            version,
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
        let long = position_with_facts(
            "long-1",
            "trader-1",
            0,
            "BTC-PERP",
            2,
            50_000,
            10,
            HyperliquidPerpMarginMode::Cross,
            0,
            3,
        );
        let short = position_with_facts(
            "short-1",
            "trader-2",
            0,
            "BTC-PERP",
            -2,
            50_000,
            10,
            HyperliquidPerpMarginMode::Cross,
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
        assert_eq!(long.margin_mode, HyperliquidPerpMarginMode::Cross);
        assert_eq!(long.qty(), 2);
        assert!(short.has_consistent_state());
    }

    #[test]
    fn derive_funding_settlement_makes_long_pay_when_rate_is_positive() {
        let position = open_position(2, 50_000, 10);

        let settlement = position
            .derive_funding_settlement("funding-batch-1", 1_717_977_600_000, 60_000, 10_000)
            .unwrap()
            .unwrap();

        assert_eq!(settlement.funding_settlement_id, "funding-batch-1-position-1");
        assert_eq!(settlement.funding_batch_id, "funding-batch-1");
        assert_eq!(settlement.account_id, "trader-1");
        assert_eq!(settlement.position_id, "position-1");
        assert_eq!(settlement.coin, "BTC-PERP");
        assert_eq!(settlement.funding_time, 1_717_977_600_000);
        assert_eq!(settlement.signed_size, 2);
        assert_eq!(settlement.oracle_price, 60_000);
        assert_eq!(settlement.funding_rate_e8, 10_000);
        assert_eq!(settlement.signed_usdc_delta, -12);
        assert_eq!(settlement.funding_fee_abs(), Some(12));
        assert!(settlement.is_payment());
        assert_eq!(settlement.source_hash, None);
        assert_eq!(settlement.n_samples, None);
    }

    #[test]
    fn derive_funding_settlement_makes_short_receive_when_rate_is_positive() {
        let position = open_position(-2, 50_000, 10);

        let settlement = position
            .derive_funding_settlement("funding-batch-1", 1_717_977_600_000, 60_000, 10_000)
            .unwrap()
            .unwrap();

        assert_eq!(settlement.signed_size, -2);
        assert_eq!(settlement.signed_usdc_delta, 12);
        assert_eq!(settlement.funding_fee_abs(), Some(12));
        assert!(!settlement.is_payment());
    }

    #[test]
    fn derive_funding_settlement_returns_none_for_zero_rate_or_ineligible_position() {
        let position = open_position(2, 50_000, 10);
        assert_eq!(
            position
                .derive_funding_settlement("funding-batch-1", 1_717_977_600_000, 60_000, 0)
                .unwrap(),
            None
        );

        let flat = HyperliquidPerpPosition::empty_slot(
            "position-flat".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            10,
        );
        assert_eq!(
            flat.derive_funding_settlement("funding-batch-1", 1_717_977_600_000, 60_000, 10_000)
                .unwrap(),
            None
        );

        let inconsistent = position_with_facts(
            "position-bad",
            "trader-1",
            7,
            "BTC-PERP",
            2,
            0,
            10,
            HyperliquidPerpMarginMode::Cross,
            0,
            1,
        );
        assert!(!inconsistent.has_consistent_state());
        assert_eq!(
            inconsistent
                .derive_funding_settlement("funding-batch-1", 1_717_977_600_000, 60_000, 10_000)
                .unwrap(),
            None
        );
    }

    #[test]
    fn derive_funding_settlement_returns_overflow_when_notional_or_fee_overflows() {
        let notional_overflow = position_with_facts(
            "position-overflow",
            "trader-1",
            7,
            "BTC-PERP",
            i64::MAX,
            1,
            1,
            HyperliquidPerpMarginMode::Cross,
            0,
            1,
        );
        assert_eq!(
            notional_overflow.derive_funding_settlement(
                "funding-batch-1",
                1_717_977_600_000,
                3,
                10_000
            ),
            Err(HyperliquidPerpPositionError::ArithmeticOverflow)
        );

        let fee_overflow = position_with_facts(
            "position-fee-overflow",
            "trader-1",
            7,
            "BTC-PERP",
            i64::MAX,
            1,
            1,
            HyperliquidPerpMarginMode::Cross,
            0,
            1,
        );
        assert_eq!(
            fee_overflow.derive_funding_settlement(
                "funding-batch-1",
                1_717_977_600_000,
                1,
                i64::MAX
            ),
            Err(HyperliquidPerpPositionError::ArithmeticOverflow)
        );
    }

    #[test]
    fn liquidation_trigger_uses_mark_and_bankruptcy_price() {
        let long = position_with_facts(
            "position-long",
            "trader-1",
            0,
            "BTC-PERP",
            2,
            60_000,
            5,
            HyperliquidPerpMarginMode::Cross,
            0,
            1,
        );
        let short = position_with_facts(
            "position-short",
            "trader-2",
            0,
            "BTC-PERP",
            -2,
            60_000,
            5,
            HyperliquidPerpMarginMode::Isolated,
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
    fn open_position_creates_position_without_mutating_empty_slot() {
        let slot = HyperliquidPerpPosition::empty_slot(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            10,
        );

        let (position, outcome) = HyperliquidPerpPosition::open_position(
            slot.position_key.clone(),
            slot.account_id.clone(),
            slot.perp_asset_id,
            slot.coin.clone(),
            3,
            100,
            slot.leverage_value,
            slot.margin_mode,
        )
        .unwrap();

        assert!(position.is_long());
        assert_eq!(position.qty(), 3);
        assert_eq!(position.entry_price, 100);
        assert_eq!(position.required_margin(), Some(30));
        assert_eq!(position.lifecycle_status(), HyperliquidPerpPositionStatus::Open);
        assert_eq!(position.cumulative_realized_pnl, 0);
        assert_eq!(position.version, 1);
        assert_eq!(slot.lifecycle_status(), HyperliquidPerpPositionStatus::EmptySlot);
        assert_eq!(outcome.realized_pnl_delta, 0);
        assert_eq!(outcome.required_margin_delta, 30);
    }

    #[test]
    fn increase_position_adds_same_side_with_weighted_entry() {
        let mut position = open_position(2, 100, 10);

        let outcome = position.increase_position(2, 120).unwrap();

        assert_eq!(position.qty(), 4);
        assert_eq!(position.entry_price, 110);
        assert_eq!(position.required_margin(), Some(44));
        assert_eq!(position.version, 3);
        assert_eq!(outcome.realized_pnl_delta, 0);
        assert_eq!(outcome.required_margin_delta, 24);
    }

    #[test]
    fn reduce_and_close_position_apply_reverse_trade() {
        let mut reduced = open_position(3, 100, 10);
        let reduce = reduced.reduce_position(-1, 130).unwrap();
        assert!(reduced.is_long());
        assert_eq!(reduced.qty(), 2);
        assert_eq!(reduced.entry_price, 100);
        assert_eq!(reduced.required_margin(), Some(20));
        assert_eq!(reduced.cumulative_realized_pnl, 30);
        assert_eq!(reduce.realized_pnl_delta, 30);
        assert_eq!(reduce.required_margin_delta, -10);

        let mut closed = open_position(2, 100, 10);
        let close = closed.close_position(-2, 90).unwrap();
        assert!(closed.is_flat());
        assert_eq!(closed.qty(), 0);
        assert_eq!(closed.entry_price, 0);
        assert_eq!(closed.required_margin(), Some(0));
        assert_eq!(closed.lifecycle_status(), HyperliquidPerpPositionStatus::Closed);
        assert_eq!(closed.cumulative_realized_pnl, -20);
        assert_eq!(close.required_margin_delta, -20);
    }

    #[test]
    fn apply_leverage_setting_recomputes_open_margin() {
        let mut position = open_position(3, 100, 5);

        let outcome = position
            .apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 10)
            .unwrap();

        assert_eq!(position.leverage_value, 10);
        assert_eq!(position.required_margin(), Some(30));
        assert_eq!(position.lifecycle_status(), HyperliquidPerpPositionStatus::Open);
        assert_eq!(position.version, 3);
        assert_eq!(outcome.required_margin_delta, -30);
    }

    #[test]
    fn apply_leverage_setting_rejects_mismatch_and_zero_leverage() {
        let mut position = open_position(3, 100, 5);
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
        assert_eq!(empty.lifecycle_status(), HyperliquidPerpPositionStatus::Closed);
        assert_eq!(empty.required_margin(), Some(0));
        assert_eq!(empty_outcome.required_margin_delta, 0);

        let mut closed = position_with_facts(
            "position-1",
            "trader-1",
            7,
            "BTC-PERP",
            0,
            0,
            5,
            HyperliquidPerpMarginMode::Cross,
            12,
            3,
        );
        let closed_outcome = closed
            .apply_leverage_setting("trader-1", 7, HyperliquidPerpMarginMode::Cross, 20)
            .unwrap();
        assert_eq!(closed.lifecycle_status(), HyperliquidPerpPositionStatus::Closed);
        assert_eq!(closed.required_margin(), Some(0));
        assert_eq!(closed_outcome.required_margin_delta, 0);
    }

    #[test]
    fn created_field_changes_and_diff_include_core_fields() {
        let position = position_with_facts(
            "position-1",
            "trader-1",
            7,
            "BTC-PERP",
            3,
            55_000,
            8,
            HyperliquidPerpMarginMode::Isolated,
            -333,
            4,
        );

        let changes = position.created_field_changes();

        assert!(changes.iter().any(|change| {
            change.field_name.as_ref() == "margin_mode" && change.new_value == "isolated"
        }));
        assert!(changes.iter().any(|change| {
            change.field_name.as_ref() == "cumulative_realized_pnl" && change.new_value == "-333"
        }));

        let mut after = position.clone();
        after.signed_size = 0;
        after.entry_price = 0;
        after.cumulative_realized_pnl = 0;

        let diff = position.diff(&after);
        assert!(diff.iter().any(|change| {
            change.field_name.as_ref() == "signed_size"
                && change.old_value == "3"
                && change.new_value == "0"
        }));
    }
}
