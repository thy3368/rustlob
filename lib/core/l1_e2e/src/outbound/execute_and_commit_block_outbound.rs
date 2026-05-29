use cmd_handler::{EntityReplayableEvent, use_case_def2::CommandUseCaseOutbound};
use l1_adapter::{ExecuteAndCommitBlockStatePipeline, MempoolReadingLoadPort};
use l1_core::{
    ExecuteAndCommitBlockCmd, ExecuteAndCommitBlockError, ExecuteAndCommitBlockStateSnapshot,
};

pub struct ExecuteAndCommitBlockOutbound {
    pub load_state: MempoolReadingLoadPort,
    pub pipeline: ExecuteAndCommitBlockStatePipeline,
}

impl
    CommandUseCaseOutbound<
        ExecuteAndCommitBlockCmd,
        ExecuteAndCommitBlockStateSnapshot,
        ExecuteAndCommitBlockError,
    > for ExecuteAndCommitBlockOutbound
{
    fn load_state(
        &self,
        cmd: &ExecuteAndCommitBlockCmd,
    ) -> Result<ExecuteAndCommitBlockStateSnapshot, ExecuteAndCommitBlockError> {
        self.load_state.load_state(cmd)
    }

    fn persist(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ExecuteAndCommitBlockError> {
        self.pipeline.persist(events)
    }

    fn replay(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ExecuteAndCommitBlockError> {
        self.pipeline.replay(events)
    }

    fn publish(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ExecuteAndCommitBlockError> {
        self.pipeline.publish(events)
    }
}
