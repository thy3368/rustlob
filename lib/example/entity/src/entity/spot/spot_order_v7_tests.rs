use common_entity::fsm_v2::Fsm;
use common_entity::{Entity, LogEntry, LogEntrySequencer, Shard, TrackDiff};
use rust_decimal::Decimal;

use super::{PlaceSpotOrderPipelineV2Cmd, SpotOrderV7, SpotOrderV7Command};
use crate::account::coin::Coin;
use crate::account::symbol::Symbol;
use crate::common::PriceType;
use crate::entity::common::PartyId;
use crate::entity::common::common_order::OrderBase;
use crate::entity::common::order_primitives::{
    OrderGroupRelation, OrderSide, OrderTif, OrderTriggerRole, OrderType, SpotOrderStatus,
};

const TIMESTAMP: u64 = 500;

fn sequenced(command: SpotOrderV7Command) -> LogEntry<SpotOrderV7Command> {
    LogEntrySequencer::from_metadata_with_topic(0, 1, 10, TIMESTAMP, 0, 0, String::new(), command)
}

fn field_names(event: &common_entity::ReplayableDomainEvent) -> Vec<&str> {
    event
        .field_changes
        .iter()
        .map(|change| change.field_name_as_str().unwrap())
        .collect()
}

fn assert_buy_conserved(order: &SpotOrderV7) {
    let order = order.order_base();
    assert_eq!(
        order.buy_quote_principal_frozen + order.buy_quote_fee_reserve_frozen
            - order.buy_quote_fee_reserve_paid
            - order.buy_quote_principal_paid
            - order.buy_quote_principal_unfrozen
            - order.buy_quote_fee_reserve_unfrozen,
        Decimal::ZERO
    );
}

fn order(side: OrderSide, status: SpotOrderStatus) -> SpotOrderV7 {
    let frozen_principal = if side == OrderSide::Buy && status != SpotOrderStatus::Pending {
        Decimal::from(200)
    } else {
        Decimal::ZERO
    };
    SpotOrderV7::from_record(OrderBase {
        owner: PartyId("11".to_owned()),
        order_id: 7,
        symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
        side,
        limit_px: Decimal::from(100),
        size: Decimal::from(2),
        orig_size: Decimal::from(2),
        tif: OrderTif::Gtc,
        status,
        execution: PriceType::Limit,
        buy_quote_fee_reserve_frozen: Decimal::from(3),
        buy_quote_principal_frozen: frozen_principal,
        lsn: 4,
        ..OrderBase::default()
    })
    .unwrap()
}

fn create_command(order_id: u64, side: OrderSide, order_type: OrderType) -> SpotOrderV7Command {
    SpotOrderV7Command::Create(LogEntrySequencer::from_metadata_with_topic(
        3,
        4,
        10,
        TIMESTAMP,
        0,
        0,
        "11".to_owned(),
        PlaceSpotOrderPipelineV2Cmd {
            party_id: PartyId::from("11"),
            order_id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            price: Decimal::from(100),
            size: Decimal::from(2),
            order_type,
            relation: OrderGroupRelation::Standalone,
            cloid: Some("client-1".to_owned()),
            fee_bps: 25,
        },
    ))
}

#[test]
fn create_rejects_invalid_facts_and_non_create_commands() {
    for mutate in [0_u8, 1, 2] {
        let command =
            match create_command(7, OrderSide::Buy, OrderType::Limit { tif: OrderTif::Gtc }) {
                SpotOrderV7Command::Create(entry) => {
                    let mut command = entry.into_payload();
                    match mutate {
                        0 => command.order_id = 0,
                        1 => command.price = Decimal::ZERO,
                        _ => command.size = Decimal::ZERO,
                    }
                    SpotOrderV7Command::Create(LogEntrySequencer::from_metadata_with_topic(
                        3,
                        4,
                        10,
                        TIMESTAMP,
                        0,
                        0,
                        "11".to_owned(),
                        command,
                    ))
                }
                _ => unreachable!(),
            };
        assert!(SpotOrderV7::create(command).is_err());
    }
    assert!(
        SpotOrderV7::create(create_command(
            7,
            OrderSide::Sell,
            OrderType::Trigger {
                is_market: false,
                trigger_price: 0,
                tpsl: OrderTriggerRole::TakeProfit,
            },
        ))
        .is_err()
    );
    let SpotOrderV7Command::Create(entry) =
        create_command(7, OrderSide::Buy, OrderType::Limit { tif: OrderTif::Gtc })
    else {
        unreachable!()
    };
    let mut command = entry.into_payload();
    command.price = Decimal::MAX;
    command.size = Decimal::MAX;
    assert!(
        SpotOrderV7::create(SpotOrderV7Command::Create(
            LogEntrySequencer::from_metadata_with_topic(
                3,
                4,
                10,
                TIMESTAMP,
                0,
                0,
                "11".to_owned(),
                command,
            )
        ))
        .is_err()
    );
    assert!(SpotOrderV7::create(SpotOrderV7Command::Freeze).is_err());
}

#[test]
fn create_event_and_freeze_update_have_lifecycle_event_codes() {
    let created = SpotOrderV7::create(create_command(
        7,
        OrderSide::Buy,
        OrderType::Limit { tif: OrderTif::Gtc },
    ))
    .unwrap();
    let mut create_event = created.track_create_event().unwrap();
    create_event.domain_event_type = 1;
    assert_eq!(create_event.domain_event_type, 1);

    let (_, freeze_event) = created
        .transition(sequenced(SpotOrderV7Command::Freeze))
        .unwrap();
    assert_eq!(freeze_event.domain_event_type, 2);
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
fn buy_match_pays_from_fee_reserve_and_sell_match_does_not() {
    let buyer = order(OrderSide::Buy, SpotOrderStatus::Open);
    let (buyer, _) = buyer
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_notional: Decimal::from(100),
            quote_fee: Decimal::from(2),
        }))
        .unwrap();
    assert_eq!(
        buyer.order_base().buy_quote_fee_reserve_paid,
        Decimal::from(2)
    );

    let seller = order(OrderSide::Sell, SpotOrderStatus::Open);
    let (seller, _) = seller
        .transition(sequenced(SpotOrderV7Command::MatchedAsMaker {
            filled_size: Decimal::ONE,
            quote_notional: Decimal::from(100),
            quote_fee: Decimal::from(2),
        }))
        .unwrap();
    assert_eq!(
        seller.order_base().buy_quote_fee_reserve_paid,
        Decimal::ZERO
    );
}

#[test]
fn match_failure_keeps_order_unchanged() {
    let subject = order(OrderSide::Buy, SpotOrderStatus::Open);
    let before = subject.clone();
    assert!(
        subject
            .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_notional: Decimal::from(100),
                quote_fee: Decimal::from(4),
            }))
            .is_err()
    );
    assert_eq!(subject, before);

    let subject = order(OrderSide::Buy, SpotOrderStatus::Open);
    let before = subject.clone();
    assert!(
        subject
            .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ZERO,
                quote_notional: Decimal::ZERO,
                quote_fee: Decimal::ONE,
            }))
            .is_err()
    );
    assert_eq!(subject, before);
}

#[test]
fn stale_lsn_keeps_order_unchanged() {
    let subject = order(OrderSide::Buy, SpotOrderStatus::Open);
    let before = subject.clone();
    assert!(
        subject
            .transition(LogEntrySequencer::from_metadata_with_topic(
                0,
                1,
                3,
                TIMESTAMP,
                0,
                0,
                String::new(),
                SpotOrderV7Command::MatchedAsTaker {
                    filled_size: Decimal::ONE,
                    quote_notional: Decimal::from(100),
                    quote_fee: Decimal::ZERO,
                },
            ))
            .is_err()
    );
    assert_eq!(subject, before);
}

#[test]
fn command_lsn_can_jump_and_be_reused_within_one_command() {
    let subject = order(OrderSide::Buy, SpotOrderStatus::Pending);
    let (subject, _) = subject
        .transition(LogEntrySequencer::from_metadata_with_topic(
            0,
            1,
            20,
            TIMESTAMP,
            0,
            0,
            String::new(),
            SpotOrderV7Command::Freeze,
        ))
        .unwrap();
    let (subject, _) = subject
        .transition(LogEntrySequencer::from_metadata_with_topic(
            0,
            1,
            20,
            TIMESTAMP + 1,
            0,
            0,
            String::new(),
            SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_notional: Decimal::from(100),
                quote_fee: Decimal::ZERO,
            },
        ))
        .unwrap();
    assert_eq!(subject.order_base().lsn, 20);
    assert_eq!(subject.status(), SpotOrderStatus::PartiallyFilled);
}

#[test]
fn taker_tif_controls_final_state() {
    let gtc = order(OrderSide::Buy, SpotOrderStatus::Open);
    let (gtc, _) = gtc
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_notional: Decimal::from(100),
            quote_fee: Decimal::ZERO,
        }))
        .unwrap();
    assert_eq!(gtc.status(), SpotOrderStatus::PartiallyFilled);

    let mut ioc_record = order(OrderSide::Buy, SpotOrderStatus::Open)
        .order_base()
        .clone();
    ioc_record.tif = OrderTif::Ioc;
    let ioc_before = SpotOrderV7::from_record(ioc_record).unwrap();
    let (ioc, _) = ioc_before
        .clone()
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_notional: Decimal::from(100),
            quote_fee: Decimal::ZERO,
        }))
        .unwrap();
    assert_eq!(ioc.status(), SpotOrderStatus::Canceled);
    assert_eq!(
        ioc.terminal_unfreeze_delta(&ioc_before).unwrap().amount(),
        Decimal::from(103)
    );

    let mut alo_record = order(OrderSide::Buy, SpotOrderStatus::Open)
        .order_base()
        .clone();
    alo_record.tif = OrderTif::Alo;
    let alo_before = SpotOrderV7::from_record(alo_record).unwrap();
    let (alo, _) = alo_before
        .clone()
        .transition(sequenced(SpotOrderV7Command::RejectedAsTaker))
        .unwrap();
    assert_eq!(alo.status(), SpotOrderStatus::Rejected);
    assert_eq!(
        alo.terminal_unfreeze_delta(&alo_before).unwrap().amount(),
        Decimal::from(203)
    );
}

#[test]
fn filled_sell_taker_returns_zero_base_release_requirement() {
    let seller_before = order(OrderSide::Sell, SpotOrderStatus::Open);
    let (seller, _) = seller_before
        .clone()
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::from(2),
            quote_notional: Decimal::from(200),
            quote_fee: Decimal::ZERO,
        }))
        .unwrap();

    assert_eq!(
        seller
            .terminal_unfreeze_delta(&seller_before)
            .unwrap()
            .amount(),
        Decimal::ZERO
    );
}

#[test]
fn sequenced_commands_emit_stable_event_codes_with_replay_metadata() {
    let cases = [
        (
            order(OrderSide::Buy, SpotOrderStatus::Pending),
            SpotOrderV7Command::Freeze,
            2,
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_notional: Decimal::from(100),
                quote_fee: Decimal::ONE,
            },
            3,
        ),
        (
            order(OrderSide::Sell, SpotOrderStatus::Open),
            SpotOrderV7Command::MatchedAsMaker {
                filled_size: Decimal::ONE,
                quote_notional: Decimal::from(100),
                quote_fee: Decimal::ZERO,
            },
            4,
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
            5,
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            SpotOrderV7Command::Modify {
                price: Decimal::from(110),
                size: Decimal::from(3),
                tif: Some(OrderTif::Ioc),
                cloid: Some("next-cloid".to_owned()),
            },
            6,
        ),
        (
            order(OrderSide::Sell, SpotOrderStatus::Open),
            SpotOrderV7Command::Cancel,
            7,
        ),
    ];

    for (subject, command, event_type) in cases {
        let before = subject.clone();
        let (after, replay) = subject.transition(sequenced(command)).unwrap();
        assert_eq!(subject, before);
        assert_eq!(replay.t, after.topic());
        assert_eq!(replay.p, after.partition());
        assert_eq!(replay.previous_lsn, before.order_base().lsn);
        assert_eq!(replay.lsn, 10);
        assert_eq!(replay.logical_ts, TIMESTAMP);
        assert_eq!(replay.entity_id, after.order_base().order_id);
        assert_eq!(replay.entity_type, 16);
        assert_eq!(replay.domain_event_type, event_type);
        assert!(!replay.field_changes.is_empty());
    }
}

#[test]
fn modify_and_buy_match_replay_all_business_field_changes() {
    let modified = order(OrderSide::Buy, SpotOrderStatus::Open);
    let (_, event) = modified
        .transition(sequenced(SpotOrderV7Command::Modify {
            price: Decimal::from(110),
            size: Decimal::from(3),
            tif: Some(OrderTif::Ioc),
            cloid: Some("next-cloid".to_owned()),
        }))
        .unwrap();
    let fields = field_names(&event);
    for expected in [
        "limit_px",
        "size",
        "orig_size",
        "tif",
        "cloid",
        "updated_at",
    ] {
        assert!(fields.contains(&expected), "missing {expected}: {fields:?}");
    }

    let matched = order(OrderSide::Buy, SpotOrderStatus::Open);
    let (_, event) = matched
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_notional: Decimal::from(100),
            quote_fee: Decimal::ONE,
        }))
        .unwrap();
    let fields = field_names(&event);
    for expected in [
        "size",
        "status",
        "buy_quote_fee_reserve_paid",
        "buy_quote_principal_paid",
        "updated_at",
    ] {
        assert!(fields.contains(&expected), "missing {expected}: {fields:?}");
    }
    assert_eq!(fields.len(), 5, "unexpected fields: {fields:?}");
}

#[test]
fn sequenced_failures_are_atomic() {
    let cases = vec![
        (
            order(OrderSide::Buy, SpotOrderStatus::Pending),
            sequenced(create_command(
                8,
                OrderSide::Buy,
                OrderType::Limit { tif: OrderTif::Gtc },
            )),
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            LogEntrySequencer::from_metadata_with_topic(
                0,
                1,
                3,
                TIMESTAMP,
                0,
                0,
                String::new(),
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
                quote_notional: Decimal::from(100),
                quote_fee: Decimal::from(4),
            }),
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: -Decimal::ONE,
                quote_notional: Decimal::ZERO,
                quote_fee: Decimal::ZERO,
            }),
        ),
        (
            order(OrderSide::Buy, SpotOrderStatus::Open),
            sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::from(3),
                quote_notional: Decimal::from(300),
                quote_fee: Decimal::ZERO,
            }),
        ),
        (
            {
                let mut record = order(OrderSide::Buy, SpotOrderStatus::Open)
                    .order_base()
                    .clone();
                record.buy_quote_fee_reserve_paid = Decimal::MAX;
                record.buy_quote_fee_reserve_frozen = Decimal::MAX;
                SpotOrderV7::from_record(record).unwrap()
            },
            sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_notional: Decimal::from(100),
                quote_fee: Decimal::ONE,
            }),
        ),
    ];
    for (subject, command) in cases {
        let before = subject.clone();
        assert!(subject.transition(command).is_err());
        assert_eq!(subject, before);
    }
}

#[test]
fn pending_cancel_releases_zero_and_exact_fee_reserve_is_allowed() {
    let pending = order(OrderSide::Sell, SpotOrderStatus::Pending);
    let (canceled, _) = pending
        .clone()
        .transition(sequenced(SpotOrderV7Command::Cancel))
        .unwrap();
    assert_eq!(
        canceled.terminal_unfreeze_delta(&pending).unwrap().amount(),
        Decimal::ZERO
    );

    let buyer = order(OrderSide::Buy, SpotOrderStatus::Open);
    let (buyer, _) = buyer
        .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ONE,
            quote_notional: Decimal::from(100),
            quote_fee: Decimal::from(3),
        }))
        .unwrap();
    assert_eq!(
        buyer.order_base().buy_quote_fee_reserve_paid,
        Decimal::from(3)
    );
}

#[test]
fn replay_projection_failure_keeps_order_unchanged() {
    let subject = order(OrderSide::Buy, SpotOrderStatus::Open);
    let before = subject.clone();
    let command = LogEntrySequencer::from_metadata_with_topic(
        before.topic(),
        before.partition(),
        before.order_base().lsn,
        before.order_base().updated_at,
        0,
        0,
        before.party_id(),
        SpotOrderV7Command::MatchedAsTaker {
            filled_size: Decimal::ZERO,
            quote_notional: Decimal::ZERO,
            quote_fee: Decimal::ZERO,
        },
    );
    assert!(subject.transition(command).is_err());
    assert_eq!(subject, before);
}

#[test]
fn freeze_and_modify_classify_buy_principal_adjustments() {
    let pending = order(OrderSide::Buy, SpotOrderStatus::Pending);
    let (open, event) = pending
        .transition(sequenced(SpotOrderV7Command::Freeze))
        .unwrap();
    assert_eq!(
        open.order_base().buy_quote_principal_frozen,
        Decimal::from(200)
    );
    assert!(field_names(&event).contains(&"buy_quote_principal_frozen"));

    let (increased, _) = open
        .transition(sequenced(SpotOrderV7Command::Modify {
            price: Decimal::from(100),
            size: Decimal::from(3),
            tif: None,
            cloid: None,
        }))
        .unwrap();
    assert_eq!(
        increased.order_base().buy_quote_principal_frozen,
        Decimal::from(300)
    );

    let (decreased, event) = increased
        .transition(sequenced(SpotOrderV7Command::Modify {
            price: Decimal::from(90),
            size: Decimal::from(2),
            tif: None,
            cloid: None,
        }))
        .unwrap();
    assert_eq!(
        decreased.order_base().buy_quote_principal_unfrozen,
        Decimal::from(120)
    );
    assert!(field_names(&event).contains(&"buy_quote_principal_unfrozen"));
}

#[test]
fn pending_modify_is_rejected_without_mutating_order() {
    let pending = order(OrderSide::Buy, SpotOrderStatus::Pending);
    let before = pending.clone();

    assert!(
        pending
            .transition(sequenced(SpotOrderV7Command::Modify {
                price: Decimal::from(110),
                size: Decimal::ONE,
                tif: None,
                cloid: None,
            }))
            .is_err()
    );
    assert_eq!(pending, before);
}

#[test]
fn buy_modify_adjusts_principal_and_fee_from_cumulative_holds() {
    let mut record = order(OrderSide::Buy, SpotOrderStatus::Open)
        .order_base()
        .clone();
    record.buy_quote_fee_reserve_bps = 50;
    record.buy_quote_fee_reserve_frozen = Decimal::ONE;
    let open = SpotOrderV7::from_record(record).unwrap();

    let (increased, _) = open
        .transition(sequenced(SpotOrderV7Command::Modify {
            price: Decimal::from(100),
            size: Decimal::from(3),
            tif: None,
            cloid: None,
        }))
        .unwrap();
    assert_eq!(
        increased.order_base().buy_quote_principal_frozen,
        Decimal::from(300)
    );
    assert_eq!(
        increased.order_base().buy_quote_fee_reserve_frozen,
        Decimal::new(15, 1)
    );
    assert_eq!(
        increased.modify_freeze_delta(&open).unwrap().amount(),
        Decimal::new(1005, 1)
    );

    let (decreased, _) = increased
        .transition(sequenced(SpotOrderV7Command::Modify {
            price: Decimal::from(100),
            size: Decimal::ONE,
            tif: None,
            cloid: None,
        }))
        .unwrap();
    assert_eq!(
        decreased.order_base().buy_quote_principal_unfrozen,
        Decimal::from(200)
    );
    assert_eq!(
        decreased.order_base().buy_quote_fee_reserve_unfrozen,
        Decimal::ONE
    );
    assert_eq!(
        decreased.modify_freeze_delta(&increased).unwrap().amount(),
        Decimal::new(-201, 0)
    );
}

#[test]
fn partially_filled_modify_does_not_deduct_paid_fee_from_remaining_target() {
    let mut record = order(OrderSide::Buy, SpotOrderStatus::PartiallyFilled)
        .order_base()
        .clone();
    record.size = Decimal::ONE;
    record.buy_quote_fee_reserve_bps = 100;
    record.buy_quote_principal_paid = Decimal::from(100);
    record.buy_quote_fee_reserve_frozen = Decimal::from(2);
    record.buy_quote_fee_reserve_paid = Decimal::ONE;
    let partially_filled = SpotOrderV7::from_record(record).unwrap();

    let (unchanged_hold, _) = partially_filled
        .transition(sequenced(SpotOrderV7Command::Modify {
            price: Decimal::from(100),
            size: Decimal::ONE,
            tif: None,
            cloid: None,
        }))
        .unwrap();
    assert_eq!(
        unchanged_hold.order_base().buy_quote_fee_reserve_unfrozen,
        Decimal::ZERO
    );
    assert_eq!(
        unchanged_hold
            .modify_freeze_delta(&partially_filled)
            .unwrap()
            .amount(),
        Decimal::ZERO
    );

    let (reduced, _) = unchanged_hold
        .transition(sequenced(SpotOrderV7Command::Modify {
            price: Decimal::from(100),
            size: Decimal::new(5, 1),
            tif: None,
            cloid: None,
        }))
        .unwrap();
    assert_eq!(
        reduced.order_base().buy_quote_fee_reserve_unfrozen,
        Decimal::new(5, 1)
    );
    assert_eq!(
        reduced
            .modify_freeze_delta(&unchanged_hold)
            .unwrap()
            .amount(),
        Decimal::new(-505, 1)
    );
}

#[test]
fn buy_modify_rounds_fee_target_up_and_is_atomic_on_overflow() {
    let mut record = order(OrderSide::Buy, SpotOrderStatus::Open)
        .order_base()
        .clone();
    record.limit_px = Decimal::ONE;
    record.size = Decimal::new(1, 6);
    record.orig_size = record.size;
    record.buy_quote_principal_frozen = record.size;
    record.buy_quote_fee_reserve_bps = 1;
    record.buy_quote_fee_reserve_frozen = Decimal::ZERO;
    let precise = SpotOrderV7::from_record(record).unwrap();
    let (rounded, _) = precise
        .transition(sequenced(SpotOrderV7Command::Modify {
            price: Decimal::ONE,
            size: Decimal::new(1, 6),
            tif: None,
            cloid: None,
        }))
        .unwrap();
    assert_eq!(
        rounded.order_base().buy_quote_fee_reserve_frozen,
        Decimal::new(1, 6)
    );

    let before = rounded.clone();
    assert!(
        rounded
            .transition(sequenced(SpotOrderV7Command::Modify {
                price: Decimal::MAX,
                size: Decimal::from(2),
                tif: None,
                cloid: None,
            }))
            .is_err()
    );
    assert_eq!(rounded, before);
}

#[test]
fn multi_price_buy_fill_classifies_paid_improvement_and_terminal_fee_release() {
    let buyer = order(OrderSide::Buy, SpotOrderStatus::Open);
    let (buyer, first_event) = buyer
        .transition(sequenced(SpotOrderV7Command::MatchedAsMaker {
            filled_size: Decimal::ONE,
            quote_notional: Decimal::from(90),
            quote_fee: Decimal::ONE,
        }))
        .unwrap();
    assert_eq!(
        buyer.order_base().buy_quote_principal_paid,
        Decimal::from(90)
    );
    assert_eq!(
        buyer.order_base().buy_quote_principal_unfrozen,
        Decimal::ZERO
    );
    let fields = field_names(&first_event);
    assert!(fields.contains(&"buy_quote_principal_paid"));
    assert!(!fields.contains(&"buy_quote_principal_unfrozen"));

    let buyer_before_terminal = buyer;
    let (buyer, event) = buyer_before_terminal
        .clone()
        .transition(sequenced(SpotOrderV7Command::MatchedAsMaker {
            filled_size: Decimal::ONE,
            quote_notional: Decimal::from(80),
            quote_fee: Decimal::ONE,
        }))
        .unwrap();
    assert_eq!(
        buyer
            .terminal_unfreeze_delta(&buyer_before_terminal)
            .unwrap()
            .amount(),
        Decimal::from(31)
    );
    assert_eq!(
        buyer.order_base().buy_quote_principal_paid,
        Decimal::from(170)
    );
    assert_eq!(
        buyer.order_base().buy_quote_principal_unfrozen,
        Decimal::from(30)
    );
    assert_eq!(
        buyer.order_base().buy_quote_fee_reserve_unfrozen,
        Decimal::ONE
    );
    assert!(field_names(&event).contains(&"buy_quote_fee_reserve_unfrozen"));
    assert_buy_conserved(&buyer);
}

#[test]
fn sell_match_accumulates_only_sell_fee() {
    let seller = order(OrderSide::Sell, SpotOrderStatus::Open);
    let (seller, event) = seller
        .transition(sequenced(SpotOrderV7Command::MatchedAsMaker {
            filled_size: Decimal::ONE,
            quote_notional: Decimal::from(120),
            quote_fee: Decimal::from(2),
        }))
        .unwrap();
    let order = seller.order_base();
    assert_eq!(order.sell_quote_fee_paid, Decimal::from(2));
    assert_eq!(order.buy_quote_principal_paid, Decimal::ZERO);
    assert_eq!(order.buy_quote_fee_reserve_paid, Decimal::ZERO);
    let fields = field_names(&event);
    assert!(fields.contains(&"sell_quote_fee_paid"));
    assert_eq!(fields.len(), 4, "unexpected fields: {fields:?}");
}

#[test]
fn invalid_buy_notional_and_counter_overflow_are_atomic() {
    let buyer = order(OrderSide::Buy, SpotOrderStatus::Open);
    let before = buyer.clone();
    assert!(
        buyer
            .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_notional: Decimal::from(101),
                quote_fee: Decimal::ZERO,
            }))
            .is_err()
    );
    assert_eq!(buyer, before);

    for quote_notional in [Decimal::ZERO, -Decimal::ONE] {
        let buyer = before.clone();
        let original = buyer.clone();
        assert!(
            buyer
                .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
                    filled_size: Decimal::ONE,
                    quote_notional,
                    quote_fee: Decimal::ZERO,
                }))
                .is_err()
        );
        assert_eq!(buyer, original);
    }

    let mut record = before.order_base().clone();
    record.buy_quote_principal_paid = Decimal::MAX;
    let buyer = SpotOrderV7::from_record(record).unwrap();
    let before = buyer.clone();
    assert!(
        buyer
            .transition(sequenced(SpotOrderV7Command::MatchedAsTaker {
                filled_size: Decimal::ONE,
                quote_notional: Decimal::ONE,
                quote_fee: Decimal::ZERO,
            }))
            .is_err()
    );
    assert_eq!(buyer, before);
}

#[test]
fn terminal_buy_conservation_rejects_unclassified_funds() {
    let mut record = order(OrderSide::Buy, SpotOrderStatus::Open)
        .order_base()
        .clone();
    record.status = SpotOrderStatus::Filled;
    record.buy_quote_principal_paid = Decimal::from(199);
    record.buy_quote_fee_reserve_paid = Decimal::from(3);

    assert!(super::validate_terminal_buy_conservation(&record).is_err());
}
