use common_entity::EntityReplayableEvent;
use common_entity::{StateSink, StateSource};
use example_core_use_case::{
    ActivateMatchSpotOrderV2Cmd, ActivateMatchSpotOrderV2State, ActivateMatchSpotOrderV2UseCase,
    ActivateSpotOrderV2Cmd, ActivateSpotOrderV2State, ActivateSpotOrderV2UseCase,
    MatchSpotOrderV3Cmd, MatchSpotOrderV3State, MatchSpotOrderV3UseCase,
    PlaceMatchSpotOrderV2State, PlaceMatchSpotOrderV2UseCase, PlaceOnlySpotOrderV2Cmd,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultSpotOrderV2PlaceOutboundError {
    #[error("spot order v2 place state is not wired for default HTTP path")]
    StateUnavailable,
}

#[derive(Debug, Default)]
pub struct DefaultSpotOrderV2PlaceOutbound;

impl StateSource<MatchSpotOrderV3UseCase> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

    fn load_given_state(
        &self,
        _cmd: &LogEntry<MatchSpotOrderV3Cmd>,
    ) -> Result<MatchSpotOrderV3State, Self::Error> {
        Err(DefaultSpotOrderV2PlaceOutboundError::StateUnavailable)
    }
}

impl StateSink<MatchSpotOrderV3UseCase> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

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

impl StateSource<ActivateSpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

    fn load_given_state(
        &self,
        _cmd: &LogEntry<ActivateSpotOrderV2Cmd>,
    ) -> Result<ActivateSpotOrderV2State, Self::Error> {
        Err(DefaultSpotOrderV2PlaceOutboundError::StateUnavailable)
    }
}

impl StateSink<ActivateSpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

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

impl StateSource<ActivateMatchSpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

    fn load_given_state(
        &self,
        _cmd: &LogEntry<ActivateMatchSpotOrderV2Cmd>,
    ) -> Result<ActivateMatchSpotOrderV2State, Self::Error> {
        Err(DefaultSpotOrderV2PlaceOutboundError::StateUnavailable)
    }
}

impl StateSink<ActivateMatchSpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

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

impl StateSource<PlaceMatchSpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

    fn load_given_state(
        &self,
        _cmd: &LogEntry<PlaceOnlySpotOrderV2Cmd>,
    ) -> Result<PlaceMatchSpotOrderV2State, Self::Error> {
        Err(DefaultSpotOrderV2PlaceOutboundError::StateUnavailable)
    }
}

impl StateSink<PlaceMatchSpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

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
