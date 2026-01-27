use base_types::cqrs::cqrs_types::CmdResp;
use spot_behavior::proc::behavior::spot_trade_behavior::SpotCmdError;
use spot_behavior::proc::behavior::v2::spot_user_data_sse_behavior::{SpotUserDataListenKeyBehavior, SpotUserDataListenKeyCmdAny, SpotUserDataStreamResAny};

pub struct abc{}

impl SpotUserDataListenKeyBehavior for abc{
    fn handle(&mut self, cmd: SpotUserDataListenKeyCmdAny) -> Result<CmdResp<SpotUserDataStreamResAny>, SpotCmdError> {
        todo!()
    }
}