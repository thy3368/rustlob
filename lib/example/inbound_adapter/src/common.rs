use actix_web::{HttpResponse, ResponseError};
use anyhow::Error as AnyError;
use axum::Json;
use axum::response::{IntoResponse, Response};
use common_entity::{
    EntityReplayableEvent, ExecutionError, LogEntry, StateMachineExecutor, StateMachineV2Diff,
    StateSink, StateSource,
};
use example_core_use_case::{
    DepositQuoteCmd, DepositQuoteError, DepositQuoteUseCase, WithdrawQuoteCmd, WithdrawQuoteError,
    WithdrawQuoteUseCase,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExampleHttpErrorBody {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExampleHttpErrorResponse {
    pub error: ExampleHttpErrorBody,
}

impl ExampleHttpErrorResponse {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self { error: ExampleHttpErrorBody { code: code.to_string(), message: message.into() } }
    }
}

pub trait ExampleCliParseErrorMapping: std::error::Error {
    fn cli_error_code(&self) -> &'static str;
}

pub(crate) trait InboundReplyMapper {
    type Reply;
    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply;
}

pub trait ExampleBusinessErrorMapping: std::error::Error {
    fn inbound_error_code(&self) -> &'static str;

    fn http_status_code(&self) -> u16;
}

#[derive(Debug)]
pub struct HttpInboundError {
    status_code: u16,
    code: &'static str,
    message: String,
    source: Option<AnyError>,
}

impl HttpInboundError {
    pub fn from_execution_error<BE, OE>(error: ExecutionError<BE, OE>) -> Self
    where
        BE: ExampleBusinessErrorMapping + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
    {
        match error {
            ExecutionError::Business(error) => Self {
                status_code: error.http_status_code(),
                code: error.inbound_error_code(),
                message: error.to_string(),
                source: None,
            },
            ExecutionError::ProjectEvents(error) => Self {
                status_code: 500,
                code: "event_project_failed",
                message: error.to_string(),
                source: Some(AnyError::new(error)),
            },
            phase => Self {
                status_code: 500,
                code: outbound_phase_code(&phase),
                message: format!("{phase:?}"),
                source: None,
            },
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn from_mi_execution_error<BE, OE>(error: ExecutionError<BE, OE>) -> Self
    where
        BE: ExampleBusinessErrorMapping + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
    {
        Self::from_execution_error(mi_error_to_command_error(error))
    }
}

impl<BE, OE> From<ExecutionError<BE, OE>> for HttpInboundError
where
    BE: ExampleBusinessErrorMapping + Send + Sync + 'static,
    OE: std::error::Error + Send + Sync + 'static,
{
    fn from(error: ExecutionError<BE, OE>) -> Self {
        Self::from_execution_error(error)
    }
}

impl std::fmt::Display for HttpInboundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for HttpInboundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|error| error.as_ref() as _)
    }
}

impl IntoResponse for HttpInboundError {
    fn into_response(self) -> Response {
        let status = axum::http::StatusCode::from_u16(self.status_code)
            .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(ExampleHttpErrorResponse::new(self.code, self.message))).into_response()
    }
}

impl ResponseError for HttpInboundError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.status_code)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(ResponseError::status_code(self))
            .json(ExampleHttpErrorResponse::new(self.code, self.message.clone()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliInboundErrorCategory {
    Parse,
    Business,
    Outbound,
    Runtime,
}

impl std::fmt::Display for CliInboundErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Parse => "parse",
            Self::Business => "business",
            Self::Outbound => "outbound",
            Self::Runtime => "runtime",
        })
    }
}

#[derive(Debug)]
pub struct CliInboundError {
    category: CliInboundErrorCategory,
    code: &'static str,
    message: String,
    exit_code: i32,
    source: Option<AnyError>,
}

impl CliInboundError {
    pub fn from_parse_error<E>(error: E) -> Self
    where
        E: ExampleCliParseErrorMapping + Send + Sync + 'static,
    {
        Self {
            category: CliInboundErrorCategory::Parse,
            code: error.cli_error_code(),
            message: error.to_string(),
            exit_code: 2,
            source: Some(AnyError::new(error)),
        }
    }

    pub fn from_execution_error<BE, OE>(error: ExecutionError<BE, OE>) -> Self
    where
        BE: ExampleBusinessErrorMapping + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
    {
        match error {
            ExecutionError::Business(error) => Self {
                category: CliInboundErrorCategory::Business,
                code: error.inbound_error_code(),
                message: error.to_string(),
                exit_code: 3,
                source: Some(AnyError::new(error)),
            },
            ExecutionError::ProjectEvents(error) => Self {
                category: CliInboundErrorCategory::Runtime,
                code: "event_project_failed",
                message: error.to_string(),
                exit_code: 1,
                source: Some(AnyError::new(error)),
            },
            phase => Self {
                category: CliInboundErrorCategory::Outbound,
                code: outbound_phase_code(&phase),
                message: format!("{phase:?}"),
                exit_code: 4,
                source: None,
            },
        }
    }

    pub fn from_mi_execution_error<BE, OE>(error: ExecutionError<BE, OE>) -> Self
    where
        BE: ExampleBusinessErrorMapping + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
    {
        Self::from_execution_error(mi_error_to_command_error(error))
    }

    pub fn runtime<E>(code: &'static str, error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self {
            category: CliInboundErrorCategory::Runtime,
            code,
            message: error.to_string(),
            exit_code: 1,
            source: Some(AnyError::new(error)),
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

fn mi_error_to_command_error<BE, OE>(error: ExecutionError<BE, OE>) -> ExecutionError<BE, OE>
where
    BE: std::error::Error + 'static,
    OE: std::error::Error + 'static,
{
    error
}

impl std::fmt::Display for CliInboundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}: {}", self.category, self.code, self.message)
    }
}

impl std::error::Error for CliInboundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|error| error.as_ref() as _)
    }
}

fn outbound_phase_code<BE, OE>(error: &ExecutionError<BE, OE>) -> &'static str {
    match error {
        ExecutionError::LoadState(_) => "outbound_load_state_failed",
        ExecutionError::Persist(_) => "outbound_persist_failed",
        ExecutionError::Replay(_) => "outbound_replay_failed",
        ExecutionError::Publish(_) => "outbound_publish_failed",
        _ => "event_project_failed",
    }
}

pub(crate) fn execute_with_mapper<U, OB, E, M>(
    use_case: &U,
    command: U::Command,
    outbound: &OB,
    mapper: &M,
) -> Result<M::Reply, ExecutionError<U::Error, E>>
where
    U: StateMachineV2Diff,
    OB: Send + Sync + StateSource<U, Error = E> + StateSink<U, Error = E>,
    E: std::error::Error + 'static,
    M: InboundReplyMapper,
{
    let executor = StateMachineExecutor;
    let result = executor.execute(use_case, &command, outbound, outbound)?;
    Ok(mapper.map(result.events))
}

pub(crate) fn execute_deposit_quote_with_mapper<OB, M>(
    command: LogEntry<DepositQuoteCmd>,
    outbound: &OB,
    mapper: &M,
) -> Result<
    M::Reply,
    ExecutionError<DepositQuoteError, <OB as StateSource<DepositQuoteUseCase>>::Error>,
>
where
    OB: Send
        + Sync
        + StateSource<DepositQuoteUseCase>
        + StateSink<DepositQuoteUseCase, Error = <OB as StateSource<DepositQuoteUseCase>>::Error>,
    M: InboundReplyMapper,
    <OB as StateSource<DepositQuoteUseCase>>::Error: 'static,
{
    execute_with_mapper(&DepositQuoteUseCase, command, outbound, mapper)
}

pub(crate) fn execute_withdraw_quote_with_mapper<OB, M>(
    command: LogEntry<WithdrawQuoteCmd>,
    outbound: &OB,
    mapper: &M,
) -> Result<
    M::Reply,
    ExecutionError<WithdrawQuoteError, <OB as StateSource<WithdrawQuoteUseCase>>::Error>,
>
where
    OB: Send
        + Sync
        + StateSource<WithdrawQuoteUseCase>
        + StateSink<WithdrawQuoteUseCase, Error = <OB as StateSource<WithdrawQuoteUseCase>>::Error>,
    M: InboundReplyMapper,
    <OB as StateSource<WithdrawQuoteUseCase>>::Error: 'static,
{
    execute_with_mapper(&WithdrawQuoteUseCase, command, outbound, mapper)
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

pub(crate) fn find_last_u64_field(
    events: &[EntityReplayableEvent],
    field_name: &str,
) -> Option<u64> {
    events.iter().rev().find_map(|event| {
        event.field_changes.iter().rev().find_map(|change| {
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

    use common_entity::{EntityReplayableEvent, LogEntry, StateSink, StateSource};
    use example_core_use_case::{
        Balance, DepositQuoteCmd, DepositQuoteState, DepositQuoteUseCase,
        PlaceMatchSpotOrderV2State, PlaceMatchSpotOrderV2UseCase, PlaceOnlySpotOrderV2Cmd,
        WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteUseCase,
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

    impl StateSource<PlaceMatchSpotOrderV2UseCase> for PlaceOrderTestOutbound {
        type Error = TestOutboundError;

        fn load_given_state(
            &self,
            _cmd: &LogEntry<PlaceOnlySpotOrderV2Cmd>,
        ) -> Result<PlaceMatchSpotOrderV2State, Self::Error> {
            let settlement_balances = vec![
                Balance::new(
                    common_entity::stable_string_entity_id("trader-1"),
                    common_entity::stable_string_entity_id("trader-1"),
                    common_entity::stable_string_entity_id("BTC"),
                    0,
                    0,
                    5,
                ),
                Balance::new(
                    common_entity::stable_string_entity_id("trader-1"),
                    common_entity::stable_string_entity_id("trader-1"),
                    common_entity::stable_string_entity_id("USDT"),
                    1_000,
                    0,
                    5,
                ),
                Balance::new(
                    common_entity::stable_string_entity_id("fee"),
                    common_entity::stable_string_entity_id("fee"),
                    common_entity::stable_string_entity_id("USDT"),
                    0,
                    0,
                    1,
                ),
            ];
            Ok(PlaceMatchSpotOrderV2State {
                maker_orders: Vec::new(),
                settlement_balances,
                fee_account_id: "fee".to_string(),
            })
        }
    }

    impl StateSink<PlaceMatchSpotOrderV2UseCase> for PlaceOrderTestOutbound {
        type Error = TestOutboundError;

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

    impl StateSource<DepositQuoteUseCase> for DepositQuoteTestOutbound {
        type Error = TestOutboundError;

        fn load_given_state(
            &self,
            _cmd: &LogEntry<DepositQuoteCmd>,
        ) -> Result<DepositQuoteState, Self::Error> {
            Ok(DepositQuoteState {
                quote_balance: Balance::new(
                    common_entity::stable_string_entity_id("trader-1"),
                    common_entity::stable_string_entity_id("trader-1"),
                    common_entity::stable_string_entity_id("USDT"),
                    1_000,
                    0,
                    5,
                ),
            })
        }
    }

    impl StateSink<DepositQuoteUseCase> for DepositQuoteTestOutbound {
        type Error = TestOutboundError;

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

    impl StateSource<WithdrawQuoteUseCase> for WithdrawQuoteTestOutbound {
        type Error = TestOutboundError;

        fn load_given_state(
            &self,
            _cmd: &LogEntry<WithdrawQuoteCmd>,
        ) -> Result<WithdrawQuoteState, Self::Error> {
            Ok(WithdrawQuoteState {
                quote_balance: Balance::new(
                    common_entity::stable_string_entity_id("trader-1"),
                    common_entity::stable_string_entity_id("trader-1"),
                    common_entity::stable_string_entity_id("USDT"),
                    1_000,
                    0,
                    5,
                ),
            })
        }
    }

    impl StateSink<WithdrawQuoteUseCase> for WithdrawQuoteTestOutbound {
        type Error = TestOutboundError;

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

        fn command_metadata(&self) -> (u64, u64, u64, u64) {
            (1, 1_000_000_000, 1, 1)
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
