use std::sync::Arc;
use push::k_line::k_line_service::KLineBehaviorV2Imp;
use push::push::push_service::PushBehaviorV2Imp;
use crate::proc::v2::actor::kafka_config::KafkaConfig;
use crate::proc::v2::actor::spot_trade_k_line_actor::SpotKLineStage;

pub struct SpotPushStage {
    /// 推送服务，负责将变更日志推送到客户端
    push_service: Arc<PushBehaviorV2Imp>,

    kafka_config: KafkaConfig,
}
impl SpotKLineStage{


    //todo     sub kafka的 trade balance order kline事件， 调用      self.kline_service.handle_events(&kline_trade_logs);

}
