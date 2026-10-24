use crate::perp::fund::funding_rate_tick::FundingRateTick;

fn premium_sample(bid: Price, ask: Price, oracle: Price) -> Rate {
    let impact_diff = max(bid.0 - oracle.0, 0) - max(oracle.0 - ask.0, 0);
    Rate::new(impact_diff * Rate::SCALE / oracle.0)
}

fn tick_hourly(coin: AssetId, time_ms: u64) {
    let samples = get_samples(coin, time_ms); // 过去 1h 的 premium 窗口
    let oracle = get_oracle_px(coin, time_ms); // 观测事实

    let tick = make_funding_rate_tick(coin, time_ms, &samples, oracle);

    append_fact(PerpFact::FundingTick(tick)); // 写 L0（市场观测）
}

/// 纯函数：样本窗口 + oracle → 一条 Tick
fn make_funding_rate_tick(
    coin: AssetId,
    time_ms: u64,
    samples: &[Rate], // 本小时所有 5s premium 样本
    oracle_px: Price,
) -> FundingRateTick {
    // 1. 时间平均 premium
    let avg_premium =
        Rate::new(samples.iter().map(|r| r.raw()).sum::<i128>() / samples.len() as i128);

    // 2. 固定项 r = 0.01% / 8h
    let r = Rate::parse_str("0.0001").unwrap() / 8;

    // 3. 套公式：F = avgP + clamp(r - avgP, ±0.05%)
    let clamp_max = Rate::from_bps(5); // 0.0005
    let clamp_min = Rate::from_bps(-5);
    let f_8h = avg_premium + clamp_rate(r - avg_premium, clamp_min, clamp_max);

    // 4. 除 8 → 小时费率，封顶 ±4%
    let hourly = clamp_rate(f_8h / 8, Rate::from_percent(-4), Rate::from_percent(4));

    FundingRateTick {
        coin,
        time_ms,
        funding_rate: hourly,
        premium: avg_premium,
        oracle_px,
        samples: samples.len() as u32,
    }
}
