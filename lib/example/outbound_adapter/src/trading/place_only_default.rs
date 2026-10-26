use entity::EntityReplayableEvent;
use entity::{StateSink, StateSource};
use example_core_use_case::{
    PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2State, PlaceOnlySpotOrderV2UseCase,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("no-op spot order v2 place-only outbound failed")]
pub struct NoopSpotOrderV2PlaceOnlyOutboundError;

#[derive(Debug, Default, Clone, Copy)]
pub struct NoopSpotOrderV2PlaceOnlyOutbound;

impl StateSource<PlaceOnlySpotOrderV2UseCase> for NoopSpotOrderV2PlaceOnlyOutbound {
    type Error = NoopSpotOrderV2PlaceOnlyOutboundError;

    fn load_given_state(
        &self,
        _cmd: &LogEntry<PlaceOnlySpotOrderV2Cmd>,
    ) -> Result<PlaceOnlySpotOrderV2State, Self::Error> {
        Ok(())
    }
}

impl StateSink<PlaceOnlySpotOrderV2UseCase> for NoopSpotOrderV2PlaceOnlyOutbound {
    type Error = NoopSpotOrderV2PlaceOnlyOutboundError;

    fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultSpotOrderV2PlaceOnlyOutboundError {
    #[error("spot order v2 place-only outbound failed")]
    Unavailable,
}

#[derive(Debug, Default)]
pub struct DefaultSpotOrderV2PlaceOnlyOutbound;

impl StateSource<PlaceOnlySpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOnlyOutbound {
    type Error = DefaultSpotOrderV2PlaceOnlyOutboundError;

    fn load_given_state(
        &self,
        _cmd: &LogEntry<PlaceOnlySpotOrderV2Cmd>,
    ) -> Result<PlaceOnlySpotOrderV2State, Self::Error> {
        Ok(())
    }
}

impl StateSink<PlaceOnlySpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOnlyOutbound {
    type Error = DefaultSpotOrderV2PlaceOnlyOutboundError;

    fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }
}
use common_entity::LogEntry;
