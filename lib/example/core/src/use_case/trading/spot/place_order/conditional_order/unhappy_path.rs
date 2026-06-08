use cmd_handler::use_case_def2::CommandUseCase2;

use super::test_support::sample_cmd;
use super::*;

#[test]
fn pre_check_rejects_zero_trigger_price() {
    let use_case = PlaceConditionalOrderUseCase;
    let mut cmd = sample_cmd();
    cmd.trigger_price = 0;

    let result = use_case.pre_check_command(&cmd);
    assert_eq!(result, Err(PlaceOrderError::InvalidTriggerPrice));
}

#[test]
fn pre_check_rejects_zero_limit_price() {
    let use_case = PlaceConditionalOrderUseCase;
    let mut cmd = sample_cmd();
    cmd.execution = PlaceOrderExecution::Limit { price: 0 };

    let result = use_case.pre_check_command(&cmd);
    assert_eq!(result, Err(PlaceOrderError::InvalidPrice));
}
