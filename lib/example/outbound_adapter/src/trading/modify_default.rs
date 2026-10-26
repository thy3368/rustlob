use common_entity::{EntityReplayableEvent, StateSink, StateSource};
use example_core_use_case::{
    ModifySpotOrderV2Cmd, ModifySpotOrderV2State, ModifySpotOrderV2UseCase,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultSpotOrderV2ModifyOutboundError {
    #[error("spot order v2 modify state is not wired for default HTTP path")]
    StateUnavailable,
}

#[derive(Debug, Default)]
pub struct DefaultSpotOrderV2ModifyOutbound;

impl StateSource<ModifySpotOrderV2UseCase> for DefaultSpotOrderV2ModifyOutbound {
    type Error = DefaultSpotOrderV2ModifyOutboundError;

    fn load_given_state(
        &self,
        _cmd: &LogEntry<ModifySpotOrderV2Cmd>,
    ) -> Result<ModifySpotOrderV2State, Self::Error> {
        Err(DefaultSpotOrderV2ModifyOutboundError::StateUnavailable)
    }
}

impl StateSink<ModifySpotOrderV2UseCase> for DefaultSpotOrderV2ModifyOutbound {
    type Error = DefaultSpotOrderV2ModifyOutboundError;

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
