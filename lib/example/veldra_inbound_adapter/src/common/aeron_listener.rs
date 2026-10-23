use std::cell::RefCell;
use std::ffi::CString;
use std::fmt::Debug;
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

pub const DEFAULT_AERON_CHANNEL: &str = "aeron:ipc";
pub const DEFAULT_AERON_FRAGMENT_LIMIT: i32 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AeronListenerConfig {
    pub channel: String,
    pub stream_id: i32,
    pub fragment_limit: i32,
}

impl AeronListenerConfig {
    pub fn default_for_stream(stream_id: i32) -> Self {
        Self {
            channel: DEFAULT_AERON_CHANNEL.to_string(),
            stream_id,
            fragment_limit: DEFAULT_AERON_FRAGMENT_LIMIT,
        }
    }

    pub fn from_env_vars(
        env: AeronListenerEnv,
        default_stream_id: i32,
        legacy_env: Option<AeronListenerEnv>,
    ) -> Result<Self, AeronListenerConfigError> {
        let legacy_values = legacy_env.map(read_value_set).transpose()?;
        Self::from_value_sets(read_value_set(env)?, default_stream_id, env, legacy_values)
    }

    pub fn validate(&self) -> Result<(), AeronListenerConfigError> {
        if self.channel.is_empty() {
            return Err(AeronListenerConfigError::EmptyChannel);
        }
        if self.channel.as_bytes().contains(&0) {
            return Err(AeronListenerConfigError::ChannelContainsNul);
        }
        if self.fragment_limit <= 0 {
            return Err(AeronListenerConfigError::NonPositiveFragmentLimit {
                variable: "fragment_limit",
                value: self.fragment_limit,
            });
        }
        Ok(())
    }

    fn from_value_sets(
        values: ListenerValues,
        default_stream_id: i32,
        env: AeronListenerEnv,
        legacy_values: Option<ListenerValues>,
    ) -> Result<Self, AeronListenerConfigError> {
        let legacy_values = legacy_values.unwrap_or_default();

        Self::from_values(
            values.channel.or(legacy_values.channel),
            values.stream_id.or(legacy_values.stream_id),
            values.fragment_limit.or(legacy_values.fragment_limit),
            env.stream_id,
            env.fragment_limit,
            default_stream_id,
        )
    }

    fn from_values(
        channel: Option<String>,
        stream_id: Option<String>,
        fragment_limit: Option<String>,
        stream_id_variable: &'static str,
        fragment_limit_variable: &'static str,
        default_stream_id: i32,
    ) -> Result<Self, AeronListenerConfigError> {
        let config = Self {
            channel: channel.unwrap_or_else(|| DEFAULT_AERON_CHANNEL.to_string()),
            stream_id: parse_i32(stream_id_variable, stream_id, default_stream_id)?,
            fragment_limit: parse_positive_i32(
                fragment_limit_variable,
                fragment_limit,
                DEFAULT_AERON_FRAGMENT_LIMIT,
            )?,
        };
        config.validate()?;
        Ok(config)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AeronListenerEnv {
    pub channel: &'static str,
    pub stream_id: &'static str,
    pub fragment_limit: &'static str,
}

#[derive(Debug, Default)]
struct ListenerValues {
    channel: Option<String>,
    stream_id: Option<String>,
    fragment_limit: Option<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AeronListenerConfigError {
    #[error("Aeron channel must not be empty")]
    EmptyChannel,
    #[error("Aeron channel contains an embedded NUL byte")]
    ChannelContainsNul,
    #[error("{0} contains invalid UTF-8")]
    InvalidUnicode(&'static str),
    #[error("{variable} must be a valid signed 32-bit integer, got {value:?}")]
    InvalidInteger {
        variable: &'static str,
        value: String,
    },
    #[error("{variable} must be greater than zero, got {value}")]
    NonPositiveFragmentLimit { variable: &'static str, value: i32 },
}

#[derive(Debug, Error)]
pub enum AeronListenerError<HandlerError>
where
    HandlerError: Debug,
{
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
    InvalidFragmentBounds {
        offset: Index,
        length: Index,
        capacity: Index,
    },
    #[error("failed to decode Aeron fragment as EntityReplayableEvent: {0}")]
    Decode(#[source] serde_json::Error),
    #[error("failed to handle Aeron replayable event: {0:?}")]
    EventHandler(HandlerError),
}

pub fn decode_entity_replayable_event(
    payload: &[u8],
) -> Result<EntityReplayableEvent, serde_json::Error> {
    serde_json::from_slice(payload)
}

pub fn handle_aeron_event_fragment<H>(
    payload: &[u8],
    handler: &mut H,
) -> Result<Option<H::Output>, AeronListenerError<H::Error>>
where
    H: EntityReplayableEventHandler,
    H::Error: Debug,
{
    let event = decode_entity_replayable_event(payload).map_err(AeronListenerError::Decode)?;
    handler
        .handle(&event)
        .map_err(AeronListenerError::EventHandler)
}

pub fn run_aeron_listener<H>(
    config: AeronListenerConfig,
    mut handler: H,
) -> Result<(), AeronListenerError<H::Error>>
where
    H: EntityReplayableEventHandler,
    H::Error: Debug,
{
    config
        .validate()
        .map_err(AeronListenerError::Configuration)?;

    let channel = CString::new(config.channel.as_str()).map_err(|_| {
        AeronListenerError::Configuration(AeronListenerConfigError::ChannelContainsNul)
    })?;
    let mut aeron = Aeron::new(Context::new()).map_err(AeronListenerError::Aeron)?;
    let subscription_id = aeron
        .add_subscription(channel, config.stream_id)
        .map_err(AeronListenerError::Subscription)?;
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

            if let Err(error) = handle_aeron_event_fragment(payload, &mut handler) {
                *first_error.borrow_mut() = Some(error);
            }
        };

    loop {
        let fragments_read = {
            let mut subscription = subscription
                .lock()
                .map_err(|_| AeronListenerError::SubscriptionLock)?;
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

fn wait_for_subscription<HandlerError>(
    aeron: &mut Aeron,
    subscription_id: i64,
) -> Result<Arc<Mutex<aeron_rs::subscription::Subscription>>, AeronListenerError<HandlerError>>
where
    HandlerError: Debug,
{
    loop {
        match aeron.find_subscription(subscription_id) {
            Ok(subscription) => return Ok(subscription),
            Err(AeronError::SubscriptionNotReady(_)) => thread::yield_now(),
            Err(error) => return Err(AeronListenerError::Subscription(error)),
        }
    }
}

fn fragment_payload<HandlerError>(
    buffer: &AtomicBuffer,
    offset: Index,
    length: Index,
) -> Result<&[u8], AeronListenerError<HandlerError>>
where
    HandlerError: Debug,
{
    let capacity = buffer.capacity();
    let (Ok(offset), Ok(length)) = (usize::try_from(offset), usize::try_from(length)) else {
        return Err(AeronListenerError::InvalidFragmentBounds {
            offset,
            length,
            capacity,
        });
    };
    let Some(end) = offset.checked_add(length) else {
        return Err(AeronListenerError::InvalidFragmentBounds {
            offset: offset as Index,
            length: length as Index,
            capacity,
        });
    };
    buffer
        .as_slice()
        .get(offset..end)
        .ok_or(AeronListenerError::InvalidFragmentBounds {
            offset: offset as Index,
            length: length as Index,
            capacity,
        })
}

fn read_env(name: &'static str) -> Result<Option<String>, AeronListenerConfigError> {
    match std::env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => {
            Err(AeronListenerConfigError::InvalidUnicode(name))
        }
    }
}

fn read_value_set(env: AeronListenerEnv) -> Result<ListenerValues, AeronListenerConfigError> {
    Ok(ListenerValues {
        channel: read_env(env.channel)?,
        stream_id: read_env(env.stream_id)?,
        fragment_limit: read_env(env.fragment_limit)?,
    })
}

fn parse_i32(
    variable: &'static str,
    value: Option<String>,
    default: i32,
) -> Result<i32, AeronListenerConfigError> {
    value.map_or(Ok(default), |value| {
        value
            .parse::<i32>()
            .map_err(|_| AeronListenerConfigError::InvalidInteger { variable, value })
    })
}

fn parse_positive_i32(
    variable: &'static str,
    value: Option<String>,
    default: i32,
) -> Result<i32, AeronListenerConfigError> {
    let value = parse_i32(variable, value, default)?;
    if value <= 0 {
        return Err(AeronListenerConfigError::NonPositiveFragmentLimit { variable, value });
    }
    Ok(value)
}
