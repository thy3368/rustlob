use crate::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_user_data_sse_behavior::{
    SpotUserDataListenKeyBehavior, SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny, UserDataStreamEventAny,
};

pub struct SpotUserDataStreamImpl {}

impl SpotUserDataListenKeyBehavior for SpotUserDataStreamImpl {
    async fn handle(&mut self, cmd: SpotUserDataListenKeyCmdAny) -> Result<CmdResp<SpotUserDataListenKeyResAny>, SpotCmdErrorAny> {
        match cmd {
            SpotUserDataListenKeyCmdAny::CreateListenKey(_) => {}
            SpotUserDataListenKeyCmdAny::KeepAliveListenKey(_) => {}
            SpotUserDataListenKeyCmdAny::CloseListenKey(_) => {}
        }
        todo!()
    }
}
