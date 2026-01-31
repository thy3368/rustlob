use base_types::{cqrs::cqrs_types::CmdResp, handler::handler::Handler};
use immutable_derive::immutable;
use crate::proc::behavior::{
    spot_trade_behavior::SpotCmdErrorAny,
    v2::spot_market_data_behavior::{
        AvgPriceData, OrderBookData, SpotMarketDataCmdAny, SpotMarketDataResAny
    }
};


#[immutable]
pub struct SpotMarketDataImpl {}

impl SpotMarketDataImpl {
    pub fn new() -> Self { Self {} }
}


impl Handler<SpotMarketDataCmdAny, SpotMarketDataResAny, SpotCmdErrorAny> for SpotMarketDataImpl {
    async fn handle(&self, cmd: SpotMarketDataCmdAny) -> Result<CmdResp<SpotMarketDataResAny>, SpotCmdErrorAny> {
        // 使用固定的 nonce 值，实际应用中应该从命令元数据中获取
        let nonce = 0;

        match cmd {
            SpotMarketDataCmdAny::OrderBook(_) => {
                // 暂时返回一个空的订单簿响应
                Ok(CmdResp::new(
                    nonce,
                    SpotMarketDataResAny::OrderBook(OrderBookData {
                        last_update_id: 0,
                        bids: vec![],
                        asks: vec![]
                    })
                ))
            }
            SpotMarketDataCmdAny::RecentTrades(_) => Ok(CmdResp::new(nonce, SpotMarketDataResAny::Trades(vec![]))),
            SpotMarketDataCmdAny::HistoricalTrades(_) => Ok(CmdResp::new(nonce, SpotMarketDataResAny::Trades(vec![]))),
            SpotMarketDataCmdAny::AggTrades(_) => Ok(CmdResp::new(nonce, SpotMarketDataResAny::AggTrades(vec![]))),
            SpotMarketDataCmdAny::Klines(_) => Ok(CmdResp::new(nonce, SpotMarketDataResAny::Klines(vec![]))),
            SpotMarketDataCmdAny::UIKlines(_) => Ok(CmdResp::new(nonce, SpotMarketDataResAny::Klines(vec![]))),
            SpotMarketDataCmdAny::AvgPrice(_) => {
                // 暂时返回一个默认的平均价格响应
                Ok(CmdResp::new(
                    nonce,
                    SpotMarketDataResAny::AvgPrice(AvgPriceData {
                        mins: 5,
                        price: "50000.00".to_string(),
                        close_time: 0
                    })
                ))
            }
            SpotMarketDataCmdAny::Ticker24hr(_) => {
                // 暂时返回一个空的24小时Ticker响应列表
                Ok(CmdResp::new(nonce, SpotMarketDataResAny::Ticker24hrList(vec![])))
            }
            SpotMarketDataCmdAny::TradingDayTicker(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataResAny::TradingDayTickerList(vec![])))
            }
            SpotMarketDataCmdAny::SymbolPriceTicker(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataResAny::PriceTickerList(vec![])))
            }
            SpotMarketDataCmdAny::BookTicker(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataResAny::BookTickerList(vec![])))
            }
            SpotMarketDataCmdAny::RollingWindowTicker(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataResAny::RollingWindowTickerList(vec![])))
            }
        }
    }
}
