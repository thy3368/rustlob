use spot_behavior::proc::behavior::{
    spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    v2::spot_market_data_sse_behavior::{
        MarketDataSubscriptionCmdAny, SpotMarketDataSubscriptionBehavior, SubscriptionResponse
    }
};

/// Spot 市场数据 SSE 实现
#[derive(Clone)]
pub struct SpotMarketDataSSEImpl {
    /// 已订阅的流列表
    subscriptions: Vec<String>,
    /// 是否启用复合流属性
    combined_stream_enabled: bool
}

impl SpotMarketDataSSEImpl {
    /// 创建新的 SpotMarketDataSSEImpl 实例
    pub fn new() -> Self {
        Self {
            subscriptions: Vec::new(),
            combined_stream_enabled: false
        }
    }
}

impl SpotMarketDataSubscriptionBehavior for SpotMarketDataSSEImpl {
    fn handle_subscription(
        &mut self, cmd: MarketDataSubscriptionCmdAny
    ) -> Result<CmdResp<SubscriptionResponse>, SpotCmdErrorAny> {
        let nonce = 0; // 临时值，实际应该从 metadata 中获取或生成

        match cmd {
            MarketDataSubscriptionCmdAny::Subscribe {
                ..
            } => {
                todo!()
            }
            MarketDataSubscriptionCmdAny::Unsubscribe {
                ..
            } => {
                todo!()
            }
            MarketDataSubscriptionCmdAny::ListSubscriptions {
                ..
            } => {
                todo!()
            }
            MarketDataSubscriptionCmdAny::SetProperty {
                ..
            } => {
                todo!()
            }
            MarketDataSubscriptionCmdAny::GetProperty {
                ..
            } => {
                todo!()
            }
        }
    }
}
