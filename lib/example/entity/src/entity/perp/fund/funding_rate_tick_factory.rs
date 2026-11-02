use rust_decimal::Decimal;

use super::funding_rate_tick::FundingRateTick;
use super::price::Price;
use crate::account::symbol::Symbol;
use crate::entity::common::FundingRate;

/// 根据一个小时内的 premium 样本构造市场级资金费率观测。
///
/// 行情采集、样本窗口管理和事实持久化由调用方负责；本函数只执行确定性计算。
pub fn make_funding_rate_tick(
    coin: Symbol,
    time_ms: u64,
    samples: &[FundingRate],
    _oracle_px: Price,
) -> Option<FundingRateTick> {
    let sample_count = i128::try_from(samples.len()).ok()?;
    if sample_count == 0 {
        return None;
    }

    let sum =
        samples.iter().try_fold(Decimal::ZERO, |sum, sample| sum.checked_add(sample.value()))?;
    let avg_premium = sum.checked_div(Decimal::from(sample_count))?;
    let fixed_rate = Decimal::new(1, 4).checked_div(Decimal::from(8u32))?;
    let correction =
        fixed_rate.checked_sub(avg_premium)?.max(Decimal::new(-5, 4)).min(Decimal::new(5, 4));
    let hourly_rate = avg_premium
        .checked_add(correction)?
        .checked_div(Decimal::from(8u32))?
        .max(Decimal::new(-4, 2))
        .min(Decimal::new(4, 2));

    Some(FundingRateTick {
        coin,
        time_ms,
        funding_rate: FundingRate::new(hourly_rate).ok()?,
        premium: FundingRate::new(avg_premium).ok()?,
    })
}
