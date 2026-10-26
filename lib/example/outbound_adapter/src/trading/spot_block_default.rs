use entity::EntityReplayableEvent;
use entity::{StateSink, StateSource};
use example_core_use_case::{SpotBlockCmd, SpotBlockState, SpotBlockUseCase};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultSpotBlockOutboundError {
    #[error("spot block state is not wired for default HTTP path")]
    StateUnavailable,
}

#[derive(Debug, Default)]
pub struct DefaultSpotBlockOutbound;

impl StateSource<SpotBlockUseCase> for DefaultSpotBlockOutbound {
    type Error = DefaultSpotBlockOutboundError;

    fn load_given_state(
        &self,
        _cmd: &LogEntry<SpotBlockCmd>,
    ) -> Result<SpotBlockState, Self::Error> {
        Err(DefaultSpotBlockOutboundError::StateUnavailable)
    }
}

impl StateSink<SpotBlockUseCase> for DefaultSpotBlockOutbound {
    type Error = DefaultSpotBlockOutboundError;

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
