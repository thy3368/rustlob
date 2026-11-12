use common_entity::Fsm;
use rstest::rstest;
use rust_decimal::Decimal;

use super::{
    MatchSpotOrderV7Input, MatchSpotOrderV7Outcome, SpotOrderV7, SpotOrderV7Event,
    SpotOrderV7FreezeRequirement, SpotOrderV7HoldAdjustment, SpotOrderV7Output,
};
use crate::account::coin::Coin;
use crate::entity::common::common_order::OrderBase;
use crate::{OrderSide, OrderTif, SpotOrderStatus};

const EVENT_TIMESTAMP: u64 = 42;

#[derive(Clone, Copy, Debug)]
enum EventKind {
    Activate,
    Match,
    ReplaceOrFill,
    Modify,
    Cancel,
}

fn order_record(
    id: u64,
    side: OrderSide,
    orig_size: i64,
    remaining_size: i64,
    status: SpotOrderStatus,
    version: u64,
) -> OrderBase {
    OrderBase {
        order_id: id,
        side,
        limit_px: Decimal::from(100),
        size: Decimal::from(remaining_size),
        orig_size: Decimal::from(orig_size),
        status,
        version,
        ..OrderBase::default()
    }
}

fn order(
    id: u64,
    side: OrderSide,
    orig_size: i64,
    remaining_size: i64,
    status: SpotOrderStatus,
    version: u64,
) -> SpotOrderV7 {
    SpotOrderV7::from_record(order_record(id, side, orig_size, remaining_size, status, version))
        .unwrap()
}

fn taker(status: SpotOrderStatus) -> SpotOrderV7 {
    taker_with_tif(status, OrderTif::Gtc)
}

fn taker_with_tif(status: SpotOrderStatus, tif: OrderTif) -> SpotOrderV7 {
    let (orig_size, remaining_size) =
        if status == SpotOrderStatus::PartiallyFilled { (2, 1) } else { (1, 1) };
    let mut record = order_record(1, OrderSide::Buy, orig_size, remaining_size, status, 7);
    record.tif = tif;
    SpotOrderV7::from_record(record).unwrap()
}

fn sized_taker(tif: OrderTif, size: i64) -> SpotOrderV7 {
    let mut record = order_record(1, OrderSide::Buy, size, size, SpotOrderStatus::Pending, 7);
    record.tif = tif;
    SpotOrderV7::from_record(record).unwrap()
}

fn maker(id: u64, size: i64) -> SpotOrderV7 {
    order(id, OrderSide::Sell, size, size, SpotOrderStatus::Open, 3)
}

fn input() -> MatchSpotOrderV7Input {
    MatchSpotOrderV7Input {
        trade_id_start: 1_000,
        maker_fee_bps: 0,
        taker_fee_bps: 0,
        timestamp: EVENT_TIMESTAMP,
    }
}

fn event(kind: EventKind) -> SpotOrderV7Event {
    match kind {
        EventKind::Activate => SpotOrderV7Event::Activate { timestamp: EVENT_TIMESTAMP },
        EventKind::Match => SpotOrderV7Event::Match {
            makers: Vec::new(),
            input: input(),
            timestamp: EVENT_TIMESTAMP,
        },
        EventKind::ReplaceOrFill => SpotOrderV7Event::ReplaceOrFill {
            price: Decimal::from(101),
            size: Decimal::from(2),
            makers: Vec::new(),
            input: input(),
            timestamp: EVENT_TIMESTAMP,
        },
        EventKind::Modify => SpotOrderV7Event::Modify {
            price: Decimal::from(101),
            size: Decimal::from(2),
            tif: None,
            cloid: None,
            timestamp: EVENT_TIMESTAMP,
        },
        EventKind::Cancel => SpotOrderV7Event::Cancel { timestamp: EVENT_TIMESTAMP },
    }
}

#[rstest]
#[case(SpotOrderStatus::Pending, EventKind::Modify)]
#[case(SpotOrderStatus::Pending, EventKind::Cancel)]
#[case(SpotOrderStatus::Open, EventKind::Match)]
#[case(SpotOrderStatus::Open, EventKind::ReplaceOrFill)]
#[case(SpotOrderStatus::Open, EventKind::Modify)]
#[case(SpotOrderStatus::Open, EventKind::Cancel)]
#[case(SpotOrderStatus::PartiallyFilled, EventKind::Match)]
#[case(SpotOrderStatus::PartiallyFilled, EventKind::ReplaceOrFill)]
#[case(SpotOrderStatus::PartiallyFilled, EventKind::Modify)]
#[case(SpotOrderStatus::PartiallyFilled, EventKind::Cancel)]
fn active_state_event_matrix_accepts_every_event(
    #[case] initial_status: SpotOrderStatus,
    #[case] event_kind: EventKind,
) {
    let mut subject = taker(initial_status);

    let (state, output) = subject.transition(event(event_kind)).unwrap();

    match event_kind {
        EventKind::Activate => unreachable!(),
        EventKind::Match | EventKind::ReplaceOrFill => {
            assert_eq!(state, initial_status);
            assert!(matches!(
                (event_kind, output),
                (
                    EventKind::Match,
                    SpotOrderV7Output::Matched(super::MatchSpotOrderV7Output {
                        outcome: MatchSpotOrderV7Outcome::Resting { .. },
                    })
                ) | (
                    EventKind::ReplaceOrFill,
                    SpotOrderV7Output::ReplacedOrFilled(super::ReplaceOrFillSpotOrderV7Output {
                        outcome: MatchSpotOrderV7Outcome::Resting { .. },
                    })
                )
            ));
        }
        EventKind::Modify => {
            assert_eq!(state, initial_status);
            assert!(matches!(output, SpotOrderV7Output::Modified(_)));
        }
        EventKind::Cancel => {
            assert_eq!(state, SpotOrderStatus::Canceled);
            assert!(matches!(output, SpotOrderV7Output::Canceled(_)));
        }
    }
}

#[rstest]
#[case(SpotOrderStatus::Filled, EventKind::Match)]
#[case(SpotOrderStatus::Filled, EventKind::ReplaceOrFill)]
#[case(SpotOrderStatus::Filled, EventKind::Modify)]
#[case(SpotOrderStatus::Filled, EventKind::Cancel)]
#[case(SpotOrderStatus::Canceled, EventKind::Match)]
#[case(SpotOrderStatus::Canceled, EventKind::ReplaceOrFill)]
#[case(SpotOrderStatus::Canceled, EventKind::Modify)]
#[case(SpotOrderStatus::Canceled, EventKind::Cancel)]
#[case(SpotOrderStatus::Rejected, EventKind::Match)]
#[case(SpotOrderStatus::Rejected, EventKind::ReplaceOrFill)]
#[case(SpotOrderStatus::Rejected, EventKind::Modify)]
#[case(SpotOrderStatus::Rejected, EventKind::Cancel)]
#[case(SpotOrderStatus::Filled, EventKind::Activate)]
#[case(SpotOrderStatus::Canceled, EventKind::Activate)]
#[case(SpotOrderStatus::Rejected, EventKind::Activate)]
fn terminal_state_event_matrix_rejects_without_mutation(
    #[case] terminal_status: SpotOrderStatus,
    #[case] event_kind: EventKind,
) {
    let mut subject = taker(terminal_status);
    let before = subject.clone();

    assert!(subject.transition(event(event_kind)).is_err());
    assert_eq!(subject, before);
}

#[rstest]
#[case(SpotOrderStatus::Pending)]
#[case(SpotOrderStatus::Open)]
#[case(SpotOrderStatus::PartiallyFilled)]
fn modify_preserves_status_and_updates_order_facts(#[case] status: SpotOrderStatus) {
    let mut subject = taker(status);

    let (next_status, output) = subject
        .transition(SpotOrderV7Event::Modify {
            price: Decimal::from(105),
            size: Decimal::from(4),
            tif: None,
            cloid: None,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();

    assert_eq!(next_status, status);
    let SpotOrderV7Output::Modified(output) = output else { panic!("unexpected output") };
    assert_eq!(output.order_after, subject);
    assert_eq!(subject.order_base().limit_px, Decimal::from(105));
    assert_eq!(subject.order_base().orig_size, Decimal::from(4));
    let expected_remaining = if status == SpotOrderStatus::PartiallyFilled {
        Decimal::from(3)
    } else {
        Decimal::from(4)
    };
    assert_eq!(subject.order_base().size, expected_remaining);
    assert_eq!(subject.order_base().version, 8);
    assert_eq!(subject.order_base().updated_at, EVENT_TIMESTAMP);
}

#[rstest]
#[case(OrderSide::Buy, Coin::USDT)]
#[case(OrderSide::Sell, Coin::BTC)]
fn pending_modify_reports_unchanged_with_held_currency(
    #[case] side: OrderSide,
    #[case] expected_currency: Coin,
) {
    let mut subject = order(1, side, 2, 2, SpotOrderStatus::Pending, 7);

    let (_, output) = subject
        .transition(SpotOrderV7Event::Modify {
            price: Decimal::from(105),
            size: Decimal::from(3),
            tif: None,
            cloid: None,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();

    let SpotOrderV7Output::Modified(output) = output else { panic!("unexpected output") };
    assert_eq!(
        output.hold_adjustment,
        SpotOrderV7HoldAdjustment::Unchanged { currency: expected_currency }
    );
}

#[test]
fn active_buy_modify_reports_increase_using_remaining_size_and_unused_fee() {
    let mut record = order_record(1, OrderSide::Buy, 3, 1, SpotOrderStatus::PartiallyFilled, 7);
    record.buy_quote_fee_reserve_limit = Decimal::from(5);
    record.buy_quote_fee_reserve_consumed = Decimal::from(2);
    let mut subject = SpotOrderV7::from_record(record).unwrap();

    let (_, output) = subject
        .transition(SpotOrderV7Event::Modify {
            price: Decimal::from(110),
            size: Decimal::from(4),
            tif: None,
            cloid: None,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();

    let SpotOrderV7Output::Modified(output) = output else { panic!("unexpected output") };
    let SpotOrderV7HoldAdjustment::Increase { amount } = output.hold_adjustment else {
        panic!("expected increase")
    };
    assert_eq!(amount.currency(), Coin::USDT);
    assert_eq!(amount.amount(), Decimal::from(120));
}

#[test]
fn active_buy_modify_reports_decrease_and_unchanged_deltas() {
    let mut record = order_record(1, OrderSide::Buy, 2, 2, SpotOrderStatus::Open, 7);
    record.buy_quote_fee_reserve_limit = Decimal::from(5);
    record.buy_quote_fee_reserve_consumed = Decimal::from(2);

    let mut decreased = SpotOrderV7::from_record(record.clone()).unwrap();
    let (_, output) = decreased
        .transition(SpotOrderV7Event::Modify {
            price: Decimal::from(90),
            size: Decimal::from(2),
            tif: None,
            cloid: None,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();
    let SpotOrderV7Output::Modified(output) = output else { panic!("unexpected output") };
    let SpotOrderV7HoldAdjustment::Decrease { amount } = output.hold_adjustment else {
        panic!("expected decrease")
    };
    assert_eq!(amount.amount(), Decimal::from(20));

    let mut unchanged = SpotOrderV7::from_record(record).unwrap();
    let (_, output) = unchanged
        .transition(SpotOrderV7Event::Modify {
            price: Decimal::from(50),
            size: Decimal::from(4),
            tif: None,
            cloid: None,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();
    let SpotOrderV7Output::Modified(output) = output else { panic!("unexpected output") };
    assert_eq!(
        output.hold_adjustment,
        SpotOrderV7HoldAdjustment::Unchanged { currency: Coin::USDT }
    );
}

#[rstest]
#[case(4, SpotOrderV7HoldAdjustment::Increase { amount: crate::account::coin::CoinAmount::new(Coin::BTC, Decimal::ONE).unwrap() })]
#[case(2, SpotOrderV7HoldAdjustment::Decrease { amount: crate::account::coin::CoinAmount::new(Coin::BTC, Decimal::ONE).unwrap() })]
fn active_sell_modify_reports_remaining_base_delta(
    #[case] new_size: i64,
    #[case] expected: SpotOrderV7HoldAdjustment,
) {
    let mut subject = order(1, OrderSide::Sell, 3, 3, SpotOrderStatus::Open, 7);

    let (_, output) = subject
        .transition(SpotOrderV7Event::Modify {
            price: Decimal::ONE,
            size: Decimal::from(new_size),
            tif: None,
            cloid: None,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();

    let SpotOrderV7Output::Modified(output) = output else { panic!("unexpected output") };
    assert_eq!(output.hold_adjustment, expected);
}

#[test]
fn hold_requirement_arithmetic_error_preserves_complete_order() {
    let mut subject = order(1, OrderSide::Buy, 1, 1, SpotOrderStatus::Open, 7);
    let before = subject.clone();

    let result = subject.transition(SpotOrderV7Event::Modify {
        price: Decimal::MAX,
        size: Decimal::from(2),
        tif: None,
        cloid: None,
        timestamp: EVENT_TIMESTAMP,
    });

    assert!(result.is_err());
    assert_eq!(subject, before);
}

#[rstest]
#[case(SpotOrderStatus::Pending)]
#[case(SpotOrderStatus::Open)]
#[case(SpotOrderStatus::PartiallyFilled)]
fn cancel_enters_canceled_and_updates_metadata(#[case] status: SpotOrderStatus) {
    let mut subject = taker(status);

    let (next_status, output) =
        subject.transition(SpotOrderV7Event::Cancel { timestamp: EVENT_TIMESTAMP }).unwrap();

    assert_eq!(next_status, SpotOrderStatus::Canceled);
    let SpotOrderV7Output::Canceled(output) = output else { panic!("unexpected output") };
    assert_eq!(output.order_after, subject);
    if status == SpotOrderStatus::Pending {
        assert_eq!(output.release_requirement.amount(), Decimal::ZERO);
    } else {
        assert_eq!(output.release_requirement, subject.release_requirement().unwrap());
    }
    assert_eq!(subject.order_base().version, 8);
    assert_eq!(subject.order_base().updated_at, EVENT_TIMESTAMP);
}

#[rstest]
#[case(OrderTif::Gtc, 1, Vec::new(), SpotOrderStatus::Open)]
#[case(OrderTif::Gtc, 2, vec![1], SpotOrderStatus::PartiallyFilled)]
#[case(OrderTif::Gtc, 1, vec![1], SpotOrderStatus::Filled)]
#[case(OrderTif::Ioc, 1, Vec::new(), SpotOrderStatus::Rejected)]
#[case(OrderTif::Ioc, 2, vec![1], SpotOrderStatus::Canceled)]
#[case(OrderTif::Ioc, 1, vec![1], SpotOrderStatus::Filled)]
fn gtc_and_ioc_match_matrix(
    #[case] tif: OrderTif,
    #[case] taker_size: i64,
    #[case] maker_sizes: Vec<i64>,
    #[case] expected_status: SpotOrderStatus,
) {
    let mut subject = sized_taker(tif, taker_size);
    subject.transition(SpotOrderV7Event::Activate { timestamp: EVENT_TIMESTAMP - 1 }).unwrap();
    let makers =
        maker_sizes.into_iter().enumerate().map(|(i, size)| maker(2 + i as u64, size)).collect();
    let output = subject.match_with_input(makers, input()).unwrap();
    assert_eq!(subject.status(), expected_status);
    match &output {
        MatchSpotOrderV7Outcome::Filled { release_requirement, .. }
        | MatchSpotOrderV7Outcome::CanceledAfterPartialFill { release_requirement, .. }
        | MatchSpotOrderV7Outcome::Rejected { release_requirement, .. } => {
            assert_eq!(*release_requirement, subject.release_requirement().unwrap());
        }
        MatchSpotOrderV7Outcome::Resting { .. }
        | MatchSpotOrderV7Outcome::PartiallyFilled { .. } => {}
    }
    assert!(matches!(
        (expected_status, output),
        (SpotOrderStatus::Open, MatchSpotOrderV7Outcome::Resting { .. })
            | (SpotOrderStatus::PartiallyFilled, MatchSpotOrderV7Outcome::PartiallyFilled { .. })
            | (SpotOrderStatus::Filled, MatchSpotOrderV7Outcome::Filled { .. })
            | (SpotOrderStatus::Canceled, MatchSpotOrderV7Outcome::CanceledAfterPartialFill { .. })
            | (SpotOrderStatus::Rejected, MatchSpotOrderV7Outcome::Rejected { .. })
    ));
}

#[test]
fn buy_activation_preserves_freeze_breakdown() {
    let mut record = order_record(1, OrderSide::Buy, 2, 2, SpotOrderStatus::Pending, 7);
    record.buy_quote_fee_reserve_limit = Decimal::from(3);
    let mut subject = SpotOrderV7::from_record(record).unwrap();

    let (status, output) =
        subject.transition(SpotOrderV7Event::Activate { timestamp: EVENT_TIMESTAMP }).unwrap();

    assert_eq!(status, SpotOrderStatus::Open);
    let SpotOrderV7Output::Activated(output) = output else { panic!("unexpected output") };
    let SpotOrderV7FreezeRequirement::Buy { quote_principal, quote_fee_reserve, total_quote } =
        output.freeze_requirement
    else {
        panic!("expected buy requirement")
    };
    assert_eq!(quote_principal.amount(), Decimal::from(200));
    assert_eq!(quote_fee_reserve.amount(), Decimal::from(3));
    assert_eq!(total_quote.amount(), Decimal::from(203));
}

#[test]
fn sell_activation_freezes_only_base_quantity() {
    let mut subject = order(1, OrderSide::Sell, 2, 2, SpotOrderStatus::Pending, 7);
    let (_, output) =
        subject.transition(SpotOrderV7Event::Activate { timestamp: EVENT_TIMESTAMP }).unwrap();
    let SpotOrderV7Output::Activated(output) = output else { panic!("unexpected output") };
    let SpotOrderV7FreezeRequirement::Sell { base_quantity } = output.freeze_requirement else {
        panic!("expected sell requirement")
    };
    assert_eq!(base_quantity.amount(), Decimal::from(2));
}

#[test]
fn pending_cannot_match_and_cancel_releases_zero() {
    let mut subject = taker(SpotOrderStatus::Pending);
    let before = subject.clone();
    assert!(subject.transition(event(EventKind::Match)).is_err());
    assert_eq!(subject, before);

    let (_, output) =
        subject.transition(SpotOrderV7Event::Cancel { timestamp: EVENT_TIMESTAMP }).unwrap();
    let SpotOrderV7Output::Canceled(output) = output else { panic!("unexpected output") };
    assert_eq!(output.release_requirement.amount(), Decimal::ZERO);
}

#[rstest]
#[case(Vec::new(), SpotOrderStatus::Open)]
#[case(vec![1], SpotOrderStatus::Rejected)]
#[case(vec![2], SpotOrderStatus::Rejected)]
fn alo_match_matrix(#[case] maker_sizes: Vec<i64>, #[case] expected_status: SpotOrderStatus) {
    let mut subject = sized_taker(OrderTif::Alo, 2);
    subject.transition(SpotOrderV7Event::Activate { timestamp: EVENT_TIMESTAMP - 1 }).unwrap();
    let expected_maker_sizes = maker_sizes.clone();
    let makers = maker_sizes
        .into_iter()
        .enumerate()
        .map(|(i, size)| maker(2 + i as u64, size))
        .collect::<Vec<_>>();
    let makers_before = makers.clone();
    let output = subject.match_with_input(makers, input()).unwrap();
    assert_eq!(subject.status(), expected_status);
    assert!(matches!(
        output,
        MatchSpotOrderV7Outcome::Resting { .. } | MatchSpotOrderV7Outcome::Rejected { .. }
    ));
    if expected_status == SpotOrderStatus::Rejected {
        for (maker, expected_size) in makers_before.into_iter().zip(expected_maker_sizes) {
            assert_eq!(maker.status(), SpotOrderStatus::Open);
            assert_eq!(maker.order_base().size, Decimal::from(expected_size));
        }
    }
}

#[test]
fn replace_or_fill_replaces_and_matches_in_one_version() {
    let mut subject = order(1, OrderSide::Buy, 2, 2, SpotOrderStatus::Open, 7);

    let output = subject
        .replace_or_fill(
            Decimal::from(101),
            Decimal::from(3),
            vec![maker(2, 1)],
            input(),
            EVENT_TIMESTAMP,
        )
        .unwrap();

    let MatchSpotOrderV7Outcome::PartiallyFilled { taker_order_after, created_trades, .. } = output
    else {
        panic!("expected partially filled outcome");
    };
    assert_eq!(taker_order_after, subject);
    assert_eq!(subject.order_base().limit_px, Decimal::from(101));
    assert_eq!(subject.order_base().orig_size, Decimal::from(3));
    assert_eq!(subject.order_base().size, Decimal::from(2));
    assert_eq!(subject.order_base().version, 8);
    assert_eq!(subject.order_base().updated_at, EVENT_TIMESTAMP);
    assert_eq!(created_trades.len(), 1);
}

#[rstest]
#[case(Decimal::ZERO, Decimal::ONE)]
#[case(Decimal::from(-1), Decimal::ONE)]
#[case(Decimal::ONE, Decimal::ZERO)]
#[case(Decimal::ONE, Decimal::from(-1))]
#[case(Decimal::ONE, Decimal::ONE)]
fn invalid_modify_preserves_complete_order(#[case] price: Decimal, #[case] size: Decimal) {
    let mut subject = order(1, OrderSide::Buy, 3, 1, SpotOrderStatus::PartiallyFilled, 7);
    let before = subject.clone();

    assert!(subject.modify(price, size, EVENT_TIMESTAMP).is_err());
    assert_eq!(subject, before);
}

#[rstest]
#[case(OrderSide::Buy, SpotOrderStatus::Open, 2, 100)]
#[case(OrderSide::Sell, SpotOrderStatus::Pending, 2, 100)]
#[case(OrderSide::Sell, SpotOrderStatus::Filled, 2, 100)]
#[case(OrderSide::Sell, SpotOrderStatus::Open, 1, 100)]
#[case(OrderSide::Sell, SpotOrderStatus::Open, 2, 102)]
fn invalid_replace_maker_preserves_complete_order(
    #[case] maker_side: OrderSide,
    #[case] maker_status: SpotOrderStatus,
    #[case] maker_id: u64,
    #[case] maker_price: i64,
) {
    let mut subject = taker(SpotOrderStatus::Open);
    let before = subject.clone();
    let mut maker_record = order_record(maker_id, maker_side, 1, 1, maker_status, 3);
    maker_record.limit_px = Decimal::from(maker_price);
    let invalid_maker = SpotOrderV7::from_record(maker_record).unwrap();

    assert!(
        subject
            .replace_or_fill(
                Decimal::from(101),
                Decimal::ONE,
                vec![invalid_maker],
                input(),
                EVENT_TIMESTAMP,
            )
            .is_err()
    );
    assert_eq!(subject, before);
}

#[test]
fn insufficient_fee_reserve_preserves_complete_order() {
    let mut subject = taker(SpotOrderStatus::Pending);
    let before = subject.clone();
    let mut costly_input = input();
    costly_input.taker_fee_bps = 1;

    assert!(subject.match_with_input(vec![maker(2, 1)], costly_input).is_err());
    assert_eq!(subject, before);
}

#[test]
fn role_match_events_update_only_the_receiving_order_once() {
    let mut taker_record = order_record(1, OrderSide::Buy, 3, 3, SpotOrderStatus::Open, 7);
    taker_record.buy_quote_fee_reserve_limit = Decimal::from(5);
    let mut taker = SpotOrderV7::from_record(taker_record).unwrap();
    let (_, output) = taker
        .transition(SpotOrderV7Event::MatchedAsTaker {
            filled_size: Decimal::from(2),
            quote_fee: Decimal::from(2),
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();
    assert_eq!(taker.status(), SpotOrderStatus::PartiallyFilled);
    assert_eq!(taker.order_base().size, Decimal::ONE);
    assert_eq!(taker.order_base().buy_quote_fee_reserve_consumed, Decimal::from(2));
    assert_eq!(taker.order_base().version, 8);
    let SpotOrderV7Output::MatchedAsTaker(output) = output else {
        panic!("expected taker match output")
    };
    assert_eq!(output.order_after, taker);

    let mut maker_record = order_record(2, OrderSide::Buy, 2, 2, SpotOrderStatus::Open, 11);
    maker_record.buy_quote_fee_reserve_limit = Decimal::from(3);
    let mut maker = SpotOrderV7::from_record(maker_record).unwrap();
    maker
        .apply(SpotOrderV7Event::MatchedAsMaker {
            filled_size: Decimal::from(2),
            quote_fee: Decimal::ONE,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();
    assert_eq!(maker.status(), SpotOrderStatus::Filled);
    assert_eq!(maker.order_base().buy_quote_fee_reserve_consumed, Decimal::ONE);
    assert_eq!(maker.order_base().version, 12);
}

#[test]
fn sell_match_does_not_consume_buy_fee_reserve_and_errors_are_atomic() {
    let mut seller = order(2, OrderSide::Sell, 2, 2, SpotOrderStatus::Open, 4);
    seller
        .apply(SpotOrderV7Event::MatchedAsMaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::MAX,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();
    assert_eq!(seller.order_base().buy_quote_fee_reserve_consumed, Decimal::ZERO);

    let mut buyer_record = order_record(1, OrderSide::Buy, 2, 2, SpotOrderStatus::Open, 7);
    buyer_record.buy_quote_fee_reserve_limit = Decimal::ONE;
    let mut buyer = SpotOrderV7::from_record(buyer_record).unwrap();
    let before = buyer.clone();
    assert!(
        buyer
            .transition(SpotOrderV7Event::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_fee: Decimal::from(2),
                timestamp: EVENT_TIMESTAMP,
            })
            .is_err()
    );
    assert_eq!(buyer, before);
}

#[test]
fn taker_tif_controls_zero_and_partial_fill_terminal_state() {
    let mut ioc_zero = sized_taker(OrderTif::Ioc, 2);
    ioc_zero.apply(SpotOrderV7Event::Activate { timestamp: 1 }).unwrap();
    ioc_zero
        .apply(SpotOrderV7Event::MatchedAsTaker {
            filled_size: Decimal::ZERO,
            quote_fee: Decimal::ZERO,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();
    assert_eq!(ioc_zero.status(), SpotOrderStatus::Rejected);

    let mut ioc_partial = sized_taker(OrderTif::Ioc, 2);
    ioc_partial.apply(SpotOrderV7Event::Activate { timestamp: 1 }).unwrap();
    ioc_partial
        .apply(SpotOrderV7Event::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::ZERO,
            timestamp: EVENT_TIMESTAMP,
        })
        .unwrap();
    assert_eq!(ioc_partial.status(), SpotOrderStatus::Canceled);

    let mut alo = sized_taker(OrderTif::Alo, 2);
    alo.apply(SpotOrderV7Event::Activate { timestamp: 1 }).unwrap();
    alo.apply(SpotOrderV7Event::RejectedAsTaker { timestamp: EVENT_TIMESTAMP }).unwrap();
    assert_eq!(alo.status(), SpotOrderStatus::Rejected);
}
