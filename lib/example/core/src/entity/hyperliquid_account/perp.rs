use decimal::Decimal;

use super::AccountId;
use crate::entity::{HyperliquidPerpPosition, HyperliquidPerpPositionSide};

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
    /// 当前 perp 仓位快照集合。
    positions: Vec<HyperliquidPerpPosition>,
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
        positions: Vec<HyperliquidPerpPosition>,
        margin_summary: MarginSummary,
        cross_margin_summary: MarginSummary,
        cross_maintenance_margin_used: Option<Decimal>,
        withdrawable: Decimal,
        risk_state: RiskState,
    ) -> Self {
        Self {
            account_id,
            positions,
            margin_summary,
            cross_margin_summary,
            cross_maintenance_margin_used,
            withdrawable,
            risk_state,
        }
    }

    /// 返回子账户标识。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 返回当前持仓集合。
    pub fn positions(&self) -> &[HyperliquidPerpPosition] {
        &self.positions
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
        self.positions.iter().any(|position| !position.is_flat())
    }

    /// 按展示合约符号查找仓位。
    pub fn position_of(&self, symbol: &str) -> Option<&HyperliquidPerpPosition> {
        self.positions.iter().find(|position| position.trades_symbol(symbol))
    }
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

    #[test]
    fn perp_state_detects_open_positions_and_queries_by_symbol() {
        let state = PerpClearinghouseState::new(
            AccountId::from("sub-1"),
            vec![sample_position(0, "BTC-PERP", 0), sample_position(1, "ETH-PERP", 3)],
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
            vec![sample_position(0, "BTC-PERP", 0)],
            MarginSummary::new(dec(20_000), dec(0), dec(0), dec(0)),
            MarginSummary::new(dec(20_000), dec(0), dec(0), dec(0)),
            None,
            dec(20_000),
            RiskState::Normal,
        );

        assert!(!state.has_open_positions());
    }
}
