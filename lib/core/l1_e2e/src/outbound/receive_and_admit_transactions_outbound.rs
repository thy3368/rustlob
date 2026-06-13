use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCaseOutbound;
use l1_adapter::MempoolWritingPipeline;
use l1_core::{
    ReceiveAndAdmitTransactionsCmd, ReceiveAndAdmitTransactionsError,
    ReceiveAndAdmitTransactionsStateSnapshot,
};

use crate::ingress_load_port::IngressLoadPort;

pub struct ReceiveAndAdmitTransactionsOutbound {
    pub load_state: IngressLoadPort,
    pub pipeline: MempoolWritingPipeline<()>,
}

impl CommandUseCaseOutbound for ReceiveAndAdmitTransactionsOutbound {
    type Command = ReceiveAndAdmitTransactionsCmd;
    type State = ReceiveAndAdmitTransactionsStateSnapshot;
    type Error = ReceiveAndAdmitTransactionsError;

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
