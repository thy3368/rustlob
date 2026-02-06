use base_types::Timestamp;
use base_types::cqrs::cqrs_types::{CmdResp, ResMetadata};
use base_types::handler::handler::Handler;
use immutable_derive::immutable;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_market_data_behavior::{
    AvgPriceData, OrderBookData, SpotMarketDataCmdAny, SpotMarketDataResAny,
};

#[immutable]
pub struct SpotMarketDataImpl {}

impl Handler<SpotMarketDataCmdAny, SpotMarketDataResAny, SpotCmdErrorAny> for SpotMarketDataImpl {
    async fn handle(
        &self,
        cmd: SpotMarketDataCmdAny,
    ) -> Result<CmdResp<SpotMarketDataResAny>, SpotCmdErrorAny> {
        // 使用固定的 nonce 值，实际应用中应该从命令元数据中获取
        let nonce = 0;

        //todo ResMetadata::new received_at有错
        match cmd {
            SpotMarketDataCmdAny::OrderBook(_) => {
                // 暂时返回一个空的订单簿响应
                Ok(CmdResp::new(
                    ResMetadata::new(nonce, false, Timestamp(0)),
                    SpotMarketDataResAny::OrderBook(OrderBookData::new(0, vec![], vec![])),
                ))
            }
            SpotMarketDataCmdAny::RecentTrades(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp(0)),
                SpotMarketDataResAny::Trades(vec![]),
            )),
            SpotMarketDataCmdAny::HistoricalTrades(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp(0)),
                SpotMarketDataResAny::Trades(vec![]),
            )),
            SpotMarketDataCmdAny::AggTrades(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp(0)),
                SpotMarketDataResAny::AggTrades(vec![]),
            )),
            SpotMarketDataCmdAny::Klines(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp(0)),
                SpotMarketDataResAny::Klines(vec![]),
            )),
            SpotMarketDataCmdAny::UIKlines(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp(0)),
                SpotMarketDataResAny::Klines(vec![]),
            )),
            SpotMarketDataCmdAny::AvgPrice(_) => {
                // 暂时返回一个默认的平均价格响应
                Ok(CmdResp::new(
                    ResMetadata::new(nonce, false, Timestamp(0)),
                    SpotMarketDataResAny::AvgPrice(AvgPriceData::new(5, "50000.00".to_string(), 0)),
                ))
            }
            SpotMarketDataCmdAny::Ticker24hr(_) => {
                // 暂时返回一个空的24小时Ticker响应列表
                Ok(CmdResp::new(
                    ResMetadata::new(nonce, false, Timestamp(0)),
                    SpotMarketDataResAny::Ticker24hrList(vec![]),
                ))
            }
            SpotMarketDataCmdAny::TradingDayTicker(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp(0)),
                SpotMarketDataResAny::TradingDayTickerList(vec![]),
            )),
            SpotMarketDataCmdAny::SymbolPriceTicker(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp(0)),
                SpotMarketDataResAny::PriceTickerList(vec![]),
            )),
            SpotMarketDataCmdAny::BookTicker(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp(0)),
                SpotMarketDataResAny::BookTickerList(vec![]),
            )),
            SpotMarketDataCmdAny::RollingWindowTicker(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp(0)),
                SpotMarketDataResAny::RollingWindowTickerList(vec![]),
            )),
        }
    }
}
