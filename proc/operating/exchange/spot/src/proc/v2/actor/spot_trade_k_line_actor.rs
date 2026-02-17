use std::sync::Arc;

use push::k_line::k_line_service::KLineBehaviorV2Imp;
use push::push::push_service::PushBehaviorV2Imp;

use crate::proc::v2::actor::kafka_config::KafkaConfig;

pub struct SpotKLineStage {

    /// K线服务，负责聚合交易数据生成K线
    kline_service: Arc<KLineBehaviorV2Imp>,
    kafka_config: KafkaConfig,
}


impl SpotKLineStage {
    
    //todo     sub kafka的 trade create事件， 使用        self.kline_service.handle_events(&kline_trade_logs); 发送事件 kline  参考  SpotMatchStage
}
