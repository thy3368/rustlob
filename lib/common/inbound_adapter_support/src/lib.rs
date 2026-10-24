mod descriptor;

use actix_web::{HttpResponse, ResponseError};
use anyhow::Error as AnyError;
use axum::Json;
use axum::response::{IntoResponse, Response};
use cmd_handler::command_use_case_def2::{
    CommandUseCaseExecutionError, CommandUseCaseOutboundPhase,
};
pub use descriptor::{
    CliApiDescriptor, CliArgDescriptor, CliErrorCodeDescriptor, HttpApiDescriptor,
    HttpErrorCodeDescriptor, InboundApiDescriptor, build_api_manifest, build_cli_schema,
    build_http_openapi, cli_arg, cli_error, http_error, insert_schema, object_schema,
    optional_string_schema, schema_ref, string_schema, u64_schema,
};
pub use inbound_adapter_support_macros::collect_http_endpoint;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorResponse {
    pub error: ApiErrorBody,
}

impl ApiErrorResponse {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self { error: ApiErrorBody { code: code.to_string(), message: message.into() } }
    }
}

pub trait CliParseErrorMapping: std::error::Error {
    fn cli_error_code(&self) -> &'static str;
}

#[derive(Debug)]
pub struct HttpInboundError {
    status_code: u16,
    code: &'static str,
    message: String,
    source: Option<AnyError>,
}

impl HttpInboundError {
    pub fn new(status_code: u16, code: &'static str, message: impl Into<String>) -> Self {
        Self { status_code, code, message: message.into(), source: None }
    }

    pub fn from_execution_error_with<BE, OE, F>(
        error: CommandUseCaseExecutionError<BE, OE>,
        map_business: F,
    ) -> Self
    where
        BE: std::error::Error + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
        F: FnOnce(&BE) -> (u16, &'static str),
    {
        match error {
            CommandUseCaseExecutionError::Business(error) => {
                let (status_code, code) = map_business(&error);
                Self::new(status_code, code, error.to_string())
            }
            CommandUseCaseExecutionError::EventProject(error) => Self {
                status_code: 500,
                code: "event_project_failed",
                message: error.to_string(),
                source: Some(AnyError::new(error)),
            },
            CommandUseCaseExecutionError::Outbound { phase, source } => {
                let code = outbound_phase_code(phase);
                Self {
                    status_code: 500,
                    code,
                    message: source.to_string(),
                    source: Some(AnyError::new(source)),
                }
            }
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }
}

impl std::fmt::Display for HttpInboundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for HttpInboundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|error| error.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl IntoResponse for HttpInboundError {
    fn into_response(self) -> Response {
        let status = axum::http::StatusCode::from_u16(self.status_code)
            .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(ApiErrorResponse::new(self.code, self.message))).into_response()
    }
}

impl ResponseError for HttpInboundError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.status_code)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(ResponseError::status_code(self))
            .json(ApiErrorResponse::new(self.code, self.message.clone()))
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
        match self {
            Self::Parse => f.write_str("parse"),
            Self::Business => f.write_str("business"),
            Self::Outbound => f.write_str("outbound"),
            Self::Runtime => f.write_str("runtime"),
        }
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
        E: CliParseErrorMapping + Send + Sync + 'static,
    {
        Self {
            category: CliInboundErrorCategory::Parse,
            code: error.cli_error_code(),
            message: error.to_string(),
            exit_code: 2,
            source: Some(AnyError::new(error)),
        }
    }

    pub fn from_execution_error_with<BE, OE, F>(
        error: CommandUseCaseExecutionError<BE, OE>,
        map_business_code: F,
    ) -> Self
    where
        BE: std::error::Error + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
        F: FnOnce(&BE) -> &'static str,
    {
        match error {
            CommandUseCaseExecutionError::Business(error) => Self {
                category: CliInboundErrorCategory::Business,
                code: map_business_code(&error),
                message: error.to_string(),
                exit_code: 3,
                source: Some(AnyError::new(error)),
            },
            CommandUseCaseExecutionError::EventProject(error) => Self {
                category: CliInboundErrorCategory::Runtime,
                code: "event_project_failed",
                message: error.to_string(),
                exit_code: 1,
                source: Some(AnyError::new(error)),
            },
            CommandUseCaseExecutionError::Outbound { phase, source } => Self {
                category: CliInboundErrorCategory::Outbound,
                code: outbound_phase_code(phase),
                message: source.to_string(),
                exit_code: 4,
                source: Some(AnyError::new(source)),
            },
        }
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

impl std::fmt::Display for CliInboundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}: {}", self.category, self.code, self.message)
    }
}

impl std::error::Error for CliInboundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|error| error.as_ref() as &(dyn std::error::Error + 'static))
    }
}

pub fn outbound_phase_code(phase: CommandUseCaseOutboundPhase) -> &'static str {
    match phase {
        CommandUseCaseOutboundPhase::LoadState => "outbound_load_state_failed",
        CommandUseCaseOutboundPhase::Persist => "outbound_persist_failed",
        CommandUseCaseOutboundPhase::Replay => "outbound_replay_failed",
        CommandUseCaseOutboundPhase::Publish => "outbound_publish_failed",
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{
        CommandUseCaseExecutionError, CommandUseCaseOutboundPhase, EventProjectError,
    };

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum StubOutboundError {
        StoreUnavailable,
    }

    impl std::fmt::Display for StubOutboundError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::StoreUnavailable => f.write_str("test store unavailable"),
            }
        }
    }

    impl std::error::Error for StubOutboundError {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum StubBusinessError {
        BadRequest,
        Internal,
    }

    impl std::fmt::Display for StubBusinessError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::BadRequest => f.write_str("bad request"),
                Self::Internal => f.write_str("internal business error"),
            }
        }
    }

    impl std::error::Error for StubBusinessError {}

    fn stub_business_mapping(error: &StubBusinessError) -> (u16, &'static str) {
        match error {
            StubBusinessError::BadRequest => (400, "bad_request"),
            StubBusinessError::Internal => (500, "internal_business_error"),
        }
    }

    fn stub_business_code(error: &StubBusinessError) -> &'static str {
        match error {
            StubBusinessError::BadRequest => "bad_request",
            StubBusinessError::Internal => "internal_business_error",
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum StubParseError {
        InvalidFlag,
    }

    impl std::fmt::Display for StubParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::InvalidFlag => f.write_str("invalid flag"),
            }
        }
    }

    impl std::error::Error for StubParseError {}

    impl CliParseErrorMapping for StubParseError {
        fn cli_error_code(&self) -> &'static str {
            "invalid_flag"
        }
    }

    #[test]
    fn http_inbound_error_maps_business_error() {
        let error =
            HttpInboundError::from_execution_error_with::<StubBusinessError, StubOutboundError, _>(
                CommandUseCaseExecutionError::Business(StubBusinessError::BadRequest),
                stub_business_mapping,
            );

        assert_eq!(error.status_code(), 400);
        assert_eq!(error.to_string(), "bad_request: bad request");
    }

    #[test]
    fn http_inbound_error_maps_outbound_phase() {
        let error =
            HttpInboundError::from_execution_error_with::<StubBusinessError, StubOutboundError, _>(
                CommandUseCaseExecutionError::outbound(
                    CommandUseCaseOutboundPhase::Replay,
                    StubOutboundError::StoreUnavailable,
                ),
                stub_business_mapping,
            );

        assert_eq!(error.status_code(), 500);
        assert_eq!(error.to_string(), "outbound_replay_failed: test store unavailable");
    }

    #[test]
    fn http_inbound_error_maps_event_project_error() {
        let error =
            HttpInboundError::from_execution_error_with::<StubBusinessError, StubOutboundError, _>(
                CommandUseCaseExecutionError::event_project(stub_event_project_error()),
                stub_business_mapping,
            );

        assert_eq!(error.status_code(), 500);
        assert!(error.to_string().starts_with("event_project_failed: "));
    }

    #[test]
    fn cli_inbound_error_maps_parse_business_outbound_and_runtime_errors() {
        let parse = CliInboundError::from_parse_error(StubParseError::InvalidFlag);
        let business =
            CliInboundError::from_execution_error_with::<StubBusinessError, StubOutboundError, _>(
                CommandUseCaseExecutionError::Business(StubBusinessError::Internal),
                stub_business_code,
            );
        let outbound =
            CliInboundError::from_execution_error_with::<StubBusinessError, StubOutboundError, _>(
                CommandUseCaseExecutionError::outbound(
                    CommandUseCaseOutboundPhase::Publish,
                    StubOutboundError::StoreUnavailable,
                ),
                stub_business_code,
            );
        let runtime = CliInboundError::runtime(
            "snapshot_failed",
            std::io::Error::other("snapshot unavailable"),
        );

        assert_eq!(parse.exit_code(), 2);
        assert_eq!(business.exit_code(), 3);
        assert_eq!(outbound.exit_code(), 4);
        assert_eq!(runtime.exit_code(), 1);
        assert_eq!(parse.to_string(), "parse invalid_flag: invalid flag");
        assert_eq!(
            outbound.to_string(),
            "outbound outbound_publish_failed: test store unavailable"
        );
    }

    #[test]
    fn cli_inbound_error_maps_event_project_error() {
        let error =
            CliInboundError::from_execution_error_with::<StubBusinessError, StubOutboundError, _>(
                CommandUseCaseExecutionError::event_project(stub_event_project_error()),
                stub_business_code,
            );

        assert_eq!(error.exit_code(), 1);
        assert_eq!(error.to_string(), "runtime event_project_failed: no replayable events");
    }

    fn stub_event_project_error() -> EventProjectError {
        "no replayable events".into()
    }

    #[test]
    fn outbound_phase_code_matches_contract() {
        assert_eq!(
            outbound_phase_code(CommandUseCaseOutboundPhase::LoadState),
            "outbound_load_state_failed"
        );
        assert_eq!(
            outbound_phase_code(CommandUseCaseOutboundPhase::Persist),
            "outbound_persist_failed"
        );
        assert_eq!(
            outbound_phase_code(CommandUseCaseOutboundPhase::Replay),
            "outbound_replay_failed"
        );
        assert_eq!(
            outbound_phase_code(CommandUseCaseOutboundPhase::Publish),
            "outbound_publish_failed"
        );
    }
}
