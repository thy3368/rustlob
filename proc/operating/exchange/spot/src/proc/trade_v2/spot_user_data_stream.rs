use crate::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError};
use crate::proc::behavior::v2::spot_user_data_stream_behavior::{
    SpotUserDataStreamBehavior, SpotUserDataStreamCmd, SpotUserDataStreamRes, UserDataStreamEvent,
};

pub struct SpotUserDataStreamImpl {}

impl SpotUserDataStreamBehavior for SpotUserDataStreamImpl {
    fn handle(&mut self, cmd: SpotUserDataStreamCmd) -> Result<CmdResp<SpotUserDataStreamRes>, SpotCmdError> {
        match cmd {
            SpotUserDataStreamCmd::CreateListenKey(_) => {}
            SpotUserDataStreamCmd::KeepAliveListenKey(_) => {}
            SpotUserDataStreamCmd::CloseListenKey(_) => {}
        }
        todo!()
    }

    fn on_event(&mut self, event: UserDataStreamEvent) -> Result<(), SpotCmdError> {
        match event {
            UserDataStreamEvent::OutboundAccountPosition(_) => {}
            UserDataStreamEvent::BalanceUpdate(_) => {}
            UserDataStreamEvent::ExecutionReport(_) => {}
            UserDataStreamEvent::ListStatus(_) => {}
            UserDataStreamEvent::EventStreamTerminated(_) => {}
            UserDataStreamEvent::ExternalLockUpdate(_) => {}
        }
        todo!()
    }
}
