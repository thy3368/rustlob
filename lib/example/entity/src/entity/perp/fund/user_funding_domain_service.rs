use std::collections::HashSet;

use common_entity::stable_string_entity_id;
use rust_decimal::Decimal;
use thiserror::Error;

use crate::account::symbol::Symbol;
use crate::perp::fund::price::Price;
use crate::perp::fund::user_funding::UserFunding;
use crate::{BalanceV2, FundingRate, PerpPositionV3};

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


// 将资金结算事实生效,结算到哪里？
pub fn settle_hour2(
    user_funding: UserFunding,
    position: PerpPositionV3,
    balance: BalanceV2,

) -> Result<Vec<UserFunding>, SettleHourError> {




}
#[cfg(test)]
mod tests {
    use base_types_v2::Coin;
    use rust_decimal::Decimal;

    use super::*;
    use crate::entity::common::UserId;
    use crate::perp::fund::user_funding::UserFundingState;
    use crate::{FundingAccumulatorV3, MarginModeV3};

    fn symbol() -> Symbol {
        Symbol::new(7, Coin::BTC, Coin::USDC)
    }

    fn rate(value: i64) -> FundingRate {
        FundingRate::new(Decimal::new(value, 4)).unwrap()
    }

    fn position(position_id: u64, user_id: u64, coin: Coin, size: i64) -> PerpPositionV3 {
        if size == 0 {
            return PerpPositionV3::empty(position_id, UserId(user_id), coin);
        }
        PerpPositionV3::new(
            position_id,
            UserId(user_id),
            coin,
            Decimal::from(size),
            Decimal::ONE,
            Decimal::ONE,
            MarginModeV3::Cross,
            None,
            Decimal::ZERO,
            FundingAccumulatorV3::default(),
            Decimal::ZERO,
            3,
        )
        .unwrap()
    }

    #[test]
    fn positive_rate_charges_longs_and_credits_shorts() {
        let positions = [position(11, 101, Coin::BTC, 2), position(12, 102, Coin::BTC, -3)];
        let facts =
            settle_hour(symbol(), 9, 1_700, rate(1), Decimal::from(100), &positions).unwrap();

        assert_eq!(facts.len(), 2);
        assert_eq!(facts[0].party_id(), 101);
        assert_eq!(facts[0].coin(), symbol());
        assert_eq!(facts[0].signed_size(), Decimal::from(2));
        assert_eq!(facts[0].usdc(), Decimal::new(-2, 2));
        assert_eq!(facts[0].funding_rate(), Decimal::new(1, 4));
        assert_eq!(facts[0].created_at(), 1_700);
        assert_eq!(facts[0].updated_at(), 1_700);
        assert_eq!(facts[0].state(), UserFundingState::Pending);
        assert_eq!(facts[0].version(), 1);
        assert_eq!(facts[1].party_id(), 102);
        assert_eq!(facts[1].signed_size(), Decimal::from(-3));
        assert_eq!(facts[1].usdc(), Decimal::new(3, 2));
    }

    #[test]
    fn flat_positions_and_zero_rate_do_not_generate_facts() {
        let flat = [position(11, 101, Coin::BTC, 0)];
        let open = [position(12, 102, Coin::BTC, 2)];
        assert!(settle_hour(symbol(), 9, 1_700, rate(1), Decimal::ONE, &flat).unwrap().is_empty());
        assert!(
            settle_hour(symbol(), 9, 1_700, FundingRate::zero(), Decimal::ONE, &open)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn funding_identity_is_stable_across_retries_and_reordering() {
        let first = position(11, 101, Coin::BTC, 2);
        let second = position(12, 102, Coin::BTC, -3);
        let original = settle_hour(
            symbol(),
            9,
            1_700,
            rate(1),
            Decimal::ONE,
            &[first.clone(), second.clone()],
        )
        .unwrap();
        let reordered =
            settle_hour(symbol(), 9, 1_700, rate(1), Decimal::ONE, &[second, first]).unwrap();
        let next_batch = settle_hour(
            symbol(),
            10,
            1_700,
            rate(1),
            Decimal::ONE,
            &[position(11, 101, Coin::BTC, 2)],
        )
        .unwrap();

        assert_eq!(original[0].funding_id(), reordered[1].funding_id());
        assert_eq!(original[1].funding_id(), reordered[0].funding_id());
        assert_ne!(original[0].funding_id(), next_batch[0].funding_id());
    }

    #[test]
    fn rejects_invalid_batch_or_oracle() {
        let positions = [position(11, 101, Coin::BTC, 2)];
        assert_eq!(
            settle_hour(symbol(), 0, 1_700, rate(1), Decimal::ONE, &positions),
            Err(SettleHourError::InvalidFundingBatchId)
        );
        for oracle in [Decimal::ZERO, Decimal::NEGATIVE_ONE] {
            assert_eq!(
                settle_hour(symbol(), 9, 1_700, rate(1), oracle, &positions),
                Err(SettleHourError::NonPositiveOracle)
            );
        }
    }

    #[test]
    fn rejects_duplicate_positions_and_market_mismatches() {
        let duplicate = [position(11, 101, Coin::BTC, 2), position(11, 102, Coin::BTC, -3)];
        assert_eq!(
            settle_hour(symbol(), 9, 1_700, rate(1), Decimal::ONE, &duplicate),
            Err(SettleHourError::DuplicatePositionId { position_id: 11 })
        );

        let wrong_market = [position(12, 101, Coin::ETH, 2)];
        assert_eq!(
            settle_hour(symbol(), 9, 1_700, rate(1), Decimal::ONE, &wrong_market),
            Err(SettleHourError::PositionMarketMismatch { position_id: 12 })
        );
    }

    #[test]
    fn arithmetic_overflow_fails_the_whole_batch() {
        let valid = position(11, 101, Coin::BTC, 2);
        let overflowing = PerpPositionV3::new(
            12,
            UserId(102),
            Coin::BTC,
            Decimal::MAX,
            Decimal::ONE,
            Decimal::ONE,
            MarginModeV3::Cross,
            None,
            Decimal::ZERO,
            FundingAccumulatorV3::default(),
            Decimal::ZERO,
            1,
        )
        .unwrap();

        assert_eq!(
            settle_hour(
                symbol(),
                9,
                1_700,
                FundingRate::new(Decimal::ONE).unwrap(),
                Decimal::from(2),
                &[valid, overflowing]
            ),
            Err(SettleHourError::ArithmeticOverflow { position_id: 12 })
        );
    }
}
