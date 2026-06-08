use cmd_handler::use_case_def2::CommandUseCase2;
use proptest::prelude::*;

use super::command_examples::{ImmediateCommandExample, supported_command_examples};
use super::test_support::{event_field, sample_cmd, sample_state, state_with_balances};
use super::*;
use crate::use_case::support::field_as_u64;

#[derive(Debug, Clone)]
struct ImmediateHappyPathCase {
    scenario: ImmediateCommandExample,
    cmd: PlaceImmediateOrderCmd,
    state: PlaceImmediateOrderState,
    reserved_quote: u64,
}

fn required_happy_path_scenarios(
    base_cmd: PlaceImmediateOrderCmd,
    state: PlaceImmediateOrderState,
    reserved_quote: u64,
) -> Vec<ImmediateHappyPathCase> {
    // 覆盖充分性判断：
    // command 业务组合由 `command_examples.rs` 单独枚举；本文件只验证这些成功组合
    // 进入 `compute_replayable_events` 后，是否完整产出订单创建事件和账户冻结事件。
    supported_command_examples(base_cmd)
        .into_iter()
        .map(|(scenario, cmd)| {
            let scenario_reserved_quote =
                cmd.execution.reserve_price().unwrap_or_default() * cmd.size;
            let mut scenario_state = state.clone();
            if scenario_reserved_quote > reserved_quote {
                scenario_state.account.available_quote += scenario_reserved_quote - reserved_quote;
            }

            ImmediateHappyPathCase {
                scenario,
                cmd,
                state: scenario_state,
                reserved_quote: scenario_reserved_quote,
            }
        })
        .collect()
}

fn required_happy_path_compute_cases_strategy() -> impl Strategy<Value = Vec<ImmediateHappyPathCase>>
{
    (
        1_u64..=1_000_000,
        1_u64..=1_000_000,
        0_u64..=1_000_000,
        0_u64..=1_000_000,
        0_u64..u64::MAX,
        any::<u16>(),
    )
        .prop_map(|(price, size, extra_available, frozen_quote, version, asset_suffix)| {
            let reserved_quote = price * size;
            let cmd = PlaceImmediateOrderCmd {
                asset: 10_000 + u32::from(asset_suffix),
                size,
                execution: PlaceImmediateOrderExecution::Limit {
                    price,
                    time_in_force: PlaceOrderTimeInForce::Gtc,
                },
                ..sample_cmd()
            };
            let state =
                state_with_balances(reserved_quote + extra_available, frozen_quote, version);

            required_happy_path_scenarios(cmd, state, reserved_quote)
        })
}

#[test]
fn role_is_trader() {
    let use_case = PlaceImmediateOrderUseCase;
    assert_eq!(use_case.role(), "Trader");
}

#[test]
fn compute_replayable_events_produces_order_and_account_events() -> Result<(), PlaceOrderError> {
    let use_case = PlaceImmediateOrderUseCase;
    let events = use_case.compute_replayable_events(&sample_cmd(), sample_state())?;

    assert_eq!(events.len(), 2);
    assert!(events[0].is_created());
    assert!(events[1].is_updated());
    assert_eq!(field_as_u64(&events[0], "order_sequence"), Some(7));
    assert_eq!(field_as_u64(&events[0], "reserved_quote"), Some(200));
    assert_eq!(field_as_u64(&events[1], "available_quote"), Some(800));
    assert_eq!(field_as_u64(&events[1], "frozen_quote"), Some(200));

    Ok(())
}

proptest! {
    #[test]
    fn property_happy_path_scenarios_prove_command_and_event_completeness(
        cases in required_happy_path_compute_cases_strategy(),
    ) {
        let use_case = PlaceImmediateOrderUseCase;

        prop_assert_eq!(cases.len(), 8);
        for scenario in ImmediateCommandExample::ALL {
            prop_assert!(cases.iter().any(|case| case.scenario == scenario));
        }

        for case in cases {
            let cmd = case.cmd;
            let state = case.state;
            let reserved_quote = case.reserved_quote;
            let expected_available = state.account.available_quote - reserved_quote;
            let expected_frozen = state.account.frozen_quote + reserved_quote;
            let expected_order_id = format!(
                "{}-{}-{}",
                cmd.party_id,
                cmd.symbol,
                state.next_order_sequence
            );
            let expected_price = match cmd.execution {
                PlaceImmediateOrderExecution::Limit { price, .. } => price.to_string(),
                PlaceImmediateOrderExecution::Market { .. } => String::new(),
            };
            let expected_size = cmd.size.to_string();
            let expected_reserved = reserved_quote.to_string();
            let expected_available = expected_available.to_string();
            let expected_frozen = expected_frozen.to_string();
            let expected_tif = cmd.execution.stored_time_in_force().as_str();
            let expected_cloid = cmd.cloid.clone().unwrap_or_default();

            prop_assert_eq!(use_case.pre_check_command(&cmd), Ok(()));

            let events = use_case.compute_replayable_events(&cmd, state)?;

            prop_assert_eq!(events.len(), 2);
            prop_assert!(events[0].is_created());
            prop_assert!(events[1].is_updated());
            prop_assert_eq!(
                event_field(&events[0], "order_id"),
                Some(expected_order_id.as_str())
            );
            prop_assert_eq!(event_field(&events[0], "account_id"), Some(cmd.party_id.as_str()));
            prop_assert_eq!(event_field(&events[0], "symbol"), Some(cmd.symbol.as_str()));
            prop_assert_eq!(event_field(&events[0], "side"), Some("buy"));
            prop_assert_eq!(event_field(&events[0], "order_kind"), Some("immediate"));
            prop_assert_eq!(
                event_field(&events[0], "execution"),
                Some(case.scenario.expected_execution())
            );
            prop_assert_eq!(event_field(&events[0], "time_in_force"), Some(expected_tif));
            prop_assert_eq!(event_field(&events[0], "price"), Some(expected_price.as_str()));
            prop_assert_eq!(event_field(&events[0], "qty"), Some(expected_size.as_str()));
            prop_assert_eq!(
                event_field(&events[0], "reserved_quote"),
                Some(expected_reserved.as_str())
            );
            prop_assert_eq!(
                event_field(&events[0], "client_order_id"),
                Some(expected_cloid.as_str())
            );
            prop_assert_eq!(field_as_u64(&events[0], "order_sequence"), Some(7));
            prop_assert_eq!(
                event_field(&events[1], "available_quote"),
                Some(expected_available.as_str())
            );
            prop_assert_eq!(
                event_field(&events[1], "frozen_quote"),
                Some(expected_frozen.as_str())
            );
        }
    }
}
