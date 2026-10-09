use use_case_executor::trading::spot::on_event::{
    SpotTradeCreatedEventError, SpotTradeCreatedEventHandler,
};

use crate::common::aeron_listener::{
    AeronListenerConfig, AeronListenerConfigError, AeronListenerEnv, AeronListenerError,
    run_aeron_listener,
};

const DEFAULT_SPOT_TRADE_STREAM_ID: i32 = 1002;

const SPOT_TRADE_CHANNEL_ENV: &str = "AERON_SPOT_TRADE_CHANNEL";
const SPOT_TRADE_STREAM_ID_ENV: &str = "AERON_SPOT_TRADE_STREAM_ID";
const SPOT_TRADE_FRAGMENT_LIMIT_ENV: &str = "AERON_SPOT_TRADE_FRAGMENT_LIMIT";

impl AeronListenerConfig {
    pub fn spot_trade_from_env() -> Result<Self, AeronListenerConfigError> {
        Self::from_env_vars(
            AeronListenerEnv {
                channel: SPOT_TRADE_CHANNEL_ENV,
                stream_id: SPOT_TRADE_STREAM_ID_ENV,
                fragment_limit: SPOT_TRADE_FRAGMENT_LIMIT_ENV,
            },
            DEFAULT_SPOT_TRADE_STREAM_ID,
            None,
        )
    }

    pub fn spot_trade_default() -> Self {
        Self::default_for_stream(DEFAULT_SPOT_TRADE_STREAM_ID)
    }
}

pub type SpotTradeAeronError = AeronListenerError<SpotTradeCreatedEventError>;

pub fn run_spot_trade_aeron() -> Result<(), SpotTradeAeronError> {
    let config =
        AeronListenerConfig::spot_trade_from_env().map_err(AeronListenerError::Configuration)?;
    run_aeron_listener(config, SpotTradeCreatedEventHandler::default())
}
