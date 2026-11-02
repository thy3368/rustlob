use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::account::coin::Coin;
use crate::entity::common::{FundingRate, UserId};
use crate::entity::hyperliquid_account::usdc::Usdc;
use crate::entity::perp::fund::funding_payment::FundingPayment;
use crate::entity::perp::fund::price::Price;
use crate::entity::perp::trade::perp_position_v2::PerpPositionV3;

pub type Size = Decimal;
pub type TxHash = [u8; 32];

pub fn settle_hour(
    coin: Coin,
    time_ms: u64,
    rate: FundingRate,
    oracle: Price,
    batch_hash: TxHash,
    seq_start: u64,
    positions: &[(UserId, PerpPositionV3)],
) -> Vec<FundingPayment> {
    let mut next_seq = seq_start;
    positions
        .iter()
        .filter_map(|(user, position)| {
            let szi = position.signed_size();
            if szi.is_zero() {
                return None;
            }
            let payment = make_funding_payment(
                *user, coin, time_ms, next_seq, batch_hash, rate, oracle, szi,
            )?;
            next_seq = next_seq.checked_add(1)?;
            Some(payment)
        })
        .collect()
}

fn make_funding_payment(
    user: UserId,
    coin: Coin,
    time_ms: u64,
    seq: u64,
    hash: TxHash,
    rate: FundingRate,
    oracle: Price,
    szi: Size,
) -> Option<FundingPayment> {
    let usdc = Decimal::ZERO.checked_sub(szi.checked_mul(oracle)?.checked_mul(rate.value())?)?;
    let raw = usdc.checked_mul(Decimal::from(1_000_000))?.trunc().to_i128()?;
    Some(FundingPayment {
        user,
        time_ms,
        seq,
        hash,
        coin,
        funding_rate: rate,
        szi,
        usdc: Usdc::new(raw),
        n_samples: None,
    })
}
