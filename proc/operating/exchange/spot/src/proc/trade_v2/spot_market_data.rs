use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_market_data_behavior::{
    SpotMarketDataBehavior, SpotMarketDataCmdAny, SpotMarketDataRes, OrderBookData, AvgPriceData,
};
use base_types::cqrs::cqrs_types::CmdResp;

pub struct SpotMarketDataImpl {}

impl SpotMarketDataBehavior for SpotMarketDataImpl {
    fn handle(&mut self, cmd: SpotMarketDataCmdAny) -> Result<CmdResp<SpotMarketDataRes>, SpotCmdErrorAny> {
        // 使用固定的 nonce 值，实际应用中应该从命令元数据中获取
        let nonce = 0;

        match cmd {
            SpotMarketDataCmdAny::OrderBook(_) => {
                // 暂时返回一个空的订单簿响应
                Ok(CmdResp::new(nonce, SpotMarketDataRes::OrderBook(OrderBookData {
                    last_update_id: 0,
                    bids: vec![],
                    asks: vec![],
                })))
            }
            SpotMarketDataCmdAny::RecentTrades(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataRes::Trades(vec![])))
            }
            SpotMarketDataCmdAny::HistoricalTrades(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataRes::Trades(vec![])))
            }
            SpotMarketDataCmdAny::AggTrades(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataRes::AggTrades(vec![])))
            }
            SpotMarketDataCmdAny::Klines(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataRes::Klines(vec![])))
            }
            SpotMarketDataCmdAny::UIKlines(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataRes::Klines(vec![])))
            }
            SpotMarketDataCmdAny::AvgPrice(_) => {
                // 暂时返回一个默认的平均价格响应
                Ok(CmdResp::new(nonce, SpotMarketDataRes::AvgPrice(
                    AvgPriceData {
                        mins: 5,
                        price: "50000.00".to_string(),
                        close_time: 0,
                    },
                )))
            }
            SpotMarketDataCmdAny::Ticker24hr(_) => {
                // 暂时返回一个空的24小时Ticker响应列表
                Ok(CmdResp::new(nonce, SpotMarketDataRes::Ticker24hrList(vec![])))
            }
            SpotMarketDataCmdAny::TradingDayTicker(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataRes::TradingDayTickerList(vec![])))
            }
            SpotMarketDataCmdAny::SymbolPriceTicker(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataRes::PriceTickerList(vec![])))
            }
            SpotMarketDataCmdAny::BookTicker(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataRes::BookTickerList(vec![])))
            }
            SpotMarketDataCmdAny::RollingWindowTicker(_) => {
                Ok(CmdResp::new(nonce, SpotMarketDataRes::RollingWindowTickerList(vec![])))
            }
        }
    }
}
