use base_types::handler::handler::Handler;

use crate::proc::behavior::{
    spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    v2::spot_user_data_sse_behavior::{SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny}
};

pub struct SpotUserDataStreamImpl {}


impl Handler<SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny, SpotCmdErrorAny> for SpotUserDataStreamImpl {
    async fn handle(
        &self, cmd: SpotUserDataListenKeyCmdAny
    ) -> Result<CmdResp<SpotUserDataListenKeyResAny>, SpotCmdErrorAny> {
        match cmd {
            SpotUserDataListenKeyCmdAny::CreateListenKey(_) => {}
            SpotUserDataListenKeyCmdAny::KeepAliveListenKey(_) => {}
            SpotUserDataListenKeyCmdAny::CloseListenKey(_) => {}
        }
        todo!()
    }
}
