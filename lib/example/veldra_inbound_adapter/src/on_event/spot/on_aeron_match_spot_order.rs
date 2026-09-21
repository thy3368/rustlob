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
use common_entity::{EntityReplayableEvent, ExecutionError};
use example_core_use_case::{MatchSpotOrderV2Cmd, MatchSpotOrderV2Error, ORDER_ENTITY_TYPE};
use example_outbound_adapter::DefaultSpotOrderV2PlaceOutboundError;
use thiserror::Error;
use use_case_executor::trading::spot::open_match_spot_order_v2_executor::execute_place_spot_order_v2;

const DEFAULT_AERON_CHANNEL: &str = "aeron:ipc";
const DEFAULT_AERON_STREAM_ID: i32 = 1001;
const DEFAULT_AERON_FRAGMENT_LIMIT: i32 = 10;
const AERON_CHANNEL_ENV: &str = "AERON_CHANNEL";
const AERON_STREAM_ID_ENV: &str = "AERON_STREAM_ID";
const AERON_FRAGMENT_LIMIT_ENV: &str = "AERON_FRAGMENT_LIMIT";

pub type MatchSpotOrderExecutorError =
    ExecutionError<MatchSpotOrderV2Error, DefaultSpotOrderV2PlaceOutboundError>;

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
) -> Option<MatchSpotOrderV2Cmd> {
    if event.entity_type != ORDER_ENTITY_TYPE || !event.is_created() {
        return None;
    }

    let party_id = event_field_value(event, "account_id")
        .and_then(|value| std::str::from_utf8(value).ok())
        .map(str::to_owned)?;
    let asset = event_field_value(event, "asset")
        .and_then(|value| std::str::from_utf8(value).ok())
        .and_then(|value| value.parse::<u32>().ok())?;
    let order_id = event_field_value(event, "order_id")
        .and_then(|value| std::str::from_utf8(value).ok())
        .map(str::to_owned)?;

    Some(MatchSpotOrderV2Cmd { party_id, asset, order_id })
}

pub fn handle_match_spot_order_fragment(payload: &[u8]) -> Result<(), AeronMatchSpotOrderError> {
    let event = decode_match_spot_order_event(payload).map_err(AeronMatchSpotOrderError::Decode)?;
    let Some(command) = match_spot_order_command_from_event(&event) else {
        return Ok(());
    };

    execute_place_spot_order_v2(&command).map(|_| ()).map_err(AeronMatchSpotOrderError::Executor)
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

fn event_field_value<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a [u8]> {
    event.field_changes.iter().find_map(|change| {
        (change.field_name_as_str().ok() == Some(field_name)).then(|| change.new_value_bytes())
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use common_entity::ReplayFieldChange;

    use super::*;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn decodes_valid_entity_replayable_event() {
        let event = decode_match_spot_order_event(&order_event_payload(
            &[("account_id", b"buyer"), ("asset", b"10001"), ("order_id", b"order-1")],
            ORDER_ENTITY_TYPE,
        ));

        assert!(event.is_ok());
        assert!(event.expect("valid replay event").is_created());
    }

    #[test]
    fn maps_order_created_event_to_match_command() {
        let event = decode_match_spot_order_event(&order_event_payload(
            &[("account_id", b"buyer"), ("asset", b"10001"), ("order_id", b"order-1")],
            ORDER_ENTITY_TYPE,
        ))
        .expect("valid replay event");

        assert_eq!(
            match_spot_order_command_from_event(&event),
            Some(MatchSpotOrderV2Cmd {
                party_id: "buyer".to_string(),
                asset: 10001,
                order_id: "order-1".to_string(),
            })
        );
    }

    #[test]
    fn ignores_non_order_entity_events() {
        let event = decode_match_spot_order_event(&order_event_payload(
            &[("account_id", b"buyer"), ("asset", b"10001"), ("order_id", b"order-1")],
            5,
        ))
        .expect("valid replay event");

        assert_eq!(match_spot_order_command_from_event(&event), None);
    }

    #[test]
    fn ignores_updated_and_deleted_order_events() {
        for change_type in [1, 2] {
            let mut event =
                EntityReplayableEvent::new(1, 1, 1, 2, 1, ORDER_ENTITY_TYPE, change_type);
            add_order_fields(&mut event);

            assert_eq!(match_spot_order_command_from_event(&event), None);
        }
    }

    #[test]
    fn ignores_order_events_with_missing_fields() {
        for missing_field in ["account_id", "asset", "order_id"] {
            let fields = [
                ("account_id", b"buyer".as_slice()),
                ("asset", b"10001".as_slice()),
                ("order_id", b"order-1".as_slice()),
            ]
            .into_iter()
            .filter(|(field_name, _)| *field_name != missing_field)
            .collect::<Vec<_>>();
            let mut event = EntityReplayableEvent::new_created(1, 1, 1, ORDER_ENTITY_TYPE);
            for (field_name, value) in fields {
                event.add_field_change(ReplayFieldChange::new(
                    ReplayFieldChange::field_name_from_str(field_name),
                    &[],
                    value,
                    0,
                ));
            }

            assert_eq!(match_spot_order_command_from_event(&event), None);
        }
    }

    #[test]
    fn ignores_order_events_with_invalid_asset_or_utf8_fields() {
        let invalid_asset = event_with_fields(
            &[("account_id", b"buyer"), ("asset", b"not-a-number"), ("order_id", b"order-1")],
            ORDER_ENTITY_TYPE,
        );
        assert_eq!(match_spot_order_command_from_event(&invalid_asset), None);

        let invalid_utf8 = event_with_fields(
            &[("account_id", &[0xff]), ("asset", b"10001"), ("order_id", b"order-1")],
            ORDER_ENTITY_TYPE,
        );
        assert_eq!(match_spot_order_command_from_event(&invalid_utf8), None);
    }

    #[test]
    fn malformed_json_is_returned_as_decode_error_by_fragment_handler() {
        let result = handle_match_spot_order_fragment(br#"{"entity_type":"order""#);

        assert!(matches!(result, Err(AeronMatchSpotOrderError::Decode(_))));
    }

    #[test]
    fn non_matching_events_are_ignored_by_fragment_handler() {
        let result = handle_match_spot_order_fragment(&order_event_payload(
            &[("account_id", b"buyer"), ("asset", b"10001"), ("order_id", b"order-1")],
            5,
        ));

        assert!(result.is_ok());
    }

    #[test]
    fn fragment_handler_reaches_existing_spot_order_executor() {
        let result = handle_match_spot_order_fragment(&order_event_payload(
            &[("account_id", b"buyer"), ("asset", b"10001"), ("order_id", b"order-1")],
            ORDER_ENTITY_TYPE,
        ));

        assert!(matches!(
            result,
            Err(AeronMatchSpotOrderError::Executor(ExecutionError::LoadState(
                DefaultSpotOrderV2PlaceOutboundError::StateUnavailable
            )))
        ));
    }

    #[test]
    fn config_uses_defaults_when_environment_is_missing() {
        with_environment(None, None, None, || {
            assert_eq!(AeronMatchSpotOrderConfig::from_env().ok(), Some(Default::default()));
        });
    }

    #[test]
    fn config_reads_environment_overrides() {
        with_environment(
            Some("aeron:udp?endpoint=127.0.0.1:40123"),
            Some("2001"),
            Some("25"),
            || {
                assert_eq!(
                    AeronMatchSpotOrderConfig::from_env().ok(),
                    Some(AeronMatchSpotOrderConfig {
                        channel: "aeron:udp?endpoint=127.0.0.1:40123".to_string(),
                        stream_id: 2001,
                        fragment_limit: 25,
                    })
                );
            },
        );
    }

    #[test]
    fn config_rejects_invalid_numeric_environment_values() {
        with_environment(None, Some("not-a-number"), None, || {
            assert!(matches!(
                AeronMatchSpotOrderConfig::from_env(),
                Err(AeronMatchSpotOrderConfigError::InvalidInteger {
                    variable: AERON_STREAM_ID_ENV,
                    ..
                })
            ));
        });
    }

    fn order_event_payload(fields: &[(&str, &[u8])], entity_type: u8) -> Vec<u8> {
        serde_json::to_vec(&event_with_fields(fields, entity_type)).expect("event should serialize")
    }

    fn event_with_fields(fields: &[(&str, &[u8])], entity_type: u8) -> EntityReplayableEvent {
        let mut event = EntityReplayableEvent::new_created(1, 1, 1, entity_type);
        for (field_name, value) in fields {
            event.add_field_change(ReplayFieldChange::new(
                ReplayFieldChange::field_name_from_str(field_name),
                &[],
                value,
                0,
            ));
        }
        event
    }

    fn add_order_fields(event: &mut EntityReplayableEvent) {
        for (field_name, value) in [
            ("account_id", b"buyer".as_slice()),
            ("asset", b"10001".as_slice()),
            ("order_id", b"order-1".as_slice()),
        ] {
            event.add_field_change(ReplayFieldChange::new(
                ReplayFieldChange::field_name_from_str(field_name),
                &[],
                value,
                0,
            ));
        }
    }

    fn with_environment<T>(
        channel: Option<&str>,
        stream_id: Option<&str>,
        fragment_limit: Option<&str>,
        test: impl FnOnce() -> T,
    ) -> T {
        let _guard = match ENV_LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let _environment = EnvironmentGuard::new();
        set_or_remove(AERON_CHANNEL_ENV, channel);
        set_or_remove(AERON_STREAM_ID_ENV, stream_id);
        set_or_remove(AERON_FRAGMENT_LIMIT_ENV, fragment_limit);
        test()
    }

    fn set_or_remove(name: &str, value: Option<&str>) {
        match value {
            Some(value) => std::env::set_var(name, value),
            None => std::env::remove_var(name),
        }
    }

    struct EnvironmentGuard {
        previous: [(&'static str, Option<std::ffi::OsString>); 3],
    }

    impl EnvironmentGuard {
        fn new() -> Self {
            Self {
                previous: [
                    (AERON_CHANNEL_ENV, std::env::var_os(AERON_CHANNEL_ENV)),
                    (AERON_STREAM_ID_ENV, std::env::var_os(AERON_STREAM_ID_ENV)),
                    (AERON_FRAGMENT_LIMIT_ENV, std::env::var_os(AERON_FRAGMENT_LIMIT_ENV)),
                ],
            }
        }
    }

    impl Drop for EnvironmentGuard {
        fn drop(&mut self) {
            for (name, value) in &self.previous {
                match value {
                    Some(value) => std::env::set_var(name, value),
                    None => std::env::remove_var(name),
                }
            }
        }
    }
}
