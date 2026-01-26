use base_types::cqrs::cqrs_types::CmdResp;
use crate::proc::usds_m_future::behavior::market_data_behavior::{UsdsMFutureMarketDataBehavior, UsdsMFutureMarketDataCmdAny, UsdsMFutureMarketDataCmdError, UsdsMFutureMarketDataRes};

pub struct UsdsMFutureMarketDataBehaviorImpl{}
impl UsdsMFutureMarketDataBehavior for UsdsMFutureMarketDataBehaviorImpl{
    fn handle(&mut self, cmd: UsdsMFutureMarketDataCmdAny) -> Result<CmdResp<UsdsMFutureMarketDataRes>, UsdsMFutureMarketDataCmdError> {
       
        match cmd {
            UsdsMFutureMarketDataCmdAny::TestConnectivity(_) => {}
            UsdsMFutureMarketDataCmdAny::CheckServerTime(_) => {}
            UsdsMFutureMarketDataCmdAny::ExchangeInformation(_) => {}
            UsdsMFutureMarketDataCmdAny::OrderBook(_) => {}
            UsdsMFutureMarketDataCmdAny::RecentTradesList(_) => {}
            UsdsMFutureMarketDataCmdAny::OldTradesLookup(_) => {}
            UsdsMFutureMarketDataCmdAny::CompressedAggregateTradesList(_) => {}
            UsdsMFutureMarketDataCmdAny::KlineCandlestickData(_) => {}
            UsdsMFutureMarketDataCmdAny::ContinuousContractKlineCandlestickData(_) => {}
            UsdsMFutureMarketDataCmdAny::IndexPriceKlineCandlestickData(_) => {}
            UsdsMFutureMarketDataCmdAny::MarkPriceKlineCandlestickData(_) => {}
            UsdsMFutureMarketDataCmdAny::PremiumIndexKlineData(_) => {}
            UsdsMFutureMarketDataCmdAny::MarkPrice(_) => {}
            UsdsMFutureMarketDataCmdAny::GetFundingRateHistory(_) => {}
            UsdsMFutureMarketDataCmdAny::GetFundingRateInfo(_) => {}
            UsdsMFutureMarketDataCmdAny::Ticker24hr(_) => {}
            UsdsMFutureMarketDataCmdAny::SymbolPriceTicker(_) => {}
            UsdsMFutureMarketDataCmdAny::SymbolPriceTickerV2(_) => {}
            UsdsMFutureMarketDataCmdAny::SymbolOrderBookTicker(_) => {}
            UsdsMFutureMarketDataCmdAny::OpenInterest(_) => {}
            UsdsMFutureMarketDataCmdAny::OpenInterestStatistics(_) => {}
            UsdsMFutureMarketDataCmdAny::TopLongShortAccountRatio(_) => {}
            UsdsMFutureMarketDataCmdAny::TopTraderLongShortRatio(_) => {}
            UsdsMFutureMarketDataCmdAny::LongShortRatio(_) => {}
            UsdsMFutureMarketDataCmdAny::TakerBuySellVolume(_) => {}
            UsdsMFutureMarketDataCmdAny::HistoricalBLVTNAVKlineCandlestick(_) => {}
            UsdsMFutureMarketDataCmdAny::CompositeIndexSymbolInformation(_) => {}
            UsdsMFutureMarketDataCmdAny::IndexConstituents(_) => {}
            UsdsMFutureMarketDataCmdAny::MultiAssetsModeAssetIndex(_) => {}
            UsdsMFutureMarketDataCmdAny::DeliveryPrice(_) => {}
        }
        todo!()
    }
}