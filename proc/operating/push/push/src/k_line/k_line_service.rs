use std::sync::Arc;

use rust_queue::queue::queue::{DefaultQueueConfig, Queue};

use crate::k_line::{
    aggregator::m100_simd_k_line_aggregator::M100SimdKLineAggregator,
    k_line_types::{KLineAggMut, KLineUpdateEvent}
};

pub struct KLineService {
    aggregator: M100SimdKLineAggregator,
    queue: Arc<dyn Queue<Config = DefaultQueueConfig>>
}

impl KLineService {
    pub fn new(queue: Arc<dyn Queue<Config = DefaultQueueConfig>>) -> Self {
        let mut aggregator = M100SimdKLineAggregator::new();

        // 订阅聚合器的K线更新事件，将事件发送到队列
        let queue_clone = queue.clone();
        aggregator.subscribe(move |event: KLineUpdateEvent| {
            // 将K线更新事件发送到队列，供push服务推送
            if let Err(e) = queue_clone.send("SpotTopic_KLine", event, None) {
                tracing::error!("Failed to publish KLineUpdateEvent: {:?}", e);
            }
        });

        Self {
            aggregator,
            queue
        }
    }

    // 从queue订阅entity_change_log，如果是trade/create事件，
    // 则调用aggregator进行聚合
    pub async fn start_listening(&self) {
        let mut receiver = self.queue.subscribe::<Vec<u8>>("SpotTopic_EntityChangeLog", None);
        let aggregator = Arc::new(tokio::sync::Mutex::new(self.aggregator.clone()));

        tokio::spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                // 解析entity_change_log消息，判断是否是trade/create事件
                if let Ok(trade_event) = Self::parse_trade_event(&msg) {
                    // 调用聚合器处理交易
                    let mut agg = aggregator.lock().await;
                    if let Err(e) = agg.process_trade(trade_event.timestamp, trade_event.price, trade_event.volume) {
                        tracing::error!("Failed to process trade: {:?}", e);
                    }
                }
            }
        });
    }

    // 解析交易事件的辅助方法（需要根据实际消息格式实现）
    fn parse_trade_event(msg: &[u8]) -> Result<TradeEvent, String> {
        // 这里只是示例，需要根据实际的消息格式进行解析
        // 假设消息是JSON格式
        let msg_str = String::from_utf8(msg.to_vec()).map_err(|e| format!("Invalid UTF-8: {:?}", e))?;
        let trade_event: TradeEvent =
            serde_json::from_str(&msg_str).map_err(|e| format!("Failed to parse trade event: {:?}", e))?;
        Ok(trade_event)
    }
}

// 交易事件的示例结构体（需要根据实际消息格式定义）
#[derive(Debug, serde::Deserialize, Clone)]
struct TradeEvent {
    timestamp: u64,
    price: f64,
    volume: f64 // 其他字段
}
