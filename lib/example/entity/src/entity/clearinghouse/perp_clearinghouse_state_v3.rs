use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::BalanceV2;
use crate::account::coin::Coin;
use crate::entity::perp::trade::position_v4::{MarginMode, PositionState, PositionV4};

/// 永续合约账户在指定时点的全仓风险快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerpClearinghouseStateV3 {
    summary: MarginSummary,
    maintenance_margin_used: Decimal,
    withdrawable: Decimal,
    positions: Vec<PositionV4>,
    time: u64,
}

/// 全仓账户的汇总风险事实。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarginSummary {
    /// 账户权益，等于已结算 USDC 加全部持仓浮动盈亏。
    pub account_value: Decimal,
    /// 全部活动仓位的绝对名义价值之和。
    pub total_notional: Decimal,
    /// 已结算 USDC 总额，不包含任何持仓浮动盈亏。
    pub total_raw_usd: Decimal,
    /// 全部持仓的初始保证金占用，不包含挂单保证金占用。
    pub initial_margin_used: Decimal,
}

/// 构建单个全仓仓位风险所需的权威输入。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossPositionInput {
    pub position: PositionV4,
    pub mark_price: Decimal,
    pub maintenance_margin_rate: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum BuildClearinghouseStateError {
    #[error("balance must be a USDC balance")]
    BalanceMustBeUsdc,
    #[error("balance amounts cannot be negative")]
    NegativeBalance,
    #[error("positions must belong to the balance account")]
    AccountMismatch,
    #[error("position must be open with a non-zero size")]
    InactivePosition,
    #[error("position must use cross margin")]
    PositionMustBeCrossMargin,
    #[error("only one active position is allowed per coin")]
    DuplicateCoin,
    #[error("mark price must be greater than zero")]
    InvalidMarkPrice,
    #[error("maintenance margin rate must be in (0, 1]")]
    InvalidMaintenanceMarginRate,
    #[error("snapshot time is older than an input")]
    StaleSnapshot,
    #[error("arithmetic overflow while building clearinghouse state")]
    ArithmeticOverflow,
}

impl PerpClearinghouseStateV3 {
    /// 从余额和活动全仓仓位原子地构建指定时点的风险快照。
    pub fn build(
        balance: BalanceV2,
        positions: Vec<CrossPositionInput>,
        time: u64,
    ) -> Result<Self, BuildClearinghouseStateError> {
        if !balance.is_coin(u64::from(Coin::USDC.index())) {
            return Err(BuildClearinghouseStateError::BalanceMustBeUsdc);
        }
        if balance.available() < Decimal::ZERO || balance.frozen() < Decimal::ZERO {
            return Err(BuildClearinghouseStateError::NegativeBalance);
        }
        if time < balance.updated_at() {
            return Err(BuildClearinghouseStateError::StaleSnapshot);
        }

        let total_raw_usd =
            balance.total().ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;
        let mut total_notional = Decimal::ZERO;
        let mut unrealized_pnl = Decimal::ZERO;
        let mut initial_margin_used = Decimal::ZERO;
        let mut maintenance_margin_used = Decimal::ZERO;
        let mut seen_coins = Vec::with_capacity(positions.len());
        let mut snapshot_positions = Vec::with_capacity(positions.len());

        for input in positions {
            let position = input.position;
            if position.account_id() != balance.account_id() {
                return Err(BuildClearinghouseStateError::AccountMismatch);
            }
            if position.state() != PositionState::Open || position.signed_size().is_zero() {
                return Err(BuildClearinghouseStateError::InactivePosition);
            }
            if position.leverage().leverage_type != MarginMode::Cross {
                return Err(BuildClearinghouseStateError::PositionMustBeCrossMargin);
            }
            if seen_coins.contains(&position.coin()) {
                return Err(BuildClearinghouseStateError::DuplicateCoin);
            }
            if input.mark_price <= Decimal::ZERO {
                return Err(BuildClearinghouseStateError::InvalidMarkPrice);
            }
            if input.maintenance_margin_rate <= Decimal::ZERO
                || input.maintenance_margin_rate > Decimal::ONE
            {
                return Err(BuildClearinghouseStateError::InvalidMaintenanceMarginRate);
            }
            if time < position.updated_at() {
                return Err(BuildClearinghouseStateError::StaleSnapshot);
            }

            let size = position.signed_size().abs();
            let notional = size
                .checked_mul(input.mark_price)
                .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;
            let pnl = input
                .mark_price
                .checked_sub(position.entry_price())
                .and_then(|difference| difference.checked_mul(position.signed_size()))
                .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;
            let position_initial_margin = notional
                .checked_div(Decimal::from(position.leverage().value))
                .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;
            let position_maintenance_margin =
                notional
                    .checked_mul(input.maintenance_margin_rate)
                    .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;

            total_notional = total_notional
                .checked_add(notional)
                .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;
            unrealized_pnl = unrealized_pnl
                .checked_add(pnl)
                .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;
            initial_margin_used = initial_margin_used
                .checked_add(position_initial_margin)
                .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;
            maintenance_margin_used = maintenance_margin_used
                .checked_add(position_maintenance_margin)
                .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;
            seen_coins.push(position.coin());
            snapshot_positions.push(position);
        }

        let account_value = total_raw_usd
            .checked_add(unrealized_pnl)
            .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?;
        let excess_equity = account_value
            .checked_sub(initial_margin_used)
            .ok_or(BuildClearinghouseStateError::ArithmeticOverflow)?
            .max(Decimal::ZERO);
        let withdrawable = balance.available().min(excess_equity);

        Ok(Self {
            summary: MarginSummary {
                account_value,
                total_notional,
                total_raw_usd,
                initial_margin_used,
            },
            maintenance_margin_used,
            withdrawable,
            positions: snapshot_positions,
            time,
        })
    }

    pub fn summary(&self) -> &MarginSummary {
        &self.summary
    }
    pub fn positions(&self) -> &[PositionV4] {
        &self.positions
    }
    pub fn equity(&self) -> Decimal {
        self.summary.account_value
    }
    pub fn maintenance_margin_required(&self) -> Decimal {
        self.maintenance_margin_used
    }
    pub fn withdrawable(&self) -> Decimal {
        self.withdrawable
    }
    pub fn can_withdraw(&self, amount: Decimal) -> bool {
        amount >= Decimal::ZERO && amount <= self.withdrawable
    }
    pub fn is_liquidatable(&self) -> bool {
        self.equity() <= self.maintenance_margin_used
    }
    /// 权益严格低于维持保证金的三分之二时进入 backstop 区。
    pub fn is_backstop_zone(&self) -> bool {
        match (
            self.equity().checked_mul(Decimal::from(3)),
            self.maintenance_margin_used.checked_mul(Decimal::from(2)),
        ) {
            (Some(equity), Some(maintenance_margin)) => equity < maintenance_margin,
            _ => self.equity() < self.maintenance_margin_used,
        }
    }
    pub fn snapshot_at(&self) -> u64 {
        self.time
    }
}

#[cfg(test)]
mod tests {
    use common_entity::Fsm;
    use rust_decimal::Decimal;

    use super::*;
    use crate::entity::perp::trade::position_v4::{
        FillSource, Leverage, PositionEvent, PositionFill,
    };

    const ACCOUNT: u64 = 7;
    fn balance(available: Decimal, frozen: Decimal, time: u64) -> BalanceV2 {
        BalanceV2::new(1, ACCOUNT, u64::from(Coin::USDC.index()), available, frozen, 1, time)
    }
    fn position(
        id: u64,
        account: u64,
        coin: Coin,
        size: Decimal,
        entry: Decimal,
        leverage: u8,
        mode: MarginMode,
        time: u64,
    ) -> PositionV4 {
        let mut p = PositionV4::new(
            id,
            account,
            coin,
            Leverage {
                leverage_type: mode,
                value: leverage,
                raw_usd: (mode == MarginMode::Isolated).then_some(Decimal::ZERO),
            },
            1,
        )
        .unwrap();
        p.apply(PositionEvent::TradeFilled {
            fill: PositionFill { signed_quantity: size, price: entry, source: FillSource::Regular },
            logical_ts: time,
        })
        .unwrap();
        p
    }
    fn input(position: PositionV4, mark: Decimal, mmr: Decimal) -> CrossPositionInput {
        CrossPositionInput { position, mark_price: mark, maintenance_margin_rate: mmr }
    }

    #[test]
    fn builds_empty_snapshot_with_frozen_balance() {
        let state =
            PerpClearinghouseStateV3::build(balance(80.into(), 20.into(), 5), vec![], 5).unwrap();
        assert_eq!(
            state.summary(),
            &MarginSummary {
                account_value: 100.into(),
                total_notional: Decimal::ZERO,
                total_raw_usd: 100.into(),
                initial_margin_used: Decimal::ZERO
            }
        );
        assert_eq!(state.withdrawable(), 80.into());
        assert!(state.positions().is_empty());
    }

    #[test]
    fn summarizes_long_short_profit_loss_and_margin() {
        let long = position(1, ACCOUNT, Coin::BTC, 2.into(), 100.into(), 10, MarginMode::Cross, 4);
        let short =
            position(2, ACCOUNT, Coin::ETH, (-3).into(), 50.into(), 5, MarginMode::Cross, 4);
        let state = PerpClearinghouseStateV3::build(
            balance(100.into(), 20.into(), 4),
            vec![
                input(long, 110.into(), Decimal::new(5, 2)),
                input(short, 60.into(), Decimal::new(1, 1)),
            ],
            4,
        )
        .unwrap();
        assert_eq!(state.summary().total_notional, 400.into());
        assert_eq!(state.equity(), 110.into());
        assert_eq!(state.summary().initial_margin_used, 58.into());
        assert_eq!(state.maintenance_margin_required(), 29.into());
        assert_eq!(state.withdrawable(), 52.into());
    }

    #[test]
    fn enforces_withdrawal_liquidation_and_backstop_boundaries() {
        let p = position(1, ACCOUNT, Coin::BTC, Decimal::ONE, 100.into(), 1, MarginMode::Cross, 1);
        let state = PerpClearinghouseStateV3::build(
            balance(10.into(), Decimal::ZERO, 1),
            vec![input(p, 100.into(), Decimal::new(1, 1))],
            1,
        )
        .unwrap();
        assert!(state.is_liquidatable());
        assert!(!state.is_backstop_zone());
        assert_eq!(state.withdrawable(), Decimal::ZERO);
        assert!(!state.can_withdraw(Decimal::NEGATIVE_ONE));

        let p = position(2, ACCOUNT, Coin::ETH, Decimal::ONE, 100.into(), 1, MarginMode::Cross, 1);
        let state = PerpClearinghouseStateV3::build(
            balance(6.into(), Decimal::ZERO, 1),
            vec![input(p, 100.into(), Decimal::new(1, 1))],
            1,
        )
        .unwrap();
        assert!(state.is_backstop_zone());
    }

    #[test]
    fn rejects_invalid_balance_price_rate_and_time() {
        let wrong_coin = BalanceV2::new(1, ACCOUNT, u64::from(Coin::USDT.index()), 1, 0, 1, 1);
        assert_eq!(
            PerpClearinghouseStateV3::build(wrong_coin, vec![], 1).unwrap_err(),
            BuildClearinghouseStateError::BalanceMustBeUsdc
        );
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(Decimal::NEGATIVE_ONE, Decimal::ZERO, 1),
                vec![],
                1
            )
            .unwrap_err(),
            BuildClearinghouseStateError::NegativeBalance
        );
        assert_eq!(
            PerpClearinghouseStateV3::build(balance(Decimal::ONE, Decimal::ZERO, 2), vec![], 1)
                .unwrap_err(),
            BuildClearinghouseStateError::StaleSnapshot
        );
        let p =
            position(1, ACCOUNT, Coin::BTC, Decimal::ONE, Decimal::ONE, 1, MarginMode::Cross, 1);
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(1.into(), 0.into(), 1),
                vec![input(p.clone(), Decimal::ZERO, Decimal::ONE)],
                1
            )
            .unwrap_err(),
            BuildClearinghouseStateError::InvalidMarkPrice
        );
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(1.into(), 0.into(), 1),
                vec![input(p, Decimal::ONE, Decimal::ZERO)],
                1
            )
            .unwrap_err(),
            BuildClearinghouseStateError::InvalidMaintenanceMarginRate
        );
        let p =
            position(2, ACCOUNT, Coin::ETH, Decimal::ONE, Decimal::ONE, 1, MarginMode::Cross, 1);
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(1.into(), 0.into(), 1),
                vec![input(p, Decimal::ONE, Decimal::from(2))],
                1
            )
            .unwrap_err(),
            BuildClearinghouseStateError::InvalidMaintenanceMarginRate
        );
    }

    #[test]
    fn rejects_position_scope_duplicates_and_overflow() {
        let a =
            position(1, ACCOUNT, Coin::BTC, Decimal::ONE, Decimal::ONE, 1, MarginMode::Cross, 2);
        let b =
            position(2, ACCOUNT, Coin::BTC, Decimal::ONE, Decimal::ONE, 1, MarginMode::Cross, 2);
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(1.into(), 0.into(), 2),
                vec![input(a, 1.into(), 1.into()), input(b, 1.into(), 1.into())],
                2
            )
            .unwrap_err(),
            BuildClearinghouseStateError::DuplicateCoin
        );
        let wrong = position(
            3,
            ACCOUNT + 1,
            Coin::ETH,
            Decimal::ONE,
            Decimal::ONE,
            1,
            MarginMode::Cross,
            2,
        );
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(1.into(), 0.into(), 2),
                vec![input(wrong, 1.into(), 1.into())],
                2
            )
            .unwrap_err(),
            BuildClearinghouseStateError::AccountMismatch
        );
        let isolated =
            position(4, ACCOUNT, Coin::SOL, Decimal::ONE, Decimal::ONE, 1, MarginMode::Isolated, 2);
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(1.into(), 0.into(), 2),
                vec![input(isolated, 1.into(), 1.into())],
                2
            )
            .unwrap_err(),
            BuildClearinghouseStateError::PositionMustBeCrossMargin
        );
        let inactive = PositionV4::empty(5, ACCOUNT, Coin::SOL);
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(1.into(), 0.into(), 2),
                vec![input(inactive, 1.into(), 1.into())],
                2
            )
            .unwrap_err(),
            BuildClearinghouseStateError::InactivePosition
        );
        let stale =
            position(6, ACCOUNT, Coin::SOL, Decimal::ONE, Decimal::ONE, 1, MarginMode::Cross, 3);
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(1.into(), 0.into(), 2),
                vec![input(stale, 1.into(), 1.into())],
                2
            )
            .unwrap_err(),
            BuildClearinghouseStateError::StaleSnapshot
        );
        let huge =
            position(7, ACCOUNT, Coin::ETH, Decimal::MAX, Decimal::ONE, 1, MarginMode::Cross, 2);
        assert_eq!(
            PerpClearinghouseStateV3::build(
                balance(1.into(), 0.into(), 2),
                vec![input(huge, 2.into(), 1.into())],
                2
            )
            .unwrap_err(),
            BuildClearinghouseStateError::ArithmeticOverflow
        );
    }
}
