use std::sync::{Arc, Mutex};

use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use crossbeam_channel::{Receiver, Sender};
use rust_queue::queue::queue::Queue;
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;
use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

/// SpotAcquiringActor 命令发送器类型（无锁队列）
pub type NewOrderCmdSender = Sender<NewOrderCmd>;

/// SpotAcquiringActor 命令接收器类型（无锁队列）
pub type NewOrderCmdReceiver = Receiver<NewOrderCmd>;

//收单
pub struct SpotAcquiringActor {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    /// 无锁命令队列接收端（使用 Mutex 允许在 Arc 中可变）
    command_receiver: Mutex<Option<NewOrderCmdReceiver>>,
    /// MPMC 队列，用于发送事件到不同 topic
    queue: Arc<MPMCQueue>,
}

impl SpotAcquiringActor {
    pub fn new(
        trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
        queue: Arc<MPMCQueue>,
        command_receiver: NewOrderCmdReceiver,
    ) -> Self {
        Self { trade_behavior, command_receiver: Mutex::new(Some(command_receiver)), queue }
    }
    /// 处理 NewOrderCmd 命令
    /// 1. 验证订单
    /// 2. 生成 spot_order
    /// 3. 发送相应事件 (order_pending / order_cond_pending)
    ///
    async fn handle_new_order(&self, cmd: NewOrderCmd) -> Result<(), Box<dyn std::error::Error>> {
        use base_types::exchange::spot::spot_types::{OrderStatus, SpotOrder};
        use diff::Entity;

        tracing::info!(
            "SpotAcquiringActor 收到 NewOrderCmd: symbol={:?}, side={:?}, type={:?}",
            cmd.symbol(),
            cmd.side(),
            cmd.order_type()
        );

        // 包括验证、生成订单、冻结资金等
        let (balance_change_log, order_change_log) =
            match self.trade_behavior.handle_acquiring2(cmd) {
                Ok((balance_log, order_log)) => (balance_log, order_log),
                Err(e) => {
                    tracing::error!("订单处理失败: {:?}", e);
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("订单处理失败: {:?}", e),
                    )));
                }
            };

        // 发送 balance_change_log 到 BalanceChangeLog topic
        let balance_bytes = match serde_json::to_vec(&balance_change_log) {
            Ok(bytes) => bytes::Bytes::from(bytes),
            Err(e) => {
                tracing::error!("序列化 balance_change_log 失败: {:?}", e);
                return Err(Box::new(e));
            }
        };
        if let Err(e) = self.queue.send(SpotTopic::BalanceChangeLog.name(), balance_bytes, None) {
            tracing::error!("发送 balance_change_log 到 topic 失败: {:?}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("发送 balance_change_log 失败: {:?}", e),
            )));
        }
        tracing::info!("成功发送 balance_change_log 到 BalanceChangeLog topic, entity_id={}", balance_change_log.entity_id());

        // 发送 order_change_log 到 OrderChangeLog topic
        let order_bytes = match serde_json::to_vec(&order_change_log) {
            Ok(bytes) => bytes::Bytes::from(bytes),
            Err(e) => {
                tracing::error!("序列化 order_change_log 失败: {:?}", e);
                return Err(Box::new(e));
            }
        };
        if let Err(e) = self.queue.send(SpotTopic::OrderChangeLog.name(), order_bytes, None) {
            tracing::error!("发送 order_change_log 到 topic 失败: {:?}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("发送 order_change_log 失败: {:?}", e),
            )));
        }
        tracing::info!("成功发送 order_change_log 到 OrderChangeLog topic, entity_id={}", order_change_log.entity_id());

        Ok(())
    }
}

impl ActorX for SpotAcquiringActor {
    fn start(self: &Arc<Self>) {
        // 从 Mutex 中取出 receiver（只能在 start 时调用一次）
        let receiver = match self.command_receiver.lock().unwrap().take() {
            Some(r) => r,
            None => {
                tracing::error!("SpotAcquiringActor command_receiver 已经被取走，无法启动");
                return;
            }
        };

        let self_clone = Arc::clone(self);

        // 使用 blocking 任务在无锁队列上接收命令
        tokio::task::spawn_blocking(move || {
            // 从单播无锁 channel 接收 NewOrderCmd 命令
            while let Ok(cmd) = receiver.recv() {
                // 创建异步任务处理命令
                let self_ref = Arc::clone(&self_clone);
                tokio::spawn(async move {
                    if let Err(e) = self_ref.handle_new_order(cmd).await {
                        tracing::error!("处理 NewOrderCmd 失败: {:?}", e);
                    }
                });
            }

            tracing::info!("SpotAcquiringActor command_receiver 已关闭");
        });
    }
}

#[cfg(test)]
mod tests {
    use base_types::cqrs::cqrs_types::CMetadata;
    use base_types::exchange::spot::spot_types::{OrderType, TimeInForce};
    use base_types::{OrderSide, Price, Quantity, Timestamp};
    use crossbeam_channel::unbounded;

    use super::*;

    /// 创建测试用的 NewOrderCmd
    /// 使用 immutable 宏自动生成的 new() 构造函数
    fn create_test_new_order_cmd() -> NewOrderCmd {
        use base_types::TradingPair;

        NewOrderCmd::new(
            CMetadata::new(
                "test_order_123".to_string(),
                Timestamp(chrono::Utc::now().timestamp_millis() as u64),
                None,
                None,
                Some("test_user".to_string()),
                Vec::new(),
                None, // recv_window
            ),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::from_f64(0.1)),
            None, // quote_order_qty
            Some(Price::from_f64(45000.0)),
            Some("client_order_456".to_string()),
            None, // strategy_id
            None, // strategy_type
            None, // stop_price
            None, // trailing_delta
            None, // iceberg_qty
            None, // new_order_resp_type
            None, // self_trade_prevention_mode
            None, // peg_price_type
            None, // peg_offset_value
            None, // peg_offset_type
            None, // recv_window
            Timestamp::now_as_nanos(),
        )
    }

    #[tokio::test]
    async fn test_send_new_order_command() {
        // 初始化 tracing
        let _ = tracing_subscriber::fmt::try_init();

        // 创建无锁 channel
        let (cmd_sender, cmd_receiver) = unbounded::<NewOrderCmd>();

        // 创建 MPMCQueue
        let queue = Arc::new(MPMCQueue::new());

        // 创建一个 mock 的 SpotTradeBehaviorV2Impl
        // 注意：由于 SpotTradeBehaviorV2Impl 使用了 #[immutable] 宏且依赖较多，
        // 实际测试时需要根据项目具体情况提供合适的 mock 或测试实例

        // todo: 替换为实际的 SpotTradeBehaviorV2Impl 实例
        // let trade_behavior = Arc::new(SpotTradeBehaviorV2Impl::new(...));

        // 由于结构体字段都是私有的，我们需要通过其他方式获取实例
        // 比如通过配置文件、依赖注入或者 mock 对象

        // 创建 Actor
        // let acquiring_actor = Arc::new(SpotAcquiringActor::new(
        //     trade_behavior,
        //     queue.clone(),
        //     cmd_receiver,
        // ));

        // 启动 Actor
        // acquiring_actor.start();

        // 发送 NewOrderCmd 命令
        let cmd = create_test_new_order_cmd();
        cmd_sender.send(cmd.clone()).expect("发送命令失败");

        tracing::info!("成功发送 NewOrderCmd: {:?}", cmd);

        // 等待一段时间让 Actor 处理命令
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // 验证：在实际测试中，这里可以检查订单是否被正确处理
        // 例如检查队列中是否有相应的事件、订单状态等

        // 关闭 channel
        drop(cmd_sender);

        tracing::info!("测试完成");
    }

    #[test]
    fn test_channel_creation() {
        // 测试 channel 创建和基本操作
        let (sender, receiver) = unbounded::<NewOrderCmd>();

        let cmd = create_test_new_order_cmd();
        sender.send(cmd.clone()).expect("发送失败");

        let received = receiver.recv().expect("接收失败");

        // 使用 immutable 宏生成的 getter 方法访问字段
        assert_eq!(received.symbol(), cmd.symbol());
        assert_eq!(received.side(), cmd.side());
        assert_eq!(received.order_type(), cmd.order_type());

        println!("Channel 测试通过: 成功发送和接收 NewOrderCmd");
    }
}
