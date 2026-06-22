use super::*;

fn build_order(
    order_id: &str,
    account_id: &str,
    side: SpotOrderSide,
    execution: SpotOrderExecution,
    time_in_force: SpotOrderTimeInForce,
    qty: u64,
) -> SpotOrder {
    let order_price = execution.order_price();
    let (reserved_base, reserved_quote) = match side {
        SpotOrderSide::Buy => (0, qty * order_price),
        SpotOrderSide::Sell => (qty, 0),
    };

    SpotOrder::new(
        order_id.to_string(),
        10_001,
        Some(42),
        account_id.to_string(),
        "BTCUSDT".to_string(),
        side,
        execution,
        time_in_force,
        qty,
        reserved_base,
        reserved_quote,
        None,
    )
}

fn buy_limit(price: u64, tif: SpotOrderTimeInForce, qty: u64) -> SpotOrder {
    build_order("buy-1", "buyer", SpotOrderSide::Buy, SpotOrderExecution::Limit { price }, tif, qty)
}

fn sell_limit(price: u64, tif: SpotOrderTimeInForce, qty: u64) -> SpotOrder {
    build_order(
        "sell-1",
        "seller",
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price },
        tif,
        qty,
    )
}

fn buy_market(aggressive_price: u64, qty: u64) -> SpotOrder {
    build_order(
        "buy-market-1",
        "buyer",
        SpotOrderSide::Buy,
        SpotOrderExecution::Market { aggressive_price },
        SpotOrderTimeInForce::Ioc,
        qty,
    )
}

#[test]
fn crosses_maker_price_buy_limit() {
    let taker = buy_limit(100, SpotOrderTimeInForce::Gtc, 3);

    assert!(taker.crosses_maker_price(99));
    assert!(taker.crosses_maker_price(100));
    assert!(!taker.crosses_maker_price(101));
}

#[test]
fn crosses_maker_price_sell_limit() {
    let taker = sell_limit(95, SpotOrderTimeInForce::Gtc, 3);

    assert!(taker.crosses_maker_price(96));
    assert!(taker.crosses_maker_price(95));
    assert!(!taker.crosses_maker_price(94));
}

#[test]
fn crosses_maker_price_market_uses_aggressive_price() {
    let taker = buy_market(105, 3);

    assert!(taker.crosses_maker_price(103));
    assert!(taker.crosses_maker_price(105));
    assert!(!taker.crosses_maker_price(106));
}

#[test]
fn would_be_rejected_as_alo_when_best_maker_crosses() -> Result<(), SpotOrderMatchError> {
    let taker = buy_limit(100, SpotOrderTimeInForce::Alo, 3);
    let crossing_maker = sell_limit(99, SpotOrderTimeInForce::Gtc, 1);
    let resting_maker = sell_limit(101, SpotOrderTimeInForce::Gtc, 1);

    assert!(taker.would_be_rejected_as_alo(Some(&crossing_maker))?);
    assert!(!taker.would_be_rejected_as_alo(Some(&resting_maker))?);
    assert!(!taker.would_be_rejected_as_alo(None)?);

    Ok(())
}

#[test]
fn ensure_matchable_accepts_open_and_partially_filled_orders() -> Result<(), SpotOrderMatchError> {
    buy_limit(100, SpotOrderTimeInForce::Gtc, 3).ensure_matchable()?;
    buy_limit(100, SpotOrderTimeInForce::Gtc, 3)
        .with_execution_state(SpotOrderStatus::PartiallyFilled, 1)
        .ensure_matchable()?;
    Ok(())
}

#[test]
fn ensure_matchable_rejects_non_matchable_status_and_inconsistent_state() {
    assert_eq!(
        buy_limit(100, SpotOrderTimeInForce::Gtc, 3)
            .with_execution_state(SpotOrderStatus::Filled, 3)
            .ensure_matchable(),
        Err(SpotOrderMatchError::OrderNotMatchable)
    );
    assert_eq!(
        buy_limit(100, SpotOrderTimeInForce::Gtc, 3)
            .with_execution_state(SpotOrderStatus::Canceled, 1)
            .ensure_matchable(),
        Err(SpotOrderMatchError::OrderNotMatchable)
    );
    assert_eq!(
        buy_limit(100, SpotOrderTimeInForce::Gtc, 3)
            .with_execution_state(SpotOrderStatus::Rejected, 0)
            .ensure_matchable(),
        Err(SpotOrderMatchError::OrderNotMatchable)
    );
    assert_eq!(
        buy_limit(100, SpotOrderTimeInForce::Gtc, 3)
            .with_execution_state(SpotOrderStatus::Open, 4)
            .ensure_matchable(),
        Err(SpotOrderMatchError::InconsistentExecutionState)
    );
}

#[test]
fn ensure_compatible_maker_for_checks_matching_constraints() -> Result<(), SpotOrderMatchError> {
    let taker = buy_limit(100, SpotOrderTimeInForce::Gtc, 3);
    let maker = sell_limit(101, SpotOrderTimeInForce::Gtc, 2);

    maker.ensure_compatible_maker_for(&taker)?;

    assert_eq!(taker.ensure_compatible_maker_for(&taker), Err(SpotOrderMatchError::MakerIsTaker));
    assert_eq!(
        build_order(
            "buy-maker-same-side",
            "other-buyer",
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 99 },
            SpotOrderTimeInForce::Gtc,
            2,
        )
        .ensure_compatible_maker_for(&taker),
        Err(SpotOrderMatchError::SameSideMaker)
    );
    assert_eq!(
        build_order(
            "sell-market",
            "seller",
            SpotOrderSide::Sell,
            SpotOrderExecution::Market { aggressive_price: 99 },
            SpotOrderTimeInForce::Ioc,
            2,
        )
        .ensure_compatible_maker_for(&taker),
        Err(SpotOrderMatchError::MakerMustBeLimit)
    );
    assert_eq!(
        SpotOrder::new(
            "sell-asset-mismatch".to_string(),
            10_002,
            Some(42),
            "seller".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price: 101 },
            SpotOrderTimeInForce::Gtc,
            2,
            2,
            0,
            None,
        )
        .ensure_compatible_maker_for(&taker),
        Err(SpotOrderMatchError::AssetMismatch)
    );
    assert_eq!(
        SpotOrder::new(
            "sell-symbol-mismatch".to_string(),
            10_001,
            Some(42),
            "seller".to_string(),
            "ETHUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price: 101 },
            SpotOrderTimeInForce::Gtc,
            2,
            2,
            0,
            None,
        )
        .ensure_compatible_maker_for(&taker),
        Err(SpotOrderMatchError::SymbolMismatch)
    );

    Ok(())
}

#[test]
fn apply_fill_updates_status_and_version() -> Result<(), SpotOrderMatchError> {
    let mut order = sell_limit(101, SpotOrderTimeInForce::Gtc, 3);
    order.apply_fill(1)?;
    assert_eq!(order.filled_qty, 1);
    assert_eq!(order.status, SpotOrderStatus::PartiallyFilled);
    assert_eq!(order.version, 2);

    order.apply_fill(2)?;
    assert_eq!(order.filled_qty, 3);
    assert_eq!(order.status, SpotOrderStatus::Filled);
    assert_eq!(order.version, 3);

    Ok(())
}

#[test]
fn apply_fill_rejects_overflow() {
    let mut order = sell_limit(101, SpotOrderTimeInForce::Gtc, 3)
        .with_execution_state(SpotOrderStatus::PartiallyFilled, u64::MAX);
    assert_eq!(order.apply_fill(1), Err(SpotOrderMatchError::ArithmeticOverflow));
}

#[test]
fn apply_finalization_updates_order_status_reason_and_version() -> Result<(), SpotOrderMatchError> {
    let mut order = buy_limit(100, SpotOrderTimeInForce::Ioc, 3);
    order.apply_finalization(SpotOrderFinalization {
        next_filled_qty: 1,
        status: SpotOrderStatus::Canceled,
        status_reason: Some(SpotOrderStatusReason::IocCancelRejected),
    })?;

    assert_eq!(order.filled_qty, 1);
    assert_eq!(order.status, SpotOrderStatus::Canceled);
    assert_eq!(order.status_reason, Some(SpotOrderStatusReason::IocCancelRejected));
    assert_eq!(order.version, 2);

    Ok(())
}

#[test]
fn reject_as_bad_alo_marks_order_rejected_and_bumps_version() -> Result<(), SpotOrderMatchError> {
    let mut order = buy_limit(100, SpotOrderTimeInForce::Alo, 3);
    order.reject_as_bad_alo()?;

    assert_eq!(order.status, SpotOrderStatus::Rejected);
    assert_eq!(order.status_reason, Some(SpotOrderStatusReason::BadAloPxRejected));
    assert_eq!(order.version, 2);

    Ok(())
}

#[test]
fn finalize_after_match_for_gtc_zero_partial_full() -> Result<(), SpotOrderMatchError> {
    let order = buy_limit(100, SpotOrderTimeInForce::Gtc, 3);

    assert_eq!(order.finalize_after_match(0), Err(SpotOrderMatchError::NoTradesMatched));
    assert_eq!(
        order.finalize_after_match(1)?,
        SpotOrderFinalization {
            next_filled_qty: 1,
            status: SpotOrderStatus::PartiallyFilled,
            status_reason: None,
        }
    );
    assert_eq!(
        order.finalize_after_match(3)?,
        SpotOrderFinalization {
            next_filled_qty: 3,
            status: SpotOrderStatus::Filled,
            status_reason: None,
        }
    );

    Ok(())
}

#[test]
fn finalize_after_match_for_ioc_zero_partial_full() -> Result<(), SpotOrderMatchError> {
    let limit_order = buy_limit(100, SpotOrderTimeInForce::Ioc, 3);
    let market_order = buy_market(105, 3);

    assert_eq!(
        limit_order.finalize_after_match(0)?,
        SpotOrderFinalization {
            next_filled_qty: 0,
            status: SpotOrderStatus::Rejected,
            status_reason: Some(SpotOrderStatusReason::IocCancelRejected),
        }
    );
    assert_eq!(
        market_order.finalize_after_match(0)?,
        SpotOrderFinalization {
            next_filled_qty: 0,
            status: SpotOrderStatus::Rejected,
            status_reason: Some(SpotOrderStatusReason::MarketOrderNoLiquidityRejected),
        }
    );
    assert_eq!(
        limit_order.finalize_after_match(1)?,
        SpotOrderFinalization {
            next_filled_qty: 1,
            status: SpotOrderStatus::Canceled,
            status_reason: Some(SpotOrderStatusReason::IocCancelRejected),
        }
    );
    assert_eq!(
        limit_order.finalize_after_match(3)?,
        SpotOrderFinalization {
            next_filled_qty: 3,
            status: SpotOrderStatus::Filled,
            status_reason: None,
        }
    );

    Ok(())
}

#[test]
fn finalize_after_match_for_alo_non_crossing_zero_partial_full() -> Result<(), SpotOrderMatchError>
{
    let order = buy_limit(100, SpotOrderTimeInForce::Alo, 3);

    assert_eq!(order.finalize_after_match(0), Err(SpotOrderMatchError::NoTradesMatched));
    assert_eq!(
        order.finalize_after_match(1)?,
        SpotOrderFinalization {
            next_filled_qty: 1,
            status: SpotOrderStatus::PartiallyFilled,
            status_reason: None,
        }
    );
    assert_eq!(
        order.finalize_after_match(3)?,
        SpotOrderFinalization {
            next_filled_qty: 3,
            status: SpotOrderStatus::Filled,
            status_reason: None,
        }
    );

    Ok(())
}
