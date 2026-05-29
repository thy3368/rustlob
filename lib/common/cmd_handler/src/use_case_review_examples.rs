use crate::TraceableEventSet;
use crate::use_case_def2::{CommandUseCase2, IssuedByParty};

mod good_case {
    use super::*;

    // Good case:
    // - `party_id` is explicit business input on the command.
    // - `role()` is a business-game role, not a technical component name.
    // - business validation happens in the use case instead of being precomputed elsewhere.
    // - emitted events are derived from command + state, not copied from a prepared answer.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PlaceOrderCmd {
        pub party_id: String,
        pub symbol: String,
        pub qty: u64,
    }

    impl IssuedByParty for PlaceOrderCmd {
        fn party_id(&self) -> Option<&str> {
            Some(self.party_id.as_str())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum PlaceOrderError {
        InvalidQty,
        TradingDisabled,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PlaceOrderState {
        pub trading_enabled: bool,
    }

    impl PlaceOrderState {
        fn can_trade(&self, _party_id: &str, _symbol: &str) -> bool {
            self.trading_enabled
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PlaceOrderEvents {
        pub party_id: String,
        pub symbol: String,
        pub qty: u64,
    }

    impl PlaceOrderEvents {
        fn accepted(party_id: String, symbol: String, qty: u64) -> Self {
            Self { party_id, symbol, qty }
        }
    }

    impl TraceableEventSet for PlaceOrderEvents {
        fn event_count(&self) -> usize {
            1
        }
    }

    pub struct PlaceOrderUseCase;

    impl CommandUseCase2 for PlaceOrderUseCase {
        type Command = PlaceOrderCmd;
        type GivenState = PlaceOrderState;
        type ThenTraceableEvents = PlaceOrderEvents;
        type Error = PlaceOrderError;

        fn role(&self) -> &'static str {
            "Trader"
        }

        fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
            if cmd.qty == 0 {
                return Err(PlaceOrderError::InvalidQty);
            }
            Ok(())
        }

        fn validate_against_state(
            &self,
            cmd: &Self::Command,
            state: &Self::GivenState,
        ) -> Result<(), Self::Error> {
            if state.can_trade(cmd.party_id.as_str(), cmd.symbol.as_str()) {
                Ok(())
            } else {
                Err(PlaceOrderError::TradingDisabled)
            }
        }

        fn gen_traceable_events(
            &self,
            cmd: &Self::Command,
            _state: Self::GivenState,
        ) -> Result<Self::ThenTraceableEvents, Self::Error> {
            Ok(PlaceOrderEvents::accepted(
                cmd.party_id.clone(),
                cmd.symbol.clone(),
                cmd.qty,
            ))
        }
    }
}

mod bad_case {
    use super::*;

    // Bad case:
    // - `role()` uses a technical component name, so four-color Role clarity is weak.
    // - `party_id` is missing even though a business party is clearly issuing the command.
    // - `trace_id` is placed on the business command, which encourages identity confusion.
    // - `GivenState` already contains the decision result and final event, so the use case
    //   does very little business work and mostly copies a precomputed answer.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SubmitCmd {
        pub trace_id: String,
        pub symbol: String,
    }

    impl IssuedByParty for SubmitCmd {}

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum SubmitError {
        Rejected,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SubmitEvent {
        pub status: &'static str,
    }

    impl TraceableEventSet for SubmitEvent {
        fn event_count(&self) -> usize {
            1
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SubmitState {
        pub accepted: bool,
        pub generated_event: SubmitEvent,
    }

    pub struct OrderCheckingEngineUseCase;

    impl CommandUseCase2 for OrderCheckingEngineUseCase {
        type Command = SubmitCmd;
        type GivenState = SubmitState;
        type ThenTraceableEvents = SubmitEvent;
        type Error = SubmitError;

        fn role(&self) -> &'static str {
            "OrderCheckingEngine"
        }

        fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
            Ok(())
        }

        fn validate_against_state(
            &self,
            _cmd: &Self::Command,
            state: &Self::GivenState,
        ) -> Result<(), Self::Error> {
            if state.accepted { Ok(()) } else { Err(SubmitError::Rejected) }
        }

        fn gen_traceable_events(
            &self,
            _cmd: &Self::Command,
            state: Self::GivenState,
        ) -> Result<Self::ThenTraceableEvents, Self::Error> {
            Ok(state.generated_event)
        }
    }
}

#[test]
fn good_case_uses_business_role_and_party_identity() {
    let cmd = good_case::PlaceOrderCmd {
        party_id: "acct-1".to_string(),
        symbol: "BTCUSDT".to_string(),
        qty: 1,
    };
    let state = good_case::PlaceOrderState { trading_enabled: true };
    let use_case = good_case::PlaceOrderUseCase;

    assert_eq!(use_case.role(), "Trader");
    assert_eq!(cmd.party_id(), Some("acct-1"));
    assert!(use_case.validate_against_state(&cmd, &state).is_ok());
}

#[test]
fn bad_case_uses_technical_role_and_precomputed_answers() {
    let cmd = bad_case::SubmitCmd { trace_id: "trace-1".to_string(), symbol: "BTCUSDT".to_string() };
    let state = bad_case::SubmitState {
        accepted: true,
        generated_event: bad_case::SubmitEvent { status: "accepted" },
    };
    let use_case = bad_case::OrderCheckingEngineUseCase;

    assert_eq!(use_case.role(), "OrderCheckingEngine");
    assert_eq!(cmd.party_id(), None);
    assert_eq!(
        use_case.gen_traceable_events(&cmd, state).unwrap(),
        bad_case::SubmitEvent { status: "accepted" }
    );
}
