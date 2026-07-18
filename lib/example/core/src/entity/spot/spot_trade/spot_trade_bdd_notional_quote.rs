use super::*;

fn trade(price: u64, qty: u64, taker_fee: u64, maker_fee: u64) -> SpotTrade {
    SpotTrade::new(
        "match-1-1".to_string(),
        "match-1".to_string(),
        10_001,
        "BTCUSDT".to_string(),
        "taker-1".to_string(),
        "maker-1".to_string(),
        "buyer".to_string(),
        "seller".to_string(),
        SpotOrderSide::Buy,
        price,
        qty,
        taker_fee,
        maker_fee,
    )
}

#[test]
fn notional_quote_returns_price_times_qty() {
    // Given: 一笔成交价格为 100，数量为 2。
    let trade = trade(100, 2, 1, 1);

    // When: 计算成交 quote 名义价值。
    let notional = trade.notional_quote();

    // Then: 返回 price * qty。
    assert_eq!(notional, Some(200));
}

#[test]
fn notional_quote_returns_none_when_price_times_qty_overflows() {
    // Given: 一笔成交价格为 u64::MAX，数量为 2。
    let trade = trade(u64::MAX, 2, 1, 1);

    // When: 计算成交 quote 名义价值。
    let notional = trade.notional_quote();

    // Then: 乘法溢出时返回 None。
    assert_eq!(notional, None);
}

#[test]
fn trade_stores_role_fee_amounts_alongside_notional_fact() {
    // Given: price=10_000、qty=2、maker_fee_bps=5、taker_fee_bps=10。
    let trade = trade(10_000, 2, 20, 10);

    // When: 读取成交事实。
    let notional = trade.notional_quote();

    // Then: 成交记录直接保存已确定的 taker / maker fee 金额。
    assert_eq!(notional, Some(20_000));
    assert_eq!(trade.taker_fee, 20);
    assert_eq!(trade.maker_fee, 10);
}
