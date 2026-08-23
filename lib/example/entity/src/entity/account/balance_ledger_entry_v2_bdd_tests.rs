use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{AggregateRole, Entity, FinancialClassification};

use super::balance::Balance;
use super::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error, BalanceLedgerOperation,
};
use super::{BalanceLedgerReason, SettlementTransferPurpose};

fn order_reason() -> BalanceLedgerReason {
    BalanceLedgerReason::FreezeForOrder { order_id: "order-1".to_string() }
}

fn balance() -> Balance {
    Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 100, 3)
}

fn balance_identity(balance: &Balance) -> (String, String, String) {
    (balance.account_id.clone(), balance.asset_id.clone(), balance.entity_id())
}

fn assert_balance_update_matches_entry(
    before: Balance,
    after: &Balance,
    entry: &BalanceLedgerEntryV2,
) {
    assert!(entry.matches_balance_update(&UpdatedEntityPair { before, after: after.clone() }));
}

fn assert_balance_unchanged(balance: &Balance, available: u64, frozen: u64, version: u64) {
    assert_eq!(balance.available, available);
    assert_eq!(balance.frozen, frozen);
    assert_eq!(balance.version, version);
}

fn created_field_value(entry: &BalanceLedgerEntryV2, field_name: &str) -> Option<String> {
    entry.track_create_event().ok().and_then(|event| {
        event.field_changes.into_iter().find_map(|change| {
            if change.field_name_as_str().ok() == Some(field_name) {
                Some(String::from_utf8_lossy(change.new_value_bytes()).into_owned())
            } else {
                None
            }
        })
    })
}

fn derived_deltas(operation: BalanceLedgerOperation, amount: u64) -> (String, String) {
    let amount = amount as i128;
    let available_delta = match operation {
        BalanceLedgerOperation::Freeze => -amount,
        BalanceLedgerOperation::Unfreeze => amount,
        BalanceLedgerOperation::CreditAvailable => amount,
        BalanceLedgerOperation::DebitAvailable => -amount,
        BalanceLedgerOperation::DebitFrozen => 0,
    };
    let frozen_delta = match operation {
        BalanceLedgerOperation::Freeze => amount,
        BalanceLedgerOperation::Unfreeze => -amount,
        BalanceLedgerOperation::CreditAvailable => 0,
        BalanceLedgerOperation::DebitAvailable => 0,
        BalanceLedgerOperation::DebitFrozen => -amount,
    };
    (available_delta.to_string(), frozen_delta.to_string())
}

#[test]
fn freeze_constructs_identity_operation_amount_then_applies_to_balance() {
    let before = balance();
    let mut balance = before.clone();
    let (account_id, asset_id, balance_entity_id) = balance_identity(&balance);

    let mut entry = BalanceLedgerEntryV2::freeze(
        "ledger-1".to_string(),
        account_id,
        asset_id,
        balance_entity_id,
        200,
        order_reason(),
    )
    .unwrap();

    assert_eq!(BalanceLedgerEntryV2::aggregate_role(), AggregateRole::AggregateRoot);
    assert_eq!(
        BalanceLedgerEntryV2::financial_classification(),
        FinancialClassification::LedgerEntry
    );
    assert_eq!(entry.account_id, "trader-1");
    assert_eq!(entry.asset_id, "USDT");
    assert_eq!(entry.operation, BalanceLedgerOperation::Freeze);
    assert_eq!(entry.amount, 200);
    assert_eq!(entry.reason.order_id(), Some("order-1"));
    assert!(!entry.is_applied());
    assert_eq!(entry.before_available, None);
    assert_eq!(entry.before_frozen, None);
    assert_eq!(entry.after_available, None);
    assert_eq!(entry.after_frozen, None);
    assert!(!entry.matches_balance_update(&UpdatedEntityPair {
        before: before.clone(),
        after: balance.clone(),
    }));
    assert_balance_unchanged(&balance, 1_000, 100, 3);

    entry.apply_to(&mut balance).unwrap();

    assert_eq!(balance.available, 800);
    assert_eq!(balance.frozen, 300);
    assert_eq!(balance.version, 4);
    assert!(entry.is_applied());
    assert_eq!(entry.before_available, Some(1_000));
    assert_eq!(entry.before_frozen, Some(100));
    assert_eq!(entry.after_available, Some(800));
    assert_eq!(entry.after_frozen, Some(300));
    assert_balance_update_matches_entry(before, &balance, &entry);
}

#[test]
fn all_operations_apply_and_project_expected_snapshots() {
    let cases = [
        (
            BalanceLedgerOperation::Unfreeze,
            Balance::new("trader-1".to_string(), "USDT".to_string(), 500, 400, 7),
            150,
            650,
            250,
        ),
        (
            BalanceLedgerOperation::CreditAvailable,
            Balance::new("trader-1".to_string(), "USDT".to_string(), 10, 2, 1),
            5,
            15,
            2,
        ),
        (
            BalanceLedgerOperation::DebitAvailable,
            Balance::new("trader-1".to_string(), "USDT".to_string(), 900, 20, 11),
            300,
            600,
            20,
        ),
        (
            BalanceLedgerOperation::DebitFrozen,
            Balance::new("trader-1".to_string(), "USDT".to_string(), 5, 7, 4),
            3,
            5,
            4,
        ),
    ];

    for (operation, before, amount, after_available, after_frozen) in cases {
        let mut balance = before.clone();
        let (account_id, asset_id, balance_entity_id) = balance_identity(&balance);
        let reason = BalanceLedgerReason::SettleSpotTrade {
            trade_id: format!("trade-{operation:?}"),
            match_id: format!("match-{operation:?}"),
            settlement_batch_id: format!("batch-{operation:?}"),
            purpose: SettlementTransferPurpose::SpotBuyerPayQuote,
        };
        let mut entry = match operation {
            BalanceLedgerOperation::Unfreeze => BalanceLedgerEntryV2::unfreeze(
                format!("ledger-{operation:?}"),
                account_id,
                asset_id,
                balance_entity_id,
                amount,
                reason,
            ),
            BalanceLedgerOperation::CreditAvailable => BalanceLedgerEntryV2::credit_available(
                format!("ledger-{operation:?}"),
                account_id,
                asset_id,
                balance_entity_id,
                amount,
                reason,
            ),
            BalanceLedgerOperation::DebitAvailable => BalanceLedgerEntryV2::debit_available(
                format!("ledger-{operation:?}"),
                account_id,
                asset_id,
                balance_entity_id,
                amount,
                reason,
            ),
            BalanceLedgerOperation::DebitFrozen => BalanceLedgerEntryV2::debit_frozen(
                format!("ledger-{operation:?}"),
                account_id,
                asset_id,
                balance_entity_id,
                amount,
                reason,
            ),
            BalanceLedgerOperation::Freeze => unreachable!(),
        }
        .unwrap();

        let (available_delta, frozen_delta) = derived_deltas(operation, amount);
        let operation_label = operation.as_str();
        let amount_string = amount.to_string();
        assert_eq!(entry.operation, operation);
        assert_eq!(entry.amount, amount);
        assert_eq!(created_field_value(&entry, "operation").as_deref(), Some(operation_label));
        assert_eq!(created_field_value(&entry, "amount").as_deref(), Some(amount_string.as_str()));
        assert_eq!(
            created_field_value(&entry, "available_delta").as_deref(),
            Some(available_delta.as_str())
        );
        assert_eq!(
            created_field_value(&entry, "frozen_delta").as_deref(),
            Some(frozen_delta.as_str())
        );
        assert!(!entry.is_applied());

        entry.apply_to(&mut balance).unwrap();

        assert_eq!(balance.available, after_available);
        assert_eq!(balance.frozen, after_frozen);
        assert_eq!(entry.before_available, Some(before.available));
        assert_eq!(entry.before_frozen, Some(before.frozen));
        assert_eq!(entry.after_available, Some(after_available));
        assert_eq!(entry.after_frozen, Some(after_frozen));
        assert_balance_update_matches_entry(before, &balance, &entry);
    }
}

#[test]
fn zero_amount_is_rejected_at_construction() {
    let balance = balance();
    let (account_id, asset_id, balance_entity_id) = balance_identity(&balance);

    let result = BalanceLedgerEntryV2::freeze(
        "ledger-6".to_string(),
        account_id,
        asset_id,
        balance_entity_id,
        0,
        order_reason(),
    );

    assert_eq!(result, Err(BalanceLedgerEntryV2Error::InvalidAmount));
}

#[test]
fn large_amount_is_accepted_and_projected_at_construction() {
    let balance = balance();
    let (account_id, asset_id, balance_entity_id) = balance_identity(&balance);
    let amount = i64::MAX as u64 + 1;

    let entry = BalanceLedgerEntryV2::credit_available(
        "ledger-overflow".to_string(),
        account_id,
        asset_id,
        balance_entity_id,
        amount,
        order_reason(),
    )
    .unwrap();

    let (available_delta, frozen_delta) =
        derived_deltas(BalanceLedgerOperation::CreditAvailable, amount);
    let amount_string = amount.to_string();
    assert_eq!(entry.operation, BalanceLedgerOperation::CreditAvailable);
    assert_eq!(entry.amount, amount);
    assert_eq!(created_field_value(&entry, "operation").as_deref(), Some("credit_available"));
    assert_eq!(created_field_value(&entry, "amount").as_deref(), Some(amount_string.as_str()));
    assert_eq!(created_field_value(&entry, "command").as_deref(), Some("credit_available"));
    assert_eq!(
        created_field_value(&entry, "command_amount").as_deref(),
        Some(amount_string.as_str())
    );
    assert_eq!(
        created_field_value(&entry, "available_delta").as_deref(),
        Some(available_delta.as_str())
    );
    assert_eq!(created_field_value(&entry, "frozen_delta").as_deref(), Some(frozen_delta.as_str()));
}

#[test]
fn apply_failure_keeps_balance_and_entry_unapplied() {
    let mut balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 100, 0, 1);
    let (account_id, asset_id, balance_entity_id) = balance_identity(&balance);
    let mut entry = BalanceLedgerEntryV2::freeze(
        "ledger-7".to_string(),
        account_id,
        asset_id,
        balance_entity_id,
        200,
        order_reason(),
    )
    .unwrap();

    let result = entry.apply_to(&mut balance);

    assert_eq!(result, Err(BalanceLedgerEntryV2Error::InsufficientAvailableBalance));
    assert_balance_unchanged(&balance, 100, 0, 1);
    assert!(!entry.is_applied());
    assert_eq!(entry.before_available, None);
    assert_eq!(entry.before_frozen, None);
    assert_eq!(entry.after_available, None);
    assert_eq!(entry.after_frozen, None);
}

#[test]
fn apply_rejects_identity_mismatch_without_mutating_balance() {
    let original = balance();
    let (account_id, asset_id, balance_entity_id) = balance_identity(&original);
    let mut entry = BalanceLedgerEntryV2::freeze(
        "ledger-mismatch".to_string(),
        account_id,
        asset_id,
        balance_entity_id,
        200,
        order_reason(),
    )
    .unwrap();
    let mut other = Balance::new("other".to_string(), "USDT".to_string(), 1_000, 100, 3);

    let result = entry.apply_to(&mut other);

    assert_eq!(result, Err(BalanceLedgerEntryV2Error::BalanceIdentityMismatch));
    assert_balance_unchanged(&other, 1_000, 100, 3);
    assert!(!entry.is_applied());
}

#[test]
fn duplicate_apply_is_rejected_without_second_mutation() {
    let mut balance = balance();
    let before = balance.clone();
    let (account_id, asset_id, balance_entity_id) = balance_identity(&balance);
    let mut entry = BalanceLedgerEntryV2::freeze(
        "ledger-duplicate".to_string(),
        account_id,
        asset_id,
        balance_entity_id,
        200,
        order_reason(),
    )
    .unwrap();

    entry.apply_to(&mut balance).unwrap();
    let result = entry.apply_to(&mut balance);

    assert_eq!(result, Err(BalanceLedgerEntryV2Error::AlreadyApplied));
    assert_eq!(balance.available, 800);
    assert_eq!(balance.frozen, 300);
    assert_eq!(balance.version, 4);
    assert_balance_update_matches_entry(before, &balance, &entry);
}

#[test]
fn version_overflow_apply_failure_keeps_entry_unapplied() {
    let mut balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 100, 50, u64::MAX);
    let (account_id, asset_id, balance_entity_id) = balance_identity(&balance);
    let mut entry = BalanceLedgerEntryV2::debit_available(
        "ledger-9".to_string(),
        account_id,
        asset_id,
        balance_entity_id,
        10,
        order_reason(),
    )
    .unwrap();

    let result = entry.apply_to(&mut balance);

    assert_eq!(result, Err(BalanceLedgerEntryV2Error::ArithmeticOverflow));
    assert_balance_unchanged(&balance, 100, 50, u64::MAX);
    assert!(!entry.is_applied());
}

#[test]
fn pending_create_event_projects_operation_amount_and_compatibility_fields() {
    let balance = balance();
    let (account_id, asset_id, balance_entity_id) = balance_identity(&balance);
    let entry = BalanceLedgerEntryV2::freeze(
        "ledger-10".to_string(),
        account_id,
        asset_id,
        balance_entity_id,
        200,
        BalanceLedgerReason::ReserveForImmediateOrder { order_id: "order-10".to_string() },
    )
    .unwrap();

    let event = entry.track_create_event().unwrap();

    assert!(event.is_created());
    assert_eq!(created_field_value(&entry, "operation").as_deref(), Some("freeze"));
    assert_eq!(created_field_value(&entry, "amount").as_deref(), Some("200"));
    assert_eq!(created_field_value(&entry, "command").as_deref(), Some("freeze"));
    assert_eq!(created_field_value(&entry, "command_amount").as_deref(), Some("200"));
    assert_eq!(created_field_value(&entry, "available_delta").as_deref(), Some("-200"));
    assert_eq!(created_field_value(&entry, "frozen_delta").as_deref(), Some("200"));
    assert_eq!(created_field_value(&entry, "before_available").as_deref(), Some(""));
    assert_eq!(created_field_value(&entry, "before_frozen").as_deref(), Some(""));
    assert_eq!(created_field_value(&entry, "after_available").as_deref(), Some(""));
    assert_eq!(created_field_value(&entry, "after_frozen").as_deref(), Some(""));
    assert_eq!(
        created_field_value(&entry, "reason").as_deref(),
        Some("reserve_for_immediate_order")
    );
}

#[test]
fn applied_create_event_projects_snapshots_and_spot_settlement_purpose() {
    let mut balance = Balance::new("buyer".to_string(), "USDT".to_string(), 0, 200, 2);
    let (account_id, asset_id, balance_entity_id) = balance_identity(&balance);
    let mut entry = BalanceLedgerEntryV2::debit_frozen(
        "ledger-11".to_string(),
        account_id,
        asset_id,
        balance_entity_id,
        200,
        BalanceLedgerReason::SettleSpotTrade {
            trade_id: "trade-11".to_string(),
            match_id: "match-11".to_string(),
            settlement_batch_id: "batch-11".to_string(),
            purpose: SettlementTransferPurpose::SpotBuyerPayQuote,
        },
    )
    .unwrap();

    entry.apply_to(&mut balance).unwrap();

    assert_eq!(created_field_value(&entry, "operation").as_deref(), Some("debit_frozen"));
    assert_eq!(created_field_value(&entry, "amount").as_deref(), Some("200"));
    assert_eq!(created_field_value(&entry, "before_available").as_deref(), Some("0"));
    assert_eq!(created_field_value(&entry, "before_frozen").as_deref(), Some("200"));
    assert_eq!(created_field_value(&entry, "after_available").as_deref(), Some("0"));
    assert_eq!(created_field_value(&entry, "after_frozen").as_deref(), Some("0"));
    assert_eq!(
        created_field_value(&entry, "reason_settlement_leg").as_deref(),
        Some("spot_buyer_pay_quote")
    );
}
