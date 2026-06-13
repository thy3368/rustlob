use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCaseOutbound;
use l1_adapter::{ExecuteAndCommitBlockStatePipeline, MempoolReadingLoadPort};
use l1_core::{
    ExecuteAndCommitBlockCmd, ExecuteAndCommitBlockError, ExecuteAndCommitBlockStateSnapshot,
};

pub struct ExecuteAndCommitBlockOutbound {
    pub load_state: MempoolReadingLoadPort,
    pub pipeline: ExecuteAndCommitBlockStatePipeline,
}

impl CommandUseCaseOutbound for ExecuteAndCommitBlockOutbound {
    type Command = ExecuteAndCommitBlockCmd;
    type State = ExecuteAndCommitBlockStateSnapshot;
    type Error = ExecuteAndCommitBlockError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        self.load_state.load_state(cmd)
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.pipeline.persist(events)
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.pipeline.replay(events)
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.pipeline.publish(events)
    }
}
