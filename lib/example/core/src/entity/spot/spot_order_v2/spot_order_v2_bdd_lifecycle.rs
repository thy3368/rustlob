use super::*;

fn trigger_pending_order() -> SpotOrderV2 {
    SpotOrderV2::new_trigger_pending(
        "trigger-order".to_string(),
        10_001,
        Some(77),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        2,
        95,
        SpotOrderTriggerRole::StopLoss,
        SpotOrderExecution::Limit { price: 100 },
        SpotOrderTimeInForce::Gtc,
        Some("trigger-cloid".to_string()),
        1,
    )
}

fn active_buy_order() -> SpotOrderV2 {
    match SpotOrderV2::new_active(
        "active-order".to_string(),
        10_001,
        Some(78),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: 100 },
        SpotOrderTimeInForce::Gtc,
        2,
        "BTC",
        "USDT",
        5,
        10,
        None,
    ) {
        Ok(order) => order,
        Err(error) => panic!("invalid active test order: {error}"),
    }
}

fn maker_sell_order() -> SpotOrderV2 {
    match SpotOrderV2::new_active(
        "maker-order".to_string(),
        10_001,
        Some(79),
        "maker-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: 90 },
        SpotOrderTimeInForce::Gtc,
        1,
        "BTC",
        "USDT",
        5,
        10,
        None,
    ) {
        Ok(order) => order,
        Err(error) => panic!("invalid maker test order: {error}"),
    }
}

#[test]
fn given_trigger_pending_order_when_created_then_it_has_no_reservation_and_cannot_match() {
    let mut order = trigger_pending_order();
    let mut makers = vec![maker_sell_order()];

    assert!(order.is_trigger_pending());
    assert_eq!(order.active_reservation(), None);
    assert_eq!(order.active_fee_reservation(), None);
    assert_eq!(
        order.match_with_makers(
            makers.as_mut_slice(),
            MatchSpotOrderV2Input {
                match_id: "match-trigger-pending".to_string(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
            },
        ),
        Err(SpotOrderV2BehaviorError::OrderNotMatchable)
    );
}

#[test]
fn given_trigger_pending_order_when_triggered_then_active_order_state_is_created()
-> Result<(), SpotOrderV2BehaviorError> {
    let mut order = trigger_pending_order();

    order.trigger(TriggerSpotOrderV2Input {
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    })?;

    assert!(matches!(order.lifecycle, SpotOrderLifecycle::Active(_)));
    assert_eq!(order.status(), SpotOrderStatus::Open);
    assert_eq!(order.status_reason(), Some(SpotOrderStatusReason::Triggered));
    assert_eq!(
        order.active_reservation().map(|reservation| reservation.remaining_amount),
        Some(200)
    );
    assert_eq!(order.version, 2);
    Ok(())
}

#[test]
fn given_trigger_pending_order_when_canceled_then_only_trigger_rule_is_canceled()
-> Result<(), SpotOrderV2BehaviorError> {
    let mut order = trigger_pending_order();

    let outcome = order.cancel(CancelSpotOrderV2Input {
        balance_entity_id: "balance:trader-1:USDT".to_string(),
    })?;

    assert!(matches!(order.lifecycle, SpotOrderLifecycle::Canceled(_)));
    assert_eq!(order.status(), SpotOrderStatus::Canceled);
    assert_eq!(order.status_reason(), Some(SpotOrderStatusReason::CanceledByUser));
    assert_eq!(outcome.unfreeze_ledger_entry, None);
    Ok(())
}

#[test]
fn given_active_order_when_matching_then_active_state_advances_without_trigger_fields()
-> Result<(), SpotOrderV2BehaviorError> {
    let mut taker = active_buy_order();
    let mut makers = vec![maker_sell_order()];

    let outcome = taker.match_with_makers(
        makers.as_mut_slice(),
        MatchSpotOrderV2Input {
            match_id: "match-active".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        },
    )?;

    assert_eq!(outcome.trades.len(), 1);
    assert!(matches!(taker.lifecycle, SpotOrderLifecycle::PartiallyFilled(_)));
    assert_eq!(taker.filled_qty(), 1);
    assert_eq!(taker.status(), SpotOrderStatus::PartiallyFilled);
    assert!(!taker.is_trigger_pending());
    Ok(())
}

#[test]
fn given_active_order_when_canceled_then_reservation_is_released()
-> Result<(), SpotOrderV2BehaviorError> {
    let mut order = active_buy_order();

    let outcome = order.cancel(CancelSpotOrderV2Input {
        balance_entity_id: "balance:trader-1:USDT".to_string(),
    })?;

    let unfreeze_ledger_entry =
        outcome.unfreeze_ledger_entry.ok_or(SpotOrderV2BehaviorError::OrderNotCancelable)?;
    assert_eq!(unfreeze_ledger_entry.amount, 200);
    assert_eq!(order.reservation.remaining_amount, 0);
    assert_eq!(order.status(), SpotOrderStatus::Canceled);
    assert!(matches!(order.lifecycle, SpotOrderLifecycle::Canceled(_)));
    Ok(())
}
