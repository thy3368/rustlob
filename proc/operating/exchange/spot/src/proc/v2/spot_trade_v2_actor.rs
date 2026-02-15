use std::sync::Arc;

use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use rust_queue::queue::queue::Queue;
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

//收单
pub struct SpotAcquiringActor {
    aggregator: Arc<SpotTradeBehaviorV2Impl>,
    queue: Arc<MPMCQueue>,
}

impl ActorX for SpotAcquiringActor {
    fn start(self: &Arc<Self>) {
        tokio::spawn(async move {
            // 订阅变更日志事件
            let mut receiver = self.queue.subscribe(SpotTopic::EntityChangeLog.name(), None);

            // 持续监听事件
            while let Ok(event) = receiver.recv().await {
                //todo
                //收单， 生成spot_order
                //改送相应的事件出去 order_pending/order_cond_pending
                if let Err(e) =
                    self.queue.send(SpotTopic::KLine.name(), event.to_bytes().unwrap(), None)
                {
                    tracing::error!("Failed to publish KLineUpdateEvent: {:?}", e);
                }
            }
        });
    }
}

//条件单
pub struct SpotCondActor {
    aggregator: Arc<SpotTradeBehaviorV2Impl>,
    queue: Arc<MPMCQueue>, // 使用具体类型而不是trait对象，因为Queue trait有泛型方法
}
impl ActorX for SpotCondActor {
    fn start(self: &Arc<Self>) {
        tokio::spawn(async move {
            // 订阅变更日志事件
            let mut receiver = self.queue.subscribe(SpotTopic::EntityChangeLog.name(), None);

            // 持续监听事件
            while let Ok(event) = receiver.recv().await {
                //todo 只处理order_cond_pending事件
                //todo
                //当价格满足时，spot_order状态变pending
                //改送相应的事件出去 order_pending

                self.process_event(event).await;
            }
        });
    }
}

//撮合
pub struct SpotMatchActor {
    aggregator: Arc<SpotTradeBehaviorV2Impl>,
    queue: Arc<MPMCQueue>, // 使用具体类型而不是trait对象，因为Queue trait有泛型方法
}
impl ActorX for SpotMatchActor {
    fn start(self: &Arc<Self>) {
        tokio::spawn(async move {
            // 订阅变更日志事件
            let mut receiver = self.queue.subscribe(SpotTopic::EntityChangeLog.name(), None);

            // 持续监听事件
            while let Ok(event) = receiver.recv().await {
                //todo 只处理order_pending事件
                //todo
                //生成 trade,
                // 并发关 order_change/trade_create事件

                self.process_event(event).await;
            }
        });
    }
}

//结算
pub struct SpotSettlementActor {
    aggregator: Arc<SpotTradeBehaviorV2Impl>,
    queue: Arc<MPMCQueue>, // 使用具体类型而不是trait对象，因为Queue trait有泛型方法
}
impl ActorX for SpotSettlementActor {
    fn start(self: &Arc<Self>) {
        tokio::spawn(async move {
            // 订阅变更日志事件
            let mut receiver = self.queue.subscribe(SpotTopic::EntityChangeLog.name(), None);

            // 持续监听事件
            while let Ok(event) = receiver.recv().await {
                //todo 只处理order_created事件
                //todo
                //变更 balance,
                // 并发关 balance_change事件

                self.process_event(event).await;
            }
        });
    }
}

//K线聚合
pub struct SpotKLineAggActor {
    aggregator: Arc<SpotTradeBehaviorV2Impl>,
    queue: Arc<MPMCQueue>, // 使用具体类型而不是trait对象，因为Queue trait有泛型方法
}
impl ActorX for SpotKLineAggActor {
    fn start(self: &Arc<Self>) {
        tokio::spawn(async move {
            // 订阅变更日志事件
            let mut receiver = self.queue.subscribe(SpotTopic::EntityChangeLog.name(), None);

            // 持续监听事件
            while let Ok(event) = receiver.recv().await {
                //todo 只处理order_created事件
                //todo
                //变更 kline,
                // 并发送 kline_created

                self.process_event(event).await;
            }
        });
    }
}

//user_data/market_data推送
pub struct SpotKPushActor {
    aggregator: Arc<SpotTradeBehaviorV2Impl>,
    queue: Arc<MPMCQueue>, // 使用具体类型而不是trait对象，因为Queue trait有泛型方法
}
impl ActorX for SpotKPushActor {
    fn start(self: &Arc<Self>) {
        tokio::spawn(async move {
            // 订阅变更日志事件
            let mut receiver = self.queue.subscribe(SpotTopic::EntityChangeLog.name(), None);

            // 持续监听事件
            while let Ok(event) = receiver.recv().await {
                //todo 只处理kline_created/balance_update事件
                //todo
                //变更 balance,
                // 并发关 balance_change事件

                self.process_event(event).await;
            }
        });
    }
}
