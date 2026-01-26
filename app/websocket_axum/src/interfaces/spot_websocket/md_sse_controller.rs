use spot_behavior::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError, CommonError};
use spot_behavior::proc::behavior::v2::spot_market_data_sse_behavior::{
    MarketDataSubscriptionCmdAny, SpotMarketDataStreamAny, SpotMarketDataSSEBehavior, SubscriptionResponse, SubscriptionResult,
};

/// Spot 市场数据 SSE 实现
#[derive(Clone)]
pub struct SpotMarketDataSSEImpl {
    /// 已订阅的流列表
    subscriptions: Vec<String>,
    /// 是否启用复合流属性
    combined_stream_enabled: bool,
}

impl SpotMarketDataSSEImpl {
    /// 创建新的 SpotMarketDataSSEImpl 实例
    pub fn new() -> Self {
        Self {
            subscriptions: Vec::new(),
            combined_stream_enabled: false,
        }
    }
}

impl SpotMarketDataSSEBehavior for SpotMarketDataSSEImpl {
    fn handle_subscription(
        &mut self,
        cmd: MarketDataSubscriptionCmdAny,
    ) -> Result<CmdResp<SubscriptionResponse>, SpotCmdError> {
        let nonce = 0; // 临时值，实际应该从 metadata 中获取或生成

        match cmd {
            MarketDataSubscriptionCmdAny::Subscribe { metadata, streams } => {
                // 添加新的订阅流
                for stream in streams {
                    if !self.subscriptions.contains(&stream) {
                        self.subscriptions.push(stream);
                    }
                }
                // 返回成功响应
                Ok(CmdResp::new(
                    nonce,
                    SubscriptionResponse {
                        result: Some(SubscriptionResult::Success),
                        id: None,
                    },
                ))
            }

            MarketDataSubscriptionCmdAny::Unsubscribe { metadata, streams } => {
                // 移除订阅流
                for stream in streams {
                    self.subscriptions.retain(|s| s != &stream);
                }
                // 返回成功响应
                Ok(CmdResp::new(
                    nonce,
                    SubscriptionResponse {
                        result: Some(SubscriptionResult::Success),
                        id: None,
                    },
                ))
            }

            MarketDataSubscriptionCmdAny::ListSubscriptions { metadata } => {
                // 返回当前订阅列表
                Ok(CmdResp::new(
                    nonce,
                    SubscriptionResponse {
                        result: Some(SubscriptionResult::Subscriptions(self.subscriptions.clone())),
                        id: None,
                    },
                ))
            }

            MarketDataSubscriptionCmdAny::SetProperty {
                metadata,
                property,
                value,
            } => {
                // 设置属性（目前仅支持 combined 属性）
                if property == "combined" {
                    self.combined_stream_enabled = value;
                    Ok(CmdResp::new(
                        nonce,
                        SubscriptionResponse {
                            result: Some(SubscriptionResult::Property(value)),
                            id: None,
                        },
                    ))
                } else {
                    Err(SpotCmdError::Common(CommonError::InvalidParameter {
                        field: "property",
                        reason: "Unknown property",
                    }))
                }
            }

            MarketDataSubscriptionCmdAny::GetProperty {
                metadata,
                property,
            } => {
                // 获取属性值
                if property == "combined" {
                    Ok(CmdResp::new(
                        nonce,
                        SubscriptionResponse {
                            result: Some(SubscriptionResult::Property(self.combined_stream_enabled)),
                            id: None,
                        },
                    ))
                } else {
                    Err(SpotCmdError::Common(CommonError::InvalidParameter {
                        field: "property",
                        reason: "Unknown property",
                    }))
                }
            }
        }
    }

    fn handle_stream_data(&mut self, data: SpotMarketDataStreamAny) -> Result<(), SpotCmdError> {
        // 处理市场数据流消息
        match data {
            SpotMarketDataStreamAny::AggregateTrade(msg) => {
                println!(
                    "AggregateTrade: {} @ {} ({})",
                    msg.symbol, msg.price, msg.quantity
                );
            }
            SpotMarketDataStreamAny::Trade(msg) => {
                println!(
                    "Trade: {} @ {} ({})",
                    msg.symbol, msg.price, msg.quantity
                );
            }
            SpotMarketDataStreamAny::Kline(msg) => {
                println!(
                    "Kline: {} {} [O: {}, H: {}, L: {}, C: {}]",
                    msg.symbol,
                    msg.kline.interval,
                    msg.kline.open_price,
                    msg.kline.high_price,
                    msg.kline.low_price,
                    msg.kline.close_price
                );
            }
            SpotMarketDataStreamAny::KlineWithTimezone(msg) => {
                println!(
                    "KlineWithTimezone: {} {} [O: {}, H: {}, L: {}, C: {}]",
                    msg.symbol,
                    msg.kline.interval,
                    msg.kline.open_price,
                    msg.kline.high_price,
                    msg.kline.low_price,
                    msg.kline.close_price
                );
            }
            SpotMarketDataStreamAny::MiniTicker(msg) => {
                println!(
                    "MiniTicker: {} [O: {}, H: {}, L: {}, C: {}]",
                    msg.symbol, msg.open_price, msg.high_price, msg.low_price, msg.close_price
                );
            }
            SpotMarketDataStreamAny::AllMiniTickers(tickers) => {
                println!("AllMiniTickers: {} tickers updated", tickers.len());
            }
            SpotMarketDataStreamAny::Ticker(msg) => {
                println!(
                    "Ticker: {} [24h Change: {} ({})]",
                    msg.symbol, msg.price_change, msg.price_change_percent
                );
            }
            SpotMarketDataStreamAny::RollingWindowStats(msg) => {
                println!(
                    "RollingWindowStats: {} [{} window]",
                    msg.symbol, msg.event_type
                );
            }
            SpotMarketDataStreamAny::AllRollingWindowStats(stats) => {
                println!("AllRollingWindowStats: {} symbols updated", stats.len());
            }
            SpotMarketDataStreamAny::BookTicker(msg) => {
                println!(
                    "BookTicker: {} [B: {}@{}, A: {}@{}]",
                    msg.symbol,
                    msg.best_bid_price,
                    msg.best_bid_quantity,
                    msg.best_ask_price,
                    msg.best_ask_quantity
                );
            }
            SpotMarketDataStreamAny::AveragePrice(msg) => {
                println!("AveragePrice: {} @ {}", msg.symbol, msg.avg_price);
            }
            SpotMarketDataStreamAny::PartialDepth(msg) => {
                println!(
                    "PartialDepth: {} [{} bids, {} asks]",
                    msg.last_update_id,
                    msg.bids.len(),
                    msg.asks.len()
                );
            }
            SpotMarketDataStreamAny::DiffDepth(msg) => {
                println!(
                    "DiffDepth: {} [{} -> {}] [{} bids, {} asks]",
                    msg.symbol,
                    msg.first_update_id,
                    msg.last_update_id,
                    msg.bids.len(),
                    msg.asks.len()
                );
            }
        }

        Ok(())
    }
}