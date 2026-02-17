use std::sync::Arc;

use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use diff::ChangeLogEntry;
use rust_queue::queue::queue::{Queue, ToBytes};
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

use crate::k_line::aggregator::m100_simd_k_line_aggregator::M100SimdKLineAggregator;
use crate::k_line::k_line_types::{KLineAggMut, KLineUpdateEvent};

pub struct KLineBehaviorV2Imp {
    //todo 每个交易对 分配一个aggregator
    aggregator: Arc<std::sync::Mutex<M100SimdKLineAggregator>>,
    queue: Arc<MPMCQueue>, // 使用具体类型而不是trait对象，因为Queue trait有泛型方法
}

impl KLineBehaviorV2Imp {
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

        Self { aggregator: Arc::new(std::sync::Mutex::new(aggregator)), queue }
    }

    /// 处理交易变更日志，提取交易数据并更新K线
    pub fn handle_event(&self, change_log: ChangeLogEntry) {
        // 从ChangeLogEntry中提取交易数据
        let fields_map: std::collections::HashMap<&str, &str> = match change_log.change_type() {
            diff::ChangeType::Created { fields } | diff::ChangeType::Updated { changed_fields: fields } => {
                fields.iter().map(|f| (f.field_name.as_ref(), f.new_value.as_str())).collect()
            }
            diff::ChangeType::Deleted => {
                tracing::debug!("Skipping deleted entity in ChangeLogEntry");
                return;
            }
        };

        // 解析交易必需字段
        let price: f64 = match fields_map.get("price").and_then(|v| v.parse().ok()) {
            Some(p) => p,
            None => {
                tracing::debug!("Missing or invalid price field in ChangeLogEntry");
                return;
            }
        };

        let volume: f64 = match fields_map.get("volume").or_else(|| fields_map.get("amount"))
            .and_then(|v| v.parse().ok()) {
            Some(v) => v,
            None => {
                tracing::debug!("Missing or invalid volume/amount field in ChangeLogEntry");
                return;
            }
        };

        // todo 根据交易对查找对应的aggregator（当前单交易对实现）
        let mut agg = self.aggregator.lock().unwrap();
        
        // 处理交易数据
        if let Err(e) = agg.process_trade(
            *change_log.timestamp(),
            price,
            volume,
        ) {
            tracing::error!("Failed to process trade: {:?}", e);
        }
    }

}

impl ActorX for KLineBehaviorV2Imp {
    fn start(self: &Arc<Self>) {
        let self_clone = self.clone();
        let mut receiver = self.queue.subscribe(SpotTopic::OrderChangeLog.name(), None);

        // todo 优化性能， lock导致的
        tokio::spawn(async move {
            while let Ok(msg) = receiver.recv().await {

                let trade_change_log = match serde_json::from_slice::<ChangeLogEntry>(&msg) {
                    Ok(log) =>{

                        log},
                    Err(e) => {
                        tracing::error!("Failed to deserialize event to ChangeLogEntry: {:?}", e);
                        continue;
                    }
                };
                
                // 处理交易变更日志
                self_clone.handle_event(trade_change_log);

            }
        });
    }
}


// 为M100SimdKLineAggregator实现Clone trait
impl Clone for M100SimdKLineAggregator {
    fn clone(&self) -> Self {
        Self::new()
    }
}
