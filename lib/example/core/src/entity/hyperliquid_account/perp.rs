use decimal::Decimal;
use thiserror::Error;

use super::AccountId;
use crate::entity::{
    HyperliquidPerpMarginMode, HyperliquidPerpPosition, HyperliquidPerpPositionSide,
    MarginReservation, ReservationKind, ReservationMarketKind,
};

const DECIMAL_SCALE: i128 = 100_000_000;

/// 本地统计 perp 清算状态所需的已加载领域事实。
///
/// 该输入用于生成本地估算快照，不表示 Hyperliquid 官方 `clearinghouseState` 返回值。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpClearinghouseStateCalcInput {
    /// 子账户标识。
    pub account_id: AccountId,
    /// 当前 perp 仓位事实。
    pub positions: Vec<HyperliquidPerpPosition>,
    /// 当前 collateral 事实。
    pub collateral: PerpCollateralSnapshot,
    /// 当前市场标记价格事实。
    pub market_marks: Vec<PerpMarketMark>,
    /// 当前资产风险规则事实。
    pub risk_rules: Vec<PerpAssetRiskRule>,
    /// 当前仍 active 的挂单保证金冻结事实。
    pub open_order_margin_reservations: Vec<MarginReservation>,
    /// 本地风险判定策略。
    pub risk_policy: PerpRiskPolicy,
}

/// perp collateral 余额快照。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerpCollateralSnapshot {
    /// 当前原始 USD collateral。
    pub total_raw_usd: Decimal,
    /// 待结算 collateral 变化量；正数表示增加，负数表示减少。
    pub pending_settlement_delta: Decimal,
}

/// perp 市场标记价格。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerpMarketMark {
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 当前 mark price，必须非负。
    pub mark_price: Decimal,
}

/// perp 单资产风险规则。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerpAssetRiskRule {
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 初始保证金率，必须非负。
    pub initial_margin_rate: Decimal,
    /// 维持保证金率，必须非负。
    pub maintenance_margin_rate: Decimal,
}

/// perp 本地风险状态判定策略。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerpRiskPolicy {
    /// 可提资金低于或等于该阈值时进入 reduce-only。
    pub reduce_only_withdrawable_threshold: Decimal,
}

/// 单个 perp 仓位在风险扫描时点的快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpPositionRiskSnapshot {
    /// 仓位核心事实。
    pub position: HyperliquidPerpPosition,
    /// 当前 mark price。
    pub mark_price: Decimal,
    /// 当前仓位名义价值。
    pub position_value: Decimal,
    /// 当前仓位未实现 PnL。
    pub unrealized_pnl: Decimal,
    /// 当前仓位使用的保证金额度。
    pub margin_used: Decimal,
    /// 当前仓位的清算价；未推导时为 `None`。
    pub liquidation_price: Option<Decimal>,
    /// 当前仓位的权益回报率。
    pub return_on_equity: Decimal,
}

/// 本地统计 perp 清算状态失败原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PerpClearinghouseStateCalcError {
    /// 缺少仓位对应的 mark price。
    #[error("missing perp market mark for asset {asset}")]
    MissingMarketMark { asset: u32 },
    /// 缺少仓位对应的风险规则。
    #[error("missing perp risk rule for asset {asset}")]
    MissingRiskRule { asset: u32 },
    /// 输入字段不能为负。
    #[error("perp clearinghouse calc input field {field} must not be negative")]
    NegativeInput { field: &'static str },
    /// reservation 不是 active perp margin reservation。
    #[error("reservation {reservation_id} is not an active perp margin reservation")]
    InvalidReservation { reservation_id: String },
    /// Decimal 或整数换算过程中发生溢出。
    #[error("arithmetic overflow while calculating perp clearinghouse state")]
    ArithmeticOverflow,
}

/// 子账户当前风险态势。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskState {
    /// 风险状态正常。
    Normal,
    /// 已进入降低风险或减仓区间。
    ReduceOnly,
    /// 已进入清算风险区间。
    Liquidation,
}

/// perp 账户的保证金汇总快照。
///
/// 该值对象对齐 Hyperliquid `marginSummary` / `crossMarginSummary` 中的账户级汇总字段。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarginSummary {
    /// 账户权益，即账户总价值。
    account_value: Decimal,
    /// 当前账户级总保证金占用。
    total_margin_used: Decimal,
    /// 当前账户级总持仓名义价值。
    total_position_notional: Decimal,
    /// 当前账户级原始 USD 余额。
    total_raw_usd: Decimal,
}

impl MarginSummary {
    /// 从已校验事实装配保证金汇总。
    pub fn new(
        account_value: Decimal,
        total_margin_used: Decimal,
        total_position_notional: Decimal,
        total_raw_usd: Decimal,
    ) -> Self {
        Self { account_value, total_margin_used, total_position_notional, total_raw_usd }
    }

    /// 返回账户权益。
    pub fn account_value(&self) -> Decimal {
        self.account_value
    }

    /// 返回账户级总保证金占用。
    pub fn total_margin_used(&self) -> Decimal {
        self.total_margin_used
    }

    /// 返回账户级总持仓名义价值。
    pub fn total_position_notional(&self) -> Decimal {
        self.total_position_notional
    }

    /// 返回账户级原始 USD 余额。
    pub fn total_raw_usd(&self) -> Decimal {
        self.total_raw_usd
    }
}

/// 子账户在 perp 清算域上的风险与仓位状态快照。
///
/// 该对象属于 `Snapshot` 语义，主要支持查询持仓、余额与风险状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpClearinghouseState {
    /// 子账户标识。
    account_id: AccountId,
    /// 当前 perp 仓位风险快照集合。
    position_risks: Vec<PerpPositionRiskSnapshot>,
    /// 账户整体保证金汇总。
    margin_summary: MarginSummary,
    /// 仅 cross 保证金池相关的汇总视图。
    cross_margin_summary: MarginSummary,
    /// cross 维持保证金占用；官方未返回该字段时为 `None`。
    cross_maintenance_margin_used: Option<Decimal>,
    /// 当前可提资金。
    withdrawable: Decimal,
    /// 当前风险状态结论。
    risk_state: RiskState,
}

impl PerpClearinghouseState {
    /// 从已校验事实装配子账户 perp 清算状态。
    pub fn new(
        account_id: AccountId,
        position_risks: Vec<PerpPositionRiskSnapshot>,
        margin_summary: MarginSummary,
        cross_margin_summary: MarginSummary,
        cross_maintenance_margin_used: Option<Decimal>,
        withdrawable: Decimal,
        risk_state: RiskState,
    ) -> Self {
        Self {
            account_id,
            position_risks,
            margin_summary,
            cross_margin_summary,
            cross_maintenance_margin_used,
            withdrawable,
            risk_state,
        }
    }

    /// 从已加载领域事实统计生成本地 perp 清算状态估算快照。
    ///
    /// 该入口不会冒充 Hyperliquid 官方 `clearinghouseState`；缺少 mark 或风险规则时返回业务错误。
    /// 所有 Decimal 乘加减都会显式检查溢出。
    pub fn calculate_from_facts(
        input: PerpClearinghouseStateCalcInput,
    ) -> Result<Self, PerpClearinghouseStateCalcError> {
        // 本地估算只接受非负的 collateral、mark、保证金率和风险阈值；
        // 负值表示上游装载到清算域的事实已经异常，不能继续生成风险快照。
        validate_non_negative(input.collateral.total_raw_usd, "collateral.total_raw_usd")?;
        validate_non_negative(
            input.risk_policy.reduce_only_withdrawable_threshold,
            "risk_policy.reduce_only_withdrawable_threshold",
        )?;
        for mark in &input.market_marks {
            validate_non_negative(mark.mark_price, "market_mark.mark_price")?;
        }
        for rule in &input.risk_rules {
            validate_non_negative(rule.initial_margin_rate, "risk_rule.initial_margin_rate")?;
            validate_non_negative(
                rule.maintenance_margin_rate,
                "risk_rule.maintenance_margin_rate",
            )?;
        }

        let mut position_risks = Vec::with_capacity(input.positions.len());
        let mut total_position_notional = zero();
        let mut total_initial_margin_used = zero();
        let mut total_unrealized_pnl = zero();
        let mut cross_position_notional = zero();
        let mut cross_initial_margin_used = zero();
        let mut cross_unrealized_pnl = zero();
        let mut cross_maintenance_margin_used = zero();

        for position in &input.positions {
            // 空仓不贡献名义价值、保证金占用或未实现盈亏；非空仓必须能按 asset 找到
            // 对应的 mark price 与风险规则，否则本地快照缺少必要领域事实。
            if position.is_flat() {
                continue;
            }

            let mark =
                input.market_marks.iter().find(|mark| mark.asset == position.perp_asset_id).ok_or(
                    PerpClearinghouseStateCalcError::MissingMarketMark {
                        asset: position.perp_asset_id,
                    },
                )?;
            let risk_rule =
                input.risk_rules.iter().find(|rule| rule.asset == position.perp_asset_id).ok_or(
                    PerpClearinghouseStateCalcError::MissingRiskRule {
                        asset: position.perp_asset_id,
                    },
                )?;

            let position_notional =
                checked_mul(decimal_from_u64(position.qty())?, mark.mark_price)?;
            let initial_margin_used =
                checked_mul(position_notional, risk_rule.initial_margin_rate)?;
            let position_qty = decimal_from_u64(position.qty())?;
            let entry_price = decimal_from_u64(position.entry_price)?;
            let price_delta = match position.side() {
                HyperliquidPerpPositionSide::Long => checked_sub(mark.mark_price, entry_price)?,
                HyperliquidPerpPositionSide::Short => checked_sub(entry_price, mark.mark_price)?,
                HyperliquidPerpPositionSide::Flat => zero(),
            };
            let unrealized_pnl = checked_mul(position_qty, price_delta)?;
            let return_on_equity = if initial_margin_used.is_zero() {
                zero()
            } else {
                checked_div_decimal(unrealized_pnl, initial_margin_used)?
            };
            let risk_snapshot = PerpPositionRiskSnapshot {
                position: position.clone(),
                mark_price: mark.mark_price,
                position_value: position_notional,
                unrealized_pnl,
                margin_used: initial_margin_used,
                liquidation_price: None,
                return_on_equity,
            };
            position_risks.push(risk_snapshot);

            // 账户级汇总包含所有保证金模式的非空仓：名义价值、初始保证金和未实现盈亏
            // 都以当前 mark 与仓位事实为准逐仓累加。
            total_position_notional = checked_add(total_position_notional, position_notional)?;
            total_initial_margin_used =
                checked_add(total_initial_margin_used, initial_margin_used)?;
            total_unrealized_pnl = checked_add(total_unrealized_pnl, unrealized_pnl)?;

            if position.margin_mode == HyperliquidPerpMarginMode::Cross {
                // cross 池只纳入 cross 仓位；除初始保证金外，额外累计维持保证金，
                // 用于后续清算阈值判断。
                let maintenance_margin_used =
                    checked_mul(position_notional, risk_rule.maintenance_margin_rate)?;
                cross_position_notional = checked_add(cross_position_notional, position_notional)?;
                cross_initial_margin_used =
                    checked_add(cross_initial_margin_used, initial_margin_used)?;
                cross_unrealized_pnl = checked_add(cross_unrealized_pnl, unrealized_pnl)?;
                cross_maintenance_margin_used =
                    checked_add(cross_maintenance_margin_used, maintenance_margin_used)?;
            }
        }

        // active perp 挂单冻结会并入当前保证金占用，体现未成交订单对可提资金的占用。
        let active_open_order_reservation_remaining =
            active_open_order_reservation_remaining(&input.open_order_margin_reservations)?;
        let total_margin_used =
            checked_add(total_initial_margin_used, active_open_order_reservation_remaining)?;
        // 账户权益以 collateral 事实加未实现盈亏估算；cross 视图只叠加 cross 仓位的盈亏。
        let total_raw_usd =
            checked_add(input.collateral.total_raw_usd, input.collateral.pending_settlement_delta)?;
        let account_value = checked_add(total_raw_usd, total_unrealized_pnl)?;
        let cross_account_value = checked_add(total_raw_usd, cross_unrealized_pnl)?;
        let withdrawable_before_floor = checked_sub(account_value, total_margin_used)?;
        // 可提资金不向外暴露负数；保证金不足时 floor 到 0，由风险状态表达压力。
        let withdrawable = if withdrawable_before_floor.is_negative() {
            zero()
        } else {
            withdrawable_before_floor
        };
        // 风险状态先判清算，再判 reduce-only，避免账户已触达维持保证金线时被较弱状态覆盖。
        let risk_state = if account_value <= cross_maintenance_margin_used {
            RiskState::Liquidation
        } else if withdrawable <= input.risk_policy.reduce_only_withdrawable_threshold {
            RiskState::ReduceOnly
        } else {
            RiskState::Normal
        };

        Ok(Self::new(
            input.account_id,
            position_risks,
            MarginSummary::new(
                account_value,
                total_margin_used,
                total_position_notional,
                total_raw_usd,
            ),
            MarginSummary::new(
                cross_account_value,
                cross_initial_margin_used,
                cross_position_notional,
                total_raw_usd,
            ),
            Some(cross_maintenance_margin_used),
            withdrawable,
            risk_state,
        ))
    }

    /// 返回子账户标识。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 返回当前仓位风险快照集合。
    pub fn position_risks(&self) -> &[PerpPositionRiskSnapshot] {
        &self.position_risks
    }

    /// 返回保证金汇总。
    pub fn margin_summary(&self) -> &MarginSummary {
        &self.margin_summary
    }

    /// 返回 cross 保证金汇总。
    pub fn cross_margin_summary(&self) -> &MarginSummary {
        &self.cross_margin_summary
    }

    /// 返回 cross 维持保证金占用；官方未返回该字段时为 `None`。
    pub fn cross_maintenance_margin_used(&self) -> Option<Decimal> {
        self.cross_maintenance_margin_used
    }

    /// 返回当前可提资金。
    pub fn withdrawable(&self) -> Decimal {
        self.withdrawable
    }

    /// 返回当前风险状态。
    pub fn risk_state(&self) -> RiskState {
        self.risk_state
    }

    /// 返回是否存在未平仓仓位。
    pub fn has_open_positions(&self) -> bool {
        self.position_risks.iter().any(|risk| !risk.position.is_flat())
    }

    /// 按展示合约符号查找仓位。
    pub fn position_of(&self, symbol: &str) -> Option<&HyperliquidPerpPosition> {
        self.position_risks
            .iter()
            .find(|risk| risk.position.trades_symbol(symbol))
            .map(|risk| &risk.position)
    }

    /// 按展示合约符号查找仓位风险快照。
    pub fn position_risk_of(&self, symbol: &str) -> Option<&PerpPositionRiskSnapshot> {
        self.position_risks.iter().find(|risk| risk.position.trades_symbol(symbol))
    }
}

fn active_open_order_reservation_remaining(
    reservations: &[MarginReservation],
) -> Result<Decimal, PerpClearinghouseStateCalcError> {
    let mut total = zero();
    for reservation in reservations {
        if reservation.market_kind != ReservationMarketKind::Perp
            || !matches!(
                reservation.reservation_kind,
                ReservationKind::PerpOpenMargin | ReservationKind::PerpFlipNetNewMargin
            )
            || !reservation.is_active()
        {
            return Err(PerpClearinghouseStateCalcError::InvalidReservation {
                reservation_id: reservation.reservation_id.clone(),
            });
        }
        total = checked_add(total, decimal_from_u64(reservation.remaining_amount)?)?;
    }
    Ok(total)
}

fn zero() -> Decimal {
    Decimal::from_raw(0)
}

fn validate_non_negative(
    value: Decimal,
    field: &'static str,
) -> Result<(), PerpClearinghouseStateCalcError> {
    if value.is_negative() {
        return Err(PerpClearinghouseStateCalcError::NegativeInput { field });
    }
    Ok(())
}

fn checked_add(lhs: Decimal, rhs: Decimal) -> Result<Decimal, PerpClearinghouseStateCalcError> {
    lhs.raw()
        .checked_add(rhs.raw())
        .map(Decimal::from_raw)
        .ok_or(PerpClearinghouseStateCalcError::ArithmeticOverflow)
}

fn checked_sub(lhs: Decimal, rhs: Decimal) -> Result<Decimal, PerpClearinghouseStateCalcError> {
    lhs.raw()
        .checked_sub(rhs.raw())
        .map(Decimal::from_raw)
        .ok_or(PerpClearinghouseStateCalcError::ArithmeticOverflow)
}

fn checked_mul(lhs: Decimal, rhs: Decimal) -> Result<Decimal, PerpClearinghouseStateCalcError> {
    lhs.checked_mul(rhs).ok_or(PerpClearinghouseStateCalcError::ArithmeticOverflow)
}

fn checked_div_decimal(
    lhs: Decimal,
    rhs: Decimal,
) -> Result<Decimal, PerpClearinghouseStateCalcError> {
    if rhs.is_zero() {
        return Ok(zero());
    }
    let scaled = i128::from(lhs.raw())
        .checked_mul(i128::from(DECIMAL_SCALE))
        .ok_or(PerpClearinghouseStateCalcError::ArithmeticOverflow)?;
    let quotient = scaled
        .checked_div(i128::from(rhs.raw()))
        .ok_or(PerpClearinghouseStateCalcError::ArithmeticOverflow)?;
    i64::try_from(quotient)
        .map(Decimal::from_raw)
        .map_err(|_| PerpClearinghouseStateCalcError::ArithmeticOverflow)
}

fn decimal_from_u64(value: u64) -> Result<Decimal, PerpClearinghouseStateCalcError> {
    i128::from(value)
        .checked_mul(DECIMAL_SCALE)
        .and_then(|raw| i64::try_from(raw).ok())
        .map(Decimal::from_raw)
        .ok_or(PerpClearinghouseStateCalcError::ArithmeticOverflow)
}

#[cfg(test)]
mod tests {
    use decimal::Decimal;

    use super::*;
    use crate::entity::{HyperliquidPerpMarginMode, HyperliquidPerpPosition};

    fn dec(units: i64) -> Decimal {
        Decimal::from_raw(units * 100_000_000)
    }

    fn sample_position(asset: u32, symbol: &str, quantity: u64) -> HyperliquidPerpPosition {
        let side = if quantity == 0 {
            HyperliquidPerpPositionSide::Flat
        } else {
            HyperliquidPerpPositionSide::Long
        };
        let entry_price = if quantity == 0 { 0 } else { 100_000 };
        let required_margin = if quantity == 0 { 0 } else { 10_000 };

        HyperliquidPerpPosition::new(
            format!("sub-1-{symbol}"),
            "sub-1".to_owned(),
            asset,
            symbol.to_owned(),
            side,
            quantity,
            entry_price,
            5,
            HyperliquidPerpMarginMode::Cross,
            required_margin,
            (quantity > 0).then_some(90_000),
            200,
            50,
            1,
        )
    }

    fn sample_position_risk(asset: u32, symbol: &str, quantity: u64) -> PerpPositionRiskSnapshot {
        PerpPositionRiskSnapshot {
            position: sample_position(asset, symbol, quantity),
            mark_price: dec(100_000),
            position_value: if quantity == 0 { dec(0) } else { dec(100_000) },
            unrealized_pnl: dec(200),
            margin_used: if quantity == 0 { dec(0) } else { dec(10_000) },
            liquidation_price: (quantity > 0).then_some(dec(90_000)),
            return_on_equity: dec(0),
        }
    }

    #[test]
    fn perp_state_detects_open_positions_and_queries_by_symbol() {
        let state = PerpClearinghouseState::new(
            AccountId::from("sub-1"),
            vec![sample_position_risk(0, "BTC-PERP", 0), sample_position_risk(1, "ETH-PERP", 3)],
            MarginSummary::new(dec(20_000), dec(8_000), dec(100_000), dec(19_700)),
            MarginSummary::new(dec(18_000), dec(7_000), dec(100_000), dec(17_700)),
            Some(dec(1_500)),
            dec(5_000),
            RiskState::Normal,
        );

        assert!(state.has_open_positions());
        assert!(state.position_of("ETH-PERP").is_some());
        assert!(state.position_of("SOL-PERP").is_none());
        assert_eq!(state.margin_summary().account_value(), dec(20_000));
        assert_eq!(state.margin_summary().total_margin_used(), dec(8_000));
        assert_eq!(state.margin_summary().total_position_notional(), dec(100_000));
        assert_eq!(state.margin_summary().total_raw_usd(), dec(19_700));
        assert_eq!(state.cross_margin_summary().account_value(), dec(18_000));
        assert_eq!(state.cross_margin_summary().total_margin_used(), dec(7_000));
        assert_eq!(state.cross_margin_summary().total_position_notional(), dec(100_000));
        assert_eq!(state.cross_margin_summary().total_raw_usd(), dec(17_700));
        assert_eq!(state.cross_maintenance_margin_used(), Some(dec(1_500)));
        assert_eq!(state.withdrawable(), dec(5_000));
    }

    #[test]
    fn perp_state_without_open_positions_returns_false() {
        let state = PerpClearinghouseState::new(
            AccountId::from("sub-1"),
            vec![sample_position_risk(0, "BTC-PERP", 0)],
            MarginSummary::new(dec(20_000), dec(0), dec(0), dec(0)),
            MarginSummary::new(dec(20_000), dec(0), dec(0), dec(0)),
            None,
            dec(20_000),
            RiskState::Normal,
        );

        assert!(!state.has_open_positions());
    }
}
