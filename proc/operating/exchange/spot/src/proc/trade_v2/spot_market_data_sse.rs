use crate::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError};
use crate::proc::behavior::v2::spot_market_data_sse_behavior::{MarketDataSubscriptionCmdAny, SpotMarketDataStreamAny, SpotMarketDataSSEBehavior, SubscriptionResponse};

pub struct SpotMarketDataSSEImpl {

}

impl SpotMarketDataSSEBehavior for SpotMarketDataSSEImpl {
    fn handle_subscription(&mut self, cmd: MarketDataSubscriptionCmdAny) -> Result<CmdResp<SubscriptionResponse>, SpotCmdError> {
     
        match cmd {
            MarketDataSubscriptionCmdAny::Subscribe { .. } => {}
            MarketDataSubscriptionCmdAny::Unsubscribe { .. } => {}
            MarketDataSubscriptionCmdAny::ListSubscriptions { .. } => {}
            MarketDataSubscriptionCmdAny::SetProperty { .. } => {}
            MarketDataSubscriptionCmdAny::GetProperty { .. } => {}
        }
        todo!()
    }

    fn handle_stream_data(&mut self, data: SpotMarketDataStreamAny) -> Result<(), SpotCmdError> {
        match data {
            SpotMarketDataStreamAny::AggregateTrade(_) => {}
            SpotMarketDataStreamAny::Trade(_) => {}
            SpotMarketDataStreamAny::Kline(_) => {}
            SpotMarketDataStreamAny::KlineWithTimezone(_) => {}
            SpotMarketDataStreamAny::MiniTicker(_) => {}
            SpotMarketDataStreamAny::AllMiniTickers(_) => {}
            SpotMarketDataStreamAny::Ticker(_) => {}
            SpotMarketDataStreamAny::RollingWindowStats(_) => {}
            SpotMarketDataStreamAny::AllRollingWindowStats(_) => {}
            SpotMarketDataStreamAny::BookTicker(_) => {}
            SpotMarketDataStreamAny::AveragePrice(_) => {}
            SpotMarketDataStreamAny::PartialDepth(_) => {}
            SpotMarketDataStreamAny::DiffDepth(_) => {}
        }
        todo!()
    }
}