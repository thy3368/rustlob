use std::fmt::Debug;

use common_entity::{StateSink, StateSource};
use example_core_use_case::{ActivateMatchSpotOrderV2UseCase, PendingTriggerSpotOrderSource};
use use_case_executor::trading::spot::on_event::{
    SpotTradeCreatedEventError, SpotTradeCreatedEventHandler, TriggerPendingSpotOrdersOnTradeError,
    TriggerPendingSpotOrdersOnTradeHandler,
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

pub type TriggerPendingSpotOrdersAeronError<OutboundError> =
    AeronListenerError<TriggerPendingSpotOrdersOnTradeError<OutboundError>>;

pub fn run_trigger_pending_spot_orders_aeron<OB>(
    outbound: OB,
) -> Result<
    (),
    TriggerPendingSpotOrdersAeronError<<OB as StateSink<ActivateMatchSpotOrderV2UseCase>>::Error>,
>
where
    OB: PendingTriggerSpotOrderSource<
            Error = <OB as StateSink<ActivateMatchSpotOrderV2UseCase>>::Error,
        > + StateSource<
            ActivateMatchSpotOrderV2UseCase,
            Error = <OB as StateSink<ActivateMatchSpotOrderV2UseCase>>::Error,
        > + StateSink<ActivateMatchSpotOrderV2UseCase>,
    <OB as StateSink<ActivateMatchSpotOrderV2UseCase>>::Error: Debug,
{
    let config =
        AeronListenerConfig::spot_trade_from_env().map_err(AeronListenerError::Configuration)?;
    run_aeron_listener(config, TriggerPendingSpotOrdersOnTradeHandler::new(outbound))
}
