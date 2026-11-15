use std::collections::HashSet;

use common_entity::stable_string_entity_id;
use rust_decimal::Decimal;
use thiserror::Error;

use crate::account::symbol::Symbol;
use crate::entity::common::price::Price;
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

// 将资金结算事实生效,结算到哪里？结算到余额？
pub fn settle_hour2(
    user_funding: UserFunding,
    position: PerpPositionV3,
    balance: BalanceV2,
) -> Result<Vec<UserFunding>, SettleHourError> {
}
