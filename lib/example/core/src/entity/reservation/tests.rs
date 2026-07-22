use common_entity::{Entity, FinancialClassification};

use super::*;

fn active_reservation() -> Reservation {
    Reservation::new(
        "reservation:1".to_string(),
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
fn reservation_kind_labels_distinguish_spot_principal_and_fee() {
    assert_eq!(ReservationKind::SpotBuyQuote.as_str(), "spot_buy_quote");
    assert_eq!(ReservationKind::SpotSellBase.as_str(), "spot_sell_base");
    assert_eq!(ReservationKind::SpotBuyFeeQuote.as_str(), "spot_buy_fee_quote");
    assert_eq!(ReservationKind::SpotSellFeeQuote.as_str(), "spot_sell_fee_quote");
}

#[test]
fn constructor_sets_active_amounts() {
    let reservation = active_reservation();

    assert_eq!(reservation.remaining_amount, 200);
    assert_eq!(reservation.status, ReservationStatus::Active);
    assert_eq!(reservation.version, 1);
    assert!(reservation.has_consistent_amounts());
}

#[test]
fn partial_consume_keeps_reservation_active() {
    let mut after = active_reservation();
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
fn full_consume_closes_as_exhausted() {
    let mut after = active_reservation();
    after.consume(200, Some(ReservationCloseReason::Filled)).unwrap();

    assert_eq!(after.consumed_amount, 200);
    assert_eq!(after.released_amount, 0);
    assert_eq!(after.remaining_amount, 0);
    assert_eq!(after.status, ReservationStatus::ExhaustedByConsume);
    assert_eq!(after.close_reason, Some(ReservationCloseReason::Filled));
    assert_eq!(after.version, 2);
    assert!(after.has_consistent_amounts());
}

#[test]
fn release_tail_closes_reservation() {
    let mut after = active_reservation();
    after.release(200, Some(ReservationCloseReason::Canceled)).unwrap();

    assert_eq!(after.consumed_amount, 0);
    assert_eq!(after.released_amount, 200);
    assert_eq!(after.remaining_amount, 0);
    assert_eq!(after.status, ReservationStatus::ClosedByRelease);
    assert_eq!(after.close_reason, Some(ReservationCloseReason::Canceled));
    assert_eq!(after.version, 2);
    assert!(after.has_consistent_amounts());
}

#[test]
fn mixed_lifecycle_versions_form_replayable_steps() {
    let initial = active_reservation();
    let mut after_consume = initial.clone();
    after_consume.consume(80, None).unwrap();
    let mut after_release = after_consume.clone();
    after_release.release(120, Some(ReservationCloseReason::IocRemainderCanceled)).unwrap();

    let consume_event = after_consume.track_update_event_from(&initial).unwrap();
    let release_event = after_release.track_update_event_from(&after_consume).unwrap();

    assert_eq!(consume_event.old_version, 1);
    assert_eq!(consume_event.new_version, 2);
    assert_eq!(release_event.old_version, 2);
    assert_eq!(release_event.new_version, 3);
    assert_eq!(after_release.status, ReservationStatus::ClosedMixed);
}

#[test]
fn invalid_lifecycle_cases_are_reported_by_reservation_error() {
    let mut active = active_reservation();

    assert_eq!(active.consume(0, None), Err(ReservationError::InvalidAmount));
    assert_eq!(active.consume(201, None), Err(ReservationError::AmountExceedsRemaining));
    assert_eq!(active.release(200, None), Err(ReservationError::MissingCloseReason));

    active.consume(200, Some(ReservationCloseReason::Filled)).unwrap();
    let mut closed = active;
    assert_eq!(closed.consume(1, None), Err(ReservationError::AlreadyClosed));
    assert_eq!(
        closed.release(1, Some(ReservationCloseReason::Canceled)),
        Err(ReservationError::AlreadyClosed)
    );
}

#[test]
fn reservation_uses_business_voucher_financial_classification() {
    assert_eq!(Reservation::financial_classification(), FinancialClassification::BusinessVoucher);
}
