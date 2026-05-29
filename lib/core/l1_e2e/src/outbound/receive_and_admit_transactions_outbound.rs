use cmd_handler::{EntityReplayableEvent, use_case_def2::CommandUseCaseOutbound};
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

impl
    CommandUseCaseOutbound<
        ReceiveAndAdmitTransactionsCmd,
        ReceiveAndAdmitTransactionsStateSnapshot,
        ReceiveAndAdmitTransactionsError,
    > for ReceiveAndAdmitTransactionsOutbound
{
    fn load_state(
        &self,
        cmd: &ReceiveAndAdmitTransactionsCmd,
    ) -> Result<ReceiveAndAdmitTransactionsStateSnapshot, ReceiveAndAdmitTransactionsError> {
        self.load_state.load_state(cmd)
    }

    fn persist(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        self.pipeline.persist(events)
    }

    fn replay(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        self.pipeline.replay(events)
    }

    fn publish(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        self.pipeline.publish(events)
    }
}
