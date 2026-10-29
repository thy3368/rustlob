use super::funding_payment::Rate;
use super::funding_rate_tick::FundingRateTick;
use super::price::Price;
use crate::account::symbol::Symbol;

/// 根据一个小时内的 premium 样本构造市场级资金费率观测。
///
/// 行情采集、样本窗口管理和事实持久化由调用方负责；本函数只执行确定性计算。
pub fn make_funding_rate_tick(
    coin: Symbol,
    time_ms: u64,
    samples: &[Rate],
    _oracle_px: Price,
) -> Option<FundingRateTick> {
    let sample_count = i128::try_from(samples.len()).ok()?;
    if sample_count == 0 {
        return None;
    }

    let sum = samples.iter().try_fold(0_i128, |sum, sample| sum.checked_add(sample.raw()))?;
    let avg_premium = Rate::new(sum.checked_div(sample_count)?);
    let fixed_rate = Rate::parse_str("0.0001").ok()?.raw().checked_div(8)?;
    let correction = (fixed_rate.checked_sub(avg_premium.raw())?)
        .clamp(Rate::from_bps(-5).raw(), Rate::from_bps(5).raw());
    let eight_hour_rate = avg_premium.raw().checked_add(correction)?;
    let hourly_rate = eight_hour_rate
        .checked_div(8)?
        .clamp(Rate::from_percent(-4).raw(), Rate::from_percent(4).raw());

    Some(FundingRateTick {
        coin,
        time_ms,
        funding_rate: Rate::new(hourly_rate),
        premium: avg_premium,
    })
}
