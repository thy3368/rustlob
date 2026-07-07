use common_entity::Entity;
use proptest::prelude::*;

use super::Balance;

#[derive(Debug, Clone)]
enum BalanceScenario {
    SufficientReserve { available: u64, frozen: u64, amount: u64 },
    InsufficientReserve { available: u64, frozen: u64, amount: u64 },
    Releasable { available: u64, frozen: u64, amount: u64 },
    ReleaseOverflow { available: u64, frozen: u64, amount: u64 },
    FrozenDebit { available: u64, frozen: u64, amount: u64 },
    FrozenDebitUnderflow { available: u64, frozen: u64, amount: u64 },
    AvailableCredit { available: u64, frozen: u64, amount: u64 },
    AvailableCreditOverflow { available: u64, frozen: u64, amount: u64 },
}

impl BalanceScenario {
    fn balance(&self) -> Balance {
        let (available, frozen) = match *self {
            Self::SufficientReserve { available, frozen, .. }
            | Self::InsufficientReserve { available, frozen, .. }
            | Self::Releasable { available, frozen, .. }
            | Self::ReleaseOverflow { available, frozen, .. }
            | Self::FrozenDebit { available, frozen, .. }
            | Self::FrozenDebitUnderflow { available, frozen, .. }
            | Self::AvailableCredit { available, frozen, .. }
            | Self::AvailableCreditOverflow { available, frozen, .. } => (available, frozen),
        };

        Balance::new("trader-1".to_string(), "USDT".to_string(), available, frozen, 9)
    }

    fn amount(&self) -> u64 {
        match *self {
            Self::SufficientReserve { amount, .. }
            | Self::InsufficientReserve { amount, .. }
            | Self::Releasable { amount, .. }
            | Self::ReleaseOverflow { amount, .. }
            | Self::FrozenDebit { amount, .. }
            | Self::FrozenDebitUnderflow { amount, .. }
            | Self::AvailableCredit { amount, .. }
            | Self::AvailableCreditOverflow { amount, .. } => amount,
        }
    }
}

fn small_amounts() -> impl Strategy<Value = (u64, u64, u64)> {
    (0_u64..=1_000_000, 0_u64..=1_000_000, 0_u64..=1_000_000)
}

fn balance_scenario_strategy() -> impl Strategy<Value = BalanceScenario> {
    prop_oneof![
        (0_u64..=1_000_000, 0_u64..=1_000_000).prop_flat_map(|(available, frozen)| {
            (0_u64..=available).prop_map(move |amount| BalanceScenario::SufficientReserve {
                available,
                frozen,
                amount,
            })
        }),
        (0_u64..=1_000_000, 0_u64..=1_000_000).prop_flat_map(|(available, frozen)| {
            ((available + 1)..=(available + 1_000_000)).prop_map(move |amount| {
                BalanceScenario::InsufficientReserve { available, frozen, amount }
            })
        }),
        (0_u64..=1_000_000, 0_u64..=1_000_000).prop_flat_map(|(available, frozen)| {
            (0_u64..=frozen).prop_filter_map("release must not overflow available", move |amount| {
                available.checked_add(amount).map(|_| BalanceScenario::Releasable {
                    available,
                    frozen,
                    amount,
                })
            })
        }),
        (1_u64..=1_000_000, 0_u64..=1_000_000).prop_map(|(amount, frozen)| {
            BalanceScenario::ReleaseOverflow { available: u64::MAX - amount + 1, frozen, amount }
        }),
        small_amounts().prop_flat_map(|(available, frozen, _)| {
            (0_u64..=frozen).prop_map(move |amount| BalanceScenario::FrozenDebit {
                available,
                frozen,
                amount,
            })
        }),
        (0_u64..=1_000_000, 0_u64..=1_000_000).prop_flat_map(|(available, frozen)| {
            ((frozen + 1)..=(frozen + 1_000_000)).prop_map(move |amount| {
                BalanceScenario::FrozenDebitUnderflow { available, frozen, amount }
            })
        }),
        small_amounts().prop_filter_map(
            "credit must not overflow available",
            |(available, frozen, amount)| {
                available.checked_add(amount).map(|_| BalanceScenario::AvailableCredit {
                    available,
                    frozen,
                    amount,
                })
            }
        ),
        (1_u64..=1_000_000, 0_u64..=1_000_000).prop_map(|(amount, frozen)| {
            BalanceScenario::AvailableCreditOverflow {
                available: u64::MAX - amount + 1,
                frozen,
                amount,
            }
        }),
    ]
}

fn created_value(balance: &Balance, field_name: &str) -> Option<String> {
    balance.created_field_changes().into_iter().find_map(|change| {
        if change.field_name.as_ref() != field_name {
            return None;
        }
        Some(change.new_value)
    })
}

fn diff_value(old: &Balance, new: &Balance, field_name: &str) -> Option<String> {
    old.diff(new).into_iter().find_map(|change| {
        if change.field_name.as_ref() != field_name {
            return None;
        }
        Some(change.new_value)
    })
}

proptest! {
    #[test]
    fn reserve_release_and_settlement_amounts_follow_balance_state(
        scenario in balance_scenario_strategy(),
    ) {
        let balance = scenario.balance();
        let amount = scenario.amount();

        match scenario {
            BalanceScenario::SufficientReserve { available, frozen, amount } => {
                prop_assert!(balance.can_reserve(amount));
                prop_assert_eq!(
                    balance.reserve_after(amount),
                    Some((available - amount, frozen + amount))
                );
            }
            BalanceScenario::InsufficientReserve { .. } => {
                prop_assert!(!balance.can_reserve(amount));
                prop_assert_eq!(balance.reserve_after(amount), None);
            }
            BalanceScenario::Releasable { available, frozen, amount } => {
                prop_assert_eq!(
                    balance.release_after(amount),
                    Some((available + amount, frozen - amount))
                );
            }
            BalanceScenario::ReleaseOverflow { .. } => {
                prop_assert_eq!(balance.release_after(amount), None);
            }
            BalanceScenario::FrozenDebit { frozen, amount, .. } => {
                prop_assert_eq!(balance.debit_frozen_after(amount), Some(frozen - amount));
            }
            BalanceScenario::FrozenDebitUnderflow { .. } => {
                prop_assert_eq!(balance.debit_frozen_after(amount), None);
            }
            BalanceScenario::AvailableCredit { available, amount, .. } => {
                prop_assert_eq!(balance.credit_available_after(amount), Some(available + amount));
            }
            BalanceScenario::AvailableCreditOverflow { .. } => {
                prop_assert_eq!(balance.credit_available_after(amount), None);
            }
        }
    }

    #[test]
    fn created_fields_and_diff_keep_balance_identity_and_changed_amounts(
        available in 0_u64..=1_000_000,
        frozen in 0_u64..=1_000_000,
        next_available in 0_u64..=1_000_000,
        next_frozen in 0_u64..=1_000_000,
        version in 0_u64..=1_000_000,
    ) {
        let old = Balance::new(
            "trader-1".to_string(),
            "USDT".to_string(),
            available,
            frozen,
            version,
        );
        let mut new = old.clone();
        new.apply_after(next_available, next_frozen, version + 1);

        prop_assert_eq!(old.entity_id(), "trader-1:USDT");
        prop_assert!(matches!(old.replay_entity_id(), Ok(entity_id) if entity_id >= 0));
        prop_assert_eq!(created_value(&old, "account_id"), Some("trader-1".to_string()));
        prop_assert_eq!(created_value(&old, "asset_id"), Some("USDT".to_string()));
        prop_assert_eq!(created_value(&old, "available"), Some(available.to_string()));
        prop_assert_eq!(created_value(&old, "frozen"), Some(frozen.to_string()));
        prop_assert_eq!(created_value(&old, "entry_notional"), Some(String::new()));
        prop_assert_eq!(created_value(&old, "identifier"), Some(String::new()));

        prop_assert_eq!(diff_value(&old, &new, "account_id"), Some("trader-1".to_string()));
        prop_assert_eq!(diff_value(&old, &new, "asset_id"), Some("USDT".to_string()));
        prop_assert_eq!(
            diff_value(&old, &new, "available"),
            (available != next_available).then(|| next_available.to_string())
        );
        prop_assert_eq!(
            diff_value(&old, &new, "frozen"),
            (frozen != next_frozen).then(|| next_frozen.to_string())
        );
        prop_assert_eq!(diff_value(&old, &new, "entry_notional"), None);
        prop_assert_eq!(diff_value(&old, &new, "identifier"), None);
    }

    #[test]
    fn snapshot_facts_enter_create_and_diff_only_when_changed(
        available in 0_u64..=1_000_000,
        frozen in 0_u64..=1_000_000,
        entry_notional in proptest::option::of(0_u64..=1_000_000),
        next_entry_notional in proptest::option::of(0_u64..=1_000_000),
        identifier in proptest::option::of("[a-z0-9-]{1,16}"),
        next_identifier in proptest::option::of("[a-z0-9-]{1,16}"),
        version in 0_u64..=1_000_000,
    ) {
        let old = Balance::new_with_snapshot_facts(
            "trader-1".to_string(),
            "USDT".to_string(),
            available,
            frozen,
            entry_notional,
            identifier.clone(),
            version,
        );
        let new = Balance::new_with_snapshot_facts(
            "trader-1".to_string(),
            "USDT".to_string(),
            available,
            frozen,
            next_entry_notional,
            next_identifier.clone(),
            version + 1,
        );

        prop_assert_eq!(
            created_value(&old, "entry_notional"),
            Some(entry_notional.map(|value| value.to_string()).unwrap_or_default())
        );
        prop_assert_eq!(
            created_value(&old, "identifier"),
            Some(identifier.clone().unwrap_or_default())
        );
        prop_assert_eq!(
            diff_value(&old, &new, "entry_notional"),
            (entry_notional != next_entry_notional)
                .then(|| next_entry_notional.map(|value| value.to_string()).unwrap_or_default())
        );
        prop_assert_eq!(
            diff_value(&old, &new, "identifier"),
            (identifier != next_identifier).then(|| next_identifier.unwrap_or_default())
        );
    }
}
