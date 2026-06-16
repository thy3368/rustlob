use cmd_handler::command_use_case_def2::CommandUseCase3;

use super::test_support::{event_field, sample_cmd, sample_state};
use super::*;
use crate::use_case::support::field_as_u64;

#[test]
fn role_is_trader() {
    let use_case = PlaceConditionalOrderUseCase;
    assert_eq!(use_case.role(), "Trader");
}

#[test]
fn pre_check_accepts_market_execution() {
    let use_case = PlaceConditionalOrderUseCase;

    let result = use_case.pre_check_command(&sample_cmd());
    assert_eq!(result, Ok(()));
}

#[test]
fn validate_against_state_does_not_require_available_quote() {
    let use_case = PlaceConditionalOrderUseCase;
    let state = sample_state();

    let result = use_case.validate_against_state(&sample_cmd(), &state);
    assert_eq!(result, Ok(()));
}

#[test]
fn compute_output_and_events_only_creates_order() -> Result<(), PlaceOrderError> {
    let use_case = PlaceConditionalOrderUseCase;
    let result = use_case.compute_output_and_events(&sample_cmd(), sample_state())?;
    let events = result.events;

    assert_eq!(events.len(), 1);
    assert!(events[0].is_created());
    assert_eq!(event_field(&events[0], "trigger_order_id"), Some("trader-1-BTCUSDT-7"));
    assert_eq!(field_as_u64(&events[0], "asset"), Some(10_001));
    assert_eq!(field_as_u64(&events[0], "trigger_price"), Some(90));
    assert_eq!(event_field(&events[0], "trigger_role"), Some("stop_loss"));
    assert_eq!(event_field(&events[0], "execution"), Some("market"));
    assert_eq!(result.output.order.trigger_order_id, "trader-1-BTCUSDT-7");
    assert_eq!(result.output.order.trigger_price, 90);
    assert_eq!(result.output.order.client_order_id, None);

    Ok(())
}
