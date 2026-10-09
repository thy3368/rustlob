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
use use_case_executor::trading::spot::on_event::{
    DefaultMatchSpotOrderCreatedEventError, MatchSpotOrderCreatedEventHandler,
    MatchSpotOrderCreatedEventParseError, MatchSpotOrderExecutorError,
    match_spot_order_command_from_event as command_from_event,
};

const DEFAULT_AERON_CHANNEL: &str = "aeron:ipc";
const DEFAULT_MATCH_SPOT_ORDER_STREAM_ID: i32 = 1001;
const DEFAULT_SPOT_TRADE_STREAM_ID: i32 = 1002;
const DEFAULT_AERON_FRAGMENT_LIMIT: i32 = 10;

const LEGACY_AERON_CHANNEL_ENV: &str = "AERON_CHANNEL";
const LEGACY_AERON_STREAM_ID_ENV: &str = "AERON_STREAM_ID";
const LEGACY_AERON_FRAGMENT_LIMIT_ENV: &str = "AERON_FRAGMENT_LIMIT";

const MATCH_SPOT_ORDER_CHANNEL_ENV: &str = "AERON_MATCH_SPOT_ORDER_CHANNEL";
const MATCH_SPOT_ORDER_STREAM_ID_ENV: &str = "AERON_MATCH_SPOT_ORDER_STREAM_ID";
const MATCH_SPOT_ORDER_FRAGMENT_LIMIT_ENV: &str = "AERON_MATCH_SPOT_ORDER_FRAGMENT_LIMIT";

const SPOT_TRADE_CHANNEL_ENV: &str = "AERON_SPOT_TRADE_CHANNEL";
const SPOT_TRADE_STREAM_ID_ENV: &str = "AERON_SPOT_TRADE_STREAM_ID";
const SPOT_TRADE_FRAGMENT_LIMIT_ENV: &str = "AERON_SPOT_TRADE_FRAGMENT_LIMIT";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AeronListenerConfig {
    pub channel: String,
    pub stream_id: i32,
    pub fragment_limit: i32,
}

impl AeronListenerConfig {
    pub fn from_env() -> Result<Self, AeronListenerConfigError> {
        Self::match_spot_order_from_env()
    }

    pub fn match_spot_order_from_env() -> Result<Self, AeronListenerConfigError> {
        let env = ListenerEnv {
            channel: MATCH_SPOT_ORDER_CHANNEL_ENV,
            stream_id: MATCH_SPOT_ORDER_STREAM_ID_ENV,
            fragment_limit: MATCH_SPOT_ORDER_FRAGMENT_LIMIT_ENV,
        };
        let legacy_env = ListenerEnv {
            channel: LEGACY_AERON_CHANNEL_ENV,
            stream_id: LEGACY_AERON_STREAM_ID_ENV,
            fragment_limit: LEGACY_AERON_FRAGMENT_LIMIT_ENV,
        };
        Self::from_value_sets(
            read_value_set(env)?,
            DEFAULT_MATCH_SPOT_ORDER_STREAM_ID,
            env,
            Some(read_value_set(legacy_env)?),
        )
    }

    pub fn spot_trade_from_env() -> Result<Self, AeronListenerConfigError> {
        let env = ListenerEnv {
            channel: SPOT_TRADE_CHANNEL_ENV,
            stream_id: SPOT_TRADE_STREAM_ID_ENV,
            fragment_limit: SPOT_TRADE_FRAGMENT_LIMIT_ENV,
        };
        Self::from_value_sets(read_value_set(env)?, DEFAULT_SPOT_TRADE_STREAM_ID, env, None)
    }

    pub fn match_spot_order_default() -> Self {
        Self::default_for_stream(DEFAULT_MATCH_SPOT_ORDER_STREAM_ID)
    }

    pub fn spot_trade_default() -> Self {
        Self::default_for_stream(DEFAULT_SPOT_TRADE_STREAM_ID)
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

    fn default_for_stream(stream_id: i32) -> Self {
        Self {
            channel: DEFAULT_AERON_CHANNEL.to_string(),
            stream_id,
            fragment_limit: DEFAULT_AERON_FRAGMENT_LIMIT,
        }
    }

    fn from_value_sets(
        values: ListenerValues,
        default_stream_id: i32,
        env: ListenerEnv,
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

impl Default for AeronListenerConfig {
    fn default() -> Self {
        Self::match_spot_order_default()
    }
}

#[derive(Debug, Clone, Copy)]
struct ListenerEnv {
    channel: &'static str,
    stream_id: &'static str,
    fragment_limit: &'static str,
}

#[derive(Debug, Default)]
struct ListenerValues {
    channel: Option<String>,
    stream_id: Option<String>,
    fragment_limit: Option<String>,
}

pub type AeronMatchSpotOrderConfig = AeronListenerConfig;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AeronListenerConfigError {
    #[error("Aeron channel must not be empty")]
    EmptyChannel,
    #[error("Aeron channel contains an embedded NUL byte")]
    ChannelContainsNul,
    #[error("{0} contains invalid UTF-8")]
    InvalidUnicode(&'static str),
    #[error("{variable} must be a valid signed 32-bit integer, got {value:?}")]
    InvalidInteger { variable: &'static str, value: String },
    #[error("{variable} must be greater than zero, got {value}")]
    NonPositiveFragmentLimit { variable: &'static str, value: i32 },
}

pub type AeronMatchSpotOrderConfigError = AeronListenerConfigError;

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
    InvalidFragmentBounds { offset: Index, length: Index, capacity: Index },
    #[error("failed to decode Aeron fragment as EntityReplayableEvent: {0}")]
    Decode(#[source] serde_json::Error),
    #[error("failed to handle Aeron replayable event: {0:?}")]
    EventHandler(HandlerError),
}

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

pub fn decode_entity_replayable_event(
    payload: &[u8],
) -> Result<EntityReplayableEvent, serde_json::Error> {
    serde_json::from_slice(payload)
}

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

pub fn handle_aeron_event_fragment<H>(
    payload: &[u8],
    handler: &mut H,
) -> Result<Option<H::Output>, AeronListenerError<H::Error>>
where
    H: EntityReplayableEventHandler,
    H::Error: Debug,
{
    let event = decode_entity_replayable_event(payload).map_err(AeronListenerError::Decode)?;
    handler.handle(&event).map_err(AeronListenerError::EventHandler)
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

pub fn run_aeron_listener<H>(
    config: AeronListenerConfig,
    mut handler: H,
) -> Result<(), AeronListenerError<H::Error>>
where
    H: EntityReplayableEventHandler,
    H::Error: Debug,
{
    config.validate().map_err(AeronListenerError::Configuration)?;

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
            let mut subscription =
                subscription.lock().map_err(|_| AeronListenerError::SubscriptionLock)?;
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
        return Err(AeronListenerError::InvalidFragmentBounds { offset, length, capacity });
    };
    let Some(end) = offset.checked_add(length) else {
        return Err(AeronListenerError::InvalidFragmentBounds {
            offset: offset as Index,
            length: length as Index,
            capacity,
        });
    };
    buffer.as_slice().get(offset..end).ok_or(AeronListenerError::InvalidFragmentBounds {
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

fn read_value_set(env: ListenerEnv) -> Result<ListenerValues, AeronListenerConfigError> {
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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
    use common_entity::{Entity, EntityChangeType, EntityReplayableEvent, ReplayFieldChange};
    use example_core_use_case::{KlineUpdate, SpotTrade};
    use use_case_executor::trading::spot::on_event::SpotTradeCreatedEventHandler;

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum TestHandlerError {
        Failed,
    }

    #[derive(Debug, Default)]
    struct RecordingHandler {
        handled: Vec<EntityReplayableEvent>,
    }

    impl EntityReplayableEventHandler for RecordingHandler {
        type Output = usize;
        type Error = TestHandlerError;

        fn handle(
            &mut self,
            event: &EntityReplayableEvent,
        ) -> Result<Option<Self::Output>, Self::Error> {
            self.handled.push(event.clone());
            Ok(Some(self.handled.len()))
        }
    }

    #[derive(Debug)]
    struct FailingHandler;

    impl EntityReplayableEventHandler for FailingHandler {
        type Output = ();
        type Error = TestHandlerError;

        fn handle(
            &mut self,
            _event: &EntityReplayableEvent,
        ) -> Result<Option<Self::Output>, Self::Error> {
            Err(TestHandlerError::Failed)
        }
    }

    #[derive(Debug, Default, Clone)]
    struct SharedCountingHandler {
        calls: Arc<Mutex<usize>>,
    }

    impl EntityReplayableEventHandler for SharedCountingHandler {
        type Output = usize;
        type Error = TestHandlerError;

        fn handle(
            &mut self,
            _event: &EntityReplayableEvent,
        ) -> Result<Option<Self::Output>, Self::Error> {
            let mut calls = self.calls.lock().expect("test counter mutex should not be poisoned");
            *calls += 1;
            Ok(Some(*calls))
        }
    }

    #[test]
    fn match_spot_order_default_config_uses_legacy_stream() {
        assert_eq!(
            AeronListenerConfig::match_spot_order_default(),
            AeronListenerConfig {
                channel: DEFAULT_AERON_CHANNEL.to_owned(),
                stream_id: 1001,
                fragment_limit: 10,
            }
        );
    }

    #[test]
    fn spot_trade_default_config_uses_trade_stream() {
        assert_eq!(
            AeronListenerConfig::spot_trade_default(),
            AeronListenerConfig {
                channel: DEFAULT_AERON_CHANNEL.to_owned(),
                stream_id: 1002,
                fragment_limit: 10,
            }
        );
    }

    #[test]
    fn dedicated_config_values_are_used_when_present() {
        let config = AeronListenerConfig::from_values(
            Some("aeron:udp?endpoint=127.0.0.1:40123".to_string()),
            Some("2001".to_string()),
            Some("33".to_string()),
            MATCH_SPOT_ORDER_STREAM_ID_ENV,
            MATCH_SPOT_ORDER_FRAGMENT_LIMIT_ENV,
            DEFAULT_MATCH_SPOT_ORDER_STREAM_ID,
        )
        .expect("dedicated values should parse");

        assert_eq!(config.channel, "aeron:udp?endpoint=127.0.0.1:40123");
        assert_eq!(config.stream_id, 2001);
        assert_eq!(config.fragment_limit, 33);
    }

    #[test]
    fn legacy_config_values_are_order_listener_fallbacks() {
        let env = ListenerEnv {
            channel: MATCH_SPOT_ORDER_CHANNEL_ENV,
            stream_id: MATCH_SPOT_ORDER_STREAM_ID_ENV,
            fragment_limit: MATCH_SPOT_ORDER_FRAGMENT_LIMIT_ENV,
        };
        let config = AeronListenerConfig::from_value_sets(
            ListenerValues::default(),
            DEFAULT_MATCH_SPOT_ORDER_STREAM_ID,
            env,
            Some(ListenerValues {
                channel: Some("aeron:udp?endpoint=127.0.0.1:40124".to_string()),
                stream_id: Some("3001".to_string()),
                fragment_limit: Some("44".to_string()),
            }),
        )
        .expect("legacy values should parse");

        assert_eq!(config.channel, "aeron:udp?endpoint=127.0.0.1:40124");
        assert_eq!(config.stream_id, 3001);
        assert_eq!(config.fragment_limit, 44);
    }

    #[test]
    fn dedicated_order_config_values_override_legacy_fallbacks() {
        let env = ListenerEnv {
            channel: MATCH_SPOT_ORDER_CHANNEL_ENV,
            stream_id: MATCH_SPOT_ORDER_STREAM_ID_ENV,
            fragment_limit: MATCH_SPOT_ORDER_FRAGMENT_LIMIT_ENV,
        };
        let config = AeronListenerConfig::from_value_sets(
            ListenerValues {
                channel: Some("aeron:udp?endpoint=127.0.0.1:40125".to_string()),
                stream_id: Some("4001".to_string()),
                fragment_limit: Some("55".to_string()),
            },
            DEFAULT_MATCH_SPOT_ORDER_STREAM_ID,
            env,
            Some(ListenerValues {
                channel: Some("aeron:udp?endpoint=127.0.0.1:40124".to_string()),
                stream_id: Some("3001".to_string()),
                fragment_limit: Some("44".to_string()),
            }),
        )
        .expect("dedicated values should parse");

        assert_eq!(config.channel, "aeron:udp?endpoint=127.0.0.1:40125");
        assert_eq!(config.stream_id, 4001);
        assert_eq!(config.fragment_limit, 55);
    }

    #[test]
    fn config_rejects_invalid_stream_id() {
        let error = AeronListenerConfig::from_values(
            None,
            Some("not-a-stream".to_string()),
            None,
            SPOT_TRADE_STREAM_ID_ENV,
            SPOT_TRADE_FRAGMENT_LIMIT_ENV,
            DEFAULT_SPOT_TRADE_STREAM_ID,
        )
        .expect_err("invalid stream id should fail");

        assert_eq!(
            error,
            AeronListenerConfigError::InvalidInteger {
                variable: SPOT_TRADE_STREAM_ID_ENV,
                value: "not-a-stream".to_string(),
            }
        );
    }

    #[test]
    fn config_rejects_invalid_fragment_limit() {
        let error = AeronListenerConfig::from_values(
            None,
            None,
            Some("0".to_string()),
            SPOT_TRADE_STREAM_ID_ENV,
            SPOT_TRADE_FRAGMENT_LIMIT_ENV,
            DEFAULT_SPOT_TRADE_STREAM_ID,
        )
        .expect_err("non-positive fragment limit should fail");

        assert_eq!(
            error,
            AeronListenerConfigError::NonPositiveFragmentLimit {
                variable: SPOT_TRADE_FRAGMENT_LIMIT_ENV,
                value: 0,
            }
        );
    }

    #[test]
    fn config_rejects_empty_channel() {
        let error = AeronListenerConfig::from_values(
            Some(String::new()),
            None,
            None,
            SPOT_TRADE_STREAM_ID_ENV,
            SPOT_TRADE_FRAGMENT_LIMIT_ENV,
            DEFAULT_SPOT_TRADE_STREAM_ID,
        )
        .expect_err("empty channel should fail");

        assert_eq!(error, AeronListenerConfigError::EmptyChannel);
    }

    #[test]
    fn config_rejects_nul_channel() {
        let error = AeronListenerConfig::from_values(
            Some("aeron:ipc\0suffix".to_string()),
            None,
            None,
            SPOT_TRADE_STREAM_ID_ENV,
            SPOT_TRADE_FRAGMENT_LIMIT_ENV,
            DEFAULT_SPOT_TRADE_STREAM_ID,
        )
        .expect_err("NUL channel should fail");

        assert_eq!(error, AeronListenerConfigError::ChannelContainsNul);
    }

    #[test]
    fn fragment_handler_decodes_event_and_calls_injected_handler() {
        let event = event_with_fields(
            7,
            EntityChangeType::Created,
            &[("symbol", "BTCUSDT"), ("price", "100"), ("qty", "2")],
        );
        let payload = serde_json::to_vec(&event).expect("event should serialize");
        let mut handler = RecordingHandler::default();

        let output = handle_aeron_event_fragment(&payload, &mut handler)
            .expect("valid event should be handled");

        assert_eq!(output, Some(1));
        assert_eq!(handler.handled, vec![event]);
    }

    #[test]
    fn fragment_handler_returns_json_decode_error() {
        let mut handler = RecordingHandler::default();
        let error = handle_aeron_event_fragment(b"{not-json", &mut handler)
            .expect_err("invalid JSON should fail");

        assert!(matches!(error, AeronListenerError::Decode(_)));
        assert!(handler.handled.is_empty());
    }

    #[test]
    fn fragment_payload_rejects_out_of_bounds_range() {
        let mut bytes = [0_u8; 4];
        let buffer = AtomicBuffer::wrap_slice(&mut bytes);

        let error = fragment_payload::<TestHandlerError>(&buffer, 3, 2)
            .expect_err("fragment range beyond the buffer should fail");

        assert!(matches!(
            error,
            AeronListenerError::InvalidFragmentBounds { offset: 3, length: 2, capacity: 4 }
        ));
    }

    #[test]
    fn fragment_handler_returns_handler_error() {
        let event = event_with_fields(7, EntityChangeType::Created, &[]);
        let payload = serde_json::to_vec(&event).expect("event should serialize");
        let mut handler = FailingHandler;
        let error = handle_aeron_event_fragment(&payload, &mut handler)
            .expect_err("handler error should fail");

        assert!(matches!(error, AeronListenerError::EventHandler(TestHandlerError::Failed)));
    }

    #[test]
    fn repeated_fragment_handling_reuses_the_same_handler_instance() {
        let event = event_with_fields(7, EntityChangeType::Created, &[]);
        let payload = serde_json::to_vec(&event).expect("event should serialize");
        let calls = Arc::new(Mutex::new(0));
        let mut handler = SharedCountingHandler { calls: Arc::clone(&calls) };

        assert_eq!(
            handle_aeron_event_fragment(&payload, &mut handler)
                .expect("first event should be handled"),
            Some(1)
        );
        assert_eq!(
            handle_aeron_event_fragment(&payload, &mut handler)
                .expect("second event should be handled"),
            Some(2)
        );
        assert_eq!(*calls.lock().expect("test counter mutex should not be poisoned"), 2);
    }

    #[test]
    fn spot_trade_handler_keeps_kline_state_between_fragments() {
        let first = spot_trade_event(1_000, 100, 2);
        let second = spot_trade_event(2_000, 120, 3);
        let first_payload = serde_json::to_vec(&first).expect("event should serialize");
        let second_payload = serde_json::to_vec(&second).expect("event should serialize");
        let mut handler = SpotTradeCreatedEventHandler::default();

        handle_aeron_event_fragment(&first_payload, &mut handler)
            .expect("first trade should update klines");
        let updates = handle_aeron_event_fragment(&second_payload, &mut handler)
            .expect("second trade should update klines")
            .expect("trade event should be observed");

        assert!(updates.iter().any(|update| match update {
            KlineUpdate::Updated(candle) => {
                candle.open == 100 && candle.high == 120 && candle.low == 100 && candle.close == 120
            }
            KlineUpdate::Closed(_) => false,
        }));
    }

    fn spot_trade_event(executed_at_ms: u64, price: u64, qty: u64) -> EntityReplayableEvent {
        event_with_fields(
            SpotTrade::entity_type(),
            EntityChangeType::Created,
            &[
                ("symbol", "BTCUSDT"),
                ("price", &price.to_string()),
                ("qty", &qty.to_string()),
                ("executed_at_ms", &executed_at_ms.to_string()),
            ],
        )
    }

    fn event_with_fields(
        entity_type: u8,
        change_type: EntityChangeType,
        fields: &[(&str, &str)],
    ) -> EntityReplayableEvent {
        let mut event =
            EntityReplayableEvent::new(1, 1, 0, 1, 1, entity_type, change_type.as_tag());
        for (field_name, value) in fields {
            event.add_field_change(ReplayFieldChange::new(
                ReplayFieldChange::field_name_from_str(field_name),
                b"",
                value.as_bytes(),
                0,
            ));
        }
        event
    }
}
