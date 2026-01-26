use spot_behavior::proc::behavior::v2::spot_market_data_sse_behavior::{
    AggregateTradeStream, MiniTickerStream, SpotMarketDataPublishBehavior, SpotMarketDataStreamAny, TradeStream
};
use tokio::sync::broadcast;


/// SpotMarketDataStreamAny 消息推送器
pub struct SpotMarketDataPusher {
    tx: broadcast::Sender<SpotMarketDataStreamAny>,
    interval: tokio::time::Duration,
    counter: u64
}

impl SpotMarketDataPusher {
    /// 创建新的 SpotMarketDataPusher 实例
    pub fn new(tx: broadcast::Sender<SpotMarketDataStreamAny>) -> Self {
        Self {
            tx,
            interval: tokio::time::Duration::from_secs(5),
            counter: 0
        }
    }

    /// 设置消息推送间隔（秒）
    pub fn with_interval(mut self, seconds: u64) -> Self {
        self.interval = tokio::time::Duration::from_secs(seconds);
        self
    }

    /// 启动消息推送任务
    pub fn start(self) {
        tokio::spawn(async move {
            self.run().await;
        });
    }

    /// 运行消息推送循环
    async fn run(mut self) {
        let mut interval = tokio::time::interval(self.interval);

        loop {
            interval.tick().await;
            self.counter += 1;

            // 模拟生成不同类型的市场数据消息
            let stream_msg: SpotMarketDataStreamAny = self.generate_stream_message();


            let _ = self.tx.send(stream_msg);
        }
    }
}
impl SpotMarketDataPublishBehavior for SpotMarketDataPusher {
    /// 生成模拟的 SpotMarketDataStreamAny 消息
    fn generate_stream_message(&self) -> SpotMarketDataStreamAny {
        let counter = self.counter;
        let now = chrono::Utc::now().timestamp_millis();

        if counter % 3 == 0 {
            SpotMarketDataStreamAny::AggregateTrade(AggregateTradeStream {
                event_type: "aggTrade".to_string(),
                event_time: now,
                symbol: "BTCUSDT".to_string(),
                agg_trade_id: counter as i64,
                price: format!("{:.2}", 45000.0 + counter as f64 * 0.1),
                quantity: format!("{:.4}", 0.001 + counter as f64 * 0.0001),
                first_trade_id: counter as i64,
                last_trade_id: counter as i64,
                trade_time: now,
                is_buyer_maker: counter % 2 == 0,
                ignore: false
            })
        } else if counter % 3 == 1 {
            SpotMarketDataStreamAny::Trade(TradeStream {
                event_type: "trade".to_string(),
                event_time: now,
                symbol: "ETHUSDT".to_string(),
                trade_id: counter as i64,
                price: format!("{:.2}", 2500.0 + counter as f64 * 0.05),
                quantity: format!("{:.4}", 0.01 + counter as f64 * 0.001),
                trade_time: now,
                is_buyer_maker: counter % 2 == 1,
                ignore: false
            })
        } else {
            SpotMarketDataStreamAny::MiniTicker(MiniTickerStream {
                event_type: "24hrMiniTicker".to_string(),
                event_time: now,
                symbol: "ADAUSDT".to_string(),
                close_price: format!("{:.4}", 0.45 + counter as f64 * 0.001),
                open_price: format!("{:.4}", 0.44 + counter as f64 * 0.0005),
                high_price: format!("{:.4}", 0.46 + counter as f64 * 0.0015),
                low_price: format!("{:.4}", 0.43 + counter as f64 * 0.0005),
                base_volume: format!("{:.0}", 1000000.0 + counter as f64 * 1000.0),
                quote_volume: format!("{:.0}", 450000.0 + counter as f64 * 500.0)
            })
        }
    }
}

/// 便捷函数：创建并启动 SpotMarketDataPusher
pub fn start_spot_market_data_pusher(tx: broadcast::Sender<SpotMarketDataStreamAny>) {
    SpotMarketDataPusher::new(tx).start();
}

/// 便捷函数：创建并启动带有自定义间隔的 SpotMarketDataPusher
pub fn start_spot_market_data_pusher_with_interval(
    tx: broadcast::Sender<SpotMarketDataStreamAny>, interval_seconds: u64
) {
    SpotMarketDataPusher::new(tx).with_interval(interval_seconds).start();
}
