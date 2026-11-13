use common_entity::Fsm;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::{
    MatchPerpOrderV3Input, MatchPerpOrderV3Output, PerpOrderV3, PerpOrderV3Error, PerpOrderV3Event,
};
use crate::entity::common::UserId;
use crate::entity::{OrderSide, OrderTif, PerpTradeKind, SpotOrderStatus};

const NOW: u64 = 20;

fn order(id: u64, side: OrderSide, size: Decimal, status: SpotOrderStatus) -> PerpOrderV3 {
    let mut base = crate::entity::common::common_order::OrderBase::default();
    base.owner = UserId(id + 100);
    base.order_id = id;
    base.side = side;
    base.limit_px = dec!(100);
    base.size = size;
    base.orig_size = dec!(10);
    base.status = status;
    base.created_at = 10;
    base.updated_at = 10;
    PerpOrderV3 { comon_order: base, reduce_only: false, is_liquidation: false }
}

fn taker(status: SpotOrderStatus) -> PerpOrderV3 {
    let size = if status == SpotOrderStatus::PartiallyFilled { dec!(5) } else { dec!(10) };
    order(1, OrderSide::Buy, size, status)
}

fn maker(id: u64, size: Decimal) -> PerpOrderV3 {
    let mut maker = order(id, OrderSide::Sell, size, SpotOrderStatus::Open);
    maker.comon_order.orig_size = size;
    maker.comon_order.limit_px = dec!(90);
    maker
}

fn input() -> MatchPerpOrderV3Input {
    MatchPerpOrderV3Input {
        match_id: 7,
        maker_fee_rate: dec!(0.001),
        taker_fee_rate: dec!(0.002),
        timestamp: NOW,
    }
}

fn events() -> Vec<PerpOrderV3Event> {
    vec![
        PerpOrderV3Event::Match { makers: vec![], input: input(), timestamp: NOW },
        PerpOrderV3Event::ReplaceOrFill {
            price: dec!(101),
            size: dec!(11),
            makers: vec![],
            input: input(),
            timestamp: NOW,
        },
        PerpOrderV3Event::Modify { price: dec!(101), size: dec!(11), timestamp: NOW },
        PerpOrderV3Event::Cancel { timestamp: NOW },
    ]
}

#[test]
fn every_active_state_accepts_every_event() {
    for status in
        [SpotOrderStatus::Pending, SpotOrderStatus::Open, SpotOrderStatus::PartiallyFilled]
    {
        for event in events() {
            let mut subject = taker(status);
            assert!(subject.transition(event).is_ok(), "{status:?}");
        }
    }
}

#[test]
fn every_terminal_state_rejects_every_event_atomically() {
    for status in [SpotOrderStatus::Filled, SpotOrderStatus::Canceled, SpotOrderStatus::Rejected] {
        for event in events() {
            let mut subject = taker(status);
            let before = subject.clone();
            assert_eq!(subject.transition(event), Err(PerpOrderV3Error::Terminal));
            assert_eq!(subject, before);
        }
    }
}

#[test]
fn gtc_match_creates_trade_at_maker_price_and_updates_each_version_once() {
    let mut subject = taker(SpotOrderStatus::Pending);
    subject.is_liquidation = true;
    let (_, outcome) = subject
        .transition(PerpOrderV3Event::Match {
            makers: vec![maker(2, dec!(4)), maker(3, dec!(6))],
            input: input(),
            timestamp: NOW,
        })
        .unwrap();
    let MatchPerpOrderV3Output::Filled {
        filled_taker_order_after,
        maker_orders_after,
        created_trades,
    } = outcome.unwrap()
    else {
        panic!("expected filled outcome")
    };
    assert_eq!(filled_taker_order_after.comon_order.lsn, 1);
    assert_eq!(maker_orders_after.iter().map(|m| m.comon_order.lsn).collect::<Vec<_>>(), [1, 1]);
    assert_eq!(created_trades.len(), 2);
    assert_eq!(created_trades[0].trade_base.trade_id, 7_000_001);
    assert_eq!(created_trades[0].trade_base.price, dec!(90));
    assert_eq!(created_trades[0].trade_base.size, dec!(4));
    assert_eq!(created_trades[0].trade_base.taker_fee, dec!(0.72));
    assert_eq!(created_trades[0].trade_base.maker_fee, dec!(0.36));
    assert_eq!(created_trades[0].kind, PerpTradeKind::Liquidation);
    assert_eq!(created_trades[0].reference_id.as_deref(), Some("7"));
}

#[test]
fn gtc_ioc_and_alo_have_distinct_no_fill_results() {
    let mut gtc = taker(SpotOrderStatus::Pending);
    let (state, outcome) = gtc
        .transition(PerpOrderV3Event::Match { makers: vec![], input: input(), timestamp: NOW })
        .unwrap();
    assert_eq!(state, SpotOrderStatus::Open);
    assert!(matches!(outcome, Some(MatchPerpOrderV3Output::Resting { .. })));

    let mut ioc = taker(SpotOrderStatus::Pending);
    ioc.comon_order.tif = OrderTif::Ioc;
    let (state, outcome) = ioc
        .transition(PerpOrderV3Event::Match { makers: vec![], input: input(), timestamp: NOW })
        .unwrap();
    assert_eq!(state, SpotOrderStatus::Rejected);
    assert!(matches!(outcome, Some(MatchPerpOrderV3Output::Rejected { .. })));

    let crossing_maker = maker(2, dec!(3));
    let maker_before = crossing_maker.clone();
    let mut alo = taker(SpotOrderStatus::Pending);
    alo.comon_order.tif = OrderTif::Alo;
    let (state, outcome) = alo
        .transition(PerpOrderV3Event::Match {
            makers: vec![crossing_maker],
            input: input(),
            timestamp: NOW,
        })
        .unwrap();
    assert_eq!(state, SpotOrderStatus::Rejected);
    let MatchPerpOrderV3Output::Rejected { .. } = outcome.unwrap() else {
        panic!("expected rejected outcome")
    };
    assert_eq!(maker_before.comon_order.lsn, 0);
}

#[test]
fn ioc_partial_fill_cancels_remainder() {
    let mut subject = taker(SpotOrderStatus::Pending);
    subject.comon_order.tif = OrderTif::Ioc;
    let (state, outcome) = subject
        .transition(PerpOrderV3Event::Match {
            makers: vec![maker(2, dec!(4))],
            input: input(),
            timestamp: NOW,
        })
        .unwrap();
    assert_eq!(state, SpotOrderStatus::Canceled);
    assert!(matches!(outcome, Some(MatchPerpOrderV3Output::CanceledAfterPartialFill { .. })));
    assert_eq!(subject.comon_order.size, dec!(6));
}

#[test]
fn incompatible_makers_are_skipped() {
    let mut same_side = maker(2, dec!(3));
    same_side.comon_order.side = OrderSide::Buy;
    let mut pending = maker(3, dec!(3));
    pending.comon_order.status = SpotOrderStatus::Pending;
    let mut non_crossing = maker(4, dec!(3));
    non_crossing.comon_order.limit_px = dec!(110);
    let mut subject = taker(SpotOrderStatus::Pending);
    let (state, outcome) = subject
        .transition(PerpOrderV3Event::Match {
            makers: vec![same_side, pending, non_crossing],
            input: input(),
            timestamp: NOW,
        })
        .unwrap();
    assert_eq!(state, SpotOrderStatus::Open);
    assert!(matches!(outcome, Some(MatchPerpOrderV3Output::Resting { .. })));
}

#[test]
fn modify_cancel_and_replace_or_fill_preserve_expected_facts() {
    let mut modified = taker(SpotOrderStatus::PartiallyFilled);
    modified
        .transition(PerpOrderV3Event::Modify { price: dec!(105), size: dec!(12), timestamp: NOW })
        .unwrap();
    assert_eq!(modified.comon_order.status, SpotOrderStatus::PartiallyFilled);
    assert_eq!(modified.comon_order.size, dec!(7));
    assert_eq!(modified.comon_order.lsn, 1);

    let remaining = modified.comon_order.size;
    modified.transition(PerpOrderV3Event::Cancel { timestamp: NOW }).unwrap();
    assert_eq!(modified.comon_order.size, remaining);
    assert_eq!(modified.comon_order.status, SpotOrderStatus::Canceled);

    let mut replaced = taker(SpotOrderStatus::Pending);
    replaced
        .transition(PerpOrderV3Event::ReplaceOrFill {
            price: dec!(95),
            size: dec!(4),
            makers: vec![maker(2, dec!(4))],
            input: input(),
            timestamp: NOW,
        })
        .unwrap();
    assert_eq!(replaced.comon_order.status, SpotOrderStatus::Filled);
    assert_eq!(replaced.comon_order.lsn, 1);
    assert_eq!(replaced.comon_order.orig_size, dec!(4));
}

#[test]
fn invalid_inputs_and_late_failures_are_atomic() {
    let mut cases = Vec::new();
    let mut bad_fee = input();
    bad_fee.taker_fee_rate = dec!(-0.1);
    cases.push(PerpOrderV3Event::Match { makers: vec![], input: bad_fee, timestamp: NOW });
    cases.push(PerpOrderV3Event::Modify { price: dec!(1), size: Decimal::ZERO, timestamp: NOW });
    cases.push(PerpOrderV3Event::Cancel { timestamp: 9 });

    for event in cases {
        let mut subject = taker(SpotOrderStatus::Pending);
        let before = subject.clone();
        assert!(subject.transition(event).is_err());
        assert_eq!(subject, before);
    }

    let mut subject = taker(SpotOrderStatus::Pending);
    subject.comon_order.lsn = u64::MAX;
    let before = subject.clone();
    assert_eq!(
        subject.transition(PerpOrderV3Event::Cancel { timestamp: NOW }),
        Err(PerpOrderV3Error::VersionOverflow { version: u64::MAX })
    );
    assert_eq!(subject, before);
}
