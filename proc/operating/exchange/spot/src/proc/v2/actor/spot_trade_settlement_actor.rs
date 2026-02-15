use std::sync::Arc;

use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use diff::{ChangeLogEntry, ChangeType};
use rust_queue::queue::queue::Queue;
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

pub struct SpotSettlementActor {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    queue: Arc<MPMCQueue>,
}

impl SpotSettlementActor {
    /// 处理变更日志事件
    /// 如果事件是 SpotTrade.create=true（即ChangeType::Created）则进行结算处理
    async fn handle_change_log(&self, event_bytes: bytes::Bytes) {
        // 解析 ChangeLogEntry
        let change_log: ChangeLogEntry = match serde_json::from_slice(&event_bytes) {
            Ok(log) => log,
            Err(e) => {
                tracing::error!("Failed to deserialize ChangeLogEntry: {:?}", e);
                return;
            }
        };

        // 检查是否是 SpotTrade 实体
        if change_log.entity_type() != "SpotTrade" {
            return;
        }

        // 检查变更类型是否为 Created（create=true）
        let is_created = match change_log.change_type() {
            ChangeType::Created { .. } => true,
            _ => false,
        };

        if !is_created {
            return;
        }

        tracing::info!("检测到新交易创建，开始结算处理: trade_id={}", change_log.entity_id());

        // 解析 trade_id
        let trade_id = match change_log.entity_id().parse::<u64>() {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("Failed to parse trade_id: {:?}", e);
                return;
            }
        };

        // 执行结算处理
        self.process_settlement(trade_id).await;
    }

    /// 执行结算处理
    async fn process_settlement(&self, trade_id: u64) {
        tracing::info!("执行结算处理: trade_id={}", trade_id);

        // 调用 handle_settlement2 进行结算并获取余额变更日志
        let balance_change_logs = match self.trade_behavior.handle_settlement2(trade_id) {
            Ok(logs) => {
                tracing::info!("结算成功: trade_id={}, 生成 {} 条余额变更日志", trade_id, logs.len());
                logs
            }
            Err(e) => {
                tracing::error!("结算失败: trade_id={}, error={:?}", trade_id, e);
                return;
            }
        };



        tracing::info!("结算处理完成: trade_id={}", trade_id);
    }
}

impl ActorX for SpotSettlementActor {
    fn start(self: &Arc<Self>) {
        // 同时订阅变更日志事件（用于接收其他系统事件）
        let self_clone2 = Arc::clone(self);
        tokio::spawn(async move {
            let mut receiver = self_clone2.queue.subscribe(SpotTopic::EntityChangeLog.name(), None);

            while let Ok(event) = receiver.recv().await {
                // 如果事件是 trade.create=true  则进行结算处理； 并发送balance的changelog
                self_clone2.handle_change_log(event).await;
            }
        });
    }
}
