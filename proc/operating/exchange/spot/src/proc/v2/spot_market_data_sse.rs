use crate::proc::behavior::{
    spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    v2::spot_market_data_sse_behavior::{
        MarketDataSubscriptionCmdAny, SpotMarketDataSubscriptionBehavior, SubscriptionResponse
    }
};

pub struct SpotMarketDataSubscriptionImpl {}

impl SpotMarketDataSubscriptionBehavior for SpotMarketDataSubscriptionImpl {
    fn handle_subscription(
        &mut self, cmd: MarketDataSubscriptionCmdAny
    ) -> Result<CmdResp<SubscriptionResponse>, SpotCmdErrorAny> {
        match cmd {
            MarketDataSubscriptionCmdAny::Subscribe {
                ..
            } => {}
            MarketDataSubscriptionCmdAny::Unsubscribe {
                ..
            } => {}
            MarketDataSubscriptionCmdAny::ListSubscriptions {
                ..
            } => {}
            MarketDataSubscriptionCmdAny::SetProperty {
                ..
            } => {}
            MarketDataSubscriptionCmdAny::GetProperty {
                ..
            } => {}
        }
        todo!()
    }
}
