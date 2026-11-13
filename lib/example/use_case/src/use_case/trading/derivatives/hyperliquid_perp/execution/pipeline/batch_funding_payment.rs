use std::collections::{BTreeMap, BTreeSet};

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::account::symbol::Symbol;
use example_core_entity::common::FundingRate;
use example_core_entity::perp::fund::user_funding::{
    UserFunding, UserFundingError, UserFundingEvent, UserFundingOutput,
};
use example_core_entity::perp::trade::perp_position_v3::{MarginModeV3, PerpPositionV3};
use example_core_entity::{BalanceV2, BalanceV2Error, BalanceV2Event};
use rust_decimal::Decimal;
use thiserror::Error;

/// 批量结算 Hyperliquid perp 资金费的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFundingCmd {
    /// 资金费批次 ID；建议按小时窗口稳定生成。
    pub funding_batch_id: u64,
    /// 资金费结算时间，单位为毫秒。
    pub funding_time: u64,
    /// 本次结算的合约交易对。
    pub symbol: Symbol,
    /// 本次资金费使用的 spot oracle price。
    /// Hyperliquid `oraclePx` 没有固定小数位，按 API 返回值完整解析为 `Decimal`，
    /// 不按订单价格 tick 或固定 scale 截断；资金费按
    /// `position_size * oracle_price * funding_rate` 计算。
    pub oracle_price: Decimal,
    /// signed scale 10 资金费率原始整数；`0.01% = 1000000`。
    pub funding_rate: FundingRate,
    /// 本批次要结算的仓位 ID，顺序必须与 state.positions 一致。
    pub position_ids: Vec<u64>,
}

/// 结算 Hyperliquid perp 资金费所需的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFundingState {
    /// 当前批次涉及的仓位快照。
    pub positions: Vec<PerpPositionV3>,
    /// 当前批次涉及账户的 Cross 保证金币种余额。
    pub margin_balances: Vec<BalanceV2>,
}

/// 批量结算资金费时可能产生的业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SettleHyperliquidPerpFundingError {
    #[error("funding_batch_id must be greater than zero")]
    InvalidFundingBatchId,
    #[error("position_ids must not be empty")]
    EmptyPositionIds,
    #[error("oracle_price must be greater than zero")]
    InvalidOraclePrice,
    #[error("position ids do not match loaded positions")]
    PositionIdsMismatch,
    #[error("position market does not match command symbol")]
    PositionMarketMismatch,
    #[error("isolated margin funding settlement is not supported")]
    UnsupportedMarginMode,
    #[error("margin balance party appears more than once")]
    DuplicateMarginBalance,
    #[error("margin balance was not found")]
    MarginBalanceNotFound,
    #[error("margin balance asset does not match command quote coin")]
    InvalidMarginBalance,
    #[error("available margin balance is insufficient")]
    InsufficientAvailableMargin,
    #[error("balance is closed")]
    BalanceClosed,
    #[error("arithmetic overflow while deriving funding settlement")]
    ArithmeticOverflow,
    #[error("funding transition failed: {0}")]
    FundingTransition(#[from] UserFundingError),
}

/// Use case that settles one Hyperliquid perp funding batch into funding facts and balance deltas.
#[derive(Debug, Clone, Copy, Default)]
pub struct UserFundingUseCase;

/// 结算一批资金费后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFundingChanges {
    /// 本批次新创建的资金费事实。
    pub created_funding_payment: Vec<UserFunding>,
    /// 每条资金费从 pending 到 settled 的权威前后态。
    pub funding_updates: Vec<UpdatedEntityPair<UserFunding>>,
    /// 本批次实际受影响保证金余额的 after 快照，顺序与输入状态稳定对齐。
    pub changed_margin_balances: Vec<BalanceV2>,
}

impl ReplayableChanges for UserFundingChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(self.funding_updates.len() * 2);
        for pair in &self.funding_updates {
            events.push(pair.before.track_create_event()?);
            events.push(pair.to_replayable_event()?);
        }
        Ok(events)
    }
}

impl UseCase for UserFundingUseCase {
    type Command = LogEntry<UserFundingCmd>;
    type StateGiven = UserFundingState;
    type Error = SettleHyperliquidPerpFundingError;
    type StateChanged = UserFundingChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.funding_batch_id == 0 {
            return Err(SettleHyperliquidPerpFundingError::InvalidFundingBatchId);
        }
        if cmd.position_ids.is_empty() {
            return Err(SettleHyperliquidPerpFundingError::EmptyPositionIds);
        }
        if cmd.oracle_price <= Decimal::ZERO {
            return Err(SettleHyperliquidPerpFundingError::InvalidOraclePrice);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        validate_positions(cmd, state)?;
        let (_, _, deltas) = derive_funding_payments(cmd, state)?;
        derive_changed_balances(cmd, state, &deltas).map(|_| ())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let (created_funding_payment, funding_updates, deltas) =
            derive_funding_payments(cmd, state)?;
        let changed_margin_balances = derive_changed_balances(cmd, state, &deltas)?;
        Ok(UserFundingChanges { created_funding_payment, funding_updates, changed_margin_balances })
    }
}

fn validate_positions(
    cmd: &LogEntry<UserFundingCmd>,
    state: &UserFundingState,
) -> Result<(), SettleHyperliquidPerpFundingError> {
    if cmd.position_ids.len() != state.positions.len() {
        return Err(SettleHyperliquidPerpFundingError::PositionIdsMismatch);
    }
    for (position_id, position) in cmd.position_ids.iter().zip(&state.positions) {
        if *position_id != position.position_key() {
            return Err(SettleHyperliquidPerpFundingError::PositionIdsMismatch);
        }
        if position.coin() != cmd.symbol.base {
            return Err(SettleHyperliquidPerpFundingError::PositionMarketMismatch);
        }
        if position.margin_mode() != MarginModeV3::Cross {
            return Err(SettleHyperliquidPerpFundingError::UnsupportedMarginMode);
        }
    }
    Ok(())
}

fn derive_funding_payments(
    cmd: &LogEntry<UserFundingCmd>,
    state: &UserFundingState,
) -> Result<
    (Vec<UserFunding>, Vec<UpdatedEntityPair<UserFunding>>, BTreeMap<u64, Decimal>),
    SettleHyperliquidPerpFundingError,
> {
    let mut payments = Vec::with_capacity(state.positions.len());
    let mut updates = Vec::with_capacity(state.positions.len());
    let mut deltas = BTreeMap::<u64, Decimal>::new();

    for position in &state.positions {
        let Some(usdc) = position.funding_usdc_delta(cmd.oracle_price, cmd.funding_rate)
            .map_err(|error| match error {
                example_core_entity::perp::trade::perp_position_v3::PerpPositionV3Error::InvalidPrice => SettleHyperliquidPerpFundingError::InvalidOraclePrice,
                example_core_entity::perp::trade::perp_position_v3::PerpPositionV3Error::ArithmeticOverflow => SettleHyperliquidPerpFundingError::ArithmeticOverflow,
                _ => SettleHyperliquidPerpFundingError::ArithmeticOverflow,
            })? else { continue };

        let funding_key = format!("{}:{}", cmd.funding_batch_id, position.position_key());
        let mut payment = UserFunding::new(
            common_entity::stable_string_entity_id(&funding_key),
            position.user_id().0,
            cmd.symbol,
            usdc,
            position.signed_size(),
            cmd.funding_rate,
            None,
            cmd.funding_time,
        );
        let pending = payment.clone();
        let (_, impact) =
            payment.transition(UserFundingEvent::Settle { settled_at: cmd.funding_time })?;
        let UserFundingOutput::BalanceImpact { party_id, signed_usdc_delta, .. } = impact;
        updates.push(UpdatedEntityPair::new(pending, payment.clone()));
        payments.push(payment);
        let delta = deltas.entry(party_id).or_default();
        *delta = delta
            .checked_add(signed_usdc_delta)
            .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
    }

    Ok((payments, updates, deltas))
}

fn derive_changed_balances(
    cmd: &LogEntry<UserFundingCmd>,
    state: &UserFundingState,
    deltas: &BTreeMap<u64, Decimal>,
) -> Result<Vec<BalanceV2>, SettleHyperliquidPerpFundingError> {
    let quote_coin_idx = u64::from(cmd.symbol.quote.index());
    let mut parties = BTreeSet::new();
    for balance in &state.margin_balances {
        if balance.coin_idx() != quote_coin_idx {
            return Err(SettleHyperliquidPerpFundingError::InvalidMarginBalance);
        }
        if !parties.insert(balance.party_id()) {
            return Err(SettleHyperliquidPerpFundingError::DuplicateMarginBalance);
        }
    }
    if deltas.keys().any(|party_id| !parties.contains(party_id)) {
        return Err(SettleHyperliquidPerpFundingError::MarginBalanceNotFound);
    }

    let mut changed = Vec::with_capacity(deltas.len());
    for balance in &state.margin_balances {
        let Some(delta) = deltas.get(&balance.party_id()) else {
            continue;
        };
        if delta.is_zero() {
            continue;
        }
        let before = balance.clone();
        let mut after = before.clone();
        if delta.is_sign_positive() {
            common_entity::Fsm::apply(
                &mut after,
                BalanceV2Event::CreditAvailable { amount: *delta, timestamp: cmd.logical_ts() },
            )
            .map_err(map_balance_error)?;
        } else {
            let amount = Decimal::ZERO
                .checked_sub(*delta)
                .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
            common_entity::Fsm::apply(
                &mut after,
                BalanceV2Event::DebitAvailable { amount, timestamp: cmd.logical_ts() },
            )
            .map_err(map_balance_error)?;
        }
        changed.push(after);
    }
    Ok(changed)
}

fn map_balance_error(error: BalanceV2Error) -> SettleHyperliquidPerpFundingError {
    match error {
        BalanceV2Error::InsufficientAvailableBalance => {
            SettleHyperliquidPerpFundingError::InsufficientAvailableMargin
        }
        BalanceV2Error::BalanceClosed => SettleHyperliquidPerpFundingError::BalanceClosed,
        BalanceV2Error::ArithmeticOverflow => SettleHyperliquidPerpFundingError::ArithmeticOverflow,
        BalanceV2Error::InvalidAmount
        | BalanceV2Error::InsufficientFrozenBalance
        | BalanceV2Error::CannotCloseNonZeroBalance => {
            SettleHyperliquidPerpFundingError::ArithmeticOverflow
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::LogEntrySequencer;
    use example_core_entity::account::coin::Coin;
    use example_core_entity::common::UserId;
    use example_core_entity::perp::trade::perp_position_v3::FundingAccumulatorV3;

    use super::*;

    fn position(id: u64, user: u64, size: i64, mode: MarginModeV3) -> PerpPositionV3 {
        PerpPositionV3::new(
            id,
            UserId(user),
            Coin::BTC,
            Decimal::from(size),
            Decimal::from(50_000),
            Decimal::from(10),
            mode,
            (mode == MarginModeV3::Isolated).then_some(Decimal::from(1_000)),
            Decimal::ZERO,
            FundingAccumulatorV3::default(),
            Decimal::ZERO,
            3,
        )
        .unwrap()
    }

    fn command(rate_e10: i64, position_ids: Vec<u64>) -> LogEntry<UserFundingCmd> {
        LogEntrySequencer::from_metadata(
            1,
            1_717_977_600_000,
            7,
            9,
            common_entity::SYSTEM_USER_ID.to_owned(),
            UserFundingCmd {
                funding_batch_id: 1,
                funding_time: 1_717_977_600_000,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDC),
                oracle_price: Decimal::from(50_000),
                funding_rate: FundingRate::from_scaled_i128(rate_e10 as i128),
                position_ids,
            },
        )
    }

    fn balance(user: u64, available: i64) -> BalanceV2 {
        BalanceV2::new(
            user,
            user,
            u64::from(Coin::USDC.index()),
            Decimal::from(available),
            Decimal::ZERO,
            4,
            0,
        )
    }

    #[test]
    fn creates_long_payment_and_short_receipt_and_updates_balances() {
        let state = UserFundingState {
            positions: vec![
                position(11, 1, 2, MarginModeV3::Cross),
                position(12, 2, -2, MarginModeV3::Cross),
            ],
            margin_balances: vec![balance(1, 100), balance(2, 100)],
        };

        let changed = UserFundingUseCase
            .compute_state_changed(&command(1_000_000, vec![11, 12]), &state)
            .unwrap();

        assert_eq!(changed.created_funding_payment.len(), 2);
        assert!(changed.created_funding_payment.iter().all(|funding| funding.state()
            == example_core_entity::perp::fund::user_funding::UserFundingState::Settled
            && funding.version() == 2));
        assert_eq!(changed.funding_updates.len(), 2);
        assert!(changed.funding_updates.iter().all(|pair| {
            pair.before.state()
                == example_core_entity::perp::fund::user_funding::UserFundingState::Pending
                && pair.after.state()
                    == example_core_entity::perp::fund::user_funding::UserFundingState::Settled
        }));
        let events = changed.to_replayable_events().unwrap();
        assert_eq!(events.len(), 4);
        assert!(events[0].is_created());
        assert!(events[1].is_updated());
        assert_eq!(changed.created_funding_payment[0].usdc(), Decimal::from(-10));
        assert_eq!(changed.created_funding_payment[1].usdc(), Decimal::from(10));
        assert_eq!(changed.created_funding_payment[0].funding_rate(), Decimal::new(1, 4));
        assert_eq!(changed.changed_margin_balances.len(), 2);
        assert_eq!(changed.changed_margin_balances[0].available(), Decimal::from(90));
        assert_eq!(changed.changed_margin_balances[1].available(), Decimal::from(110));
        assert_eq!(changed.changed_margin_balances[0].version(), 5);
        assert_eq!(changed.changed_margin_balances[1].version(), 5);
    }

    #[test]
    fn preserves_hyperliquid_ten_decimal_funding_rate_precision() {
        let state = UserFundingState {
            positions: vec![position(11, 1, 2, MarginModeV3::Cross)],
            margin_balances: vec![balance(1, 100)],
        };

        let changed =
            UserFundingUseCase.compute_state_changed(&command(216_606, vec![11]), &state).unwrap();

        assert_eq!(changed.created_funding_payment.len(), 1);
        assert_eq!(changed.created_funding_payment[0].usdc(), Decimal::new(-216_606, 5));
        assert_eq!(changed.created_funding_payment[0].funding_rate(), Decimal::new(216_606, 10));
        assert_eq!(changed.changed_margin_balances[0].available(), Decimal::new(9_783_394, 5));
    }

    #[test]
    fn aggregates_multiple_positions_before_updating_one_balance() {
        let state = UserFundingState {
            positions: vec![
                position(11, 1, 1, MarginModeV3::Cross),
                position(12, 1, 2, MarginModeV3::Cross),
            ],
            margin_balances: vec![balance(1, 100)],
        };

        let changed = UserFundingUseCase
            .compute_state_changed(&command(1_000_000, vec![11, 12]), &state)
            .unwrap();

        assert_eq!(changed.created_funding_payment.len(), 2);
        assert_eq!(changed.changed_margin_balances.len(), 1);
        assert_eq!(changed.changed_margin_balances[0].available(), Decimal::from(85));
        assert_eq!(changed.changed_margin_balances[0].version(), 5);
    }

    #[test]
    fn zero_rate_creates_no_payment_or_balance_change() {
        let state = UserFundingState {
            positions: vec![position(11, 1, 2, MarginModeV3::Cross)],
            margin_balances: vec![balance(1, 100)],
        };

        let changed =
            UserFundingUseCase.compute_state_changed(&command(0, vec![11]), &state).unwrap();

        assert!(changed.created_funding_payment.is_empty());
        assert!(changed.changed_margin_balances.is_empty());
    }

    #[test]
    fn negative_rate_reverses_long_and_short_directions() {
        let state = UserFundingState {
            positions: vec![
                position(11, 1, 2, MarginModeV3::Cross),
                position(12, 2, -2, MarginModeV3::Cross),
            ],
            margin_balances: vec![balance(1, 100), balance(2, 100)],
        };

        let changed = UserFundingUseCase
            .compute_state_changed(&command(-1_000_000, vec![11, 12]), &state)
            .unwrap();

        assert_eq!(changed.created_funding_payment[0].usdc(), Decimal::from(10));
        assert_eq!(changed.created_funding_payment[1].usdc(), Decimal::from(-10));
        assert_eq!(changed.changed_margin_balances[0].available(), Decimal::from(110));
        assert_eq!(changed.changed_margin_balances[1].available(), Decimal::from(90));
    }

    #[test]
    fn rejects_mismatched_position_isolated_margin_and_insufficient_balance() {
        let cross = UserFundingState {
            positions: vec![position(11, 1, 2, MarginModeV3::Cross)],
            margin_balances: vec![balance(1, 5)],
        };
        assert_eq!(
            UserFundingUseCase.compute_state_changed(&command(1_000_000, vec![99]), &cross),
            Err(SettleHyperliquidPerpFundingError::PositionIdsMismatch)
        );
        assert_eq!(
            UserFundingUseCase.compute_state_changed(&command(1_000_000, vec![11]), &cross),
            Err(SettleHyperliquidPerpFundingError::InsufficientAvailableMargin)
        );

        let isolated = UserFundingState {
            positions: vec![position(11, 1, 2, MarginModeV3::Isolated)],
            margin_balances: vec![balance(1, 100)],
        };
        assert_eq!(
            UserFundingUseCase.compute_state_changed(&command(1_000_000, vec![11]), &isolated),
            Err(SettleHyperliquidPerpFundingError::UnsupportedMarginMode)
        );

        let missing_balance = UserFundingState {
            positions: vec![position(11, 1, 2, MarginModeV3::Cross)],
            margin_balances: vec![],
        };
        assert_eq!(
            UserFundingUseCase
                .compute_state_changed(&command(1_000_000, vec![11]), &missing_balance),
            Err(SettleHyperliquidPerpFundingError::MarginBalanceNotFound)
        );

        let wrong_asset = UserFundingState {
            positions: vec![position(11, 1, 2, MarginModeV3::Cross)],
            margin_balances: vec![BalanceV2::new(
                1,
                1,
                u64::from(Coin::USDT.index()),
                Decimal::from(100),
                Decimal::ZERO,
                4,
                0,
            )],
        };
        assert_eq!(
            UserFundingUseCase.compute_state_changed(&command(1_000_000, vec![11]), &wrong_asset),
            Err(SettleHyperliquidPerpFundingError::InvalidMarginBalance)
        );
    }
}
