use common_entity::Shard;
use common_entity::fsm_v2::{CommandMetadata, Fsm, ReplayableDomainEvent, SequencedCommand};
use rust_decimal::Decimal;

use super::{
    SpotOrderV7, SpotOrderV7Command, SpotOrderV7DomainEvent, SpotOrderV7Output,
    SpotOrderV7ReleaseRequirement,
};
use crate::account::coin::Coin;
use crate::account::symbol::Symbol;
use crate::common::PriceType;
use crate::entity::common::UserId;
use crate::entity::common::common_order::OrderBase;
use crate::entity::common::order_primitives::{OrderSide, OrderTif, SpotOrderStatus};

const TIMESTAMP: u64 = 500;

fn sequenced(command: SpotOrderV7Command) -> SequencedCommand<SpotOrderV7Command> {
    SequencedCommand::new(CommandMetadata::new(10, TIMESTAMP), command)
}

fn field_names(event: &SpotOrderV7DomainEvent) -> Vec<&str> {
    event
        .replayable_event()
        .field_changes
        .iter()
        .map(|change| change.field_name_as_str().unwrap())
        .collect()
}

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
        lsn: 4,
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
        assert!(
            order(OrderSide::Buy, status)
                .validate_matchable_as_taker()
                .is_err()
        );
    }
    let mut record = order(OrderSide::Sell, SpotOrderStatus::Open)
        .order_base()
        .clone();
    record.execution = PriceType::Market;
    let market = SpotOrderV7::from_record(record).unwrap();
    assert_eq!(market.validate_matchable_as_taker(), Ok(()));
    assert!(market.validate_matchable_as_maker().is_err());
}

#[test]
fn buy_match_consumes_fee_reserve_and_sell_match_does_not() {
    let mut buyer = order(OrderSide::Buy, SpotOrderStatus::Open);
    buyer
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::from(2),
        }))
        .unwrap();
    assert_eq!(
        buyer.order_base().buy_quote_fee_reserve_consumed,
        Decimal::from(2)
    );

    let mut seller = order(OrderSide::Sell, SpotOrderStatus::Open);
    seller
        .transition(sequenced(SpotOrderV7Command::MatchedAsMaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::from(2),
        }))
        .unwrap();
    assert_eq!(
        seller.order_base().buy_quote_fee_reserve_consumed,
        Decimal::ZERO
    );
}

#[test]
fn match_failure_keeps_order_unchanged() {
    let mut subject = order(OrderSide::Buy, SpotOrderStatus::Open);
    let before = subject.clone();
    assert!(
        subject
            .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_fee: Decimal::from(4),
            }))
            .is_err()
    );
    assert_eq!(subject, before);
}

#[test]
fn stale_lsn_keeps_order_unchanged() {
    let mut subject = order(OrderSide::Buy, SpotOrderStatus::Open);
    let before = subject.clone();
    assert!(
        subject
            .transition(SequencedCommand::new(
                CommandMetadata::new(3, TIMESTAMP),
                SpotOrderV7Command::MatchedAsTaker {
                    filled_size: Decimal::ONE,
                    quote_fee: Decimal::ZERO,
                },
            ))
            .is_err()
    );
    assert_eq!(subject, before);
}

#[test]
fn command_lsn_can_jump_and_be_reused_within_one_command() {
    let mut subject = order(OrderSide::Buy, SpotOrderStatus::Pending);
    subject
        .transition(SequencedCommand::new(
            CommandMetadata::new(20, TIMESTAMP),
            SpotOrderV7Command::Activate,
        ))
        .unwrap();
    subject
        .transition(SequencedCommand::new(
            CommandMetadata::new(20, TIMESTAMP + 1),
            SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_fee: Decimal::ZERO,
            },
        ))
        .unwrap();
    assert_eq!(subject.order_base().lsn, 20);
    assert_eq!(subject.status(), SpotOrderStatus::PartiallyFilled);
}

#[test]
fn taker_tif_controls_final_state() {
    let mut gtc = order(OrderSide::Buy, SpotOrderStatus::Open);
    let output = gtc
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::ZERO,
        }))
        .unwrap()
        .output;
    assert_eq!(gtc.status(), SpotOrderStatus::PartiallyFilled);
    let SpotOrderV7Output::MatchedAsTaker(output) = output else {
        panic!("expected taker match output")
    };
    assert_eq!(output.order_after, gtc);
    assert_eq!(output.release_requirement, None);

    let mut ioc_record = order(OrderSide::Buy, SpotOrderStatus::Open)
        .order_base()
        .clone();
    ioc_record.tif = OrderTif::Ioc;
    let mut ioc = SpotOrderV7::from_record(ioc_record).unwrap();
    let output = ioc
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::ZERO,
        }))
        .unwrap()
        .output;
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

    let mut alo_record = order(OrderSide::Buy, SpotOrderStatus::Open)
        .order_base()
        .clone();
    alo_record.tif = OrderTif::Alo;
    let mut alo = SpotOrderV7::from_record(alo_record).unwrap();
    let output = alo
        .transition(sequenced(SpotOrderV7Command::RejectedAsTaker))
        .unwrap()
        .output;
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
    let output = seller
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::from(2),
            quote_fee: Decimal::ZERO,
        }))
        .unwrap()
        .output;

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

#[test]
fn sequenced_commands_emit_past_tense_events_with_replay_metadata() {
    let cases = [
        (
            order(OrderSide::Buy, SpotOrderStatus::Pending),
            SpotOrderV7Command::Activate,
            "Activated",
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_fee: Decimal::ONE,
            },
            "MatchedAsTaker",
        ),
        (
            order(OrderSide::Sell, SpotOrderStatus::Open),
            SpotOrderV7Command::MatchedAsMaker {
                filled_size: Decimal::ONE,
                quote_fee: Decimal::ZERO,
            },
            "MatchedAsMaker",
        ),
        (
            {
                let mut record = order(OrderSide::Buy, SpotOrderStatus::Open)
                    .order_base()
                    .clone();
                record.tif = OrderTif::Alo;
                SpotOrderV7::from_record(record).unwrap()
            },
            SpotOrderV7Command::RejectedAsTaker,
            "RejectedAsTaker",
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            SpotOrderV7Command::Modify {
                price: Decimal::from(110),
                size: Decimal::from(3),
                tif: Some(OrderTif::Ioc),
                cloid: Some("next-cloid".to_owned()),
            },
            "Modified",
        ),
        (
            order(OrderSide::Sell, SpotOrderStatus::Open),
            SpotOrderV7Command::Cancel,
            "Canceled",
        ),
    ];

    for (event_type, (mut subject, command, expected_variant)) in (1_u8..).zip(cases.into_iter()) {
        let before = subject.clone();
        let result = subject.transition(sequenced(command)).unwrap();
        assert_eq!(result.state, subject.status());
        let actual_variant = match &result.event {
            SpotOrderV7DomainEvent::Activated { .. } => "Activated",
            SpotOrderV7DomainEvent::MatchedAsTaker { .. } => "MatchedAsTaker",
            SpotOrderV7DomainEvent::MatchedAsMaker { .. } => "MatchedAsMaker",
            SpotOrderV7DomainEvent::RejectedAsTaker { .. } => "RejectedAsTaker",
            SpotOrderV7DomainEvent::Modified { .. } => "Modified",
            SpotOrderV7DomainEvent::Canceled { .. } => "Canceled",
        };
        assert_eq!(actual_variant, expected_variant);
        let replay = result.event.replayable_event();
        assert_eq!(replay.t, subject.topic());
        assert_eq!(replay.p, subject.partition());
        assert_eq!(replay.previous_lsn, before.order_base().lsn);
        assert_eq!(replay.lsn, 10);
        assert_eq!(replay.logical_ts, TIMESTAMP);
        assert_eq!(replay.entity_id, subject.order_base().order_id);
        assert_eq!(replay.entity_type, 16);
        assert_eq!(replay.domain_event_type, event_type);
        assert!(!replay.field_changes.is_empty());
    }
}

#[test]
fn modify_and_buy_match_replay_all_business_field_changes() {
    let mut modified = order(OrderSide::Buy, SpotOrderStatus::Open);
    let result = modified
        .transition(sequenced(SpotOrderV7Command::Modify {
            price: Decimal::from(110),
            size: Decimal::from(3),
            tif: Some(OrderTif::Ioc),
            cloid: Some("next-cloid".to_owned()),
        }))
        .unwrap();
    let fields = field_names(&result.event);
    for expected in [
        "limit_px",
        "size",
        "orig_size",
        "tif",
        "cloid",
        "updated_at",
        "version",
    ] {
        assert!(fields.contains(&expected), "missing {expected}: {fields:?}");
    }

    let mut matched = order(OrderSide::Buy, SpotOrderStatus::Open);
    let result = matched
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::ONE,
        }))
        .unwrap();
    let fields = field_names(&result.event);
    for expected in [
        "size",
        "status",
        "buy_quote_fee_reserve_consumed",
        "updated_at",
        "version",
    ] {
        assert!(fields.contains(&expected), "missing {expected}: {fields:?}");
    }
}

#[test]
fn sequenced_failures_are_atomic() {
    let cases = [
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            SequencedCommand::new(
                CommandMetadata::new(3, TIMESTAMP),
                SpotOrderV7Command::Cancel,
            ),
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Filled),
            sequenced(SpotOrderV7Command::Cancel),
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            sequenced(SpotOrderV7Command::Modify {
                price: Decimal::ZERO,
                size: Decimal::ONE,
                tif: None,
                cloid: None,
            }),
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_fee: Decimal::from(4),
            }),
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: -Decimal::ONE,
                quote_fee: Decimal::ZERO,
            }),
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::from(3),
                quote_fee: Decimal::ZERO,
            }),
        ),
        (
            {
                let mut record = order(OrderSide::Buy, SpotOrderStatus::Open)
                    .order_base()
                    .clone();
                record.buy_quote_fee_reserve_consumed = Decimal::MAX;
                record.buy_quote_fee_reserve_limit = Decimal::MAX;
                SpotOrderV7::from_record(record).unwrap()
            },
            sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_fee: Decimal::ONE,
            }),
        ),
    ];
    for (mut subject, command) in cases {
        let before = subject.clone();
        assert!(subject.transition(command).is_err());
        assert_eq!(subject, before);
    }
}

#[test]
fn pending_cancel_releases_zero_and_exact_fee_reserve_is_allowed() {
    let mut pending = order(OrderSide::Sell, SpotOrderStatus::Pending);
    let result = pending
        .transition(sequenced(SpotOrderV7Command::Cancel))
        .unwrap();
    let SpotOrderV7Output::Canceled(output) = result.output else {
        panic!("expected cancel output")
    };
    assert_eq!(output.release_requirement.amount(), Decimal::ZERO);

    let mut buyer = order(OrderSide::Buy, SpotOrderStatus::Open);
    buyer
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_fee: Decimal::from(3),
        }))
        .unwrap();
    assert_eq!(
        buyer.order_base().buy_quote_fee_reserve_consumed,
        Decimal::from(3)
    );
}

#[test]
fn replay_projection_failure_keeps_order_unchanged() {
    let mut subject = order(OrderSide::Buy, SpotOrderStatus::Open);
    let before = subject.clone();
    let command = SequencedCommand::new(
        CommandMetadata::new(before.order_base().lsn, before.order_base().updated_at),
        SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ZERO,
            quote_fee: Decimal::ZERO,
        },
    );
    assert!(subject.transition(command).is_err());
    assert_eq!(subject, before);
}
