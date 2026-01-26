// 发布 SpotUserDataStreamImpl

use spot_behavior::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError};
use spot_behavior::proc::behavior::v2::spot_user_data_sse_behavior::{SpotUserDataSSEBehavior, SpotUserDataStreamCmd, SpotUserDataStreamRes, UserDataStreamEvent};

pub struct SpotUserDataSSEImpl{

}
impl SpotUserDataSSEImpl {

}

impl SpotUserDataSSEBehavior for SpotUserDataSSEImpl {
    fn handle(&mut self, cmd: SpotUserDataStreamCmd) -> Result<CmdResp<SpotUserDataStreamRes>, SpotCmdError> {
        
        match cmd {
            SpotUserDataStreamCmd::CreateListenKey(_) => {}
            SpotUserDataStreamCmd::KeepAliveListenKey(_) => {}
            SpotUserDataStreamCmd::CloseListenKey(_) => {}
        }
        todo!()
    }


}