use aeron_rs::utils::errors::AeronError;
use aeron_rs::utils::types::Index;
use common_entity::EntityReplayableEvent;
use thiserror::Error;
use use_case_executor::trading::spot::on_event::{
    DefaultMatchSpotOrderCreatedEventError, MatchSpotOrderCreatedEventHandler,
    MatchSpotOrderCreatedEventParseError, MatchSpotOrderExecutorError,
    match_spot_order_command_from_event as command_from_event,
};

pub use crate::bootstrap::on_event::spot::on_aeron_spot_trade::{
    SpotTradeAeronError, run_spot_trade_aeron,
};
pub use crate::common::aeron_listener::{
    AeronListenerConfig, AeronListenerConfigError, AeronListenerEnv, AeronListenerError,
    decode_entity_replayable_event, handle_aeron_event_fragment, run_aeron_listener,
};

const DEFAULT_MATCH_SPOT_ORDER_STREAM_ID: i32 = 1001;

const LEGACY_AERON_CHANNEL_ENV: &str = "AERON_CHANNEL";
const LEGACY_AERON_STREAM_ID_ENV: &str = "AERON_STREAM_ID";
const LEGACY_AERON_FRAGMENT_LIMIT_ENV: &str = "AERON_FRAGMENT_LIMIT";

const MATCH_SPOT_ORDER_CHANNEL_ENV: &str = "AERON_MATCH_SPOT_ORDER_CHANNEL";
const MATCH_SPOT_ORDER_STREAM_ID_ENV: &str = "AERON_MATCH_SPOT_ORDER_STREAM_ID";
const MATCH_SPOT_ORDER_FRAGMENT_LIMIT_ENV: &str = "AERON_MATCH_SPOT_ORDER_FRAGMENT_LIMIT";

impl AeronListenerConfig {
    pub fn from_env() -> Result<Self, AeronListenerConfigError> {
        Self::match_spot_order_from_env()
    }

    pub fn match_spot_order_from_env() -> Result<Self, AeronListenerConfigError> {
        Self::from_env_vars(
            AeronListenerEnv {
                channel: MATCH_SPOT_ORDER_CHANNEL_ENV,
                stream_id: MATCH_SPOT_ORDER_STREAM_ID_ENV,
                fragment_limit: MATCH_SPOT_ORDER_FRAGMENT_LIMIT_ENV,
            },
            DEFAULT_MATCH_SPOT_ORDER_STREAM_ID,
            Some(AeronListenerEnv {
                channel: LEGACY_AERON_CHANNEL_ENV,
                stream_id: LEGACY_AERON_STREAM_ID_ENV,
                fragment_limit: LEGACY_AERON_FRAGMENT_LIMIT_ENV,
            }),
        )
    }

    pub fn match_spot_order_default() -> Self {
        Self::default_for_stream(DEFAULT_MATCH_SPOT_ORDER_STREAM_ID)
    }
}

impl Default for AeronListenerConfig {
    fn default() -> Self {
        Self::match_spot_order_default()
    }
}

pub type AeronMatchSpotOrderConfig = AeronListenerConfig;
pub type AeronMatchSpotOrderConfigError = AeronListenerConfigError;

#[derive(Debug, Error)]
pub enum AeronMatchSpotOrderError {
    #[error("invalid Aeron configuration: {0}")]
    Configuration(#[from] AeronListenerConfigError),
    #[error("failed to initialize Aeron client: {0}")]
    Aeron(#[source] AeronError),
    #[error("failed to establish Aeron subscription: {0}")]
    Subscription(#[source] AeronError),
    #[error("Aeron subscription mutex was poisoned")]
    SubscriptionLock,
    #[error(
        "Aeron fragment is outside its buffer: offset={offset}, length={length}, capacity={capacity}"
    )]
    InvalidFragmentBounds { offset: Index, length: Index, capacity: Index },
    #[error("failed to decode Aeron fragment as EntityReplayableEvent: {0}")]
    Decode(#[source] serde_json::Error),
    #[error("failed to handle spot order event: {0:?}")]
    EventHandler(DefaultMatchSpotOrderCreatedEventError),
    #[error("spot order executor failed: {0:?}")]
    Executor(MatchSpotOrderExecutorError),
}

pub type MatchSpotOrderAeronError = AeronMatchSpotOrderError;

pub fn decode_match_spot_order_event(
    payload: &[u8],
) -> Result<EntityReplayableEvent, serde_json::Error> {
    decode_entity_replayable_event(payload)
}

pub fn match_spot_order_command_from_event(
    event: &EntityReplayableEvent,
) -> Result<Option<example_core_use_case::MatchSpotOrderV3Cmd>, MatchSpotOrderCreatedEventParseError>
{
    command_from_event(event)
}

pub fn handle_match_spot_order_fragment(payload: &[u8]) -> Result<(), AeronMatchSpotOrderError> {
    let mut handler = MatchSpotOrderCreatedEventHandler::new();
    handle_aeron_event_fragment(payload, &mut handler)
        .map(|_| ())
        .map_err(AeronMatchSpotOrderError::from)
}

impl From<AeronListenerError<DefaultMatchSpotOrderCreatedEventError>> for AeronMatchSpotOrderError {
    fn from(error: AeronListenerError<DefaultMatchSpotOrderCreatedEventError>) -> Self {
        match error {
            AeronListenerError::Configuration(error) => Self::Configuration(error),
            AeronListenerError::Aeron(error) => Self::Aeron(error),
            AeronListenerError::Subscription(error) => Self::Subscription(error),
            AeronListenerError::SubscriptionLock => Self::SubscriptionLock,
            AeronListenerError::InvalidFragmentBounds { offset, length, capacity } => {
                Self::InvalidFragmentBounds { offset, length, capacity }
            }
            AeronListenerError::Decode(error) => Self::Decode(error),
            AeronListenerError::EventHandler(error) => Self::from(error),
        }
    }
}

impl From<DefaultMatchSpotOrderCreatedEventError> for AeronMatchSpotOrderError {
    fn from(error: DefaultMatchSpotOrderCreatedEventError) -> Self {
        match error {
            use_case_executor::trading::spot::on_event::MatchSpotOrderCreatedEventError::Executor(
                error,
            ) => Self::Executor(error),
            error => Self::EventHandler(error),
        }
    }
}

pub fn run_match_spot_order_aeron() -> Result<(), AeronMatchSpotOrderError> {
    let config = AeronListenerConfig::match_spot_order_from_env()
        .map_err(AeronMatchSpotOrderError::Configuration)?;
    run_match_spot_order_aeron_with_config(config)
}

pub fn run_match_spot_order_aeron_with_config(
    config: AeronListenerConfig,
) -> Result<(), AeronMatchSpotOrderError> {
    run_aeron_listener(config, MatchSpotOrderCreatedEventHandler::new())
        .map_err(AeronMatchSpotOrderError::from)
}
