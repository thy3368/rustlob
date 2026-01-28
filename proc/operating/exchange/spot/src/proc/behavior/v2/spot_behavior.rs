use base_types::cqrs::cqrs_types::CmdResp;

use crate::proc::behavior::{
    spot_trade_behavior::SpotCmdErrorAny,
    v2::{
        spot_market_data_behavior::{SpotMarketDataCmdAny, SpotMarketDataResAny},
        spot_trade_behavior_v2::{SpotTradeCmdAny, SpotTradeResAny},
        spot_user_data_behavior::{SpotUserDataCmdAny, SpotUserDataResAny},
        spot_user_data_sse_behavior::{SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny}
    }
};

pub enum SpotCmdAny {
    SpotTradeCmdAny(SpotTradeCmdAny),
    SpotUserDataCmdAny(SpotUserDataCmdAny),
    SpotMarketDataCmdAny(SpotMarketDataCmdAny),
    SpotUserDataListenKeyCmdAny(SpotUserDataListenKeyCmdAny)
}

pub enum SpotResAny {
    SpotTradeResAny(SpotTradeResAny),
    SpotUserDataResAny(SpotUserDataResAny),
    SpotMarketDataResAny(SpotMarketDataResAny),
    SpotUserDataListenKeyResAny(SpotUserDataListenKeyResAny)
}


/// 命令处理行为的泛型接口
pub trait CommandHandler<C, R>: Send + Sync {
    async fn handle(&self, cmd: C) -> Result<CmdResp<R>, SpotCmdErrorAny>;
}

//todo 可以表态分发？
type SpotBehavior2 = dyn CommandHandler<SpotCmdAny, SpotResAny>;

/// Spot Trading 行为接口
pub trait SpotBehavior: Send + Sync {
    /// 处理 Spot Trading 命令
    async fn handle(&self, cmd: SpotCmdAny) -> Result<CmdResp<SpotResAny>, SpotCmdErrorAny>;
}
