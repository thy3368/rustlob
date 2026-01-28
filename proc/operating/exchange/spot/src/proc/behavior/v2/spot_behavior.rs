use base_types::handler::handler::Handler;

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


/// Spot Trading 行为接口
pub trait SpotBehavior: Send + Sync + Handler<SpotCmdAny, SpotResAny, SpotCmdErrorAny> {}
