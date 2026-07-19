use super::*;
use crate::entity::{BalanceLedgerOperation, ReservationStatus};

fn buy_order() -> SpotOrderV2 {
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
        0,
        200,
        test_principal_reservation("order-buy", "trader-1", SpotOrderSide::Buy, 2, 100),
        Some("cloid-1".to_string()),
        1,
    )
}

fn maker_sell_qty(order_id: &str, qty: u64, price: u64) -> SpotOrderV2 {
    SpotOrderV2::new(
        order_id.to_string(),
        10_001,
        Some(price),
        format!("account-{order_id}"),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price },
        SpotOrderTimeInForce::Gtc,
        qty,
        0,
        SpotOrderStatus::Open,
        None,
        qty,
        0,
        test_principal_reservation(
            order_id,
            format!("account-{order_id}").as_str(),
            SpotOrderSide::Sell,
            qty,
            price,
        ),
        None,
        1,
    )
}

fn place_input(side: SpotOrderSide) -> PlaceSpotOrderV2Input {
    PlaceSpotOrderV2Input {
        order_id: format!("place-{}", side.as_str()),
        asset: 10_001,
        account_id: "placer".to_string(),
        symbol: "BTCUSDT".to_string(),
        side,
        execution: SpotOrderExecution::Limit { price: 100 },
        time_in_force: SpotOrderTimeInForce::Gtc,
        qty: 2,
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        base_balance_entity_id: "balance:placer:BTC".to_string(),
        quote_balance_entity_id: "balance:placer:USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
        client_order_id: Some("client-1".to_string()),
    }
}

#[test]
fn place_buy_freezes_quote_and_creates_open_order() -> Result<(), SpotOrderV2BehaviorError> {
    let outcome = SpotOrderV2::place(place_input(SpotOrderSide::Buy))?;

    assert_eq!(outcome.order.status, SpotOrderStatus::Open);
    assert_eq!(outcome.order.filled_qty, 0);
    assert_eq!(outcome.order.version, 1);
    assert_eq!(outcome.order.reserved_base, 0);
    assert_eq!(outcome.order.reserved_quote, 200);
    assert_eq!(outcome.order.reservation.reservation_kind, ReservationKind::SpotBuyQuote);
    assert_eq!(outcome.order.reservation.remaining_amount, 200);
    assert_eq!(outcome.freeze_ledger_entry.operation, BalanceLedgerOperation::Freeze);
    assert_eq!(outcome.freeze_ledger_entry.asset_id, "USDT");
    assert_eq!(outcome.freeze_ledger_entry.balance_entity_id, "balance:placer:USDT");
    assert_eq!(outcome.freeze_ledger_entry.amount, 200);
    assert_eq!(
        outcome.freeze_ledger_entry.reason,
        BalanceLedgerReason::FreezeForOrder { order_id: "place-buy".to_string() }
    );
    Ok(())
}

#[test]
fn place_sell_freezes_base_and_creates_open_order() -> Result<(), SpotOrderV2BehaviorError> {
    let outcome = SpotOrderV2::place(place_input(SpotOrderSide::Sell))?;

    assert_eq!(outcome.order.status, SpotOrderStatus::Open);
    assert_eq!(outcome.order.reserved_base, 2);
    assert_eq!(outcome.order.reserved_quote, 0);
    assert_eq!(outcome.order.reservation.reservation_kind, ReservationKind::SpotSellBase);
    assert_eq!(outcome.order.reservation.remaining_amount, 2);
    assert_eq!(outcome.freeze_ledger_entry.operation, BalanceLedgerOperation::Freeze);
    assert_eq!(outcome.freeze_ledger_entry.asset_id, "BTC");
    assert_eq!(outcome.freeze_ledger_entry.balance_entity_id, "balance:placer:BTC");
    assert_eq!(outcome.freeze_ledger_entry.amount, 2);
    Ok(())
}

#[test]
fn place_rejects_invalid_quantity_price_and_overflow() {
    let mut zero_qty = place_input(SpotOrderSide::Buy);
    zero_qty.qty = 0;
    assert_eq!(SpotOrderV2::place(zero_qty), Err(SpotOrderV2BehaviorError::InvalidQuantity));

    let mut zero_price = place_input(SpotOrderSide::Buy);
    zero_price.execution = SpotOrderExecution::Limit { price: 0 };
    assert_eq!(SpotOrderV2::place(zero_price), Err(SpotOrderV2BehaviorError::InvalidPrice));

    let mut overflow = place_input(SpotOrderSide::Buy);
    overflow.qty = u64::MAX;
    overflow.execution = SpotOrderExecution::Limit { price: 2 };
    assert_eq!(SpotOrderV2::place(overflow), Err(SpotOrderV2BehaviorError::ArithmeticOverflow));
}

#[test]
fn match_with_makers_can_consume_multiple_makers() -> Result<(), SpotOrderV2BehaviorError> {
    let mut taker = SpotOrderV2 { qty: 3, reserved_quote: 300, ..buy_order() };
    taker.reservation =
        test_principal_reservation("order-buy", "trader-1", SpotOrderSide::Buy, 3, 100);
    let mut makers = vec![maker_sell_qty("maker-1", 1, 90), maker_sell_qty("maker-2", 3, 95)];

    let outcome = taker.match_with_makers(
        makers.as_mut_slice(),
        MatchSpotOrderV2Input {
            match_id: "match-1".to_string(),
            maker_fee_bps: 100,
            taker_fee_bps: 100,
        },
    )?;

    assert_eq!(outcome.trades.len(), 2);
    assert_eq!(outcome.trades[0].trade_id, "match-1-1");
    assert_eq!(outcome.trades[0].price, 90);
    assert_eq!(outcome.trades[0].qty, 1);
    assert_eq!(outcome.trades[1].trade_id, "match-1-2");
    assert_eq!(outcome.trades[1].price, 95);
    assert_eq!(outcome.trades[1].qty, 2);
    assert_eq!(taker.filled_qty, 3);
    assert_eq!(taker.status, SpotOrderStatus::Filled);
    assert_eq!(makers[0].filled_qty, 1);
    assert_eq!(makers[0].status, SpotOrderStatus::Filled);
    assert_eq!(makers[1].filled_qty, 2);
    assert_eq!(makers[1].status, SpotOrderStatus::PartiallyFilled);
    Ok(())
}

#[test]
fn match_with_makers_stops_at_first_non_crossing_maker() -> Result<(), SpotOrderV2BehaviorError> {
    let mut taker = SpotOrderV2 { qty: 3, reserved_quote: 300, ..buy_order() };
    taker.reservation =
        test_principal_reservation("order-buy", "trader-1", SpotOrderSide::Buy, 3, 100);
    let mut makers = vec![maker_sell_qty("maker-1", 1, 90), maker_sell_qty("maker-2", 1, 110)];

    let outcome = taker.match_with_makers(
        makers.as_mut_slice(),
        MatchSpotOrderV2Input {
            match_id: "match-2".to_string(),
            maker_fee_bps: 0,
            taker_fee_bps: 0,
        },
    )?;

    assert_eq!(outcome.trades.len(), 1);
    assert_eq!(taker.filled_qty, 1);
    assert_eq!(taker.status, SpotOrderStatus::PartiallyFilled);
    assert_eq!(makers[1].filled_qty, 0);
    assert_eq!(makers[1].status, SpotOrderStatus::Open);
    Ok(())
}

#[test]
fn match_with_makers_returns_empty_when_best_maker_does_not_cross()
-> Result<(), SpotOrderV2BehaviorError> {
    let mut taker = buy_order();
    let mut makers = vec![maker_sell_qty("maker-1", 1, 110)];

    let outcome = taker.match_with_makers(
        makers.as_mut_slice(),
        MatchSpotOrderV2Input {
            match_id: "match-empty".to_string(),
            maker_fee_bps: 0,
            taker_fee_bps: 0,
        },
    )?;

    assert!(outcome.trades.is_empty());
    assert_eq!(taker.status, SpotOrderStatus::Open);
    assert_eq!(taker.filled_qty, 0);
    assert_eq!(makers[0].status, SpotOrderStatus::Open);
    Ok(())
}

#[test]
fn match_with_makers_rejects_invalid_maker() {
    let mut taker = buy_order();
    let mut makers = vec![SpotOrderV2 {
        order_id: "same-side-maker".to_string(),
        account_id: "other".to_string(),
        ..buy_order()
    }];

    assert_eq!(
        taker.match_with_makers(
            makers.as_mut_slice(),
            MatchSpotOrderV2Input {
                match_id: "match-invalid".to_string(),
                maker_fee_bps: 0,
                taker_fee_bps: 0,
            },
        ),
        Err(SpotOrderV2BehaviorError::SameSideMaker)
    );
}

#[test]
fn cancel_releases_remaining_reservation_and_derives_unfreeze_ledger()
-> Result<(), SpotOrderV2BehaviorError> {
    let mut order = buy_order();

    let outcome = order.cancel(CancelSpotOrderV2Input {
        balance_entity_id: "balance:trader-1:USDT".to_string(),
    })?;

    assert_eq!(order.status, SpotOrderStatus::Canceled);
    assert_eq!(order.status_reason, Some(SpotOrderStatusReason::CanceledByUser));
    assert_eq!(order.version, 2);
    assert_eq!(order.reservation.remaining_amount, 0);
    assert_eq!(order.reservation.released_amount, 200);
    assert_eq!(order.reservation.status, ReservationStatus::ClosedByRelease);
    assert_eq!(order.reservation.close_reason, Some(ReservationCloseReason::Canceled));
    assert_eq!(outcome.unfreeze_ledger_entry.operation, BalanceLedgerOperation::Unfreeze);
    assert_eq!(outcome.unfreeze_ledger_entry.amount, 200);
    assert_eq!(outcome.unfreeze_ledger_entry.asset_id, "USDT");
    assert_eq!(outcome.unfreeze_ledger_entry.balance_entity_id, "balance:trader-1:USDT");
    assert_eq!(
        outcome.unfreeze_ledger_entry.reason,
        BalanceLedgerReason::UnfreezeForCancel { order_id: "order-buy".to_string() }
    );
    Ok(())
}

#[test]
fn cancel_rejects_terminal_order() {
    let mut filled = SpotOrderV2 {
        filled_qty: 2,
        status: SpotOrderStatus::Filled,
        status_reason: Some(SpotOrderStatusReason::Filled),
        ..buy_order()
    };

    assert_eq!(
        filled.cancel(CancelSpotOrderV2Input {
            balance_entity_id: "balance:trader-1:USDT".to_string(),
        }),
        Err(SpotOrderV2BehaviorError::OrderNotCancelable)
    );
    assert_eq!(filled.status, SpotOrderStatus::Filled);
    assert_eq!(filled.version, 1);
}
