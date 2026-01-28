// 发布 SpotUserDataStreamImpl

use spot_behavior::proc::behavior::{
    spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    v2::spot_user_data_sse_behavior::{
        ListenKeyResponse, SpotUserDataListenKeyBehavior, SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny
    }
};

/// 订单列表中的订单项
#[derive(Debug, Clone)]
pub struct ListOrderItem {
    /// 交易对
    pub symbol: String,
    /// 订单 ID
    pub order_id: i64,
    /// 客户端订单 ID
    pub client_order_id: String
}

pub struct SpotUserDataSSEImpl {}

impl SpotUserDataSSEImpl {
    pub(crate) fn new() -> Self { Self {} }
}

impl SpotUserDataSSEImpl {}

impl SpotUserDataListenKeyBehavior for SpotUserDataSSEImpl {
    async fn handle(
        &mut self, cmd: SpotUserDataListenKeyCmdAny
    ) -> Result<CmdResp<SpotUserDataListenKeyResAny>, SpotCmdErrorAny> {
        let nonce = 0; // 临时值，实际应该从 metadata 中获取或生成

        match cmd {
            SpotUserDataListenKeyCmdAny::CreateListenKey(_) => Ok(CmdResp::new(
                nonce,
                SpotUserDataListenKeyResAny::CreateListenKey(ListenKeyResponse {
                    listen_key: "test_listen_key_123".to_string()
                })
            )),
            SpotUserDataListenKeyCmdAny::KeepAliveListenKey(_) => {
                Ok(CmdResp::new(nonce, SpotUserDataListenKeyResAny::KeepAliveListenKey))
            }
            SpotUserDataListenKeyCmdAny::CloseListenKey(_) => {
                Ok(CmdResp::new(nonce, SpotUserDataListenKeyResAny::CloseListenKey))
            }
        }
    }
}
