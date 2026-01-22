use crate::proc::behavior::spot_trade_behavior::SpotCmdError;
use crate::proc::behavior::v2::spot_market_data_behavior::{
    SpotMarketDataBehavior, SpotMarketDataCmdAny, SpotMarketDataRes,
};
use base_types::cqrs::cqrs_types::CmdResp;

pub struct SpotMarketDataImpl {}

impl SpotMarketDataBehavior for SpotMarketDataImpl {
    fn handle(&mut self, cmd: SpotMarketDataCmdAny) -> Result<CmdResp<SpotMarketDataRes>, SpotCmdError> {
        match cmd {
            SpotMarketDataCmdAny::OrderBook(_) => {}
            SpotMarketDataCmdAny::RecentTrades(_) => {}
            SpotMarketDataCmdAny::HistoricalTrades(_) => {}
            SpotMarketDataCmdAny::AggTrades(_) => {}
            SpotMarketDataCmdAny::Klines(_) => {}
            SpotMarketDataCmdAny::UIKlines(_) => {}
            SpotMarketDataCmdAny::AvgPrice(_) => {}
            SpotMarketDataCmdAny::Ticker24hr(_) => {}
            SpotMarketDataCmdAny::TradingDayTicker(_) => {}
            SpotMarketDataCmdAny::SymbolPriceTicker(_) => {}
            SpotMarketDataCmdAny::BookTicker(_) => {}
            SpotMarketDataCmdAny::RollingWindowTicker(_) => {}
        }
        todo!()
    }
}
