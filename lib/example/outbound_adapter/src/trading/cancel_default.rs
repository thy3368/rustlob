use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{MiFamilyOutbound, MiFamilyStateSource};
use example_core_use_case::{
    SpotOrderV2CommandV3, SpotOrderV2GivenStateV3, SpotOrderV2UseCaseFamilyV3,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultSpotOrderV2CancelOutboundError {
    #[error("spot order v2 cancel state is not wired for default HTTP path")]
    StateUnavailable,
}

#[derive(Debug, Default)]
pub struct DefaultSpotOrderV2CancelOutbound;

impl MiFamilyStateSource<SpotOrderV2UseCaseFamilyV3> for DefaultSpotOrderV2CancelOutbound {
    type Error = DefaultSpotOrderV2CancelOutboundError;

    fn load_given_state(
        &self,
        _cmd: &SpotOrderV2CommandV3,
    ) -> Result<SpotOrderV2GivenStateV3, Self::Error> {
        Err(DefaultSpotOrderV2CancelOutboundError::StateUnavailable)
    }
}

impl MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3> for DefaultSpotOrderV2CancelOutbound {
    type Error = DefaultSpotOrderV2CancelOutboundError;

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
