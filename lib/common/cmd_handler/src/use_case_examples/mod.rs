use crate::command_use_case_def2::{CommandUseCase2, IssuedByParty};

mod bad;
mod good;

#[test]
fn good_case_shows_preferred_use_case_shape() {
    let cmd = good::PlaceOrderCmd {
        party_id: "acct-1".to_string(),
        symbol: "BTCUSDT".to_string(),
        qty: 1,
    };
    let state = good::PlaceOrderState { trading_enabled: true };
    let use_case = good::PlaceOrderUseCase;

    assert_eq!(use_case.role(), "Trader");
    assert_eq!(cmd.party_id(), Some("acct-1"));
    assert!(use_case.validate_against_state(&cmd, &state).is_ok());
}

#[test]
fn bad_case_shows_common_use_case_design_smells() {
    let cmd = bad::SubmitCmd { trace_id: "trace-1".to_string(), symbol: "BTCUSDT".to_string() };
    let state = bad::SubmitState { accepted: true, generated_status: "accepted" };
    let use_case = bad::OrderCheckingEngineUseCase;
    let events = use_case.compute_replayable_events(&cmd, state).unwrap();

    assert_eq!(use_case.role(), "OrderCheckingEngine");
    assert_eq!(cmd.party_id(), None);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].field_changes[0].new_value_bytes(), b"accepted");
}
