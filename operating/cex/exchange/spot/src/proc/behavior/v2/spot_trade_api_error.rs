pub use base_types::cqrs::cqrs_types::{CMetadata, Cmd, CmdResp};
pub use db_repo::core::db_repo2::RepoError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Warning => write!(f, "WARNING"),
            Self::Error => write!(f, "ERROR"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug)]
pub struct InternalError {
    pub source: anyhow::Error,
    pub context: String,
    pub severity: Severity,
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} at {}", self.severity, self.source, self.context)
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TraderError {
    #[error("余额不足: 需要 {required}, 可用 {available}. {action}")]
    InsufficientBalance { required: String, available: String, action: &'static str },
    #[error("订单{order_id}不存在. {action}")]
    OrderNotFound { order_id: String, action: &'static str },
    #[error("订单状态{current}无法{want}. {action}")]
    InvalidStatusTransition { current: String, want: String, action: &'static str },
    #[error("交易对{symbol}不存在. {suggestion}")]
    TradingPairNotFound { symbol: String, suggestion: &'static str },
    #[error("账户{account_id}已被冻结. {action}")]
    AccountFrozen { account_id: String, action: &'static str },
}

impl TraderError {
    pub fn code(&self) -> i32 {
        match self {
            Self::InsufficientBalance { .. } => -2001,
            Self::OrderNotFound { .. } => -2002,
            Self::InvalidStatusTransition { .. } => -2003,
            Self::TradingPairNotFound { .. } => -2004,
            Self::AccountFrozen { .. } => -2005,
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::InsufficientBalance { .. } => 400,
            Self::OrderNotFound { .. } => 404,
            Self::InvalidStatusTransition { .. } => 409,
            Self::TradingPairNotFound { .. } => 400,
            Self::AccountFrozen { .. } => 403,
        }
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ApiError {
    #[error("Invalid field '{field}': {reason} (code:{code}, see {doc})")]
    InvalidField { field: &'static str, reason: &'static str, code: i32, doc: &'static str },
    #[error("Constraint '{constraint}' violated: value={value} range=[{min}, {max}] (code:{code})")]
    ConstraintViolation {
        constraint: &'static str,
        value: String,
        min: String,
        max: String,
        code: i32,
    },
    #[error("Unauthorized: {reason} (required: {required}) (code:{code})")]
    Unauthorized { reason: &'static str, required: &'static str, code: i32 },
    #[error("Rate limited: {limit} req/min, retry after {retry_after_ms}ms (code:{code})")]
    RateLimited { retry_after_ms: u64, limit: u64, code: i32 },
    #[error("Resource '{resource}' not found (code:{code})")]
    NotFound { resource: &'static str, code: i32 },
    #[error("Service '{service}' unavailable (code:{code})")]
    ServiceUnavailable { service: &'static str, code: i32 },
}

impl ApiError {
    pub fn code(&self) -> i32 {
        match self {
            Self::InvalidField { code, .. } => *code,
            Self::ConstraintViolation { code, .. } => *code,
            Self::Unauthorized { code, .. } => *code,
            Self::RateLimited { code, .. } => *code,
            Self::NotFound { code, .. } => *code,
            Self::ServiceUnavailable { code, .. } => *code,
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::InvalidField { .. } => 400,
            Self::ConstraintViolation { .. } => 422,
            Self::Unauthorized { .. } => 403,
            Self::RateLimited { .. } => 429,
            Self::NotFound { .. } => 404,
            Self::ServiceUnavailable { .. } => 503,
        }
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpotErrorAny {
    #[error("{0}")]
    Trader(TraderError),
    #[error("{0}")]
    Api(ApiError),
}

impl SpotErrorAny {
    pub fn code(&self) -> i32 {
        match self {
            Self::Trader(e) => e.code(),
            Self::Api(e) => e.code(),
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::Trader(e) => e.http_status(),
            Self::Api(e) => e.http_status(),
        }
    }
}

impl From<TraderError> for SpotErrorAny {
    fn from(e: TraderError) -> Self {
        Self::Trader(e)
    }
}

impl From<ApiError> for SpotErrorAny {
    fn from(e: ApiError) -> Self {
        Self::Api(e)
    }
}
