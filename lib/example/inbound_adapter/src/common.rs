use actix_web::{HttpResponse, ResponseError};
use axum::response::{IntoResponse, Response};
use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{
    CommandEnvelope, CommandUseCase2, CommandUseCaseExecutionError, CommandUseCaseExecutor2,
    CommandUseCaseOutbound, UseCaseReplyMapper,
};
use example_core::{
    DepositQuoteCmd, DepositQuoteError, DepositQuoteState, DepositQuoteUseCase, PlaceOrderCmd,
    PlaceOrderError, PlaceOrderState, PlaceOrderUseCase, WithdrawQuoteCmd, WithdrawQuoteError,
    WithdrawQuoteState, WithdrawQuoteUseCase,
};
pub use inbound_adapter_support::{
    ApiErrorBody as ExampleHttpErrorBody, ApiErrorResponse as ExampleHttpErrorResponse,
    CliInboundErrorCategory, CliParseErrorMapping as ExampleCliParseErrorMapping,
    outbound_phase_code,
};

pub trait ExampleBusinessErrorMapping: std::error::Error {
    fn inbound_error_code(&self) -> &'static str;

    fn http_status_code(&self) -> u16;
}

#[derive(Debug)]
pub struct HttpInboundError(inbound_adapter_support::HttpInboundError);

impl HttpInboundError {
    pub fn from_execution_error<BE, OE>(error: CommandUseCaseExecutionError<BE, OE>) -> Self
    where
        BE: ExampleBusinessErrorMapping + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
    {
        Self(inbound_adapter_support::HttpInboundError::from_execution_error_with(
            error,
            |business| (business.http_status_code(), business.inbound_error_code()),
        ))
    }

    pub fn status_code(&self) -> u16 {
        self.0.status_code()
    }
}

impl<BE, OE> From<CommandUseCaseExecutionError<BE, OE>> for HttpInboundError
where
    BE: ExampleBusinessErrorMapping + Send + Sync + 'static,
    OE: std::error::Error + Send + Sync + 'static,
{
    fn from(error: CommandUseCaseExecutionError<BE, OE>) -> Self {
        Self::from_execution_error(error)
    }
}

impl std::fmt::Display for HttpInboundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for HttpInboundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl IntoResponse for HttpInboundError {
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

impl ResponseError for HttpInboundError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        ResponseError::status_code(&self.0)
    }

    fn error_response(&self) -> HttpResponse {
        self.0.error_response()
    }
}

#[derive(Debug)]
pub struct CliInboundError(inbound_adapter_support::CliInboundError);

impl CliInboundError {
    pub fn from_parse_error<E>(error: E) -> Self
    where
        E: ExampleCliParseErrorMapping + Send + Sync + 'static,
    {
        Self(inbound_adapter_support::CliInboundError::from_parse_error(error))
    }

    pub fn from_execution_error<BE, OE>(error: CommandUseCaseExecutionError<BE, OE>) -> Self
    where
        BE: ExampleBusinessErrorMapping + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
    {
        Self(inbound_adapter_support::CliInboundError::from_execution_error_with(
            error,
            ExampleBusinessErrorMapping::inbound_error_code,
        ))
    }

    pub fn runtime<E>(code: &'static str, error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self(inbound_adapter_support::CliInboundError::runtime(code, error))
    }

    pub fn exit_code(&self) -> i32 {
        self.0.exit_code()
    }
}

impl std::fmt::Display for CliInboundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for CliInboundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

pub(crate) fn execute_with_mapper<U, OB, M>(
    use_case: &U,
    envelope: CommandEnvelope<U::Command>,
    outbound: &OB,
    mapper: &M,
) -> Result<M::Reply, CommandUseCaseExecutionError<U::Error, OB::Error>>
where
    U: CommandUseCase2,
    OB: ?Sized + Send + Sync + CommandUseCaseOutbound<Command = U::Command, State = U::GivenState>,
    M: UseCaseReplyMapper,
    OB::Error: 'static,
{
    let executor = CommandUseCaseExecutor2;
    executor.execute_and_map_reply(use_case, envelope, outbound, &(), mapper)
}

pub(crate) fn execute_place_order_with_mapper<OB, M>(
    envelope: CommandEnvelope<PlaceOrderCmd>,
    outbound: &OB,
    mapper: &M,
) -> Result<M::Reply, CommandUseCaseExecutionError<PlaceOrderError, OB::Error>>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<Command = PlaceOrderCmd, State = PlaceOrderState>,
    M: UseCaseReplyMapper,
    OB::Error: 'static,
{
    execute_with_mapper(&PlaceOrderUseCase, envelope, outbound, mapper)
}

pub(crate) fn execute_deposit_quote_with_mapper<OB, M>(
    envelope: CommandEnvelope<DepositQuoteCmd>,
    outbound: &OB,
    mapper: &M,
) -> Result<M::Reply, CommandUseCaseExecutionError<DepositQuoteError, OB::Error>>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<Command = DepositQuoteCmd, State = DepositQuoteState>,
    M: UseCaseReplyMapper,
    OB::Error: 'static,
{
    execute_with_mapper(&DepositQuoteUseCase, envelope, outbound, mapper)
}

pub(crate) fn execute_withdraw_quote_with_mapper<OB, M>(
    envelope: CommandEnvelope<WithdrawQuoteCmd>,
    outbound: &OB,
    mapper: &M,
) -> Result<M::Reply, CommandUseCaseExecutionError<WithdrawQuoteError, OB::Error>>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<Command = WithdrawQuoteCmd, State = WithdrawQuoteState>,
    M: UseCaseReplyMapper,
    OB::Error: 'static,
{
    execute_with_mapper(&WithdrawQuoteUseCase, envelope, outbound, mapper)
}

pub(crate) fn find_string_field(
    events: &[EntityReplayableEvent],
    field_name: &str,
) -> Option<String> {
    events.iter().find_map(|event| {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }

            Some(String::from_utf8_lossy(change.new_value_bytes()).to_string())
        })
    })
}

pub(crate) fn find_u64_field(events: &[EntityReplayableEvent], field_name: &str) -> Option<u64> {
    events.iter().find_map(|event| {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }

            std::str::from_utf8(change.new_value_bytes()).ok()?.parse::<u64>().ok()
        })
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::{Mutex, MutexGuard};

    use cmd_handler::EntityReplayableEvent;
    use cmd_handler::use_case_def2::CommandUseCaseOutbound;
    use example_core::{
        DepositQuoteCmd, DepositQuoteState, MarketRules, PlaceOrderCmd, PlaceOrderState,
        TradingAccount, WithdrawQuoteCmd, WithdrawQuoteState,
    };
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum TestOutboundError {
        StoreUnavailable,
    }

    impl std::fmt::Display for TestOutboundError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::StoreUnavailable => f.write_str("test store unavailable"),
            }
        }
    }

    impl std::error::Error for TestOutboundError {}

    #[derive(Debug, Default)]
    struct TestOutboundState {
        persisted_events: Mutex<Vec<EntityReplayableEvent>>,
        published_events: Mutex<Vec<EntityReplayableEvent>>,
    }

    impl TestOutboundState {
        pub(crate) fn snapshot_event_counts(&self) -> Result<(usize, usize), TestOutboundError> {
            let persisted = self
                .persisted_events
                .lock()
                .map_err(|_| TestOutboundError::StoreUnavailable)?
                .len();
            let published = self
                .published_events
                .lock()
                .map_err(|_| TestOutboundError::StoreUnavailable)?
                .len();
            Ok((persisted, published))
        }

        fn lock_events(
            mutex: &Mutex<Vec<EntityReplayableEvent>>,
        ) -> Result<MutexGuard<'_, Vec<EntityReplayableEvent>>, TestOutboundError> {
            mutex.lock().map_err(|_| TestOutboundError::StoreUnavailable)
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), TestOutboundError> {
            let mut persisted = Self::lock_events(&self.persisted_events)?;
            persisted.extend(events.iter().cloned());
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), TestOutboundError> {
            let mut published = Self::lock_events(&self.published_events)?;
            published.extend(events.iter().cloned());
            Ok(())
        }
    }

    #[derive(Debug, Default)]
    pub(crate) struct PlaceOrderTestOutbound {
        state: TestOutboundState,
    }

    impl PlaceOrderTestOutbound {
        pub(crate) fn snapshot_event_counts(&self) -> Result<(usize, usize), TestOutboundError> {
            self.state.snapshot_event_counts()
        }
    }

    #[derive(Debug, Default)]
    pub(crate) struct DepositQuoteTestOutbound {
        state: TestOutboundState,
    }

    impl DepositQuoteTestOutbound {
        pub(crate) fn snapshot_event_counts(&self) -> Result<(usize, usize), TestOutboundError> {
            self.state.snapshot_event_counts()
        }
    }

    #[derive(Debug, Default)]
    pub(crate) struct WithdrawQuoteTestOutbound {
        state: TestOutboundState,
    }

    impl WithdrawQuoteTestOutbound {
        pub(crate) fn snapshot_event_counts(&self) -> Result<(usize, usize), TestOutboundError> {
            self.state.snapshot_event_counts()
        }
    }

    impl CommandUseCaseOutbound for PlaceOrderTestOutbound {
        type Command = PlaceOrderCmd;
        type State = PlaceOrderState;
        type Error = TestOutboundError;

        fn load_state(&self, _cmd: &Self::Command) -> Result<Self::State, Self::Error> {
            Ok(PlaceOrderState {
                trading_enabled: true,
                next_order_sequence: 11,
                account: TradingAccount {
                    account_id: "trader-1".to_string(),
                    available_quote: 1_000,
                    frozen_quote: 0,
                    version: 5,
                },
                market_rules: MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
            })
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.state.persist(events)
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.state.publish(events)
        }
    }

    impl CommandUseCaseOutbound for DepositQuoteTestOutbound {
        type Command = DepositQuoteCmd;
        type State = DepositQuoteState;
        type Error = TestOutboundError;

        fn load_state(&self, _cmd: &Self::Command) -> Result<Self::State, Self::Error> {
            Ok(DepositQuoteState {
                account: TradingAccount {
                    account_id: "trader-1".to_string(),
                    available_quote: 1_000,
                    frozen_quote: 0,
                    version: 5,
                },
            })
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.state.persist(events)
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.state.publish(events)
        }
    }

    impl CommandUseCaseOutbound for WithdrawQuoteTestOutbound {
        type Command = WithdrawQuoteCmd;
        type State = WithdrawQuoteState;
        type Error = TestOutboundError;

        fn load_state(&self, _cmd: &Self::Command) -> Result<Self::State, Self::Error> {
            Ok(WithdrawQuoteState {
                account: TradingAccount {
                    account_id: "trader-1".to_string(),
                    available_quote: 1_000,
                    frozen_quote: 0,
                    version: 5,
                },
            })
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.state.persist(events)
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.state.publish(events)
        }
    }

    impl crate::trading::PlaceOrderOutboundAccess for PlaceOrderTestOutbound {
        type OutboundError = TestOutboundError;
        type Outbound = Self;

        fn place_order_outbound(&self) -> &Self::Outbound {
            self
        }
    }

    impl crate::funding::DepositQuoteOutboundAccess for DepositQuoteTestOutbound {
        type OutboundError = TestOutboundError;
        type Outbound = Self;

        fn deposit_quote_outbound(&self) -> &Self::Outbound {
            self
        }
    }

    impl crate::funding::WithdrawQuoteOutboundAccess for WithdrawQuoteTestOutbound {
        type OutboundError = TestOutboundError;
        type Outbound = Self;

        fn withdraw_quote_outbound(&self) -> &Self::Outbound {
            self
        }
    }
}
