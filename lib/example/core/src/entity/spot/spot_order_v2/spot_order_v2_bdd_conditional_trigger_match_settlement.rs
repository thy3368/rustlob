use super::*;
use crate::entity::{SettlementKind, SettlementTransferPurpose};

fn trigger_pending_buy_order() -> SpotOrderV2 {
    SpotOrderV2::new_trigger_pending(
        "conditional-buy".to_string(),
        10_001,
        Some(7001),
        "buyer-account".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        2,
        95,
        SpotOrderTriggerRole::StopLoss,
        SpotOrderExecution::Limit { price: 100 },
        SpotOrderTimeInForce::Gtc,
        Some("conditional-buy-cloid".to_string()),
        1,
    )
}

fn maker_sell_order(order_id: &str, qty: u64, price: u64) -> SpotOrderV2 {
    match SpotOrderV2::new_active(
        order_id.to_string(),
        10_001,
        Some(price),
        format!("seller-account-{order_id}"),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price },
        SpotOrderTimeInForce::Gtc,
        qty,
        "BTC",
        "USDT",
        5,
        10,
        None,
    ) {
        Ok(order) => order,
        Err(error) => panic!("invalid maker sell order: {error}"),
    }
}

fn trigger_as_active(order: &mut SpotOrderV2) -> Result<(), SpotOrderV2BehaviorError> {
    order.trigger(TriggerSpotOrderV2Input {
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    })
}

fn match_input(match_id: &str) -> MatchSpotOrderV2Input {
    MatchSpotOrderV2Input { match_id: match_id.to_string(), maker_fee_bps: 5, taker_fee_bps: 10 }
}

#[test]
fn given_trigger_pending_order_when_match_is_attempted_before_trigger_then_it_is_rejected() {
    // Given: 条件买单仍处于 TriggerPending，不应冻结也不应进入撮合。
    let mut taker = trigger_pending_buy_order();
    let mut makers = vec![maker_sell_order("maker-crossing-before-trigger", 1, 90)];

    // When / Then: 未触发直接撮合被拒绝，maker 不推进，也不会产生 trade。
    assert_eq!(
        taker.match_with_makers(makers.as_mut_slice(), match_input("match-before-trigger")),
        Err(SpotOrderV2BehaviorError::OrderNotMatchable)
    );
    assert!(taker.is_trigger_pending());
    assert_eq!(taker.filled_qty(), 0);
    assert_eq!(taker.version, 1);
    assert_eq!(makers[0].filled_qty(), 0);
    assert_eq!(makers[0].status(), SpotOrderStatus::Open);
    assert_eq!(makers[0].version, 1);
}

#[test]
fn given_trigger_pending_buy_order_when_triggered_without_crossing_maker_then_it_becomes_active_resting_order()
-> Result<(), SpotOrderV2BehaviorError> {
    // Given: 条件买单触发为 100 限价 GTC。
    let mut taker = trigger_pending_buy_order();

    // When: 触发后，最优卖单价格 110，不和买单限价交叉。
    trigger_as_active(&mut taker)?;
    let mut makers = vec![maker_sell_order("maker-non-crossing", 1, 110)];
    let outcome =
        taker.match_with_makers(makers.as_mut_slice(), match_input("match-non-crossing"))?;

    // Then: 订单已进入 active，可挂单等待后续撮合；本轮不产生成交事实。
    assert!(matches!(taker.lifecycle, SpotOrderLifecycle::Active(_)));
    assert_eq!(taker.status(), SpotOrderStatus::Open);
    assert_eq!(taker.status_reason(), Some(SpotOrderStatusReason::Triggered));
    assert_eq!(
        taker.active_reservation().map(|reservation| reservation.reservation_kind),
        Some(ReservationKind::SpotBuyQuote)
    );
    assert_eq!(
        taker.active_reservation().map(|reservation| reservation.remaining_amount),
        Some(200)
    );
    assert_eq!(
        taker.active_fee_reservation().map(|reservation| reservation.reservation_kind),
        Some(ReservationKind::SpotBuyFeeQuote)
    );
    assert_eq!(
        taker.active_fee_reservation().map(|reservation| reservation.remaining_amount),
        Some(1)
    );
    assert!(outcome.trades.is_empty());
    assert_eq!(taker.filled_qty(), 0);
    assert_eq!(taker.version, 2);
    assert_eq!(makers[0].filled_qty(), 0);
    assert_eq!(makers[0].status(), SpotOrderStatus::Open);
    Ok(())
}

#[test]
fn given_trigger_pending_buy_order_when_triggered_and_crosses_maker_then_trade_is_created()
-> Result<(), SpotOrderV2BehaviorError> {
    // Given: 条件买单触发为 active 后，面对可成交的 maker sell。
    let mut taker = trigger_pending_buy_order();
    trigger_as_active(&mut taker)?;
    let mut makers = vec![maker_sell_order("maker-crossing", 1, 90)];

    // When: 以 maker 价格撮合 1 BTC。
    let outcome = taker.match_with_makers(makers.as_mut_slice(), match_input("match-triggered"))?;

    // Then: 成交事实成立，并按现有撮合规则推进 taker / maker 生命周期。
    assert_eq!(outcome.trades.len(), 1);
    let trade = &outcome.trades[0];
    assert_eq!(trade.trade_id, "match-triggered-1");
    assert_eq!(trade.taker_order_id, "conditional-buy");
    assert_eq!(trade.maker_order_id, "maker-crossing");
    assert_eq!(trade.buyer_account_id(), "buyer-account");
    assert_eq!(trade.seller_account_id(), "seller-account-maker-crossing");
    assert_eq!(trade.price, 90);
    assert_eq!(trade.qty, 1);
    assert_eq!(trade.taker_fee, 1);
    assert_eq!(trade.maker_fee, 1);
    assert_eq!(taker.filled_qty(), 1);
    assert_eq!(taker.status(), SpotOrderStatus::PartiallyFilled);
    assert_eq!(taker.version, 3);
    assert_eq!(makers[0].filled_qty(), 1);
    assert_eq!(makers[0].status(), SpotOrderStatus::Filled);
    assert_eq!(makers[0].version, 2);
    Ok(())
}

#[test]
fn given_triggered_conditional_trade_when_settlement_voucher_is_derived_then_principal_and_fee_legs_are_linked()
-> Result<(), SpotOrderV2BehaviorError> {
    // Given: 条件买单触发后已经和 maker sell 形成成交事实。
    let mut taker = trigger_pending_buy_order();
    trigger_as_active(&mut taker)?;
    let mut makers = vec![maker_sell_order("maker-settlement", 1, 90)];
    let trade = taker
        .match_with_makers(makers.as_mut_slice(), match_input("match-settlement"))?
        .trades
        .pop()
        .ok_or(SpotOrderV2BehaviorError::OrderNotMatchable)?;

    // When: 从成交事实派生包含 principal 与 fee 的 settlement voucher。
    let voucher = trade
        .derive_spot_settlement_transfer_voucher_with_fees(
            "voucher-triggered-conditional".to_string(),
            "settle-triggered-conditional".to_string(),
            "BTC",
            "USDT",
            "fee-account".to_string(),
        )
        .ok_or(SpotOrderV2BehaviorError::ArithmeticOverflow)?;

    // Then: voucher 连接到触发成交，并表达买卖双方 principal 与 fee 单据腿。
    assert_eq!(voucher.settlement_kind(), SettlementKind::Spot);
    assert_eq!(voucher.trade_id(), "match-settlement-1");
    assert_eq!(
        voucher.amount_received_by_for_purpose(
            "buyer-account",
            SettlementTransferPurpose::SpotBuyerReceiveBase
        ),
        Some(1)
    );
    assert_eq!(
        voucher.amount_sent_by_for_purpose(
            "buyer-account",
            SettlementTransferPurpose::SpotBuyerPayQuote
        ),
        Some(90)
    );
    assert_eq!(
        voucher.amount_received_by_for_purpose(
            "seller-account-maker-settlement",
            SettlementTransferPurpose::SpotSellerReceiveQuote
        ),
        Some(90)
    );
    assert_eq!(
        voucher.amount_sent_by_for_purpose(
            "seller-account-maker-settlement",
            SettlementTransferPurpose::SpotSellerDeliverBase
        ),
        Some(1)
    );
    assert_eq!(voucher.fee_amount_paid_by("buyer-account"), Some(trade.taker_fee));
    assert_eq!(
        voucher.fee_amount_paid_by("seller-account-maker-settlement"),
        Some(trade.maker_fee)
    );
    assert_eq!(voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee).len(), 2);
    Ok(())
}
