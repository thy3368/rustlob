mod descriptor;

use actix_web::{HttpResponse, ResponseError};
use anyhow::Error as AnyError;
use axum::Json;
use axum::response::{IntoResponse, Response};
use common_entity::ExecutionError;
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
        error: ExecutionError<BE, OE>,
        map_business: F,
    ) -> Self
    where
        BE: std::error::Error + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
        F: FnOnce(&BE) -> (u16, &'static str),
    {
        match error {
            ExecutionError::Business(error) => {
                let (status_code, code) = map_business(&error);
                Self::new(status_code, code, error.to_string())
            }
            ExecutionError::ProjectEvents(error) => Self {
                status_code: 500,
                code: "event_project_failed",
                message: error.to_string(),
                source: Some(AnyError::new(error)),
            },
            phase => {
                let code = match phase {
                    ExecutionError::LoadState(_) => "outbound_load_state_failed",
                    ExecutionError::Persist(_) => "outbound_persist_failed",
                    ExecutionError::Replay(_) => "outbound_replay_failed",
                    ExecutionError::Publish(_) => "outbound_publish_failed",
                    _ => "event_project_failed",
                };
                Self { status_code: 500, code, message: format!("{phase:?}"), source: None }
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
        error: ExecutionError<BE, OE>,
        map_business_code: F,
    ) -> Self
    where
        BE: std::error::Error + Send + Sync + 'static,
        OE: std::error::Error + Send + Sync + 'static,
        F: FnOnce(&BE) -> &'static str,
    {
        match error {
            ExecutionError::Business(error) => Self {
                category: CliInboundErrorCategory::Business,
                code: map_business_code(&error),
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
                code: match phase {
                    ExecutionError::LoadState(_) => "outbound_load_state_failed",
                    ExecutionError::Persist(_) => "outbound_persist_failed",
                    ExecutionError::Replay(_) => "outbound_replay_failed",
                    ExecutionError::Publish(_) => "outbound_publish_failed",
                    _ => "event_project_failed",
                },
                message: format!("{phase:?}"),
                exit_code: 4,
                source: None,
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
