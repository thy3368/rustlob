use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};
use proptest::prelude::*;

use super::spot_order_scenarios::{
    SpotOrderCancelScenario, cancelable_spot_order_scenario_strategy,
    spot_order_cancel_scenario_strategy,
};
use super::*;
use crate::entity::{SpotOrderStatus, SpotOrderStatusReason};

/// 撤单 given state 场景枚举。
///
/// 这里组合两类状态：
/// - 没有加载到订单；
/// - 已加载到某种 active `SpotOrderV2` 状态，具体订单状态枚举放在
///   `spot_order_scenarios.rs`。
#[derive(Debug, Clone)]
enum CancelSpotOrderGivenStateScenario {
    /// 按 asset + OID 没有加载到开放订单。
    MissingOpenOrder,
    /// 已加载到订单，由独立文件枚举订单状态。
    SpotOrderV2(SpotOrderCancelScenario),
}

impl CancelSpotOrderGivenStateScenario {
    fn state(&self) -> CancelSpotOrderState {
        match self {
            Self::MissingOpenOrder => CancelSpotOrderState {
                open_order: None,
                reservation: None,
                account_id: "trader-1".to_string(),
                base_balance: Balance {
                    account_id: "trader-1".to_string(),
                    asset_id: "BTC".to_string(),
                    available: 0,
                    frozen: 0,
                    entry_notional: None,
                    identifier: None,
                    version: 3,
                },
                quote_balance: Balance {
                    account_id: "trader-1".to_string(),
                    asset_id: "USDT".to_string(),
                    available: 0,
                    frozen: 0,
                    entry_notional: None,
                    identifier: None,
                    version: 3,
                },
            },
            Self::SpotOrderV2(scenario) => scenario.state(),
        }
    }

    fn expected_validate(&self) -> Result<(), CancelSpotOrderError> {
        match self {
            Self::MissingOpenOrder => Err(CancelSpotOrderError::OrderNotFound),
            Self::SpotOrderV2(scenario) => scenario.expected_validate(),
        }
    }
}

fn given_state_scenario_strategy() -> impl Strategy<Value = CancelSpotOrderGivenStateScenario> {
    prop_oneof![
        Just(CancelSpotOrderGivenStateScenario::MissingOpenOrder),
        spot_order_cancel_scenario_strategy()
            .prop_map(CancelSpotOrderGivenStateScenario::SpotOrderV2),
    ]
}

fn cancelable_given_state_scenario_strategy()
-> impl Strategy<Value = CancelSpotOrderGivenStateScenario> {
    cancelable_spot_order_scenario_strategy()
        .prop_map(CancelSpotOrderGivenStateScenario::SpotOrderV2)
}

fn cmd() -> CancelSpotOrderCmd {
    CancelSpotOrderCmd { party_id: "trader-1".to_string(), asset: 10_001, order_id: 42 }
}

fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        std::str::from_utf8(change.new_value_bytes()).ok()
    })
}

proptest! {
    #[test]
    fn given_state_scenarios_validate_loaded_state_rules(
        scenario in given_state_scenario_strategy(),
    ) {
        let use_case = CancelSpotOrderUseCase;
        let state = scenario.state();
        let result = use_case.validate_against_state(&cmd(), &state);

        prop_assert_eq!(result, scenario.expected_validate());
    }

    #[test]
    fn cancelable_spot_order_scenarios_emit_order_cancel_update_and_account_release_events(
        scenario in cancelable_given_state_scenario_strategy(),
    ) {
        let use_case = CancelSpotOrderUseCase;
        let state = scenario.state();
        let events = use_case
            .compute_changes(&cmd(), state)
            .expect("cancelable stored order state should produce events")
            .to_replayable_events()
            .expect("cancelable stored order state should project events");

        prop_assert_eq!(events.len(), 4);
        prop_assert!(events[0].is_updated());
        prop_assert!(events[1].is_updated());
        prop_assert!(events[2].is_created());
        prop_assert!(events[3].is_updated());
        prop_assert!(events[4].is_created());
        prop_assert_eq!(event_field(&events[0], "status"), Some(SpotOrderStatus::Canceled.as_str()));
        prop_assert_eq!(
            event_field(&events[0], "status_reason"),
            Some(SpotOrderStatusReason::CanceledByUser.as_str())
        );

        let expected_release = match scenario {
            CancelSpotOrderGivenStateScenario::SpotOrderV2(scenario) => {
                scenario
                    .expected_account_release()
                    .expect("cancelable strategy only generates releasable orders")
            }
            CancelSpotOrderGivenStateScenario::MissingOpenOrder => {
                unreachable!("cancelable strategy always wraps a spot order scenario")
            }
        };
        let next_available = expected_release.next_available.to_string();

        prop_assert_eq!(
            event_field(&events[3], expected_release.available_field),
            Some(next_available.as_str())
        );
        prop_assert_eq!(event_field(&events[3], expected_release.frozen_field), Some("0"));
        prop_assert!(event_field(&events[2], "close_reason").is_some());
        prop_assert!(event_field(&events[4], "reason_order_id").is_some());
    }
}
