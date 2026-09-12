use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{StateSink, StateSource};
use example_core_use_case::{
    CancelSpotOrderV2Cmd, CancelSpotOrderV2State, CancelSpotOrderV2UseCase,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultSpotOrderV2CancelOutboundError {
    #[error("spot order v2 cancel state is not wired for default HTTP path")]
    StateUnavailable,
}

#[derive(Debug, Default)]
pub struct DefaultSpotOrderV2CancelOutbound;

impl StateSource<CancelSpotOrderV2UseCase> for DefaultSpotOrderV2CancelOutbound {
    type Error = DefaultSpotOrderV2CancelOutboundError;

    fn load_given_state(
        &self,
        _cmd: &CancelSpotOrderV2Cmd,
    ) -> Result<CancelSpotOrderV2State, Self::Error> {
        // order 只有lob里面取，如果取不到，那说明状态不正确，对吧？
        //     /// 订单已进入执行流程，尚未成交。
        //     Open,
        //     /// 订单已部分成交，剩余数量仍在业务上可撤。
        //     PartiallyFilled,

        //或都只有这两种状态才能取消，否则就报错

        Err(DefaultSpotOrderV2CancelOutboundError::StateUnavailable)
    }
}

impl StateSink<CancelSpotOrderV2UseCase> for DefaultSpotOrderV2CancelOutbound {
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
