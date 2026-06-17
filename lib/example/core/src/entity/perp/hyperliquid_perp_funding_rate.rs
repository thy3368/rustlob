const HYPERLIQUID_HOURLY_SAMPLE_COUNT: usize = 720;
const RATE_SCALE_E8: i128 = 100_000_000;
const INTEREST_E8_8H: i128 = 10_000;
const CLAMP_BOUND_E8: i128 = 50_000;

/// Hyperliquid perp 单个 5 秒资金费采样点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpFundingSample {
    /// 该采样点对应的 oracle 价格，用作 premium 分母与中间基准价。
    pub oracle_price: u64,
    /// 在买盘侧吃到协议规定 impact notional 后的平均卖出成交价。
    pub impact_bid_price: u64,
    /// 在卖盘侧吃到协议规定 impact notional 后的平均买入成交价。
    pub impact_ask_price: u64,
}

/// Hyperliquid perp 资金费 impact price 计算使用的单档深度。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpBookLevel {
    /// 该档位价格。
    pub price: u64,
    /// 该档位可成交数量。
    pub quantity: u64,
}

/// Hyperliquid perp 资金费率领域计算错误。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HyperliquidPerpFundingRateError {
    #[error("samples must not be empty")]
    EmptySamples,
    #[error("book levels must not be empty")]
    EmptyBookLevels,
    #[error("samples must contain exactly 720 points for one hour")]
    InvalidSampleCount,
    #[error("impact notional must be greater than zero")]
    InvalidImpactNotional,
    #[error("oracle price must be greater than zero")]
    InvalidOraclePrice,
    #[error("impact prices must be greater than zero")]
    InvalidImpactPrice,
    #[error("book level price must be greater than zero")]
    InvalidLevelPrice,
    #[error("book level quantity must be greater than zero")]
    InvalidLevelQuantity,
    #[error("impact bid price must not exceed impact ask price")]
    CrossedImpactPrices,
    #[error("bid levels must be sorted from high price to low price")]
    InvalidBidLevelOrder,
    #[error("ask levels must be sorted from low price to high price")]
    InvalidAskLevelOrder,
    #[error("book depth is insufficient for the requested impact notional")]
    InsufficientBookDepth,
    #[error("arithmetic overflow while computing funding rate")]
    ArithmeticOverflow,
}

/// 在买盘侧按固定 `impact_notional` 卖出，返回平均成交价。
pub fn compute_impact_bid_price(
    bid_levels: &[HyperliquidPerpBookLevel],
    impact_notional: u64,
) -> Result<u64, HyperliquidPerpFundingRateError> {
    validate_book_levels(
        bid_levels,
        impact_notional,
        |prev, current| prev >= current,
        HyperliquidPerpFundingRateError::InvalidBidLevelOrder,
    )?;

    compute_impact_price_from_levels(bid_levels, impact_notional)
}

/// 在卖盘侧按固定 `impact_notional` 买入，返回平均成交价。
pub fn compute_impact_ask_price(
    ask_levels: &[HyperliquidPerpBookLevel],
    impact_notional: u64,
) -> Result<u64, HyperliquidPerpFundingRateError> {
    validate_book_levels(
        ask_levels,
        impact_notional,
        |prev, current| prev <= current,
        HyperliquidPerpFundingRateError::InvalidAskLevelOrder,
    )?;

    compute_impact_price_from_levels(ask_levels, impact_notional)
}

/// 按 Hyperliquid 协议公式，把一小时 5 秒采样序列计算成 hourly `funding_rate_e8`。
pub fn compute_hourly_funding_rate_e8(
    samples: &[HyperliquidPerpFundingSample],
) -> Result<i64, HyperliquidPerpFundingRateError> {
    if samples.is_empty() {
        return Err(HyperliquidPerpFundingRateError::EmptySamples);
    }
    if samples.len() != HYPERLIQUID_HOURLY_SAMPLE_COUNT {
        return Err(HyperliquidPerpFundingRateError::InvalidSampleCount);
    }

    let mut premium_sum_e8 = 0_i128;
    for sample in samples {
        let premium_e8 = sample_premium_e8(sample)?;
        premium_sum_e8 = premium_sum_e8
            .checked_add(premium_e8)
            .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;
    }

    let avg_premium_e8 = premium_sum_e8 / HYPERLIQUID_HOURLY_SAMPLE_COUNT as i128;
    let interest_minus_premium = INTEREST_E8_8H
        .checked_sub(avg_premium_e8)
        .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;
    let funding_8h_e8 = avg_premium_e8
        .checked_add(clamp_i128(interest_minus_premium, -CLAMP_BOUND_E8, CLAMP_BOUND_E8))
        .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;
    let hourly_funding_rate_e8 = funding_8h_e8 / 8;

    i64::try_from(hourly_funding_rate_e8)
        .map_err(|_| HyperliquidPerpFundingRateError::ArithmeticOverflow)
}

fn sample_premium_e8(
    sample: &HyperliquidPerpFundingSample,
) -> Result<i128, HyperliquidPerpFundingRateError> {
    if sample.oracle_price == 0 {
        return Err(HyperliquidPerpFundingRateError::InvalidOraclePrice);
    }
    if sample.impact_bid_price == 0 || sample.impact_ask_price == 0 {
        return Err(HyperliquidPerpFundingRateError::InvalidImpactPrice);
    }
    if sample.impact_bid_price > sample.impact_ask_price {
        return Err(HyperliquidPerpFundingRateError::CrossedImpactPrices);
    }

    let oracle_price = i128::from(sample.oracle_price);
    let impact_bid_price = i128::from(sample.impact_bid_price);
    let impact_ask_price = i128::from(sample.impact_ask_price);

    let up = impact_bid_price
        .checked_sub(oracle_price)
        .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?
        .max(0);
    let down = oracle_price
        .checked_sub(impact_ask_price)
        .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?
        .max(0);
    let signed_delta =
        up.checked_sub(down).ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;
    let scaled_delta = signed_delta
        .checked_mul(RATE_SCALE_E8)
        .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;

    Ok(scaled_delta / oracle_price)
}

fn validate_book_levels(
    levels: &[HyperliquidPerpBookLevel],
    impact_notional: u64,
    price_order_ok: impl Fn(u64, u64) -> bool,
    order_error: HyperliquidPerpFundingRateError,
) -> Result<(), HyperliquidPerpFundingRateError> {
    if levels.is_empty() {
        return Err(HyperliquidPerpFundingRateError::EmptyBookLevels);
    }
    if impact_notional == 0 {
        return Err(HyperliquidPerpFundingRateError::InvalidImpactNotional);
    }

    let mut previous_price = None;
    for level in levels {
        if level.price == 0 {
            return Err(HyperliquidPerpFundingRateError::InvalidLevelPrice);
        }
        if level.quantity == 0 {
            return Err(HyperliquidPerpFundingRateError::InvalidLevelQuantity);
        }
        if let Some(prev) = previous_price {
            if !price_order_ok(prev, level.price) {
                return Err(order_error);
            }
        }
        previous_price = Some(level.price);
    }

    Ok(())
}

fn compute_impact_price_from_levels(
    levels: &[HyperliquidPerpBookLevel],
    impact_notional: u64,
) -> Result<u64, HyperliquidPerpFundingRateError> {
    let target_notional = u128::from(impact_notional);
    let mut remaining_notional = target_notional;
    let mut filled_quantity_before_last_level = 0_u128;

    for level in levels {
        let level_price = u128::from(level.price);
        let level_quantity = u128::from(level.quantity);
        let level_notional = level_price
            .checked_mul(level_quantity)
            .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;

        if remaining_notional > level_notional {
            remaining_notional = remaining_notional
                .checked_sub(level_notional)
                .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;
            filled_quantity_before_last_level = filled_quantity_before_last_level
                .checked_add(level_quantity)
                .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;
            continue;
        }

        let denominator = filled_quantity_before_last_level
            .checked_mul(level_price)
            .and_then(|value| value.checked_add(remaining_notional))
            .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;
        let numerator = target_notional
            .checked_mul(level_price)
            .ok_or(HyperliquidPerpFundingRateError::ArithmeticOverflow)?;
        let avg_price = numerator / denominator;

        return u64::try_from(avg_price)
            .map_err(|_| HyperliquidPerpFundingRateError::ArithmeticOverflow);
    }

    Err(HyperliquidPerpFundingRateError::InsufficientBookDepth)
}

fn clamp_i128(value: i128, min: i128, max: i128) -> i128 {
    value.clamp(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn level(price: u64, quantity: u64) -> HyperliquidPerpBookLevel {
        HyperliquidPerpBookLevel { price, quantity }
    }

    fn repeated_samples(sample: HyperliquidPerpFundingSample) -> Vec<HyperliquidPerpFundingSample> {
        vec![sample; HYPERLIQUID_HOURLY_SAMPLE_COUNT]
    }

    #[test]
    fn computes_impact_bid_price_from_single_level() {
        let impact_bid_price = compute_impact_bid_price(&[level(100, 200)], 10_000).unwrap();

        assert_eq!(impact_bid_price, 100);
    }

    #[test]
    fn computes_impact_bid_price_across_multiple_levels() {
        let bid_levels = [level(100, 50), level(99, 100)];

        let impact_bid_price = compute_impact_bid_price(&bid_levels, 10_000).unwrap();

        assert_eq!(impact_bid_price, 99);
    }

    #[test]
    fn computes_impact_ask_price_across_multiple_levels() {
        let ask_levels = [level(101, 50), level(102, 100)];

        let impact_ask_price = compute_impact_ask_price(&ask_levels, 10_000).unwrap();

        assert_eq!(impact_ask_price, 101);
    }

    #[test]
    fn rejects_unsorted_bid_levels() {
        let err = compute_impact_bid_price(&[level(99, 50), level(100, 10)], 1_000).unwrap_err();

        assert_eq!(err, HyperliquidPerpFundingRateError::InvalidBidLevelOrder);
    }

    #[test]
    fn rejects_insufficient_book_depth() {
        let err = compute_impact_bid_price(&[level(100, 10)], 2_000).unwrap_err();

        assert_eq!(err, HyperliquidPerpFundingRateError::InsufficientBookDepth);
    }

    #[test]
    fn rejects_zero_impact_notional() {
        let err = compute_impact_bid_price(&[level(100, 10)], 0).unwrap_err();

        assert_eq!(err, HyperliquidPerpFundingRateError::InvalidImpactNotional);
    }

    #[test]
    fn computes_positive_hourly_funding_from_positive_premium() {
        let samples = repeated_samples(HyperliquidPerpFundingSample {
            oracle_price: 100,
            impact_bid_price: 102,
            impact_ask_price: 103,
        });

        let funding_rate_e8 = compute_hourly_funding_rate_e8(&samples).unwrap();

        assert_eq!(funding_rate_e8, 243_750);
    }

    #[test]
    fn computes_negative_hourly_funding_from_negative_premium() {
        let samples = repeated_samples(HyperliquidPerpFundingSample {
            oracle_price: 100,
            impact_bid_price: 97,
            impact_ask_price: 98,
        });

        let funding_rate_e8 = compute_hourly_funding_rate_e8(&samples).unwrap();

        assert_eq!(funding_rate_e8, -243_750);
    }

    #[test]
    fn returns_expected_result_when_average_premium_matches_interest() {
        let samples = repeated_samples(HyperliquidPerpFundingSample {
            oracle_price: 10_000,
            impact_bid_price: 10_001,
            impact_ask_price: 10_002,
        });

        let funding_rate_e8 = compute_hourly_funding_rate_e8(&samples).unwrap();

        assert_eq!(funding_rate_e8, 1_250);
    }

    #[test]
    fn clamps_large_positive_premium() {
        let samples = repeated_samples(HyperliquidPerpFundingSample {
            oracle_price: 100,
            impact_bid_price: 110,
            impact_ask_price: 111,
        });

        let funding_rate_e8 = compute_hourly_funding_rate_e8(&samples).unwrap();

        assert_eq!(funding_rate_e8, 1_243_750);
    }

    #[test]
    fn clamps_large_negative_premium() {
        let samples = repeated_samples(HyperliquidPerpFundingSample {
            oracle_price: 100,
            impact_bid_price: 89,
            impact_ask_price: 90,
        });

        let funding_rate_e8 = compute_hourly_funding_rate_e8(&samples).unwrap();

        assert_eq!(funding_rate_e8, -1_243_750);
    }

    #[test]
    fn rejects_zero_oracle_price() {
        let samples = repeated_samples(HyperliquidPerpFundingSample {
            oracle_price: 0,
            impact_bid_price: 100,
            impact_ask_price: 101,
        });

        let err = compute_hourly_funding_rate_e8(&samples).unwrap_err();

        assert_eq!(err, HyperliquidPerpFundingRateError::InvalidOraclePrice);
    }

    #[test]
    fn rejects_zero_impact_price() {
        let samples = repeated_samples(HyperliquidPerpFundingSample {
            oracle_price: 100,
            impact_bid_price: 0,
            impact_ask_price: 101,
        });

        let err = compute_hourly_funding_rate_e8(&samples).unwrap_err();

        assert_eq!(err, HyperliquidPerpFundingRateError::InvalidImpactPrice);
    }

    #[test]
    fn rejects_crossed_impact_prices() {
        let samples = repeated_samples(HyperliquidPerpFundingSample {
            oracle_price: 100,
            impact_bid_price: 102,
            impact_ask_price: 101,
        });

        let err = compute_hourly_funding_rate_e8(&samples).unwrap_err();

        assert_eq!(err, HyperliquidPerpFundingRateError::CrossedImpactPrices);
    }

    #[test]
    fn rejects_empty_samples() {
        let err = compute_hourly_funding_rate_e8(&[]).unwrap_err();

        assert_eq!(err, HyperliquidPerpFundingRateError::EmptySamples);
    }

    #[test]
    fn rejects_non_hourly_sample_count() {
        let samples = vec![
            HyperliquidPerpFundingSample {
                oracle_price: 100,
                impact_bid_price: 101,
                impact_ask_price: 102,
            };
            HYPERLIQUID_HOURLY_SAMPLE_COUNT - 1
        ];

        let err = compute_hourly_funding_rate_e8(&samples).unwrap_err();

        assert_eq!(err, HyperliquidPerpFundingRateError::InvalidSampleCount);
    }

    #[test]
    fn reports_arithmetic_overflow_when_result_exceeds_i64_range() {
        let samples = repeated_samples(HyperliquidPerpFundingSample {
            oracle_price: 1,
            impact_bid_price: u64::MAX,
            impact_ask_price: u64::MAX,
        });

        let err = compute_hourly_funding_rate_e8(&samples).unwrap_err();

        assert_eq!(err, HyperliquidPerpFundingRateError::ArithmeticOverflow);
    }
}
