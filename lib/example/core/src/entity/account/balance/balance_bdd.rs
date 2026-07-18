use super::{Balance, BalanceError};

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

fn assert_business_facts_preserved(before: &Balance, after: &Balance) {
    assert_eq!(after.account_id, before.account_id);
    assert_eq!(after.asset_id, before.asset_id);
    assert_eq!(after.entry_notional(), before.entry_notional());
    assert_eq!(after.identifier(), before.identifier());
    assert!(after.matches_business_snapshot(&Balance::new_with_snapshot_facts(
        before.account_id.clone(),
        before.asset_id.clone(),
        after.available,
        after.frozen,
        before.entry_notional(),
        before.identifier().map(str::to_string),
        after.version,
    )));
}

#[test]
fn reserve_moves_available_to_frozen_and_conserves_total() {
    let before = balance();

    let after = before.reserve(300).unwrap();

    assert_eq!(after.available, 700);
    assert_eq!(after.frozen, 500);
    assert_eq!(after.total(), before.total());
    assert_eq!(after.version, before.version + 1);
    assert_business_facts_preserved(&before, &after);
}

#[test]
fn reserve_rejects_invalid_or_impossible_amounts() {
    assert_eq!(balance().reserve(0), Err(BalanceError::InvalidAmount));
    assert_eq!(balance().reserve(1_001), Err(BalanceError::InsufficientAvailableBalance));

    let before = Balance::new("trader-1".to_string(), "USDT".to_string(), 1, u64::MAX, 3);
    assert_eq!(before.reserve(1), Err(BalanceError::ArithmeticOverflow));
}

#[test]
fn release_moves_frozen_to_available_and_conserves_total() {
    let before = balance();

    let after = before.release(150).unwrap();

    assert_eq!(after.available, 1_150);
    assert_eq!(after.frozen, 50);
    assert_eq!(after.total(), before.total());
    assert_eq!(after.version, before.version + 1);
    assert_business_facts_preserved(&before, &after);
}

#[test]
fn release_rejects_invalid_or_impossible_amounts() {
    assert_eq!(balance().release(0), Err(BalanceError::InvalidAmount));
    assert_eq!(balance().release(201), Err(BalanceError::InsufficientFrozenBalance));

    let before = Balance::new("trader-1".to_string(), "USDT".to_string(), u64::MAX, 1, 3);
    assert_eq!(before.release(1), Err(BalanceError::ArithmeticOverflow));
}

#[test]
fn credit_available_increases_available() {
    let before = balance();

    let after = before.credit_available(400).unwrap();

    assert_eq!(after.available, 1_400);
    assert_eq!(after.frozen, before.frozen);
    assert_eq!(after.version, before.version + 1);
    assert_business_facts_preserved(&before, &after);
}

#[test]
fn credit_available_rejects_zero_and_overflow() {
    assert_eq!(balance().credit_available(0), Err(BalanceError::InvalidAmount));

    let before = Balance::new("trader-1".to_string(), "USDT".to_string(), u64::MAX, 0, 3);
    assert_eq!(before.credit_available(1), Err(BalanceError::ArithmeticOverflow));
}

#[test]
fn debit_available_reduces_available() {
    let before = balance();

    let after = before.debit_available(400).unwrap();

    assert_eq!(after.available, 600);
    assert_eq!(after.frozen, before.frozen);
    assert_eq!(after.version, before.version + 1);
    assert_business_facts_preserved(&before, &after);
}

#[test]
fn debit_available_rejects_zero_and_insufficient_balance() {
    assert_eq!(balance().debit_available(0), Err(BalanceError::InvalidAmount));
    assert_eq!(balance().debit_available(1_001), Err(BalanceError::InsufficientAvailableBalance));
}

#[test]
fn debit_frozen_reduces_frozen() {
    let before = balance();

    let after = before.debit_frozen(80).unwrap();

    assert_eq!(after.available, before.available);
    assert_eq!(after.frozen, 120);
    assert_eq!(after.version, before.version + 1);
    assert_business_facts_preserved(&before, &after);
}

#[test]
fn debit_frozen_rejects_zero_and_insufficient_frozen_balance() {
    assert_eq!(balance().debit_frozen(0), Err(BalanceError::InvalidAmount));
    assert_eq!(balance().debit_frozen(201), Err(BalanceError::InsufficientFrozenBalance));
}

#[test]
fn behavior_methods_reject_version_overflow() {
    let before = Balance::new("trader-1".to_string(), "USDT".to_string(), 10, 10, u64::MAX);

    assert_eq!(before.reserve(1), Err(BalanceError::ArithmeticOverflow));
    assert_eq!(before.release(1), Err(BalanceError::ArithmeticOverflow));
    assert_eq!(before.credit_available(1), Err(BalanceError::ArithmeticOverflow));
    assert_eq!(before.debit_available(1), Err(BalanceError::ArithmeticOverflow));
    assert_eq!(before.debit_frozen(1), Err(BalanceError::ArithmeticOverflow));
}
