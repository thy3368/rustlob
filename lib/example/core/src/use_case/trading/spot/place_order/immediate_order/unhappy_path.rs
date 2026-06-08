use cmd_handler::use_case_def2::CommandUseCase2;
use proptest::prelude::*;

use super::test_support::{
    cmd_with_price_and_size, market_cmd_with_price_and_size, sample_cmd, sample_state,
};
use super::*;

fn zero_price_or_size_cmd_strategy() -> impl Strategy<Value = PlaceImmediateOrderCmd> {
    prop_oneof![
        (Just(0_u64), 1_u64..=1_000_000)
            .prop_map(|(price, size)| { cmd_with_price_and_size(price, size) }),
        (Just(0_u64), 1_u64..=1_000_000)
            .prop_map(|(price, size)| { market_cmd_with_price_and_size(price, size) }),
        (1_u64..=1_000_000, Just(0_u64))
            .prop_map(|(price, size)| { cmd_with_price_and_size(price, size) }),
        (1_u64..=1_000_000, Just(0_u64))
            .prop_map(|(price, size)| { market_cmd_with_price_and_size(price, size) }),
    ]
}

#[test]
fn pre_check_rejects_zero_qty() {
    let use_case = PlaceImmediateOrderUseCase;
    let mut cmd = sample_cmd();
    cmd.size = 0;

    let result = use_case.pre_check_command(&cmd);
    assert_eq!(result, Err(PlaceOrderError::InvalidQty));
}

#[test]
fn pre_check_rejects_zero_price() {
    let use_case = PlaceImmediateOrderUseCase;
    let mut cmd = sample_cmd();
    cmd.execution =
        PlaceImmediateOrderExecution::Limit { price: 0, time_in_force: PlaceOrderTimeInForce::Gtc };

    let result = use_case.pre_check_command(&cmd);
    assert_eq!(result, Err(PlaceOrderError::InvalidPrice));
}

#[test]
fn pre_check_rejects_zero_market_aggressive_price() {
    let use_case = PlaceImmediateOrderUseCase;
    let mut cmd = sample_cmd();
    cmd.execution = PlaceImmediateOrderExecution::Market { aggressive_price: 0 };

    let result = use_case.pre_check_command(&cmd);
    assert_eq!(result, Err(PlaceOrderError::InvalidPrice));
}

#[test]
fn pre_check_rejects_reduce_only_for_spot_order() {
    let use_case = PlaceImmediateOrderUseCase;
    let mut cmd = sample_cmd();
    cmd.reduce_only = true;

    let result = use_case.pre_check_command(&cmd);
    assert_eq!(result, Err(PlaceOrderError::UnsupportedReduceOnly));
}

#[test]
fn validate_against_state_rejects_insufficient_balance() {
    let use_case = PlaceImmediateOrderUseCase;
    let mut state = sample_state();
    state.quote_balance.available = 10;

    let result = use_case.validate_against_state(&sample_cmd(), &state);
    assert_eq!(result, Err(PlaceOrderError::InsufficientQuoteBalance));
}

#[test]
fn validate_against_state_rejects_insufficient_base_for_sell_order() {
    let use_case = PlaceImmediateOrderUseCase;
    let mut cmd = sample_cmd();
    cmd.is_buy = false;
    let mut state = sample_state();
    state.base_balance.available = 1;

    let result = use_case.validate_against_state(&cmd, &state);
    assert_eq!(result, Err(PlaceOrderError::InsufficientBaseBalance));
}

proptest! {
    #[test]
    fn property_compute_replayable_events_rejects_zero_price_or_size(
        cmd in zero_price_or_size_cmd_strategy(),
    ) {
        let use_case = PlaceImmediateOrderUseCase;
        let result = use_case.compute_replayable_events(&cmd, sample_state());

        if cmd.size == 0 {
            prop_assert_eq!(result, Err(PlaceOrderError::InvalidQty));
        } else {
            prop_assert_eq!(result, Err(PlaceOrderError::InvalidPrice));
        }
    }

    #[test]
    fn property_compute_replayable_events_rejects_notional_overflow(
        price in 2_u64..=u64::MAX,
    ) {
        let use_case = PlaceImmediateOrderUseCase;
        let cmd = cmd_with_price_and_size(price, u64::MAX);
        let result = use_case.compute_replayable_events(&cmd, sample_state());

        prop_assert_eq!(result, Err(PlaceOrderError::ArithmeticOverflow));
    }
}
