use std::sync::{Arc, Mutex};

use cmd_handler::use_case_def::DomainEventPipeline;
use l1_core::{ExecuteAndCommitBlockError, ExecuteAndCommitBlockEvents};

use crate::MdbxStateStore;

pub struct ExecuteAndCommitBlockStatePipeline {
    pub state_store: Arc<Mutex<MdbxStateStore>>,
}

impl DomainEventPipeline<ExecuteAndCommitBlockEvents, ExecuteAndCommitBlockError>
    for ExecuteAndCommitBlockStatePipeline
{
    fn persist(
        &self,
        events: &ExecuteAndCommitBlockEvents,
    ) -> Result<(), ExecuteAndCommitBlockError> {
        let mut state_store = self
            .state_store
            .lock()
            .map_err(|err| ExecuteAndCommitBlockError::LoadStateFailed(err.to_string()))?;
        state_store
            .apply_block_state_changes(events.committed_block.block_height, &events.state_changes)
            .map_err(|err| ExecuteAndCommitBlockError::LoadStateFailed(err.to_string()))
    }

    fn replay(
        &self,
        _events: &ExecuteAndCommitBlockEvents,
    ) -> Result<(), ExecuteAndCommitBlockError> {
        Ok(())
    }

    fn publish(
        &self,
        _events: &ExecuteAndCommitBlockEvents,
    ) -> Result<(), ExecuteAndCommitBlockError> {
        Ok(())
    }
}
