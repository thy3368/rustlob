use super::*;

// 本文件只承载 SpotOrderV2 订单生命周期的 happy path BDD 规格。

fn buy_order() -> SpotOrderV2 {
    // 默认是一笔已入簿、可继续成交或撤销的 GTC 限价买单。
    SpotOrderV2::new(
        "order-buy".to_string(),
        10_001,
        Some(42),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: 100 },
        SpotOrderTimeInForce::Gtc,
        2,
        0,
        SpotOrderStatus::Open,
        None,
        test_principal_reservation("order-buy", "trader-1", SpotOrderSide::Buy, 2, 100),
        Some("cloid-1".to_string()),
        1,
    )
}

fn market_buy_order() -> SpotOrderV2 {
    // 默认是一笔以激进价格立即撮合的 IOC 市价买单。
    SpotOrderV2::new(
        "order-market-buy".to_string(),
        10_001,
        None,
        "trader-3".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        SpotOrderExecution::Market { aggressive_price: 120 },
        SpotOrderTimeInForce::Ioc,
        2,
        0,
        SpotOrderStatus::Open,
        None,
        test_principal_reservation("order-market-buy", "trader-3", SpotOrderSide::Buy, 2, 120),
        None,
        1,
    )
}

#[test]
fn fill_moves_order_to_partially_filled_and_filled() -> Result<(), SpotOrderV2MatchError> {
    // Given：订单从 open 限价买单开始。
    let mut order = buy_order();

    // When：先成交一部分。
    order.fill(1)?;
    // Then：订单进入部分成交，版本推进一次。
    assert_eq!(order.filled_qty, 1);
    assert_eq!(order.status, SpotOrderStatus::PartiallyFilled);
    assert_eq!(order.status_reason, None);
    assert_eq!(order.version, 2);

    // When：剩余数量继续成交。
    order.fill(1)?;
    // Then：订单满量成交，不带拒绝或撤销原因。
    assert_eq!(order.filled_qty, 2);
    assert_eq!(order.status, SpotOrderStatus::Filled);
    assert_eq!(order.status_reason, None);
    assert_eq!(order.version, 3);

    Ok(())
}

#[test]
fn cancel_marks_order_canceled() -> Result<(), SpotOrderV2BehaviorError> {
    // Given：订单仍处于可撤状态。
    let mut order = buy_order();

    // When：用户主动撤单。
    order.cancel(CancelSpotOrderV2Input {
        balance_entity_id: "balance:trader-1:USDT".to_string(),
    })?;

    // Then：订单标记为用户撤销。
    assert_eq!(order.status, SpotOrderStatus::Canceled);
    assert_eq!(order.status_reason, Some(SpotOrderStatusReason::CanceledByUser));
    assert_eq!(order.version, 2);
    Ok(())
}

#[test]
fn reject_as_bad_alo_marks_order_rejected() -> Result<(), SpotOrderV2MatchError> {
    // Given：ALO 订单会立即吃单，因此不能挂入盘口。
    let mut order = buy_order();

    // When：撮合前判定为 bad ALO。
    order.reject_as_bad_alo()?;

    // Then：订单被拒绝并记录 ALO 价格原因。
    assert_eq!(order.status, SpotOrderStatus::Rejected);
    assert_eq!(order.status_reason, Some(SpotOrderStatusReason::BadAloPxRejected));
    assert_eq!(order.version, 2);
    Ok(())
}

#[test]
fn reject_as_no_liquidity_uses_market_or_ioc_reason() -> Result<(), SpotOrderV2MatchError> {
    // Given：市价单和 IOC 限价单都没有可用对手方流动性。
    let mut market = market_buy_order();
    let mut limit_ioc = SpotOrderV2 { time_in_force: SpotOrderTimeInForce::Ioc, ..buy_order() };

    // When：撮合侧按无流动性拒绝。
    market.reject_as_no_liquidity()?;
    limit_ioc.reject_as_no_liquidity()?;

    // Then：两类订单使用各自的业务拒绝原因。
    assert_eq!(market.status_reason, Some(SpotOrderStatusReason::MarketOrderNoLiquidityRejected));
    assert_eq!(limit_ioc.status_reason, Some(SpotOrderStatusReason::IocCancelRejected));
    assert_eq!(market.status, SpotOrderStatus::Rejected);
    assert_eq!(limit_ioc.status, SpotOrderStatus::Rejected);
    Ok(())
}

#[test]
fn finish_after_match_covers_gtc_alo_ioc_and_full_fill() -> Result<(), SpotOrderV2MatchError> {
    // Given：GTC 本轮没有任何成交。
    let mut gtc_no_fill = buy_order();
    // Then：finish 后报告本轮无成交。
    assert_eq!(gtc_no_fill.finish_after_match(0), Err(SpotOrderV2MatchError::NoTradesMatched));

    // Given：ALO 本轮也没有成交。
    let mut alo_no_fill = SpotOrderV2 { time_in_force: SpotOrderTimeInForce::Alo, ..buy_order() };
    // Then：finish 后同样报告无成交。
    assert_eq!(alo_no_fill.finish_after_match(0), Err(SpotOrderV2MatchError::NoTradesMatched));

    // Given：IOC 市价单本轮只成交一部分。
    let mut ioc_partial = market_buy_order();
    // When：taker 本轮撮合结束。
    ioc_partial.finish_after_match(1)?;
    // Then：已成交数量保留，剩余数量按 IOC 取消。
    assert_eq!(ioc_partial.filled_qty, 1);
    assert_eq!(ioc_partial.status, SpotOrderStatus::Canceled);
    assert_eq!(ioc_partial.status_reason, Some(SpotOrderStatusReason::IocCancelRejected));

    // Given：IOC 市价单完全没有流动性。
    let mut ioc_none = market_buy_order();
    // When：taker 本轮撮合结束。
    ioc_none.finish_after_match(0)?;
    // Then：订单被拒绝，并记录市价单无流动性原因。
    assert_eq!(ioc_none.filled_qty, 0);
    assert_eq!(ioc_none.status, SpotOrderStatus::Rejected);
    assert_eq!(ioc_none.status_reason, Some(SpotOrderStatusReason::MarketOrderNoLiquidityRejected));

    // Given：限价单在本轮完全成交。
    let mut full = buy_order();
    // When：taker 本轮撮合结束。
    full.finish_after_match(2)?;
    // Then：订单进入完全成交状态。
    assert_eq!(full.filled_qty, 2);
    assert_eq!(full.status, SpotOrderStatus::Filled);
    assert_eq!(full.status_reason, None);
    Ok(())
}
