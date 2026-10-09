use std::cell::RefCell;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::context::Context;
use aeron_rs::utils::errors::AeronError;
use aeron_rs::utils::types::Index;
use common_entity::{EntityReplayableEvent, EntityReplayableEventHandler};
use thiserror::Error;
use use_case_executor::trading::spot::on_event::{
    DefaultMatchSpotOrderCreatedEventError, MatchSpotOrderCreatedEventHandler,
    MatchSpotOrderCreatedEventParseError, MatchSpotOrderExecutorError,
    match_spot_order_command_from_event as command_from_event,
};

const DEFAULT_AERON_CHANNEL: &str = "aeron:ipc";
const DEFAULT_AERON_STREAM_ID: i32 = 1001;
const DEFAULT_AERON_FRAGMENT_LIMIT: i32 = 10;
const AERON_CHANNEL_ENV: &str = "AERON_CHANNEL";
const AERON_STREAM_ID_ENV: &str = "AERON_STREAM_ID";
const AERON_FRAGMENT_LIMIT_ENV: &str = "AERON_FRAGMENT_LIMIT";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AeronMatchSpotOrderConfig {
    pub channel: String,
    pub stream_id: i32,
    pub fragment_limit: i32,
}

impl Default for AeronMatchSpotOrderConfig {
    fn default() -> Self {
        Self {
            channel: DEFAULT_AERON_CHANNEL.to_string(),
            stream_id: DEFAULT_AERON_STREAM_ID,
            fragment_limit: DEFAULT_AERON_FRAGMENT_LIMIT,
        }
    }
}

impl AeronMatchSpotOrderConfig {
    pub fn from_env() -> Result<Self, AeronMatchSpotOrderConfigError> {
        Self::from_values(
            read_env(AERON_CHANNEL_ENV)?,
            read_env(AERON_STREAM_ID_ENV)?,
            read_env(AERON_FRAGMENT_LIMIT_ENV)?,
        )
    }

    fn from_values(
        channel: Option<String>,
        stream_id: Option<String>,
        fragment_limit: Option<String>,
    ) -> Result<Self, AeronMatchSpotOrderConfigError> {
        let config = Self {
            channel: channel.unwrap_or_else(|| DEFAULT_AERON_CHANNEL.to_string()),
            stream_id: parse_i32(AERON_STREAM_ID_ENV, stream_id, DEFAULT_AERON_STREAM_ID)?,
            fragment_limit: parse_positive_i32(
                AERON_FRAGMENT_LIMIT_ENV,
                fragment_limit,
                DEFAULT_AERON_FRAGMENT_LIMIT,
            )?,
        };
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), AeronMatchSpotOrderConfigError> {
        if self.channel.is_empty() {
            return Err(AeronMatchSpotOrderConfigError::EmptyChannel);
        }
        if self.channel.as_bytes().contains(&0) {
            return Err(AeronMatchSpotOrderConfigError::ChannelContainsNul);
        }
        if self.fragment_limit <= 0 {
            return Err(AeronMatchSpotOrderConfigError::NonPositiveFragmentLimit(
                self.fragment_limit,
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AeronMatchSpotOrderConfigError {
    #[error("Aeron channel must not be empty")]
    EmptyChannel,
    #[error("Aeron channel contains an embedded NUL byte")]
    ChannelContainsNul,
    #[error("{0} contains invalid UTF-8")]
    InvalidUnicode(&'static str),
    #[error("{variable} must be a valid signed 32-bit integer, got {value:?}")]
    InvalidInteger { variable: &'static str, value: String },
    #[error("AERON_FRAGMENT_LIMIT must be greater than zero, got {0}")]
    NonPositiveFragmentLimit(i32),
}

#[derive(Debug, Error)]
pub enum AeronMatchSpotOrderError {
    #[error("invalid Aeron configuration: {0}")]
    Configuration(#[from] AeronMatchSpotOrderConfigError),
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
    serde_json::from_slice(payload)
}

pub fn match_spot_order_command_from_event(
    event: &EntityReplayableEvent,
) -> Result<Option<example_core_use_case::MatchSpotOrderV3Cmd>, MatchSpotOrderCreatedEventParseError>
{
    command_from_event(event)
}

pub fn handle_match_spot_order_fragment(payload: &[u8]) -> Result<(), AeronMatchSpotOrderError> {
    let event = decode_match_spot_order_event(payload).map_err(AeronMatchSpotOrderError::Decode)?;
    let mut handler = MatchSpotOrderCreatedEventHandler::new();

    handler.handle(&event).map(|_| ()).map_err(AeronMatchSpotOrderError::from)
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
    let config =
        AeronMatchSpotOrderConfig::from_env().map_err(AeronMatchSpotOrderError::Configuration)?;
    run_match_spot_order_aeron_with_config(config)
}

pub fn run_match_spot_order_aeron_with_config(
    config: AeronMatchSpotOrderConfig,
) -> Result<(), AeronMatchSpotOrderError> {
    config.validate().map_err(MatchSpotOrderAeronError::Configuration)?;

    let channel = CString::new(config.channel.as_str()).map_err(|_| {
        MatchSpotOrderAeronError::Configuration(AeronMatchSpotOrderConfigError::ChannelContainsNul)
    })?;
    let mut aeron = Aeron::new(Context::new()).map_err(MatchSpotOrderAeronError::Aeron)?;
    let subscription_id = aeron
        .add_subscription(channel, config.stream_id)
        .map_err(MatchSpotOrderAeronError::Subscription)?;
    let subscription = wait_for_subscription(&mut aeron, subscription_id)?;

    let first_error = RefCell::new(None);
    let mut fragment_handler =
        |buffer: &AtomicBuffer, offset: Index, length: Index, _header: &Header| {
            if first_error.borrow().is_some() {
                return;
            }

            let payload = match fragment_payload(buffer, offset, length) {
                Ok(payload) => payload,
                Err(error) => {
                    *first_error.borrow_mut() = Some(error);
                    return;
                }
            };

            if let Err(error) = handle_match_spot_order_fragment(payload) {
                *first_error.borrow_mut() = Some(error);
            }
        };

    loop {
        let fragments_read = {
            let mut subscription =
                subscription.lock().map_err(|_| MatchSpotOrderAeronError::SubscriptionLock)?;
            subscription.poll(&mut fragment_handler, config.fragment_limit)
        };

        if let Some(error) = first_error.borrow_mut().take() {
            return Err(error);
        }
        if fragments_read == 0 {
            thread::yield_now();
        }
    }
}

fn wait_for_subscription(
    aeron: &mut Aeron,
    subscription_id: i64,
) -> Result<Arc<Mutex<aeron_rs::subscription::Subscription>>, AeronMatchSpotOrderError> {
    loop {
        match aeron.find_subscription(subscription_id) {
            Ok(subscription) => return Ok(subscription),
            Err(AeronError::SubscriptionNotReady(_)) => thread::yield_now(),
            Err(error) => return Err(MatchSpotOrderAeronError::Subscription(error)),
        }
    }
}

fn fragment_payload(
    buffer: &AtomicBuffer,
    offset: Index,
    length: Index,
) -> Result<&[u8], AeronMatchSpotOrderError> {
    let capacity = buffer.capacity();
    let (Ok(offset), Ok(length)) = (usize::try_from(offset), usize::try_from(length)) else {
        return Err(AeronMatchSpotOrderError::InvalidFragmentBounds { offset, length, capacity });
    };
    let Some(end) = offset.checked_add(length) else {
        return Err(AeronMatchSpotOrderError::InvalidFragmentBounds {
            offset: offset as Index,
            length: length as Index,
            capacity,
        });
    };
    buffer.as_slice().get(offset..end).ok_or(AeronMatchSpotOrderError::InvalidFragmentBounds {
        offset: offset as Index,
        length: length as Index,
        capacity,
    })
}

fn read_env(name: &'static str) -> Result<Option<String>, AeronMatchSpotOrderConfigError> {
    match std::env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => {
            Err(AeronMatchSpotOrderConfigError::InvalidUnicode(name))
        }
    }
}

fn parse_i32(
    variable: &'static str,
    value: Option<String>,
    default: i32,
) -> Result<i32, AeronMatchSpotOrderConfigError> {
    value.map_or(Ok(default), |value| {
        value
            .parse::<i32>()
            .map_err(|_| AeronMatchSpotOrderConfigError::InvalidInteger { variable, value })
    })
}

fn parse_positive_i32(
    variable: &'static str,
    value: Option<String>,
    default: i32,
) -> Result<i32, AeronMatchSpotOrderConfigError> {
    let value = parse_i32(variable, value, default)?;
    if value <= 0 {
        return Err(AeronMatchSpotOrderConfigError::NonPositiveFragmentLimit(value));
    }
    Ok(value)
}
