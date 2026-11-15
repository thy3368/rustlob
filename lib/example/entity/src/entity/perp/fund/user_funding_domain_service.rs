use std::collections::HashSet;

use common_entity::{stable_string_entity_id, Fsm};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::account::symbol::Symbol;
use crate::entity::common::price::Price;
use crate::perp::fund::user_funding::{
    UserFunding, UserFundingError, UserFundingEvent, UserFundingOutput, UserFundingState,
};
use crate::{
    BalanceV2, BalanceV2Error, BalanceV2Event, FundingRate, MarginModeV3, PerpPositionV3,
    PerpPositionV3Error, PerpPositionV3Event,
};

/// 按小时生成用户资金费事实时可能发生的错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SettleHourError {
    #[error("funding batch id must be greater than zero")]
    InvalidFundingBatchId,
    #[error("oracle price must be greater than zero")]
    NonPositiveOracle,
    #[error("duplicate position id: {position_id}")]
    DuplicatePositionId { position_id: u64 },
    #[error("position {position_id} market does not match settlement symbol")]
    PositionMarketMismatch { position_id: u64 },
    #[error("decimal arithmetic overflow for position {position_id}")]
    ArithmeticOverflow { position_id: u64 },
}

/// 按小时从仓位快照派生待结算的用户资金费事实。
///
/// 本函数先验证完整输入并计算全部金额，任一输入错误都会使整批失败；成功时保持仓位输入顺序，
/// 不修改仓位、余额，也不推进 `UserFunding` 的结算状态。
pub fn settle_hour(
    symbol: Symbol,
    funding_batch_id: u64,
    time_ms: u64,
    rate: FundingRate,
    oracle: Price,
    positions: &[PerpPositionV3],
) -> Result<Vec<UserFunding>, SettleHourError> {
    if funding_batch_id == 0 {
        return Err(SettleHourError::InvalidFundingBatchId);
    }
    if oracle <= Decimal::ZERO {
        return Err(SettleHourError::NonPositiveOracle);
    }

    let mut position_ids = HashSet::with_capacity(positions.len());
    for position in positions {
        let position_id = position.position_id();
        if !position_ids.insert(position_id) {
            return Err(SettleHourError::DuplicatePositionId { position_id });
        }
        if position.coin() != symbol.base {
            return Err(SettleHourError::PositionMarketMismatch { position_id });
        }
    }

    let mut pending = Vec::with_capacity(positions.len());
    for position in positions {
        let position_id = position.position_id();
        let usdc = position
            .funding_usdc_delta(oracle, rate)
            .map_err(|_| SettleHourError::ArithmeticOverflow { position_id })?;
        if let Some(usdc) = usdc {
            pending.push((position, usdc));
        }
    }

    Ok(pending
        .into_iter()
        .map(|(position, usdc)| {
            let funding_key = format!("{funding_batch_id}:{}", position.position_id());
            UserFunding::new(
                stable_string_entity_id(&funding_key),
                position.user_id().0,
                symbol,
                usdc,
                position.signed_size(),
                rate,
                None,
                time_ms,
            )
        })
        .collect())
}

/// 单笔小时资金费生效后的三个权威快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHour2Output {
    pub user_funding: UserFunding,
    pub position: PerpPositionV3,
    pub balance: BalanceV2,
}

/// 单笔小时资金费生效失败的明确领域原因。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SettleHour2Error {
    #[error("user funding is already settled")]
    FundingAlreadySettled,
    #[error("funding settlement time is invalid")]
    InvalidSettlementTime,
    #[error("user funding version overflow")]
    FundingVersionOverflow,
    #[error("funding party does not match position owner")]
    FundingPositionPartyMismatch,
    #[error("funding market does not match position market")]
    FundingPositionMarketMismatch,
    #[error("funding settlement only supports cross-margin positions")]
    CrossMarginRequired,
    #[error("balance account does not match requested account")]
    BalanceAccountMismatch,
    #[error("balance party does not match funding party")]
    BalancePartyMismatch,
    #[error("balance coin does not match funding quote coin")]
    BalanceQuoteCoinMismatch,
    #[error("zero funding amount cannot be settled")]
    ZeroFundingAmount,
    #[error("available balance is insufficient for funding debit")]
    InsufficientAvailableBalance,
    #[error("balance is closed")]
    BalanceClosed,
    #[error("position funding accumulator, version, or timestamp arithmetic failed")]
    PositionArithmeticOverflow,
    #[error("balance amount or version arithmetic overflow")]
    BalanceArithmeticOverflow,
}

/// 将一个待结算资金费事实原子推导为资金费、仓位和余额的 after 快照。
///
/// 本函数只接受全仓仓位。它先验证全部跨实体身份与账户不变量，再在输入的 clone 上依次推进
/// 三个 FSM；任一步失败都不会修改调用方持有的输入，也不会返回部分生效结果。
pub fn settle_hour2(
    user_funding: &UserFunding,
    position: &PerpPositionV3,
    balance: &BalanceV2,
    account_id: u64,
    settled_at: u64,
) -> Result<SettleHour2Output, SettleHour2Error> {
    if user_funding.state() == UserFundingState::Settled {
        return Err(SettleHour2Error::FundingAlreadySettled);
    }
    if settled_at < user_funding.created_at() || settled_at < position.updated_at() {
        return Err(SettleHour2Error::InvalidSettlementTime);
    }
    if user_funding.party_id() != position.user_id().0 {
        return Err(SettleHour2Error::FundingPositionPartyMismatch);
    }
    if user_funding.coin().base != position.coin() {
        return Err(SettleHour2Error::FundingPositionMarketMismatch);
    }
    if position.margin_mode() != MarginModeV3::Cross {
        return Err(SettleHour2Error::CrossMarginRequired);
    }
    if !balance.belongs_to_account(account_id) {
        return Err(SettleHour2Error::BalanceAccountMismatch);
    }
    if balance.party_id() != user_funding.party_id() {
        return Err(SettleHour2Error::BalancePartyMismatch);
    }
    let quote_coin_idx = u64::from(user_funding.coin().quote.index());
    if !balance.is_coin(quote_coin_idx) {
        return Err(SettleHour2Error::BalanceQuoteCoinMismatch);
    }
    if user_funding.usdc().is_zero() {
        return Err(SettleHour2Error::ZeroFundingAmount);
    }

    let mut user_funding_after = user_funding.clone();
    let (_, funding_output) = user_funding_after
        .apply(UserFundingEvent::Settle { settled_at })
        .map_err(map_user_funding_error)?;
    let UserFundingOutput::BalanceImpact { signed_usdc_delta, .. } = funding_output;

    let mut position_after = position.clone();
    position_after
        .apply(PerpPositionV3Event::FundingAmountSettled {
            user_id: position.user_id(),
            coin: position.coin(),
            signed_amount: signed_usdc_delta,
            logical_ts: settled_at,
        })
        .map_err(map_position_error)?;

    let mut balance_after = balance.clone();
    let balance_event = if signed_usdc_delta.is_sign_positive() {
        BalanceV2Event::CreditAvailable { amount: signed_usdc_delta, timestamp: settled_at }
    } else {
        let amount = Decimal::ZERO
            .checked_sub(signed_usdc_delta)
            .ok_or(SettleHour2Error::BalanceArithmeticOverflow)?;
        BalanceV2Event::DebitAvailable { amount, timestamp: settled_at }
    };
    balance_after.apply(balance_event).map_err(map_balance_error)?;

    Ok(SettleHour2Output {
        user_funding: user_funding_after,
        position: position_after,
        balance: balance_after,
    })
}

fn map_user_funding_error(error: UserFundingError) -> SettleHour2Error {
    match error {
        UserFundingError::AlreadySettled => SettleHour2Error::FundingAlreadySettled,
        UserFundingError::InvalidSettlementTime => SettleHour2Error::InvalidSettlementTime,
        UserFundingError::VersionOverflow => SettleHour2Error::FundingVersionOverflow,
    }
}

fn map_position_error(error: PerpPositionV3Error) -> SettleHour2Error {
    match error {
        PerpPositionV3Error::FundingUserMismatch => SettleHour2Error::FundingPositionPartyMismatch,
        PerpPositionV3Error::FundingCoinMismatch => SettleHour2Error::FundingPositionMarketMismatch,
        PerpPositionV3Error::InvalidFundingSettlementTime => {
            SettleHour2Error::InvalidSettlementTime
        }
        _ => SettleHour2Error::PositionArithmeticOverflow,
    }
}

fn map_balance_error(error: BalanceV2Error) -> SettleHour2Error {
    match error {
        BalanceV2Error::InsufficientAvailableBalance => {
            SettleHour2Error::InsufficientAvailableBalance
        }
        BalanceV2Error::BalanceClosed => SettleHour2Error::BalanceClosed,
        _ => SettleHour2Error::BalanceArithmeticOverflow,
    }
}

#[cfg(test)]
mod settle_hour2_tests {
    use common_entity::Fsm;
    use rust_decimal::Decimal;

    use super::*;
    use crate::account::coin::Coin;
    use crate::entity::common::UserId;
    use crate::FundingAccumulatorV3;

    const PARTY_ID: u64 = 7;
    const ACCOUNT_ID: u64 = 11;
    const CREATED_AT: u64 = 100;
    const SETTLED_AT: u64 = 120;

    fn symbol() -> Symbol {
        Symbol::new(1, Coin::BTC, Coin::USDC)
    }

    fn funding(amount: Decimal) -> UserFunding {
        UserFunding::new(
            1,
            PARTY_ID,
            symbol(),
            amount,
            Decimal::ONE,
            FundingRate::new(Decimal::new(1, 4)).unwrap(),
            None,
            CREATED_AT,
        )
    }

    fn position(
        user_id: u64,
        coin: Coin,
        margin_mode: MarginModeV3,
        accumulator: FundingAccumulatorV3,
        version: u64,
    ) -> PerpPositionV3 {
        PerpPositionV3::new(
            2,
            UserId(user_id),
            coin,
            Decimal::ONE,
            Decimal::from(100),
            Decimal::from(5),
            margin_mode,
            (margin_mode == MarginModeV3::Isolated).then_some(Decimal::TEN),
            Decimal::ZERO,
            accumulator,
            Decimal::ZERO,
            version,
        )
        .unwrap()
    }

    fn balance(
        party_id: u64,
        account_id: u64,
        coin: Coin,
        available: Decimal,
        version: u64,
    ) -> BalanceV2 {
        BalanceV2::new(
            party_id,
            account_id,
            u64::from(coin.index()),
            available,
            Decimal::ZERO,
            version,
            CREATED_AT,
        )
    }

    fn valid_position() -> PerpPositionV3 {
        position(PARTY_ID, Coin::BTC, MarginModeV3::Cross, FundingAccumulatorV3::default(), 3)
    }

    fn valid_balance() -> BalanceV2 {
        balance(PARTY_ID, ACCOUNT_ID, Coin::USDC, Decimal::TEN, 4)
    }

    #[test]
    fn positive_funding_returns_three_after_snapshots_without_mutating_inputs() {
        let funding = funding(Decimal::from(3));
        let position = valid_position();
        let balance = valid_balance();
        let before = (funding.clone(), position.clone(), balance.clone());

        let output = settle_hour2(&funding, &position, &balance, ACCOUNT_ID, SETTLED_AT).unwrap();

        assert_eq!((funding, position, balance), before);
        assert_eq!(output.user_funding.state(), UserFundingState::Settled);
        assert_eq!(output.user_funding.version(), 2);
        assert_eq!(output.user_funding.updated_at(), SETTLED_AT);
        assert_eq!(output.position.cum_funding().all_time, Decimal::from(3));
        assert_eq!(output.position.cum_funding().since_open, Decimal::from(3));
        assert_eq!(output.position.cum_funding().since_change, Decimal::from(3));
        assert_eq!(output.position.version(), 4);
        assert_eq!(output.balance.available(), Decimal::from(13));
        assert_eq!(output.balance.version(), 5);
    }

    #[test]
    fn negative_funding_debits_absolute_amount_and_accumulates_signed_amount() {
        let output = settle_hour2(
            &funding(Decimal::from(-3)),
            &valid_position(),
            &valid_balance(),
            ACCOUNT_ID,
            SETTLED_AT,
        )
        .unwrap();

        assert_eq!(output.position.cum_funding().all_time, Decimal::from(-3));
        assert_eq!(output.balance.available(), Decimal::from(7));
    }

    #[test]
    fn rejects_funding_state_amount_time_and_margin_mode() {
        let settled = UserFunding::restore_settled(
            1,
            PARTY_ID,
            symbol(),
            Decimal::ONE,
            Decimal::ONE,
            FundingRate::new(Decimal::new(1, 4)).unwrap(),
            None,
            CREATED_AT,
            SETTLED_AT,
            2,
        );
        let isolated = position(
            PARTY_ID,
            Coin::BTC,
            MarginModeV3::Isolated,
            FundingAccumulatorV3::default(),
            3,
        );
        let cases = [
            (
                settle_hour2(&settled, &valid_position(), &valid_balance(), ACCOUNT_ID, SETTLED_AT),
                SettleHour2Error::FundingAlreadySettled,
            ),
            (
                settle_hour2(
                    &funding(Decimal::ZERO),
                    &valid_position(),
                    &valid_balance(),
                    ACCOUNT_ID,
                    SETTLED_AT,
                ),
                SettleHour2Error::ZeroFundingAmount,
            ),
            (
                settle_hour2(
                    &funding(Decimal::ONE),
                    &valid_position(),
                    &valid_balance(),
                    ACCOUNT_ID,
                    CREATED_AT - 1,
                ),
                SettleHour2Error::InvalidSettlementTime,
            ),
            (
                settle_hour2(
                    &funding(Decimal::ONE),
                    &isolated,
                    &valid_balance(),
                    ACCOUNT_ID,
                    SETTLED_AT,
                ),
                SettleHour2Error::CrossMarginRequired,
            ),
        ];
        for (actual, expected) in cases {
            assert_eq!(actual, Err(expected));
        }
    }

    #[test]
    fn rejects_every_cross_entity_identity_mismatch() {
        let other_party_position = position(
            PARTY_ID + 1,
            Coin::BTC,
            MarginModeV3::Cross,
            FundingAccumulatorV3::default(),
            3,
        );
        let other_market_position =
            position(PARTY_ID, Coin::ETH, MarginModeV3::Cross, FundingAccumulatorV3::default(), 3);
        let funding = funding(Decimal::ONE);
        let cases = [
            (
                settle_hour2(
                    &funding,
                    &other_party_position,
                    &valid_balance(),
                    ACCOUNT_ID,
                    SETTLED_AT,
                ),
                SettleHour2Error::FundingPositionPartyMismatch,
            ),
            (
                settle_hour2(
                    &funding,
                    &other_market_position,
                    &valid_balance(),
                    ACCOUNT_ID,
                    SETTLED_AT,
                ),
                SettleHour2Error::FundingPositionMarketMismatch,
            ),
            (
                settle_hour2(
                    &funding,
                    &valid_position(),
                    &valid_balance(),
                    ACCOUNT_ID + 1,
                    SETTLED_AT,
                ),
                SettleHour2Error::BalanceAccountMismatch,
            ),
            (
                settle_hour2(
                    &funding,
                    &valid_position(),
                    &balance(PARTY_ID + 1, ACCOUNT_ID, Coin::USDC, Decimal::TEN, 4),
                    ACCOUNT_ID,
                    SETTLED_AT,
                ),
                SettleHour2Error::BalancePartyMismatch,
            ),
            (
                settle_hour2(
                    &funding,
                    &valid_position(),
                    &balance(PARTY_ID, ACCOUNT_ID, Coin::BTC, Decimal::TEN, 4),
                    ACCOUNT_ID,
                    SETTLED_AT,
                ),
                SettleHour2Error::BalanceQuoteCoinMismatch,
            ),
        ];
        for (actual, expected) in cases {
            assert_eq!(actual, Err(expected));
        }
    }

    #[test]
    fn rejects_insufficient_or_closed_balance_without_partial_result() {
        let funding_input = funding(Decimal::from(-11));
        let position = valid_position();
        let balance_input = valid_balance();
        assert_eq!(
            settle_hour2(&funding_input, &position, &balance_input, ACCOUNT_ID, SETTLED_AT,),
            Err(SettleHour2Error::InsufficientAvailableBalance)
        );
        assert_eq!(funding_input.state(), UserFundingState::Pending);
        assert_eq!(position.cum_funding(), FundingAccumulatorV3::default());
        assert_eq!(balance_input.available(), Decimal::TEN);

        let mut closed = balance(PARTY_ID, ACCOUNT_ID, Coin::USDC, Decimal::ZERO, 4);
        closed.apply(BalanceV2Event::Close { timestamp: 110 }).unwrap();
        assert_eq!(
            settle_hour2(
                &funding(Decimal::ONE),
                &valid_position(),
                &closed,
                ACCOUNT_ID,
                SETTLED_AT,
            ),
            Err(SettleHour2Error::BalanceClosed)
        );
    }

    #[test]
    fn maps_funding_position_and_balance_overflow() {
        let mut funding_value = serde_json::to_value(funding(Decimal::ONE)).unwrap();
        funding_value["version"] = serde_json::json!(u64::MAX);
        let funding_version_max: UserFunding = serde_json::from_value(funding_value).unwrap();
        assert_eq!(
            settle_hour2(
                &funding_version_max,
                &valid_position(),
                &valid_balance(),
                ACCOUNT_ID,
                SETTLED_AT,
            ),
            Err(SettleHour2Error::FundingVersionOverflow)
        );

        let accumulator_max = FundingAccumulatorV3 {
            all_time: Decimal::MAX,
            since_open: Decimal::ZERO,
            since_change: Decimal::ZERO,
        };
        assert_eq!(
            settle_hour2(
                &funding(Decimal::ONE),
                &position(PARTY_ID, Coin::BTC, MarginModeV3::Cross, accumulator_max, 3),
                &valid_balance(),
                ACCOUNT_ID,
                SETTLED_AT,
            ),
            Err(SettleHour2Error::PositionArithmeticOverflow)
        );
        assert_eq!(
            settle_hour2(
                &funding(Decimal::ONE),
                &valid_position(),
                &balance(PARTY_ID, ACCOUNT_ID, Coin::USDC, Decimal::MAX, 4),
                ACCOUNT_ID,
                SETTLED_AT,
            ),
            Err(SettleHour2Error::BalanceArithmeticOverflow)
        );
        assert_eq!(
            settle_hour2(
                &funding(Decimal::ONE),
                &position(
                    PARTY_ID,
                    Coin::BTC,
                    MarginModeV3::Cross,
                    FundingAccumulatorV3::default(),
                    u64::MAX,
                ),
                &valid_balance(),
                ACCOUNT_ID,
                SETTLED_AT,
            ),
            Err(SettleHour2Error::PositionArithmeticOverflow)
        );
        assert_eq!(
            settle_hour2(
                &funding(Decimal::ONE),
                &valid_position(),
                &balance(PARTY_ID, ACCOUNT_ID, Coin::USDC, Decimal::TEN, u64::MAX),
                ACCOUNT_ID,
                SETTLED_AT,
            ),
            Err(SettleHour2Error::BalanceArithmeticOverflow)
        );
    }
}
