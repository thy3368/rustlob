// 发布 SpotUserDataStreamImpl

use spot_behavior::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError};
use spot_behavior::proc::behavior::v2::spot_user_data_sse_behavior::{SpotUserDataSSEBehavior, SpotUserDataStreamCmd, SpotUserDataStreamRes, UserDataStreamEvent};

/// 订单列表中的订单项
#[derive(Debug, Clone)]
pub struct ListOrderItem {
    /// 交易对
    pub symbol: String,
    /// 订单 ID
    pub order_id: i64,
    /// 客户端订单 ID
    pub client_order_id: String,
}

pub struct SpotUserDataSSEImpl{

}

impl SpotUserDataSSEImpl {
    pub(crate) fn new() -> _ {
        todo!()
    }
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