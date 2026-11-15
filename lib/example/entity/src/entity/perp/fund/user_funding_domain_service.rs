use std::collections::{HashMap, HashSet};

use common_entity::{Fsm, UpdatedEntityPair, stable_string_entity_id};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::account::symbol::Symbol;
use crate::entity::common::price::Price;
use crate::perp::fund::user_funding::{
    UserFunding, UserFundingError, UserFundingEvent, UserFundingState,
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
    lsn: u64,
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
                lsn,
            )
        })
        .collect())
}

/// 一批资金费结算形成的唯一 before/after 业务真相。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleUserFundingBatchOutput {
    pub changed_fundings: Vec<UpdatedEntityPair<UserFunding>>,
    pub changed_positions: Vec<UpdatedEntityPair<PerpPositionV3>>,
    pub changed_balances: Vec<UpdatedEntityPair<BalanceV2>>,
}

/// 批量资金费结算失败的明确领域原因。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SettleUserFundingBatchError {
    #[error("duplicate funding id: {funding_id}")]
    DuplicateFunding { funding_id: u64 },
    #[error("user funding {funding_id} is already settled")]
    FundingAlreadySettled { funding_id: u64 },
    #[error("user funding {funding_id} has a zero amount")]
    ZeroFundingAmount { funding_id: u64 },
    #[error("funding settlement time is invalid for funding {funding_id}")]
    InvalidSettlementTime { funding_id: u64 },
    #[error("user funding version overflow for funding {funding_id}")]
    FundingVersionOverflow { funding_id: u64 },
    #[error("position is missing for funding {funding_id}")]
    PositionNotFound { funding_id: u64 },
    #[error("more than one position matches funding {funding_id}")]
    DuplicatePosition { funding_id: u64 },
    #[error("funding settlement only supports cross-margin positions")]
    CrossMarginRequired { position_id: u64 },
    #[error("balance is missing for funding {funding_id}")]
    BalanceNotFound { funding_id: u64 },
    #[error("more than one balance matches funding {funding_id}")]
    DuplicateBalance { funding_id: u64 },
    #[error("funding delta arithmetic overflow")]
    FundingDeltaOverflow,
    #[error("available balance is insufficient for funding debit")]
    InsufficientAvailableBalance { account_id: u64 },
    #[error("balance is closed")]
    BalanceClosed { account_id: u64 },
    #[error("position funding accumulator or version arithmetic failed")]
    PositionArithmeticOverflow { position_id: u64 },
    #[error("balance amount or version arithmetic overflow")]
    BalanceArithmeticOverflow { account_id: u64 },
    #[error("funding {funding_id} has an lsn newer than the command")]
    StaleFundingLsn { funding_id: u64 },
    #[error("position {position_id} has an lsn newer than the command")]
    StalePositionLsn { position_id: u64 },
    #[error("balance account {account_id} has an lsn newer than the command")]
    StaleBalanceLsn { account_id: u64 },
}

/// 原子计算整批资金费、仓位和余额的 before/after truth。
///
/// funding pair 保持输入顺序，仓位与余额 pair 保持各自 state slice 的稳定顺序。同一仓位或余额
/// 先聚合 signed delta，再只推进一次 FSM；净变化为零时不产生该实体的空 pair。
pub fn settle_user_funding_batch(
    fundings: &[UserFunding],
    positions: &[PerpPositionV3],
    balances: &[BalanceV2],
    settled_at: u64,
    lsn: u64,
) -> Result<SettleUserFundingBatchOutput, SettleUserFundingBatchError> {
    let mut funding_ids = HashSet::with_capacity(fundings.len());
    let mut position_by_business_key: HashMap<(u64, u32), usize> =
        HashMap::with_capacity(positions.len());
    let mut duplicate_position_keys = HashSet::new();
    let mut balance_by_business_key: HashMap<(u64, u64), usize> =
        HashMap::with_capacity(balances.len());
    let mut duplicate_balance_keys = HashSet::new();

    for (index, position) in positions.iter().enumerate() {
        let key = (position.user_id().0, position.coin().index());
        if position_by_business_key.insert(key, index).is_some() {
            duplicate_position_keys.insert(key);
        }
    }
    for (index, balance) in balances.iter().enumerate() {
        let key = (balance.party_id(), balance.coin_idx());
        if balance_by_business_key.insert(key, index).is_some() {
            duplicate_balance_keys.insert(key);
        }
    }

    let mut position_deltas = vec![Decimal::ZERO; positions.len()];
    let mut balance_deltas = vec![Decimal::ZERO; balances.len()];
    for funding in fundings {
        let funding_id = funding.funding_id();
        if !funding_ids.insert(funding_id) {
            return Err(SettleUserFundingBatchError::DuplicateFunding { funding_id });
        }
        if funding.state() == UserFundingState::Settled {
            return Err(SettleUserFundingBatchError::FundingAlreadySettled { funding_id });
        }
        if funding.usdc().is_zero() {
            return Err(SettleUserFundingBatchError::ZeroFundingAmount { funding_id });
        }
        if settled_at < funding.created_at() {
            return Err(SettleUserFundingBatchError::InvalidSettlementTime { funding_id });
        }

        let position_key = (funding.party_id(), funding.coin().base.index());
        if duplicate_position_keys.contains(&position_key) {
            return Err(SettleUserFundingBatchError::DuplicatePosition { funding_id });
        }
        let position_index = position_by_business_key
            .get(&position_key)
            .copied()
            .ok_or(SettleUserFundingBatchError::PositionNotFound { funding_id })?;
        let position = &positions[position_index];
        if position.margin_mode() != MarginModeV3::Cross {
            return Err(SettleUserFundingBatchError::CrossMarginRequired {
                position_id: position.position_id(),
            });
        }
        if settled_at < position.updated_at() {
            return Err(SettleUserFundingBatchError::InvalidSettlementTime { funding_id });
        }

        let quote_coin_idx = u64::from(funding.coin().quote.index());
        let balance_key = (funding.party_id(), quote_coin_idx);
        if duplicate_balance_keys.contains(&balance_key) {
            return Err(SettleUserFundingBatchError::DuplicateBalance { funding_id });
        }
        let balance_index = balance_by_business_key
            .get(&balance_key)
            .copied()
            .ok_or(SettleUserFundingBatchError::BalanceNotFound { funding_id })?;

        position_deltas[position_index] = position_deltas[position_index]
            .checked_add(funding.usdc())
            .ok_or(SettleUserFundingBatchError::FundingDeltaOverflow)?;
        balance_deltas[balance_index] = balance_deltas[balance_index]
            .checked_add(funding.usdc())
            .ok_or(SettleUserFundingBatchError::FundingDeltaOverflow)?;
    }

    let mut changed_fundings = Vec::with_capacity(fundings.len());
    for funding in fundings {
        let before = funding.clone();
        let mut after = before.clone();
        after
            .apply(UserFundingEvent::Settle { settled_at, lsn })
            .map_err(|error| map_user_funding_error(error, funding.funding_id()))?;
        changed_fundings.push(UpdatedEntityPair::new(before, after));
    }

    let mut changed_positions = Vec::with_capacity(fundings.len().min(positions.len()));
    for (index, (before, delta)) in positions.iter().zip(position_deltas).enumerate() {
        if delta.is_zero() {
            continue;
        }
        let mut after = before.clone();
        after
            .apply(PerpPositionV3Event::FundingAmountSettled {
                user_id: before.user_id(),
                coin: before.coin(),
                signed_amount: delta,
                logical_ts: settled_at,
                lsn,
            })
            .map_err(|error| map_position_error(error, positions[index].position_id()))?;
        changed_positions.push(UpdatedEntityPair::new(before.clone(), after));
    }

    let mut changed_balances = Vec::with_capacity(balances.len());
    for (before, delta) in balances.iter().zip(balance_deltas) {
        if delta.is_zero() {
            continue;
        }
        let mut after = before.clone();
        let event = if delta.is_sign_positive() {
            BalanceV2Event::CreditAvailable { amount: delta, timestamp: settled_at, lsn }
        } else {
            let amount = Decimal::ZERO.checked_sub(delta).ok_or(
                SettleUserFundingBatchError::BalanceArithmeticOverflow {
                    account_id: before.account_id(),
                },
            )?;
            BalanceV2Event::DebitAvailable { amount, timestamp: settled_at, lsn }
        };
        after.apply(event).map_err(|error| map_balance_error(error, before.account_id()))?;
        changed_balances.push(UpdatedEntityPair::new(before.clone(), after));
    }

    Ok(SettleUserFundingBatchOutput { changed_fundings, changed_positions, changed_balances })
}

fn map_user_funding_error(error: UserFundingError, funding_id: u64) -> SettleUserFundingBatchError {
    match error {
        UserFundingError::AlreadySettled => {
            SettleUserFundingBatchError::FundingAlreadySettled { funding_id }
        }
        UserFundingError::InvalidSettlementTime => {
            SettleUserFundingBatchError::InvalidSettlementTime { funding_id }
        }
        UserFundingError::VersionOverflow => {
            SettleUserFundingBatchError::FundingVersionOverflow { funding_id }
        }
        UserFundingError::StaleLsn { .. } => {
            SettleUserFundingBatchError::StaleFundingLsn { funding_id }
        }
    }
}

fn map_position_error(error: PerpPositionV3Error, position_id: u64) -> SettleUserFundingBatchError {
    match error {
        PerpPositionV3Error::InvalidFundingSettlementTime => {
            SettleUserFundingBatchError::InvalidSettlementTime { funding_id: position_id }
        }
        PerpPositionV3Error::InvalidQuantity
        | PerpPositionV3Error::InvalidPrice
        | PerpPositionV3Error::InvalidLeverage
        | PerpPositionV3Error::InvalidMarginConfiguration
        | PerpPositionV3Error::InvalidState
        | PerpPositionV3Error::ArithmeticOverflow
        | PerpPositionV3Error::InvalidIsolatedMargin
        | PerpPositionV3Error::FundingUserMismatch
        | PerpPositionV3Error::FundingCoinMismatch => {
            SettleUserFundingBatchError::PositionArithmeticOverflow { position_id }
        }
        PerpPositionV3Error::StaleLsn { .. } => {
            SettleUserFundingBatchError::StalePositionLsn { position_id }
        }
    }
}

fn map_balance_error(error: BalanceV2Error, account_id: u64) -> SettleUserFundingBatchError {
    match error {
        BalanceV2Error::InsufficientAvailableBalance => {
            SettleUserFundingBatchError::InsufficientAvailableBalance { account_id }
        }
        BalanceV2Error::BalanceClosed => SettleUserFundingBatchError::BalanceClosed { account_id },
        BalanceV2Error::InvalidAmount
        | BalanceV2Error::InsufficientFrozenBalance
        | BalanceV2Error::ArithmeticOverflow
        | BalanceV2Error::CannotCloseNonZeroBalance => {
            SettleUserFundingBatchError::BalanceArithmeticOverflow { account_id }
        }
        BalanceV2Error::StaleLsn { .. } => {
            SettleUserFundingBatchError::StaleBalanceLsn { account_id }
        }
    }
}

#[cfg(test)]
mod settle_user_funding_batch_tests {
    use common_entity::Fsm;

    use super::*;
    use crate::FundingAccumulatorV3;
    use crate::account::coin::Coin;
    use crate::entity::common::UserId;

    const PARTY_ID: u64 = 7;
    const ACCOUNT_ID: u64 = 11;
    const CREATED_AT: u64 = 100;
    const SETTLED_AT: u64 = 120;
    const COMMAND_LSN: u64 = 200;

    fn symbol() -> Symbol {
        Symbol::new(1, Coin::BTC, Coin::USDC)
    }

    fn funding(id: u64, amount: Decimal) -> UserFunding {
        UserFunding::new(
            id,
            PARTY_ID,
            symbol(),
            amount,
            Decimal::ONE,
            FundingRate::new(Decimal::new(1, 4)).unwrap(),
            None,
            CREATED_AT,
            1,
        )
    }

    fn position(
        margin_mode: MarginModeV3,
        accumulator: FundingAccumulatorV3,
        version: u64,
    ) -> PerpPositionV3 {
        PerpPositionV3::new(
            2,
            UserId(PARTY_ID),
            Coin::BTC,
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

    fn valid_position() -> PerpPositionV3 {
        position(MarginModeV3::Cross, FundingAccumulatorV3::default(), 3)
    }

    fn balance(available: Decimal, version: u64) -> BalanceV2 {
        BalanceV2::new(
            PARTY_ID,
            ACCOUNT_ID,
            u64::from(Coin::USDC.index()),
            available,
            Decimal::ZERO,
            version,
            CREATED_AT,
        )
    }

    #[test]
    fn aggregates_shared_entities_once_and_preserves_input_order() {
        let fundings = vec![funding(20, Decimal::from(3)), funding(10, Decimal::from(-1))];
        let positions = vec![valid_position()];
        let balances = vec![balance(Decimal::TEN, 4)];
        let before = (fundings.clone(), positions.clone(), balances.clone());

        let output =
            settle_user_funding_batch(&fundings, &positions, &balances, SETTLED_AT, COMMAND_LSN)
                .unwrap();

        assert_eq!((fundings, positions, balances), before);
        assert_eq!(
            output.changed_fundings.iter().map(|pair| pair.before.funding_id()).collect::<Vec<_>>(),
            vec![20, 10]
        );
        assert!(
            output
                .changed_fundings
                .iter()
                .all(|pair| pair.after.state() == UserFundingState::Settled)
        );
        assert_eq!(output.changed_positions.len(), 1);
        assert_eq!(output.changed_positions[0].after.cum_funding().all_time, Decimal::from(2));
        assert_eq!(output.changed_positions[0].after.version(), COMMAND_LSN);
        assert_eq!(output.changed_balances.len(), 1);
        assert_eq!(output.changed_balances[0].after.available(), Decimal::from(12));
        assert_eq!(output.changed_balances[0].after.version(), COMMAND_LSN);
    }

    #[test]
    fn net_zero_only_settles_funding_facts() {
        let output = settle_user_funding_batch(
            &[funding(1, Decimal::ONE), funding(2, Decimal::NEGATIVE_ONE)],
            &[valid_position()],
            &[balance(Decimal::TEN, 4)],
            SETTLED_AT,
            COMMAND_LSN,
        )
        .unwrap();

        assert_eq!(output.changed_fundings.len(), 2);
        assert!(output.changed_positions.is_empty());
        assert!(output.changed_balances.is_empty());
    }

    #[test]
    fn rejects_duplicate_missing_and_invalid_business_state() {
        let duplicate = funding(1, Decimal::ONE);
        assert_eq!(
            settle_user_funding_batch(
                &[duplicate.clone(), duplicate],
                &[valid_position()],
                &[balance(Decimal::TEN, 4)],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::DuplicateFunding { funding_id: 1 })
        );
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ZERO)],
                &[valid_position()],
                &[balance(Decimal::TEN, 4)],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::ZeroFundingAmount { funding_id: 1 })
        );
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[],
                &[balance(Decimal::TEN, 4)],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::PositionNotFound { funding_id: 1 })
        );
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[valid_position()],
                &[],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::BalanceNotFound { funding_id: 1 })
        );
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[position(MarginModeV3::Isolated, FundingAccumulatorV3::default(), 3)],
                &[balance(Decimal::TEN, 4)],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::CrossMarginRequired { position_id: 2 })
        );
    }

    #[test]
    fn settlement_time_accepts_equal_boundaries_and_rejects_the_previous_instant() {
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[valid_position()],
                &[balance(Decimal::TEN, 4)],
                CREATED_AT - 1,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::InvalidSettlementTime { funding_id: 1 })
        );
        assert!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[valid_position()],
                &[balance(Decimal::TEN, 4)],
                CREATED_AT,
                COMMAND_LSN,
            )
            .is_ok()
        );

        let mut position_value = serde_json::to_value(valid_position()).unwrap();
        position_value["updated_at"] = serde_json::json!(SETTLED_AT);
        let position_updated_at_settlement: PerpPositionV3 =
            serde_json::from_value(position_value).unwrap();
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[position_updated_at_settlement.clone()],
                &[balance(Decimal::TEN, 4)],
                SETTLED_AT - 1,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::InvalidSettlementTime { funding_id: 1 })
        );
        assert!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[position_updated_at_settlement],
                &[balance(Decimal::TEN, 4)],
                SETTLED_AT,
                COMMAND_LSN,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_duplicate_balance_and_atomic_downstream_failures() {
        let duplicate_balance = balance(Decimal::TEN, 4);
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[valid_position()],
                &[duplicate_balance.clone(), duplicate_balance],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::DuplicateBalance { funding_id: 1 })
        );

        let fundings = vec![funding(1, Decimal::from(-11))];
        let positions = vec![valid_position()];
        let balances = vec![balance(Decimal::TEN, 4)];
        let before = (fundings.clone(), positions.clone(), balances.clone());
        assert_eq!(
            settle_user_funding_batch(&fundings, &positions, &balances, SETTLED_AT, COMMAND_LSN,),
            Err(SettleUserFundingBatchError::InsufficientAvailableBalance {
                account_id: ACCOUNT_ID
            })
        );
        assert_eq!((fundings, positions, balances), before);

        let mut closed = balance(Decimal::ZERO, 4);
        closed.apply(BalanceV2Event::Close { timestamp: 110, lsn: COMMAND_LSN }).unwrap();
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[valid_position()],
                &[closed],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::BalanceClosed { account_id: ACCOUNT_ID })
        );
    }

    #[test]
    fn maps_funding_position_and_balance_overflow() {
        let mut funding_value = serde_json::to_value(funding(1, Decimal::ONE)).unwrap();
        funding_value["version"] = serde_json::json!(u64::MAX);
        let funding_version_max: UserFunding = serde_json::from_value(funding_value).unwrap();
        assert_eq!(
            settle_user_funding_batch(
                &[funding_version_max],
                &[valid_position()],
                &[balance(Decimal::TEN, 4)],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::StaleFundingLsn { funding_id: 1 })
        );

        let accumulator = FundingAccumulatorV3 {
            all_time: Decimal::MAX,
            since_open: Decimal::ZERO,
            since_change: Decimal::ZERO,
        };
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[position(MarginModeV3::Cross, accumulator, 3)],
                &[balance(Decimal::TEN, 4)],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::PositionArithmeticOverflow { position_id: 2 })
        );
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[valid_position()],
                &[balance(Decimal::MAX, 4)],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::BalanceArithmeticOverflow { account_id: ACCOUNT_ID })
        );
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[position(MarginModeV3::Cross, FundingAccumulatorV3::default(), u64::MAX)],
                &[balance(Decimal::TEN, 4)],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::StalePositionLsn { position_id: 2 })
        );
        assert_eq!(
            settle_user_funding_batch(
                &[funding(1, Decimal::ONE)],
                &[valid_position()],
                &[balance(Decimal::TEN, u64::MAX)],
                SETTLED_AT,
                COMMAND_LSN,
            ),
            Err(SettleUserFundingBatchError::StaleBalanceLsn { account_id: ACCOUNT_ID })
        );
    }
}
