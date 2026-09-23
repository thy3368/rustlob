use example_core_entity::*;

fn factory_entry_parent_order() -> SpotOrderV2 {
    match SpotOrderV2::new_active(
        "factory-parent-entry".to_string(),
        10_001,
        Some(1001),
        "trader-factory".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        100,
        SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
        2,
        "BTC",
        "USDT",
        5,
        10,
        Some("factory-parent-cloid".to_string()),
    ) {
        Ok(order) => order,
        Err(error) => panic!("invalid factory parent order: {error}"),
    }
}

fn factory_take_profit_child_order(parent: &SpotOrderV2) -> SpotOrderV2 {
    SpotOrderV2::new_trigger_pending(
        "factory-child-take-profit".to_string(),
        parent.asset,
        Some(1002),
        parent.account_id.clone(),
        parent.symbol.clone(),
        SpotOrderSide::Sell,
        1,
        119,
        SpotOrderType::Trigger {
            is_market: false,
            trigger_price: 120,
            tpsl: SpotOrderTriggerRole::TakeProfit,
        },
        Some("factory-tp-cloid".to_string()),
        1,
    )
}

fn factory_stop_loss_child_order(parent: &SpotOrderV2) -> SpotOrderV2 {
    SpotOrderV2::new_trigger_pending(
        "factory-child-stop-loss".to_string(),
        parent.asset,
        Some(1003),
        parent.account_id.clone(),
        parent.symbol.clone(),
        SpotOrderSide::Sell,
        1,
        89,
        SpotOrderType::Trigger {
            is_market: false,
            trigger_price: 90,
            tpsl: SpotOrderTriggerRole::StopLoss,
        },
        Some("factory-sl-cloid".to_string()),
        1,
    )
}

fn factory_parent_with_tpsl_orders() -> (SpotOrderV2, SpotOrderV2, SpotOrderV2) {
    let parent = factory_entry_parent_order();
    let take_profit = factory_take_profit_child_order(&parent);
    let stop_loss = factory_stop_loss_child_order(&parent);
    (parent, take_profit, stop_loss)
}

fn maker_buy_order() -> SpotOrderV2 {
    match SpotOrderV2::new_active(
        "factory-maker-buy".to_string(),
        10_001,
        Some(1004),
        "maker-factory".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        130,
        SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
        1,
        "BTC",
        "USDT",
        5,
        10,
        None,
    ) {
        Ok(order) => order,
        Err(error) => panic!("invalid factory maker order: {error}"),
    }
}

fn assert_same_market_and_owner(parent: &SpotOrderV2, child: &SpotOrderV2) {
    assert!(child.belongs_to_account(parent.account_id.as_str()));
    assert!(child.trades_asset(parent.asset));
    assert!(child.trades_symbol(parent.symbol.as_str()));
}

fn assert_tpsl_child_shape(parent: &SpotOrderV2, child: &SpotOrderV2) {
    assert!(child.is_pending());
    assert_eq!(child.status(), SpotOrderStatus::Pending);
    assert_same_market_and_owner(parent, child);
    assert_ne!(child.side, parent.side);
    assert!(child.qty <= parent.qty);
}

#[test]
fn given_factory_parent_with_tp_sl_when_created_then_three_spot_order_v2_orders_exist() {
    let (parent, take_profit, stop_loss) = factory_parent_with_tpsl_orders();

    assert_eq!(parent.status(), SpotOrderStatus::Open);

    assert_tpsl_child_shape(&parent, &take_profit);
    assert_tpsl_child_shape(&parent, &stop_loss);

    assert_ne!(parent.order_id, take_profit.order_id);
    assert_ne!(parent.order_id, stop_loss.order_id);
    assert_ne!(take_profit.order_id, stop_loss.order_id);
}

#[test]
fn given_factory_tp_sl_children_when_created_then_they_have_no_reservation_and_cannot_match() {
    let (_parent, mut take_profit, mut stop_loss) = factory_parent_with_tpsl_orders();

    assert_eq!(take_profit.active_reservation(), None);
    assert_eq!(take_profit.active_fee_reservation(), None);
    assert_eq!(stop_loss.active_reservation(), None);
    assert_eq!(stop_loss.active_fee_reservation(), None);

    let mut tp_makers = vec![maker_buy_order()];
    let mut sl_makers = vec![maker_buy_order()];
    assert_eq!(
        take_profit.match_with_makers(
            tp_makers.as_mut_slice(),
            MatchSpotOrderV2Input {
                match_id: "match-factory-tp-pending".to_string(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
                executed_at_ms: 1_717_171_717_000,
            },
        ),
        Err(SpotOrderV2BehaviorError::OrderNotMatchable)
    );
    assert_eq!(
        stop_loss.match_with_makers(
            sl_makers.as_mut_slice(),
            MatchSpotOrderV2Input {
                match_id: "match-factory-sl-pending".to_string(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
                executed_at_ms: 1_717_171_717_000,
            },
        ),
        Err(SpotOrderV2BehaviorError::OrderNotMatchable)
    );
}

#[test]
fn given_factory_child_when_triggered_then_it_becomes_active_spot_order_v2()
-> Result<(), SpotOrderV2BehaviorError> {
    let (_parent, mut take_profit, _stop_loss) = factory_parent_with_tpsl_orders();

    assert!(take_profit.is_pending());
    assert_eq!(take_profit.active_reservation(), None);

    take_profit.trigger(TriggerSpotOrderV2Input {
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    })?;

    assert_eq!(take_profit.status(), SpotOrderStatus::Open);
    assert_eq!(take_profit.status_reason(), None);
    assert_eq!(
        take_profit.active_reservation().map(|reservation| reservation.remaining_amount),
        Some(1)
    );
    assert_eq!(
        take_profit.active_fee_reservation().map(|reservation| reservation.remaining_amount),
        Some(1)
    );
    assert_eq!(
        take_profit.active_reservation().map(|reservation| reservation.reservation_kind),
        Some(ReservationKind::SpotSellBase)
    );
    assert_eq!(
        take_profit.active_reservation().map(|reservation| reservation.original_amount),
        Some(1)
    );
    Ok(())
}
