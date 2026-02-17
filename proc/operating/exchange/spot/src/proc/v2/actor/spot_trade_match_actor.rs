use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use base_types::actor_x::ActorX;
use base_types::base_types::TraderId;
use base_types::exchange::spot::spot_types::{
    AlgorithmStrategy, ConditionalType, ExecutionMethod, OrderStatus, OrderType,
    SelfTradePrevention, SpotOrder, SpotTrade, TimeInForce,
};
use base_types::spot_topic::SpotTopic;
use base_types::{OrderSide, Price, Quantity, TradingPair};
use diff::{ChangeLogEntry, ChangeType, FieldChange};
use rust_queue::queue::queue::Queue;
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

use crate::proc::v2::actor::spot_trade_acquiring_actor::{NewOrderCmdReceiver, SpotAcquiringActor};
use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

pub struct SpotMatchActor {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    queue: Arc<MPMCQueue>,
}

impl SpotMatchActor {
    /// 处理变更日志事件
    /// 如果事件是 order.status=pending 则进行撮合处理
    async fn handle_change_log(&self, event_bytes: bytes::Bytes) {
        // 解析 ChangeLogEntry
        let change_log: ChangeLogEntry = match serde_json::from_slice(&event_bytes) {
            Ok(log) => log,
            Err(e) => {
                tracing::error!("Failed to deserialize ChangeLogEntry: {:?}", e);
                return;
            }
        };

        // 检查是否是 SpotOrder 实体
        if change_log.entity_type() != "SpotOrder" {
            return;
        }

        // 检查变更类型是否包含 status 字段更新为 Pending
        let is_pending = match change_log.change_type() {
            ChangeType::Created { fields } | ChangeType::Updated { changed_fields: fields } => {
                fields.iter().any(|field| {
                    field.field_name.as_ref() == "status"
                        && (field.new_value == "Pending" || field.new_value == "PENDING")
                })
            }
            _ => false,
        };

        if !is_pending {
            return;
        }

        tracing::info!(
            "检测到订单进入 Pending 状态，开始撮合处理: order_id={}",
            change_log.entity_id()
        );

        // 调用 handle_match2 进行撮合并获取变更日志
        let change_logs = match self.trade_behavior.handle_match3(change_log) {
            Ok(logs) => logs,
            Err(e) => {
                return;
            }
        };

        //todo 发送 change_logs
    }
}

impl ActorX for SpotMatchActor {
    fn start(self: &Arc<Self>) {
        // 同时订阅变更日志事件（用于接收其他系统事件）
        let self_clone2 = Arc::clone(self);
        tokio::spawn(async move {
            let mut receiver = self_clone2.queue.subscribe(SpotTopic::OrderChangeLog.name(), None);

            while let Ok(event) = receiver.recv().await {
                // 如果事件是 order.status=pending 则进行撮合处理； 并发送trade和order的changelog
                self_clone2.handle_change_log(event).await;
            }
        });
    }
}
