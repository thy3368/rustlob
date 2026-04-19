use base_types::handler::handler::Handler;
use immutable_derive::immutable;

use crate::proc::behavior::v2::spot_trade_error::{CmdResp, SpotApiErrorAny};
use crate::proc::behavior::v2::spot_user_data_sse_behavior::{
    SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny,
};

#[immutable]

pub struct SpotUserDataListenKeyImpl {}

impl Handler<SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny, SpotApiErrorAny>
    for SpotUserDataListenKeyImpl
{
    async fn handle(
        &self,
        cmd: SpotUserDataListenKeyCmdAny,
    ) -> Result<CmdResp<SpotUserDataListenKeyResAny>, SpotApiErrorAny> {
        match cmd {
            SpotUserDataListenKeyCmdAny::CreateListenKey(_) => {}
            SpotUserDataListenKeyCmdAny::KeepAliveListenKey(_) => {}
            SpotUserDataListenKeyCmdAny::CloseListenKey(_) => {}
        }
        todo!()
    }
}
