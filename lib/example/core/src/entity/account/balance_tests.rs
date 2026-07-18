use common_entity::{Entity, EntityFieldChange};
use serde_json::json;

use super::balance::Balance;

fn balance() -> Balance {
    Balance::new_with_snapshot_facts(
        "trader-1".to_string(),
        "USDT".to_string(),
        1_000,
        200,
        Some(12_345),
        Some("spot-usdt".to_string()),
        3,
    )
}

#[test]
fn query_methods_expose_account_asset_and_payment_facts() {
    let balance = balance();

    assert!(balance.belongs_to_account("trader-1"));
    assert!(!balance.belongs_to_account("trader-2"));
    assert!(balance.is_asset("USDT"));
    assert!(!balance.is_asset("BTC"));
    assert_eq!(balance.total(), Some(1_200));
    assert!(balance.can_reserve(1_000));
    assert!(!balance.can_reserve(1_001));
    assert!(balance.can_debit_available(1_000));
    assert!(!balance.can_debit_available(1_001));
}

#[test]
fn query_methods_expose_snapshot_facts() {
    let balance = balance();

    assert_eq!(balance.entry_notional(), Some(12_345));
    assert_eq!(balance.identifier(), Some("spot-usdt"));
}

#[test]
fn total_returns_none_when_sum_overflows() {
    let balance = Balance::new("trader-1".to_string(), "USDT".to_string(), u64::MAX, 1, 3);

    assert_eq!(balance.total(), None);
}

#[test]
fn entity_identifier_is_account_and_asset_pair() {
    let balance = balance();

    assert_eq!(balance.entity_id(), "trader-1:USDT");
    assert_eq!(Balance::entity_type(), 7);
    assert_eq!(balance.entity_version(), 3);
    assert!(balance.replay_entity_id().unwrap() >= 0);
}

#[test]
fn matches_business_snapshot_ignores_version() {
    let same_facts_new_version = Balance::new_with_snapshot_facts(
        "trader-1".to_string(),
        "USDT".to_string(),
        1_000,
        200,
        Some(12_345),
        Some("spot-usdt".to_string()),
        99,
    );
    let different_available = Balance::new_with_snapshot_facts(
        "trader-1".to_string(),
        "USDT".to_string(),
        999,
        200,
        Some(12_345),
        Some("spot-usdt".to_string()),
        99,
    );

    assert!(balance().matches_business_snapshot(&same_facts_new_version));
    assert!(!balance().matches_business_snapshot(&different_available));
}

#[test]
fn serde_defaults_optional_snapshot_facts_when_absent() {
    let balance: Balance = serde_json::from_value(json!({
        "account_id": "trader-1",
        "asset_id": "USDT",
        "available": 1000,
        "frozen": 50,
        "version": 3
    }))
    .unwrap();

    assert_eq!(balance.entry_notional(), None);
    assert_eq!(balance.identifier(), None);
}

#[test]
fn created_field_changes_include_snapshot_fact_fields() {
    assert_eq!(
        balance().created_field_changes(),
        vec![
            EntityFieldChange::new("account_id", "", "trader-1"),
            EntityFieldChange::new("asset_id", "", "USDT"),
            EntityFieldChange::new("available", "", "1000"),
            EntityFieldChange::new("frozen", "", "200"),
            EntityFieldChange::new("entry_notional", "", "12345"),
            EntityFieldChange::new("identifier", "", "spot-usdt"),
        ]
    );
}

#[test]
fn diff_reports_changed_business_snapshot_fields() {
    let before = balance();
    let after = Balance::new_with_snapshot_facts(
        "trader-1".to_string(),
        "USDT".to_string(),
        900,
        300,
        Some(12_000),
        Some("spot-usdt-v2".to_string()),
        4,
    );

    assert_eq!(
        before.diff(&after),
        vec![
            EntityFieldChange::new("account_id", "trader-1", "trader-1"),
            EntityFieldChange::new("asset_id", "USDT", "USDT"),
            EntityFieldChange::new("available", "1000", "900"),
            EntityFieldChange::new("frozen", "200", "300"),
            EntityFieldChange::new("entry_notional", "12345", "12000"),
            EntityFieldChange::new("identifier", "spot-usdt", "spot-usdt-v2"),
        ]
    );
}

#[test]
fn replay_field_type_covers_balance_fields() {
    assert_eq!(Balance::replay_field_type("account_id"), 0);
    assert_eq!(Balance::replay_field_type("asset_id"), 0);
    assert_eq!(Balance::replay_field_type("identifier"), 0);
    assert_eq!(Balance::replay_field_type("available"), 1);
    assert_eq!(Balance::replay_field_type("frozen"), 1);
    assert_eq!(Balance::replay_field_type("entry_notional"), 1);
    assert_eq!(Balance::replay_field_type("unknown"), 0);
}
