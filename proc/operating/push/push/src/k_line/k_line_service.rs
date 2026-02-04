use immutable_derive::immutable;
use rust_queue::queue::queue::Queue;
use std::sync::Arc;

use crate::k_line::{
    aggregator::m100_simd_k_line_aggregator::M100SimdKLineAggregator,
    k_line_types::KLineUpdateEvent,
};

#[immutable]
pub struct KLineService {
    aggregator: M100SimdKLineAggregator,
    queue: Arc<dyn Queue>,
}

impl KLineService {
    pub fn new(queue: Arc<dyn Queue>) -> Self {
        let mut aggregator = M100SimdKLineAggregator::new();

        // 订阅聚合器的K线更新事件，将事件发送到队列
        let queue_clone = queue.clone();
        aggregator.subscribe(move |event: KLineUpdateEvent| {
            // 将K线更新事件发送到队列，供push服务推送
            if let Err(e) = queue_clone.publish(SpotTopic::KLine.name(), event) {
                tracing::error!("Failed to publish KLineUpdateEvent: {:?}", e);
            }
        });

        Self { aggregator, queue }
    }

    // 从queue订阅entity_change_log，如果是trade/create事件，则调用aggregator进行聚合
    pub async fn start_listening(&self) {
        if let Err(e) = self.queue.subscribe(SpotTopic::EntityChangeLog.name(), |msg| {
            // 解析entity_change_log消息，判断是否是trade/create事件
            // 这里需要根据实际的消息格式进行解析
            // 假设消息包含trade/create事件的数据
            if let Ok(trade_event) = self.parse_trade_event(msg) {
                // 调用聚合器处理交易
                if let Err(e) = self.aggregator.process_trade(
                    trade_event.timestamp,
                    trade_event.price,
                    trade_event.volume,
                ) {
                    tracing::error!("Failed to process trade: {:?}", e);
                }
            }
        }) {
            tracing::error!("Failed to subscribe to entity_change_log: {:?}", e);
        }
    }

    // 解析交易事件的辅助方法（需要根据实际消息格式实现）
    fn parse_trade_event(&self, msg: Vec<u8>) -> Result<TradeEvent, String> {
        // 这里只是示例，需要根据实际的消息格式进行解析
        // 假设消息是JSON格式
        let msg_str = String::from_utf8(msg).map_err(|e| format!("Invalid UTF-8: {:?}", e))?;
        let trade_event: TradeEvent = serde_json::from_str(&msg_str)
            .map_err(|e| format!("Failed to parse trade event: {:?}", e))?;
        Ok(trade_event)
    }
}

// 交易事件的示例结构体（需要根据实际消息格式定义）
#[derive(Debug, serde::Deserialize)]
struct TradeEvent {
    timestamp: u64,
    price: f64,
    volume: f64,
    // 其他字段
}
