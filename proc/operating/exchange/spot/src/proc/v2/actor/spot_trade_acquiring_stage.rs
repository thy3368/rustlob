use std::sync::Arc;

use base_types::actor_x::ActorX;
use base_types::cqrs::cqrs_types::CmdResp;
use base_types::handler::handler::Handler;
use base_types::spot_topic::SpotTopic;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_trade_behavior_v2::{
    NewOrderCmd, SpotTradeCmdAny, SpotTradeResAny,
};
use crate::proc::v2::actor::kafka_config::{send_single_log, KafkaConfig};
use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

//收单
pub struct SpotAcquiringStage {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    /// Kafka 配置
    kafka_config: KafkaConfig,
}

impl SpotAcquiringStage {
    pub fn new(trade_behavior: Arc<SpotTradeBehaviorV2Impl>, kafka_config: KafkaConfig) -> Self {
        Self { trade_behavior, kafka_config }
    }

    /// 创建并启动 SpotAcquiringStage（便捷方法）
    ///
    /// # Arguments
    /// * `trade_behavior` - 交易行为实现
    /// * `kafka_config` - Kafka 配置
    ///
    /// # Example
    /// ```ignore
    /// let stage = SpotAcquiringStage::create_and_start(
    ///     trade_behavior,
    ///     KafkaConfig::default_local()
    /// );
    /// ```
    pub fn create_and_start(
        trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
        kafka_config: KafkaConfig,
    ) -> Arc<Self> {
        let stage = Arc::new(Self::new(trade_behavior, kafka_config));
        stage.start();
        stage
    }

    /// 创建 Kafka 生产者
    fn create_producer(&self) -> FutureProducer {
        ClientConfig::new()
            .set("bootstrap.servers", &self.kafka_config.brokers)
            .set("message.timeout.ms", "5000")
            .set("acks", "1")
            .set("linger.ms", "5")
            .set("batch.size", "32768")
            .set("compression.type", "lz4")
            .set("retries", "3")
            .set("max.in.flight.requests.per.connection", "5")
            .create()
            .expect("Failed to create Kafka producer")
    }
    /// 处理 NewOrderCmd 命令
    /// 1. 验证订单
    /// 2. 生成 spot_order
    /// 3. 发送相应事件 (order_pending / order_cond_pending)
    ///
    async fn handle_new_order(
        &self,
        cmd: NewOrderCmd,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        use base_types::exchange::spot::spot_types::{OrderStatus, SpotOrder};
        use diff::Entity;

        // 提前克隆需要的字段
        let symbol = cmd.symbol().clone();
        let new_client_order_id = cmd.new_client_order_id().clone();

        tracing::info!(
            "SpotAcquiringStage 收到 NewOrderCmd: symbol={:?}, side={:?}, type={:?}",
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
                    return Err(e);
                }
            };

        // 创建 Kafka 生产者
        let producer = self.create_producer();

        // 发送 balance_change_log 到 BalanceChangeLog topic
        send_single_log(&producer, SpotTopic::BalanceChangeLog.name(), &balance_change_log)
            .await
            .map_err(|e| {
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("发送 balance_change_log 失败: {}", e),
            })
        })?;
        tracing::info!(
            "成功发送 balance_change_log 到 BalanceChangeLog topic, entity_id={}",
            balance_change_log.entity_id()
        );

        // 发送 order_change_log 到 OrderChangeLog topic
        send_single_log(&producer, SpotTopic::OrderChangeLog.name(), &order_change_log)
            .await
            .map_err(|e| {
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("发送 order_change_log 失败: {}", e),
                })
            })?;
        tracing::info!(
            "成功发送 order_change_log 到 OrderChangeLog topic, entity_id={}",
            order_change_log.entity_id()
        );

        // 返回成功响应
        use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderAck;
        use base_types::cqrs::cqrs_types::ResMetadata;
        use base_types::Timestamp;

        let order_id = order_change_log.entity_id().parse::<u64>()
            .map_err(|_| SpotCmdErrorAny::Common(CommonError::Internal {
                message: "Invalid order_id format".to_string(),
            }))?;

        let result = NewOrderAck::new(
            symbol,
            order_id,
            -1,  // order_list_id
            new_client_order_id,
            Timestamp::now_as_nanos(),
        );

        let metadata = ResMetadata::new(
            0,  // nonce
            false,  // is_duplicate
            Timestamp::now_as_nanos(),
        );

        Ok(CmdResp::new(metadata, SpotTradeResAny::NewOrderAck(result)))
    }
}

impl ActorX for SpotAcquiringStage {
    fn start(self: &Arc<Self>) {
        // todo: 收单阶段需要从某个来源接收 NewOrderCmd
        // 可选方案：
        // 1. 从 Kafka topic "NewOrderCmd" 接收（需要添加 serde feature）
        // 2. 从 gRPC/HTTP API 接收
        // 3. 从其他消息队列接收
        // 目前留空，由调用方根据需要实现

        tracing::info!("SpotAcquiringStage started. 需要实现命令接收机制（Kafka/gRPC/HTTP等）");
    }
}

impl Handler<SpotTradeCmdAny, SpotTradeResAny, SpotCmdErrorAny> for SpotAcquiringStage {
    async fn handle(
        &self,
        cmd: SpotTradeCmdAny,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        match cmd {
            SpotTradeCmdAny::NewOrder(new_order_cmd) => self.handle_new_order(new_order_cmd).await,
            _ => self.trade_behavior.handle(cmd).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use base_types::cqrs::cqrs_types::CMetadata;
    use base_types::exchange::spot::spot_types::{OrderType, TimeInForce};
    use base_types::{OrderSide, Price, Quantity, Timestamp};

    use super::*;

    /// 创建测试用的 NewOrderCmd
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
                None,
            ),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::from_f64(0.1)),
            None,
            Some(Price::from_f64(45000.0)),
            Some("client_order_456".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// 测试 SpotAcquiringStage 创建
    #[test]
    fn test_spot_acquiring_stage_creation() {
        // 创建 Kafka 配置
        let kafka_config = KafkaConfig::default_local();

        // 验证配置正确
        assert_eq!(kafka_config.brokers, "localhost:9092");
        assert!(!kafka_config.group_id.is_empty());
    }

    /// 测试 KafkaConfig 创建
    #[test]
    fn test_kafka_config_new() {
        let config = KafkaConfig::new("localhost:9092", "test-group");
        assert_eq!(config.brokers, "localhost:9092");
        assert_eq!(config.group_id, "test-group");
    }


}
