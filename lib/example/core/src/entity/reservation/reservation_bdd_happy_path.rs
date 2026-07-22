use super::{
    Reservation, ReservationCloseReason, ReservationKind, ReservationMarketKind, ReservationStatus,
};

fn reservation() -> Reservation {
    Reservation::new(
        "reservation:happy-path".to_string(),
        "trader-1".to_string(),
        "order-1".to_string(),
        ReservationMarketKind::Spot,
        ReservationKind::SpotBuyQuote,
        "USDT".to_string(),
        200,
    )
    .unwrap()
}

#[test]
fn reservation_new_initializes_active_snapshot() {
    let reservation = reservation();

    assert_eq!(reservation.reservation_id, "reservation:happy-path");
    assert_eq!(reservation.owner_account_id, "trader-1");
    assert_eq!(reservation.caused_by_order_id, "order-1");
    assert_eq!(reservation.market_kind, ReservationMarketKind::Spot);
    assert_eq!(reservation.reservation_kind, ReservationKind::SpotBuyQuote);
    assert_eq!(reservation.asset_id, "USDT");
    assert_eq!(reservation.original_amount, 200);
    assert_eq!(reservation.consumed_amount, 0);
    assert_eq!(reservation.released_amount, 0);
    assert_eq!(reservation.remaining_amount, 200);
    assert_eq!(reservation.status, ReservationStatus::Active);
    assert_eq!(reservation.close_reason, None);
    assert_eq!(reservation.version, 1);
    assert!(reservation.has_consistent_amounts());
}

#[test]
fn reservation_consume_updates_amounts_and_version() {
    let mut after = reservation();
    after.consume(80, None).unwrap();

    assert_eq!(after.consumed_amount, 80);
    assert_eq!(after.released_amount, 0);
    assert_eq!(after.remaining_amount, 120);
    assert_eq!(after.status, ReservationStatus::Active);
    assert_eq!(after.close_reason, None);
    assert_eq!(after.version, 2);
    assert!(after.has_consistent_amounts());
}

#[test]
fn reservation_release_closes_remaining_amount() {
    let mut after = reservation();
    after.consume(80, None).unwrap();
    after.release(120, Some(ReservationCloseReason::IocRemainderCanceled)).unwrap();

    assert_eq!(after.consumed_amount, 80);
    assert_eq!(after.released_amount, 120);
    assert_eq!(after.remaining_amount, 0);
    assert_eq!(after.status, ReservationStatus::ClosedMixed);
    assert_eq!(after.close_reason, Some(ReservationCloseReason::IocRemainderCanceled));
    assert_eq!(after.version, 3);
    assert!(after.has_consistent_amounts());
}
