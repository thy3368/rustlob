use super::*;
use crate::entity::{Reservation, ReservationCloseReason, ReservationKind, ReservationMarketKind};

fn trade(
    trade_id: &str,
    taker_side: SpotOrderSide,
    taker_account_id: &str,
    maker_account_id: &str,
    price: u64,
    qty: u64,
) -> SpotTrade {
    SpotTrade::new(
        trade_id.to_string(),
        "match-1".to_string(),
        10_001,
        "BTCUSDT".to_string(),
        format!("{trade_id}-taker"),
        format!("{trade_id}-maker"),
        taker_account_id.to_string(),
        maker_account_id.to_string(),
        taker_side,
        price,
        qty,
        0,
        0,
    )
}

fn reservation_for_trade_buyer_quote(
    trade: &SpotTrade,
    quote_amount: u64,
) -> Result<AssetReservation, String> {
    Reservation::new(
        format!("reservation:{}", trade.buyer_order_id()),
        trade.buyer_account_id().to_string(),
        trade.buyer_order_id().to_string(),
        ReservationMarketKind::Spot,
        ReservationKind::SpotBuyQuote,
        "USDT".to_string(),
        quote_amount,
    )
    .map_err(|err| err.to_string())
}

fn reservation_for_trade_seller_base(trade: &SpotTrade) -> Result<AssetReservation, String> {
    Reservation::new(
        format!("reservation:{}", trade.seller_order_id()),
        trade.seller_account_id().to_string(),
        trade.seller_order_id().to_string(),
        ReservationMarketKind::Spot,
        ReservationKind::SpotSellBase,
        "BTC".to_string(),
        trade.qty,
    )
    .map_err(|err| err.to_string())
}

#[test]
fn buyer_quote_reservation_consumed_derivation_returns_notional_amount() -> Result<(), String> {
    let trade = trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2);
    let after = reservation_for_trade_buyer_quote(&trade, 200)?
        .consume(200, Some(ReservationCloseReason::Filled))
        .map_err(|err| err.to_string())?;

    let consumed = trade
        .derive_buyer_quote_reservation_consumed(
            "reservation-consumed:buyer".to_string(),
            &after,
            "USDT",
        )
        .map_err(|err| err.to_string())?;

    assert_eq!(consumed.reservation_id, after.reservation_id);
    assert_eq!(consumed.owner_account_id, "buyer");
    assert_eq!(consumed.asset_id, "USDT");
    assert_eq!(consumed.amount, 200);
    assert_eq!(consumed.caused_by_ref_id, "trade-1");
    assert_eq!(consumed.remaining_amount_after, 0);

    Ok(())
}

#[test]
fn seller_base_reservation_consumed_derivation_returns_base_amount() -> Result<(), String> {
    let trade = trade("trade-1", SpotOrderSide::Sell, "seller", "buyer", 100, 2);
    let after = reservation_for_trade_seller_base(&trade)?
        .consume(2, Some(ReservationCloseReason::Filled))
        .map_err(|err| err.to_string())?;

    let consumed = trade
        .derive_seller_base_reservation_consumed(
            "reservation-consumed:seller".to_string(),
            &after,
            "BTC",
        )
        .map_err(|err| err.to_string())?;

    assert_eq!(consumed.reservation_id, after.reservation_id);
    assert_eq!(consumed.owner_account_id, "seller");
    assert_eq!(consumed.asset_id, "BTC");
    assert_eq!(consumed.amount, 2);
    assert_eq!(consumed.caused_by_ref_id, "trade-1");
    assert_eq!(consumed.remaining_amount_after, 0);

    Ok(())
}

#[test]
fn buyer_quote_reservation_consumed_derivation_rejects_notional_overflow() -> Result<(), String> {
    let trade = trade("trade-overflow", SpotOrderSide::Buy, "buyer", "seller", u64::MAX, 2);
    let after = Reservation::new(
        "reservation:trade-overflow-taker".to_string(),
        "buyer".to_string(),
        "trade-overflow-taker".to_string(),
        ReservationMarketKind::Spot,
        ReservationKind::SpotBuyQuote,
        "USDT".to_string(),
        u64::MAX,
    )
    .map_err(|err| err.to_string())?
    .consume(1, None)
    .map_err(|err| err.to_string())?;

    assert_eq!(
        trade
            .derive_buyer_quote_reservation_consumed(
                "reservation-consumed:overflow".to_string(),
                &after,
                "USDT",
            )
            .map(|_| ()),
        Err(SpotTradeReservationConsumedDerivationError::ArithmeticOverflow)
    );

    Ok(())
}

#[test]
fn buyer_quote_reservation_consumed_derivation_rejects_mismatched_reservation() -> Result<(), String>
{
    let trade = trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2);
    let after = reservation_for_trade_buyer_quote(&trade, 200)?
        .consume(200, Some(ReservationCloseReason::Filled))
        .map_err(|err| err.to_string())?;

    let cases = [
        (
            {
                let mut reservation = after.clone();
                reservation.owner_account_id = "other-buyer".to_string();
                reservation
            },
            SpotTradeReservationConsumedDerivationError::ReservationOwnerMismatch,
        ),
        (
            {
                let mut reservation = after.clone();
                reservation.caused_by_order_id = "other-order".to_string();
                reservation
            },
            SpotTradeReservationConsumedDerivationError::ReservationOrderMismatch,
        ),
        (
            {
                let mut reservation = after.clone();
                reservation.asset_id = "USDC".to_string();
                reservation
            },
            SpotTradeReservationConsumedDerivationError::ReservationAssetMismatch,
        ),
        (
            {
                let mut reservation = after.clone();
                reservation.reservation_kind = ReservationKind::SpotSellBase;
                reservation
            },
            SpotTradeReservationConsumedDerivationError::ReservationKindMismatch,
        ),
        (
            {
                let mut reservation = after;
                reservation.market_kind = ReservationMarketKind::Perp;
                reservation
            },
            SpotTradeReservationConsumedDerivationError::ReservationMarketMismatch,
        ),
    ];

    for (reservation, expected_error) in cases {
        assert_eq!(
            trade
                .derive_buyer_quote_reservation_consumed(
                    "reservation-consumed:mismatch".to_string(),
                    &reservation,
                    "USDT",
                )
                .map(|_| ()),
            Err(expected_error)
        );
    }

    Ok(())
}

#[test]
fn seller_base_reservation_consumed_derivation_rejects_mismatched_reservation() -> Result<(), String>
{
    let trade = trade("trade-1", SpotOrderSide::Sell, "seller", "buyer", 100, 2);
    let mut after = reservation_for_trade_seller_base(&trade)?
        .consume(2, Some(ReservationCloseReason::Filled))
        .map_err(|err| err.to_string())?;
    after.owner_account_id = "other-seller".to_string();

    assert_eq!(
        trade
            .derive_seller_base_reservation_consumed(
                "reservation-consumed:seller-mismatch".to_string(),
                &after,
                "BTC",
            )
            .map(|_| ()),
        Err(SpotTradeReservationConsumedDerivationError::ReservationOwnerMismatch)
    );

    Ok(())
}
