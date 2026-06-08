use cmd_handler::use_case_def2::CommandUseCase2;
use proptest::prelude::*;

use super::spot_order_scenarios::{
    SpotOrderCancelScenario, cancelable_spot_order_scenario_strategy,
    spot_order_cancel_scenario_strategy,
};
use super::*;

/// 撤单 given state 场景枚举。
///
/// 这里组合两类状态：
/// - 没有加载到订单；
/// - 已加载到某种 active `SpotOrder` 状态，具体订单状态枚举放在
///   `spot_order_scenarios.rs`。
#[derive(Debug, Clone)]
enum CancelSpotOrderGivenStateScenario {
    /// 按 asset + OID 没有加载到开放订单。
    MissingOpenOrder,
    /// 已加载到订单，由独立文件枚举订单状态。
    SpotOrder(SpotOrderCancelScenario),
}

impl CancelSpotOrderGivenStateScenario {
    fn state(&self) -> CancelSpotOrderState {
        match self {
            Self::MissingOpenOrder => CancelSpotOrderState {
                open_order: None,
                account: TradingAccount {
                    account_id: "trader-1".to_string(),
                    available_base: 0,
                    frozen_base: 0,
                    available_quote: 0,
                    frozen_quote: 0,
                    version: 3,
                },
            },
            Self::SpotOrder(scenario) => scenario.state(),
        }
    }

    fn expected_validate(&self) -> Result<(), CancelSpotOrderError> {
        match self {
            Self::MissingOpenOrder => Err(CancelSpotOrderError::OrderNotFound),
            Self::SpotOrder(scenario) => scenario.expected_validate(),
        }
    }
}

fn given_state_scenario_strategy() -> impl Strategy<Value = CancelSpotOrderGivenStateScenario> {
    prop_oneof![
        Just(CancelSpotOrderGivenStateScenario::MissingOpenOrder),
        spot_order_cancel_scenario_strategy()
            .prop_map(CancelSpotOrderGivenStateScenario::SpotOrder),
    ]
}

fn cancelable_given_state_scenario_strategy()
-> impl Strategy<Value = CancelSpotOrderGivenStateScenario> {
    cancelable_spot_order_scenario_strategy().prop_map(CancelSpotOrderGivenStateScenario::SpotOrder)
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
    fn cancelable_spot_order_scenarios_emit_order_delete_and_account_release_events(
        scenario in cancelable_given_state_scenario_strategy(),
    ) {
        let use_case = CancelSpotOrderUseCase;
        let state = scenario.state();
        let events = use_case
            .compute_replayable_events(&cmd(), state)
            .expect("cancelable stored order state should produce events");

        prop_assert_eq!(events.len(), 2);
        prop_assert!(events[0].is_deleted());
        prop_assert!(events[1].is_updated());

        let expected_release = match scenario {
            CancelSpotOrderGivenStateScenario::SpotOrder(scenario) => {
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
            event_field(&events[1], expected_release.available_field),
            Some(next_available.as_str())
        );
        prop_assert_eq!(event_field(&events[1], expected_release.frozen_field), Some("0"));
    }
}
