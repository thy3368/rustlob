use std::sync::Arc;

use actix::Actor;
use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use rust_queue::queue::queue::{Queue, ToBytes};
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

use crate::k_line::aggregator::m100_simd_k_line_aggregator::M100SimdKLineAggregator;
use crate::k_line::k_line_types::{KLineAggMut, KLineUpdateEvent};

pub struct KLineActor {
    //todo 每个交易对 分配一个aggregator
    aggregator: Arc<tokio::sync::Mutex<M100SimdKLineAggregator>>,
    queue: Arc<MPMCQueue>, // 使用具体类型而不是trait对象，因为Queue trait有泛型方法
}

impl KLineActor {
    pub fn new(queue: Arc<MPMCQueue>) -> Self {
        let mut aggregator = M100SimdKLineAggregator::new();

        // 订阅聚合器的K线更新事件，将事件发送到队列
        let queue_clone = queue.clone();
        aggregator.subscribe(move |event: KLineUpdateEvent| {
            // 将K线更新事件发送到队列，供push服务推送
            if let Err(e) =
                queue_clone.send(SpotTopic::KLineChangeLog.name(), event.to_bytes().unwrap(), None)
            {
                tracing::error!("Failed to publish KLineUpdateEvent: {:?}", e);
            }
        });

        Self { aggregator: Arc::new(tokio::sync::Mutex::new(aggregator)), queue }
    }

    // 解析交易事件的辅助方法（需要根据实际消息格式实现）
    fn parse_trade_event(msg: &[u8]) -> Result<TradeEvent, String> {
        // 这里只是示例，需要根据实际的消息格式进行解析
        // 假设消息是JSON格式
        let msg_str =
            String::from_utf8(msg.to_vec()).map_err(|e| format!("Invalid UTF-8: {:?}", e))?;
        let trade_event: TradeEvent = serde_json::from_str(&msg_str)
            .map_err(|e| format!("Failed to parse trade event: {:?}", e))?;
        Ok(trade_event)
    }
}

impl ActorX for KLineActor {
    fn start(self: &Arc<Self>) {
        let mut receiver = self.queue.subscribe(SpotTopic::OrderChangeLog.name(), None);
        let aggregator = self.aggregator.clone();

        // todo 优化性能， lock导致的
        tokio::spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                // 解析entity_change_log消息，判断是否是trade/create事件
                if let Ok(trade_event) = Self::parse_trade_event(&msg) {
                    // 调用聚合器处理交易
                    //todo 根据交易对 查找aggregator
                    let mut agg = aggregator.lock().await;
                    if let Err(e) = agg.process_trade(
                        trade_event.timestamp,
                        trade_event.price,
                        trade_event.volume,
                    ) {
                        tracing::error!("Failed to process trade: {:?}", e);
                    }
                }
            }
        });
    }
}

// 交易事件的示例结构体（需要根据实际消息格式定义）
#[derive(Debug, serde::Deserialize, Clone)]
struct TradeEvent {
    timestamp: u64,
    price: f64,
    volume: f64, // 其他字段
}

// 为M100SimdKLineAggregator实现Clone trait
impl Clone for M100SimdKLineAggregator {
    fn clone(&self) -> Self {
        Self::new()
    }
}
