use crate::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError};
use crate::proc::behavior::v2::spot_market_data_sse_behavior::{MarketDataSubscriptionCmd, SpotMarketDataStream, SpotMarketDataSSEBehavior, SubscriptionResponse};

pub struct SpotMarketDataSSEImpl {

}

impl SpotMarketDataSSEBehavior for SpotMarketDataSSEImpl {
    fn handle_subscription(&mut self, cmd: MarketDataSubscriptionCmd) -> Result<CmdResp<SubscriptionResponse>, SpotCmdError> {
     
        match cmd {
            MarketDataSubscriptionCmd::Subscribe { .. } => {}
            MarketDataSubscriptionCmd::Unsubscribe { .. } => {}
            MarketDataSubscriptionCmd::ListSubscriptions { .. } => {}
            MarketDataSubscriptionCmd::SetProperty { .. } => {}
            MarketDataSubscriptionCmd::GetProperty { .. } => {}
        }
        todo!()
    }

    fn handle_stream_data(&mut self, data: SpotMarketDataStream) -> Result<(), SpotCmdError> {
        match data {
            SpotMarketDataStream::AggregateTrade(_) => {}
            SpotMarketDataStream::Trade(_) => {}
            SpotMarketDataStream::Kline(_) => {}
            SpotMarketDataStream::KlineWithTimezone(_) => {}
            SpotMarketDataStream::MiniTicker(_) => {}
            SpotMarketDataStream::AllMiniTickers(_) => {}
            SpotMarketDataStream::Ticker(_) => {}
            SpotMarketDataStream::RollingWindowStats(_) => {}
            SpotMarketDataStream::AllRollingWindowStats(_) => {}
            SpotMarketDataStream::BookTicker(_) => {}
            SpotMarketDataStream::AveragePrice(_) => {}
            SpotMarketDataStream::PartialDepth(_) => {}
            SpotMarketDataStream::DiffDepth(_) => {}
        }
        todo!()
    }
}