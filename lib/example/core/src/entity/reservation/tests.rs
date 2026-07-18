use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{Entity, EntityMutationModel, FinancialClassification};

use super::*;
use crate::{Balance, BalanceLedgerEntryV2, BalanceLedgerReason};

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

fn usdt_balance(available: u64, frozen: u64, version: u64) -> Balance {
    Balance::new("trader-1".to_string(), "USDT".to_string(), available, frozen, version)
}

fn balance_update(before: Balance, after: &Balance) -> UpdatedEntityPair<Balance> {
    UpdatedEntityPair { before, after: after.clone() }
}

fn created_field_value(entity: &impl Entity, field_name: &str) -> Option<String> {
    entity
        .created_field_changes()
        .into_iter()
        .find(|change| change.field_name == field_name)
        .map(|change| change.new_value)
}

fn applied_freeze_ledger(
    entry_id: String,
    balance: &mut Balance,
    amount: u64,
    reason: BalanceLedgerReason,
) -> BalanceLedgerEntryV2 {
    let mut entry = BalanceLedgerEntryV2::freeze(
        entry_id,
        balance.account_id.clone(),
        balance.asset_id.clone(),
        balance.entity_id(),
        amount,
        reason,
    )
    .unwrap();
    entry.apply_to(balance).unwrap();
    entry
}

fn applied_unfreeze_ledger(
    entry_id: String,
    balance: &mut Balance,
    amount: u64,
    reason: BalanceLedgerReason,
) -> BalanceLedgerEntryV2 {
    let mut entry = BalanceLedgerEntryV2::unfreeze(
        entry_id,
        balance.account_id.clone(),
        balance.asset_id.clone(),
        balance.entity_id(),
        amount,
        reason,
    )
    .unwrap();
    entry.apply_to(balance).unwrap();
    entry
}

fn applied_debit_frozen_ledger(
    entry_id: String,
    balance: &mut Balance,
    amount: u64,
    reason: BalanceLedgerReason,
) -> BalanceLedgerEntryV2 {
    let mut entry = BalanceLedgerEntryV2::debit_frozen(
        entry_id,
        balance.account_id.clone(),
        balance.asset_id.clone(),
        balance.entity_id(),
        amount,
        reason,
    )
    .unwrap();
    entry.apply_to(balance).unwrap();
    entry
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
    assert!(reservation.has_consistent_amounts());
}

#[test]
fn reservation_happy_path_consumes_trade_and_releases_remainder() -> Result<(), ReservationError> {
    let reservation = Reservation::new(
        "reservation:happy-path".to_string(),
        "trader-1".to_string(),
        "order-1".to_string(),
        ReservationMarketKind::Spot,
        ReservationKind::SpotBuyQuote,
        "USDT".to_string(),
        100,
    )?;

    assert_eq!(reservation.status, ReservationStatus::Active);
    assert_eq!(reservation.original_amount, 100);
    assert_eq!(reservation.remaining_amount, 100);
    assert_eq!(reservation.consumed_amount, 0);
    assert_eq!(reservation.released_amount, 0);
    assert_eq!(reservation.close_reason, None);
    assert_eq!(reservation.version, 1);
    assert!(reservation.has_consistent_amounts());

    let after_consume = reservation.consume(40, None)?;

    assert_eq!(after_consume.status, ReservationStatus::Active);
    assert_eq!(after_consume.consumed_amount, 40);
    assert_eq!(after_consume.remaining_amount, 60);
    assert_eq!(after_consume.released_amount, 0);
    assert_eq!(after_consume.close_reason, None);
    assert_eq!(after_consume.version, 2);
    assert!(after_consume.has_consistent_amounts());

    let after_release =
        after_consume.release(60, Some(ReservationCloseReason::IocRemainderCanceled))?;

    assert_eq!(after_release.status, ReservationStatus::ClosedMixed);
    assert_eq!(after_release.consumed_amount, 40);
    assert_eq!(after_release.released_amount, 60);
    assert_eq!(after_release.remaining_amount, 0);
    assert_eq!(after_release.close_reason, Some(ReservationCloseReason::IocRemainderCanceled));
    assert_eq!(after_release.version, 3);
    assert!(after_release.has_consistent_amounts());

    Ok(())
}

#[test]
fn reservation_to_balance_ledger_happy_path_is_auditable() -> Result<(), ReservationError> {
    // Given: 订单创建 quote 冻结凭证，账户当前有足够可用余额。
    let reservation = Reservation::new(
        "reservation:audit-chain".to_string(),
        "trader-1".to_string(),
        "order-1".to_string(),
        ReservationMarketKind::Spot,
        ReservationKind::SpotBuyQuote,
        "USDT".to_string(),
        200,
    )?;
    let created = ReservationCreated::from_reservation(
        "reservation-created:audit-chain".to_string(),
        &reservation,
    );
    let mut balance = usdt_balance(1_000, 0, 1);

    // When: ReservationCreated 触发余额冻结。
    let before_freeze = balance.clone();
    let freeze_ledger = applied_freeze_ledger(
        "ledger:freeze:order-1".to_string(),
        &mut balance,
        created.original_amount,
        BalanceLedgerReason::FreezeForOrder { order_id: created.caused_by_order_id.clone() },
    );

    // Then: 冻结凭证和余额流水指向同一订单，余额 before/after 可审计。
    assert_eq!(reservation.status, ReservationStatus::Active);
    assert_eq!(reservation.remaining_amount, 200);
    assert_eq!(reservation.close_reason, None);
    assert_eq!(reservation.version, 1);
    assert_eq!(balance.available, 800);
    assert_eq!(balance.frozen, 200);
    assert_eq!(freeze_ledger.reason.order_id(), Some("order-1"));
    assert_eq!(created_field_value(&freeze_ledger, "reason_order_id").as_deref(), Some("order-1"));
    assert!(freeze_ledger.matches_balance_update(&balance_update(before_freeze, &balance)));

    // When: 成交消耗部分冻结金额，并记录 trade / settlement 因果来源。
    let trade_ids = vec!["trade-1".to_string()];
    let settlement_ids = vec!["settlement-1".to_string()];
    let after_consume = reservation.consume(120, None)?;
    let consumed = ReservationConsumed::new(
        "reservation-consumed:audit-chain".to_string(),
        &after_consume,
        120,
        trade_ids[0].clone(),
    );
    let before_debit = balance.clone();
    let debit_ledger = applied_debit_frozen_ledger(
        "ledger:debit-frozen:trade-1".to_string(),
        &mut balance,
        consumed.amount,
        BalanceLedgerReason::SettleSpotTradeBuyerReleaseFrozenQuote {
            trade_ids: trade_ids.clone(),
            settlement_ids: settlement_ids.clone(),
        },
    );

    // Then: ReservationConsumed 和余额流水共同证明冻结余额被成交消耗。
    assert_eq!(after_consume.status, ReservationStatus::Active);
    assert_eq!(after_consume.remaining_amount, 80);
    assert_eq!(after_consume.close_reason, None);
    assert_eq!(after_consume.version, 2);
    assert_eq!(consumed.caused_by_ref_id, "trade-1");
    assert_eq!(consumed.remaining_amount_after, 80);
    assert_eq!(balance.available, 800);
    assert_eq!(balance.frozen, 80);
    assert_eq!(debit_ledger.reason.trade_ids(), trade_ids.as_slice());
    assert_eq!(debit_ledger.reason.settlement_ids(), settlement_ids.as_slice());
    assert_eq!(created_field_value(&debit_ledger, "reason_trade_ids").as_deref(), Some("trade-1"));
    assert_eq!(
        created_field_value(&debit_ledger, "reason_settlement_ids").as_deref(),
        Some("settlement-1")
    );
    assert!(debit_ledger.matches_balance_update(&balance_update(before_debit, &balance)));

    // When: 释放剩余冻结金额，订单关闭为 IOC 剩余撤销。
    let after_release =
        after_consume.release(80, Some(ReservationCloseReason::IocRemainderCanceled))?;
    let released = ReservationReleased::new(
        "reservation-released:audit-chain".to_string(),
        &after_release,
        80,
        "cancel-1".to_string(),
        ReservationCloseReason::IocRemainderCanceled,
    );
    let before_unfreeze = balance.clone();
    let unfreeze_ledger = applied_unfreeze_ledger(
        "ledger:unfreeze:cancel-1".to_string(),
        &mut balance,
        released.amount,
        BalanceLedgerReason::CancelSpotOrderReleaseQuote {
            order_id: after_release.caused_by_order_id.clone(),
        },
    );

    // Then: close_reason、剩余冻结释放和余额最终状态一致。
    assert_eq!(after_release.status, ReservationStatus::ClosedMixed);
    assert_eq!(after_release.remaining_amount, 0);
    assert_eq!(after_release.close_reason, Some(ReservationCloseReason::IocRemainderCanceled));
    assert_eq!(after_release.version, 3);
    assert_eq!(released.caused_by_ref_id, "cancel-1");
    assert_eq!(released.close_reason, ReservationCloseReason::IocRemainderCanceled);
    assert_eq!(released.remaining_amount_after, 0);
    assert_eq!(balance.available, 880);
    assert_eq!(balance.frozen, 0);
    assert_eq!(unfreeze_ledger.reason.order_id(), Some("order-1"));
    assert_eq!(
        created_field_value(&unfreeze_ledger, "reason_order_id").as_deref(),
        Some("order-1")
    );
    assert!(unfreeze_ledger.matches_balance_update(&balance_update(before_unfreeze, &balance)));

    Ok(())
}

#[test]
fn partial_reservation_consume_debits_frozen_balance_without_closing()
-> Result<(), ReservationError> {
    // Given: 下单已经冻结 200 USDT。
    let reservation = active_reservation();
    let mut balance = usdt_balance(800, 200, 2);

    // When: 单笔成交只消耗其中 80。
    let after_consume = reservation.consume(80, None)?;
    let consumed = ReservationConsumed::new(
        "reservation-consumed:partial".to_string(),
        &after_consume,
        80,
        "trade-2".to_string(),
    );
    let before_debit = balance.clone();
    let ledger = applied_debit_frozen_ledger(
        "ledger:debit-frozen:trade-2".to_string(),
        &mut balance,
        consumed.amount,
        BalanceLedgerReason::SettleSpotTradeBuyerReleaseFrozenQuote {
            trade_ids: vec![consumed.caused_by_ref_id.clone()],
            settlement_ids: vec!["settlement-2".to_string()],
        },
    );

    // Then: Reservation 保持 active，冻结余额只减少成交消耗部分。
    assert_eq!(after_consume.status, ReservationStatus::Active);
    assert_eq!(after_consume.remaining_amount, 120);
    assert_eq!(after_consume.close_reason, None);
    assert_eq!(after_consume.version, 2);
    assert_eq!(balance.available, 800);
    assert_eq!(balance.frozen, 120);
    assert_eq!(ledger.reason.trade_ids(), ["trade-2".to_string()].as_slice());
    assert_eq!(ledger.reason.settlement_ids(), ["settlement-2".to_string()].as_slice());
    assert_eq!(created_field_value(&ledger, "reason_trade_ids").as_deref(), Some("trade-2"));
    assert_eq!(
        created_field_value(&ledger, "reason_settlement_ids").as_deref(),
        Some("settlement-2")
    );
    assert!(ledger.matches_balance_update(&balance_update(before_debit, &balance)));

    Ok(())
}

#[test]
fn reservation_release_tail_unfreezes_balance_and_records_close_reason()
-> Result<(), ReservationError> {
    // Given: 成交后还剩 120 USDT 冻结待释放。
    let after_consume = active_reservation().consume(80, None)?;
    let mut balance = usdt_balance(800, 120, 3);

    // When: 撤单释放全部剩余冻结金额。
    let after_release =
        after_consume.release(120, Some(ReservationCloseReason::IocRemainderCanceled))?;
    let released = ReservationReleased::new(
        "reservation-released:tail".to_string(),
        &after_release,
        120,
        "cancel-2".to_string(),
        ReservationCloseReason::IocRemainderCanceled,
    );
    let before_unfreeze = balance.clone();
    let ledger = applied_unfreeze_ledger(
        "ledger:unfreeze:cancel-2".to_string(),
        &mut balance,
        released.amount,
        BalanceLedgerReason::CancelSpotOrderReleaseQuote {
            order_id: after_release.caused_by_order_id.clone(),
        },
    );

    // Then: Reservation 以混合关闭收尾，余额解冻回可用。
    assert_eq!(after_release.status, ReservationStatus::ClosedMixed);
    assert_eq!(after_release.remaining_amount, 0);
    assert_eq!(after_release.close_reason, Some(ReservationCloseReason::IocRemainderCanceled));
    assert_eq!(after_release.version, 3);
    assert_eq!(released.close_reason, ReservationCloseReason::IocRemainderCanceled);
    assert_eq!(released.remaining_amount_after, 0);
    assert_eq!(balance.available, 920);
    assert_eq!(balance.frozen, 0);
    assert_eq!(ledger.reason.order_id(), Some("order-1"));
    assert_eq!(created_field_value(&ledger, "reason_order_id").as_deref(), Some("order-1"));
    assert!(ledger.matches_balance_update(&balance_update(before_unfreeze, &balance)));

    Ok(())
}

#[test]
fn partial_consume_keeps_reservation_active() {
    let after = active_reservation().consume(80, None).unwrap();

    assert_eq!(after.consumed_amount, 80);
    assert_eq!(after.remaining_amount, 120);
    assert_eq!(after.status, ReservationStatus::Active);
    assert_eq!(after.close_reason, None);
    assert!(after.has_consistent_amounts());
}

#[test]
fn full_consume_closes_as_exhausted() {
    let after = active_reservation().consume(200, Some(ReservationCloseReason::Filled)).unwrap();

    assert_eq!(after.remaining_amount, 0);
    assert_eq!(after.status, ReservationStatus::ExhaustedByConsume);
    assert_eq!(after.close_reason, Some(ReservationCloseReason::Filled));
    assert!(after.has_consistent_amounts());
}

#[test]
fn mixed_close_tracks_consumed_and_released() {
    let after_consume = active_reservation().consume(80, None).unwrap();
    let after_release =
        after_consume.release(120, Some(ReservationCloseReason::IocRemainderCanceled)).unwrap();

    assert_eq!(after_release.consumed_amount, 80);
    assert_eq!(after_release.released_amount, 120);
    assert_eq!(after_release.remaining_amount, 0);
    assert_eq!(after_release.status, ReservationStatus::ClosedMixed);
    assert_eq!(after_release.close_reason, Some(ReservationCloseReason::IocRemainderCanceled));
}

#[test]
fn reservation_create_record_is_append_only() {
    let reservation = active_reservation();
    let created =
        ReservationCreated::from_reservation("reservation-created:1".to_string(), &reservation);

    assert_eq!(ReservationCreated::mutation_model(), EntityMutationModel::AppendOnlyRecord);
    assert_eq!(created.original_amount, 200);
}

#[test]
fn reservation_facts_use_business_voucher_financial_classification() {
    assert_eq!(Reservation::financial_classification(), FinancialClassification::BusinessVoucher);
    assert_eq!(
        ReservationCreated::financial_classification(),
        FinancialClassification::BusinessVoucher
    );
    assert_eq!(
        ReservationConsumed::financial_classification(),
        FinancialClassification::BusinessVoucher
    );
    assert_eq!(
        ReservationReleased::financial_classification(),
        FinancialClassification::BusinessVoucher
    );
}
