use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{StateSink, StateSource};
use example_core_use_case::{
    MatchSpotOrderV2Cmd, MatchSpotOrderV2State, OpenMatchSpotOrderV2UseCase,
    PlaceMatchSpotOrderV2State, PlaceMatchSpotOrderV2UseCase, PlaceOnlySpotOrderV2Cmd,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultSpotOrderV2PlaceOutboundError {
    #[error("spot order v2 place state is not wired for default HTTP path")]
    StateUnavailable,
}

#[derive(Debug, Default)]
pub struct DefaultSpotOrderV2PlaceOutbound;

impl StateSource<OpenMatchSpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

    fn load_given_state(
        &self,
        _cmd: &MatchSpotOrderV2Cmd,
    ) -> Result<MatchSpotOrderV2State, Self::Error> {
        Err(DefaultSpotOrderV2PlaceOutboundError::StateUnavailable)
    }
}

impl StateSink<OpenMatchSpotOrderV2UseCase> for DefaultSpotOrderV2PlaceOutbound {
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
        _cmd: &PlaceOnlySpotOrderV2Cmd,
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
