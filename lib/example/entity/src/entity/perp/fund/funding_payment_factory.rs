use base_types::{AssetId, UserId};
use rust_decimal::Decimal;

use crate::entity::hyperliquid_account::usdc::Usdc;
use crate::entity::perp::fund::funding_payment::{FundingPayment, Rate};
use crate::entity::perp::fund::price::Price;
use crate::entity::perp::trade::perp_position_v2::PerpPositionV3;

pub type Size = Decimal;
pub type TxHash = [u8; 32];

pub fn settle_hour(
    coin: AssetId,
    time_ms: u64,
    rate: Rate,
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
    coin: AssetId,
    time_ms: u64,
    seq: u64,
    hash: TxHash,
    rate: Rate,
    oracle: Price,
    szi: Size,
) -> Option<FundingPayment> {
    let raw = szi
        .mantissa()
        .checked_mul(oracle.mantissa())?
        .checked_mul(rate.raw())?
        .checked_neg()?
        .checked_div(10_i128.pow(18))?;
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
