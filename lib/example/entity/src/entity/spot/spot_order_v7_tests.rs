use common_entity::{EntityError, Fsm};
use rust_decimal::Decimal;

use super::{SpotOrderV7, SpotOrderV7Event, SpotOrderV7Output, SpotOrderV7ReleaseRequirement};
use crate::account::coin::Coin;
use crate::account::symbol::Symbol;
use crate::common::PriceType;
use crate::entity::common::UserId;
use crate::entity::common::common_order::OrderBase;
use crate::entity::common::order_primitives::{OrderSide, OrderTif, SpotOrderStatus};

const TIMESTAMP: u64 = 500;

fn order(side: OrderSide, status: SpotOrderStatus) -> SpotOrderV7 {
    SpotOrderV7::from_record(OrderBase {
        owner: UserId(11),
        order_id: 7,
        symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
        side,
        limit_px: Decimal::from(100),
        size: Decimal::from(2),
        orig_size: Decimal::from(2),
        tif: OrderTif::Gtc,
        status,
        execution: PriceType::Limit,
        buy_quote_fee_reserve_limit: Decimal::from(3),
        version: 4,
        ..OrderBase::default()
    })
    .unwrap()
}

#[test]
fn taker_and_limit_maker_admission_accept_open_and_partially_filled_orders() {
    for status in [SpotOrderStatus::Open, SpotOrderStatus::PartiallyFilled] {
        let subject = order(OrderSide::Buy, status);
        assert_eq!(subject.validate_matchable_as_taker(), Ok(()));
        assert_eq!(subject.validate_matchable_as_maker(), Ok(()));
    }
}

#[test]
fn admission_rejects_non_matchable_statuses_and_market_maker() {
    for status in [
        SpotOrderStatus::Pending,
        SpotOrderStatus::Filled,
        SpotOrderStatus::Canceled,
        SpotOrderStatus::Rejected,
    ] {
        assert!(order(OrderSide::Buy, status).validate_matchable_as_taker().is_err());
    }
    let mut record = order(OrderSide::Sell, SpotOrderStatus::Open).order_base().clone();
    record.execution = PriceType::Market;
    let market = SpotOrderV7::from_record(record).unwrap();
    assert_eq!(market.validate_matchable_as_taker(), Ok(()));
    assert!(market.validate_matchable_as_maker().is_err());
}

#[test]
fn buy_match_consumes_fee_reserve_and_sell_match_does_not() {
    let mut buyer = order(OrderSide::Buy, SpotOrderStatus::Open);
    buyer
        .apply(SpotOrderV7Event::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::from(2),
            timestamp: TIMESTAMP,
        })
        .unwrap();
    assert_eq!(buyer.order_base().buy_quote_fee_reserve_consumed, Decimal::from(2));

    let mut seller = order(OrderSide::Sell, SpotOrderStatus::Open);
    seller
        .apply(SpotOrderV7Event::MatchedAsMaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::from(2),
            timestamp: TIMESTAMP,
        })
        .unwrap();
    assert_eq!(seller.order_base().buy_quote_fee_reserve_consumed, Decimal::ZERO);
}

#[test]
fn match_failure_keeps_order_unchanged() {
    let mut subject = order(OrderSide::Buy, SpotOrderStatus::Open);
    let before = subject.clone();
    assert!(
        subject
            .apply(SpotOrderV7Event::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_fee: Decimal::from(4),
                timestamp: TIMESTAMP,
            })
            .is_err()
    );
    assert_eq!(subject, before);
}

#[test]
fn version_overflow_keeps_order_unchanged() {
    let mut record = order(OrderSide::Buy, SpotOrderStatus::Open).order_base().clone();
    record.version = u64::MAX;
    let mut subject = SpotOrderV7::from_record(record).unwrap();
    let before = subject.clone();
    assert_eq!(
        subject.apply(SpotOrderV7Event::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::ZERO,
            timestamp: TIMESTAMP,
        }),
        Err(EntityError::VersionOverflow { version: u64::MAX })
    );
    assert_eq!(subject, before);
}

#[test]
fn taker_tif_controls_final_state() {
    let mut gtc = order(OrderSide::Buy, SpotOrderStatus::Open);
    let (_, output) = gtc
        .apply(SpotOrderV7Event::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::ZERO,
            timestamp: TIMESTAMP,
        })
        .unwrap();
    assert_eq!(gtc.status(), SpotOrderStatus::PartiallyFilled);
    let SpotOrderV7Output::MatchedAsTaker(output) = output else {
        panic!("expected taker match output")
    };
    assert_eq!(output.order_after, gtc);
    assert_eq!(output.release_requirement, None);

    let mut ioc_record = order(OrderSide::Buy, SpotOrderStatus::Open).order_base().clone();
    ioc_record.tif = OrderTif::Ioc;
    let mut ioc = SpotOrderV7::from_record(ioc_record).unwrap();
    let (_, output) = ioc
        .apply(SpotOrderV7Event::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::ZERO,
            timestamp: TIMESTAMP,
        })
        .unwrap();
    assert_eq!(ioc.status(), SpotOrderStatus::Canceled);
    let SpotOrderV7Output::MatchedAsTaker(output) = output else {
        panic!("expected taker match output")
    };
    assert_eq!(output.order_after, ioc);
    assert!(matches!(
        output.release_requirement,
        Some(SpotOrderV7ReleaseRequirement::Buy { quote_amount })
            if quote_amount.amount() == Decimal::from(103)
    ));

    let mut alo_record = order(OrderSide::Buy, SpotOrderStatus::Open).order_base().clone();
    alo_record.tif = OrderTif::Alo;
    let mut alo = SpotOrderV7::from_record(alo_record).unwrap();
    let (_, output) =
        alo.apply(SpotOrderV7Event::RejectedAsTaker { timestamp: TIMESTAMP }).unwrap();
    assert_eq!(alo.status(), SpotOrderStatus::Rejected);
    let SpotOrderV7Output::RejectedAsTaker(output) = output else {
        panic!("expected rejected taker output")
    };
    assert_eq!(output.order_after, alo);
    assert!(matches!(
        output.release_requirement,
        Some(SpotOrderV7ReleaseRequirement::Buy { quote_amount })
            if quote_amount.amount() == Decimal::from(203)
    ));
}

#[test]
fn filled_sell_taker_returns_zero_base_release_requirement() {
    let mut seller = order(OrderSide::Sell, SpotOrderStatus::Open);
    let (_, output) = seller
        .apply(SpotOrderV7Event::MatchedAsTaker {
            filled_size: Decimal::from(2),
            quote_fee: Decimal::ZERO,
            timestamp: TIMESTAMP,
        })
        .unwrap();

    let SpotOrderV7Output::MatchedAsTaker(output) = output else {
        panic!("expected taker match output")
    };
    assert_eq!(output.order_after, seller);
    assert!(matches!(
        output.release_requirement,
        Some(SpotOrderV7ReleaseRequirement::Sell { base_quantity })
            if base_quantity.amount() == Decimal::ZERO
    ));
}
