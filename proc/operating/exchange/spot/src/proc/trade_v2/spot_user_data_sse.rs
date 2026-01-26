use crate::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError};
use crate::proc::behavior::v2::spot_user_data_sse_behavior::{
    SpotUserDataSSEBehavior, SpotUserDataStreamCmd, SpotUserDataStreamRes, UserDataStreamEvent,
};

pub struct SpotUserDataStreamImpl {}

impl SpotUserDataSSEBehavior for SpotUserDataStreamImpl {
    fn handle(&mut self, cmd: SpotUserDataStreamCmd) -> Result<CmdResp<SpotUserDataStreamRes>, SpotCmdError> {
        match cmd {
            SpotUserDataStreamCmd::CreateListenKey(_) => {}
            SpotUserDataStreamCmd::KeepAliveListenKey(_) => {}
            SpotUserDataStreamCmd::CloseListenKey(_) => {}
        }
        todo!()
    }


}
