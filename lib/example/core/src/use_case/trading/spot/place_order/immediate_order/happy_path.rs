use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};
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
    reserved_base: u64,
    reserved_quote: u64,
}

fn required_happy_path_scenarios(
    base_cmd: PlaceImmediateOrderCmd,
    state: PlaceImmediateOrderState,
    reserved_quote: u64,
) -> Vec<ImmediateHappyPathCase> {
    // 覆盖充分性判断：
    // command 业务组合由 `command_examples.rs` 单独枚举；本文件只验证这些成功组合
    // 进入 `compute_output_and_events` 后，是否完整产出订单创建事件和账户冻结事件。
    supported_command_examples(base_cmd)
        .into_iter()
        .map(|(scenario, cmd)| {
            let scenario_reserved_base =
                if scenario.expected_side() == PlaceOrderSide::Sell { cmd.size } else { 0 };
            let scenario_reserved_quote = if scenario.expected_side() == PlaceOrderSide::Buy {
                cmd.execution.reserve_price().unwrap_or_default() * cmd.size
            } else {
                0
            };
            let mut scenario_state = state.clone();
            if scenario_reserved_quote > reserved_quote {
                scenario_state.quote_balance.available += scenario_reserved_quote - reserved_quote;
            }
            if scenario_reserved_base > scenario_state.base_balance.available {
                scenario_state.base_balance.available = scenario_reserved_base;
            }

            ImmediateHappyPathCase {
                scenario,
                cmd,
                state: scenario_state,
                reserved_base: scenario_reserved_base,
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
fn compute_output_and_events_produces_order_and_account_events() -> Result<(), PlaceOrderError> {
    let use_case = PlaceImmediateOrderUseCase;
    let result = use_case.compute_changes(&sample_cmd(), sample_state())?;
    let events = result.to_replayable_events().map_err(|_| PlaceOrderError::ArithmeticOverflow)?;

    assert_eq!(events.len(), 5);
    assert!(events[0].is_created());
    assert!(events[1].is_created());
    assert!(events[2].is_created());
    assert!(events[3].is_updated());
    assert!(events[4].is_created());
    assert_eq!(event_field(&events[0], "order_id"), Some("trader-1-BTCUSDT-7"));
    assert_eq!(field_as_u64(&events[0], "asset"), Some(10_001));
    assert_eq!(field_as_u64(&events[0], "reserved_quote"), Some(200));
    assert_eq!(event_field(&events[1], "reservation_id"), Some("reservation:trader-1-BTCUSDT-7"));
    assert_eq!(event_field(&events[2], "reservation_id"), Some("reservation:trader-1-BTCUSDT-7"));
    assert_eq!(event_field(&events[3], "asset_id"), Some("USDT"));
    assert_eq!(field_as_u64(&events[3], "available"), Some(800));
    assert_eq!(field_as_u64(&events[3], "frozen"), Some(200));
    assert_eq!(event_field(&events[4], "reason"), Some("reserve_for_immediate_order"));
    assert_eq!(event_field(&events[4], "reason_order_id"), Some("trader-1-BTCUSDT-7"));
    assert_eq!(event_field(&events[4], "balance_entity_id"), Some("trader-1:USDT"));

    Ok(())
}

#[test]
fn buy_order_updates_quote_balance_and_creates_matching_ledger_entry() -> Result<(), PlaceOrderError>
{
    let use_case = PlaceImmediateOrderUseCase;
    let state = sample_state();

    let result = CommandUseCase4::compute_changes(&use_case, &sample_cmd(), state)?;
    let events = result.to_replayable_events().map_err(|_| PlaceOrderError::ArithmeticOverflow)?;

    assert_eq!(result.created_order.order_id, "trader-1-BTCUSDT-7");
    assert_eq!(result.created_order.reserved_quote, 200);
    assert_eq!(result.updated_balance.after.asset_id, "USDT");
    assert_eq!(result.updated_balance.after.available, 800);
    assert_eq!(result.updated_balance.after.frozen, 200);
    assert_eq!(
        result.created_balance_ledger_entry.reason,
        crate::BalanceLedgerReason::ReserveForImmediateOrder {
            order_id: result.created_order.order_id.clone(),
        }
    );
    assert!(result.created_balance_ledger_entry.matches_balance_update(&result.updated_balance));
    assert_eq!(event_field(&events[0], "order_id"), Some(result.created_order.order_id.as_str()));
    assert_eq!(
        event_field(&events[3], "asset_id"),
        Some(result.updated_balance.after.asset_id.as_str())
    );
    assert_eq!(field_as_u64(&events[3], "available"), Some(result.updated_balance.after.available));
    assert_eq!(field_as_u64(&events[3], "frozen"), Some(result.updated_balance.after.frozen));
    assert_eq!(
        event_field(&events[4], "reason_order_id"),
        Some(result.created_order.order_id.as_str())
    );

    Ok(())
}

#[test]
fn sell_order_updates_base_balance_and_creates_matching_ledger_entry() -> Result<(), PlaceOrderError>
{
    let use_case = PlaceImmediateOrderUseCase;
    let mut cmd = sample_cmd();
    cmd.is_buy = false;

    let result = CommandUseCase4::compute_changes(&use_case, &cmd, sample_state())?;

    assert_eq!(result.created_order.reserved_base, 2);
    assert_eq!(result.created_order.reserved_quote, 0);
    assert_eq!(result.updated_balance.after.asset_id, "BTC");
    assert_eq!(result.updated_balance.after.available, 998);
    assert_eq!(result.updated_balance.after.frozen, 2);
    assert_eq!(
        result.created_balance_ledger_entry.reason,
        crate::BalanceLedgerReason::ReserveForImmediateOrder {
            order_id: result.created_order.order_id.clone(),
        }
    );
    assert!(result.created_balance_ledger_entry.matches_balance_update(&result.updated_balance));

    Ok(())
}

#[test]
fn replayable_events_follow_order_then_balance_then_ledger() -> Result<(), PlaceOrderError> {
    let use_case = PlaceImmediateOrderUseCase;
    let changes = CommandUseCase4::compute_changes(&use_case, &sample_cmd(), sample_state())?;
    let events = changes.to_replayable_events().map_err(|_| PlaceOrderError::ArithmeticOverflow)?;

    assert_eq!(events.len(), 5);
    assert_eq!(event_field(&events[0], "order_id"), Some(changes.created_order.order_id.as_str()));
    assert_eq!(
        event_field(&events[1], "reservation_id"),
        Some(changes.created_reservation.reservation_id.as_str())
    );
    assert_eq!(
        event_field(&events[3], "asset_id"),
        Some(changes.updated_balance.after.asset_id.as_str())
    );
    assert_eq!(
        event_field(&events[4], "entry_id"),
        Some(changes.created_balance_ledger_entry.entry_id.as_str())
    );

    Ok(())
}

proptest! {
    #[test]
    fn property_happy_path_scenarios_prove_command_and_event_completeness(
        cases in required_happy_path_compute_cases_strategy(),
    ) {
        let use_case = PlaceImmediateOrderUseCase;

        prop_assert_eq!(cases.len(), 16);
        for scenario in ImmediateCommandExample::ALL {
            prop_assert!(cases.iter().any(|case| case.scenario == scenario));
        }

        for case in cases {
            let cmd = case.cmd;
            let state = case.state;
            let reserved_base = case.reserved_base;
            let reserved_quote = case.reserved_quote;
            let expected_available_base = state.base_balance.available - reserved_base;
            let expected_frozen_base = state.base_balance.frozen + reserved_base;
            let expected_available_quote = state.quote_balance.available - reserved_quote;
            let expected_frozen_quote = state.quote_balance.frozen + reserved_quote;
            let expected_order_id = format!(
                "{}-{}-{}",
                cmd.party_id,
                cmd.symbol,
                state.next_order_sequence
            );
            let expected_reservation_id = format!("reservation:{expected_order_id}");
            let expected_price = match cmd.execution {
                PlaceImmediateOrderExecution::Limit { price, .. } => price.to_string(),
                PlaceImmediateOrderExecution::Market { aggressive_price } => {
                    aggressive_price.to_string()
                }
            };
            let expected_asset = cmd.asset.to_string();
            let expected_size = cmd.size.to_string();
            let expected_reserved_base = reserved_base.to_string();
            let expected_reserved = reserved_quote.to_string();
            let expected_available_base = expected_available_base.to_string();
            let expected_frozen_base = expected_frozen_base.to_string();
            let expected_available_quote = expected_available_quote.to_string();
            let expected_frozen_quote = expected_frozen_quote.to_string();
            let expected_tif = cmd.execution.stored_time_in_force().as_str();
            let expected_cloid = cmd.cloid.clone().unwrap_or_default();
            let expected_side = case.scenario.expected_side().as_str();

            prop_assert_eq!(use_case.pre_check_command(&cmd), Ok(()));

            let result = use_case.compute_changes(&cmd, state.clone())?;
            let events = result.to_replayable_events().map_err(|_| PlaceOrderError::ArithmeticOverflow)?;

            prop_assert_eq!(events.len(), 5);
            prop_assert!(events[0].is_created());
            prop_assert!(events[1].is_created());
            prop_assert!(events[2].is_created());
            prop_assert!(events[3].is_updated());
            prop_assert!(events[4].is_created());
            prop_assert_eq!(
                event_field(&events[0], "order_id"),
                Some(expected_order_id.as_str())
            );
            prop_assert_eq!(event_field(&events[0], "account_id"), Some(cmd.party_id.as_str()));
            prop_assert_eq!(event_field(&events[0], "asset"), Some(expected_asset.as_str()));
            prop_assert_eq!(event_field(&events[0], "symbol"), Some(cmd.symbol.as_str()));
            prop_assert_eq!(event_field(&events[0], "side"), Some(expected_side));
            prop_assert_eq!(
                event_field(&events[0], "execution"),
                Some(case.scenario.expected_execution())
            );
            prop_assert_eq!(event_field(&events[0], "time_in_force"), Some(expected_tif));
            prop_assert_eq!(event_field(&events[0], "price"), Some(expected_price.as_str()));
            prop_assert_eq!(event_field(&events[0], "qty"), Some(expected_size.as_str()));
            prop_assert_eq!(
                event_field(&events[0], "reserved_base"),
                Some(expected_reserved_base.as_str())
            );
            prop_assert_eq!(
                event_field(&events[0], "reserved_quote"),
                Some(expected_reserved.as_str())
            );
            prop_assert_eq!(
                event_field(&events[0], "client_order_id"),
                Some(expected_cloid.as_str())
            );
            prop_assert_eq!(
                event_field(&events[1], "reservation_id"),
                Some(expected_reservation_id.as_str())
            );
            prop_assert_eq!(
                event_field(&events[2], "reservation_id"),
                Some(expected_reservation_id.as_str())
            );
            prop_assert_eq!(
                event_field(&events[2], "caused_by_order_id"),
                Some(expected_order_id.as_str())
            );
            if case.scenario.expected_side() == PlaceOrderSide::Buy {
                prop_assert_eq!(
                    event_field(&events[3], "asset_id"),
                    Some("USDT")
                );
                prop_assert_eq!(
                    event_field(&events[3], "available"),
                    Some(expected_available_quote.as_str())
                );
                prop_assert_eq!(
                    event_field(&events[3], "frozen"),
                    Some(expected_frozen_quote.as_str())
                );
                prop_assert_eq!(event_field(&events[4], "balance_entity_id"), Some("trader-1:USDT"));
                prop_assert_eq!(result.updated_balance.after.asset_id.as_str(), "USDT");
                prop_assert_eq!(
                    result.updated_balance.after.available,
                    state.quote_balance.available - reserved_quote
                );
                prop_assert_eq!(
                    result.updated_balance.after.frozen,
                    state.quote_balance.frozen + reserved_quote
                );
            } else {
                prop_assert_eq!(
                    event_field(&events[3], "asset_id"),
                    Some("BTC")
                );
                prop_assert_eq!(
                    event_field(&events[3], "available"),
                    Some(expected_available_base.as_str())
                );
                prop_assert_eq!(
                    event_field(&events[3], "frozen"),
                    Some(expected_frozen_base.as_str())
                );
                prop_assert_eq!(event_field(&events[4], "balance_entity_id"), Some("trader-1:BTC"));
                prop_assert_eq!(result.updated_balance.after.asset_id.as_str(), "BTC");
                prop_assert_eq!(
                    result.updated_balance.after.available,
                    state.base_balance.available - reserved_base
                );
                prop_assert_eq!(
                    result.updated_balance.after.frozen,
                    state.base_balance.frozen + reserved_base
                );
            }
            prop_assert_eq!(
                result.created_balance_ledger_entry.reason.clone(),
                crate::BalanceLedgerReason::ReserveForImmediateOrder {
                    order_id: result.created_order.order_id.clone(),
                }
            );
            prop_assert!(result
                .created_balance_ledger_entry
                .matches_balance_update(&result.updated_balance));
            prop_assert_eq!(result.created_order.order_id, expected_order_id);
            prop_assert_eq!(
                result.created_order.client_order_id.as_deref().unwrap_or_default(),
                expected_cloid
            );
        }
    }
}
