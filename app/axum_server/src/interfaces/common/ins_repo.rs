use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::spot_topic::SpotTopic;
use db_repo::MySqlDbRepo;
use lob_repo::adapter::distributed_lob_repo::DistributedLobRepo;
use lob_repo::adapter::embedded_lob_repo::EmbeddedLobRepo;
use once_cell::sync::Lazy;
// KLine 相关服务单例
use push::k_line::{
    aggregator::m100_simd_k_line_aggregator::M100SimdKLineAggregator, k_line_service::KLineService,
};
use push::push::connection_types::ConnectionRepo;
use push::push::push_service::PushService;
use push::push::subscription_service::SubscriptionService;
use rust_queue::queue::queue::Queue;
use rust_queue::queue::queue_impl::kafka_queue::KafkaQueue;
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;
use spot_behavior::proc::v2::spot_market_data::SpotMarketDataImpl;
use spot_behavior::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;
use spot_behavior::proc::v2::spot_user_data::SpotUserDataImpl;
use spot_behavior::proc::v2::spot_user_data_key::SpotUserDataListenKeyImpl;

use crate::interfaces::spot::websocket::md_sse_controller::SpotMarketDataSSEImpl;
use crate::interfaces::spot::websocket::ud_sse_controller::SpotUserDataSSEImpl;

//todo add  mutex
static M100_SIMD_K_LINE_AGGREGATOR: Lazy<Arc<M100SimdKLineAggregator>> =
    Lazy::new(|| Arc::new(M100SimdKLineAggregator::new()));

static K_LINE_SERVICE: Lazy<Arc<KLineService>> =
    Lazy::new(|| Arc::new(KLineService::new(MPMC_QUEUE.clone())));

// KLine 相关服务访问方法
pub fn get_m100_simd_k_line_aggregator() -> Arc<M100SimdKLineAggregator> {
    M100_SIMD_K_LINE_AGGREGATOR.clone()
}

pub fn get_k_line_service() -> Arc<KLineService> {
    K_LINE_SERVICE.clone()
}

//todo  Lazy<MySqlDbRepo<Balance>> 变成 Lazy<Arc<MySqlDbRepo<Balance>>>
static BALANCE_REPO: Lazy<Arc<MySqlDbRepo<Balance>>> =
    Lazy::new(|| Arc::new(MySqlDbRepo::new_mock()));
static TRADE_REPO: Lazy<Arc<MySqlDbRepo<SpotTrade>>> =
    Lazy::new(|| Arc::new(MySqlDbRepo::new_mock()));
static ORDER_REPO: Lazy<Arc<MySqlDbRepo<SpotOrder>>> =
    Lazy::new(|| Arc::new(MySqlDbRepo::new_mock()));
static USER_DATA_REPO: Lazy<Arc<MySqlDbRepo<SpotOrder>>> =
    Lazy::new(|| Arc::new(MySqlDbRepo::new_mock()));
static MARKET_DATA_REPO: Lazy<Arc<MySqlDbRepo<SpotOrder>>> =
    Lazy::new(|| Arc::new(MySqlDbRepo::new_mock()));

// LOB Repo 单例（包装在 Arc 中）
static EMBEDDED_LOB_REPO: Lazy<Arc<EmbeddedLobRepo<SpotOrder>>> =
    Lazy::new(|| Arc::new(EmbeddedLobRepo::new(vec![])));
static DISTRIBUTED_LOB_REPO: Lazy<Arc<DistributedLobRepo<SpotOrder>>> =
    Lazy::new(|| Arc::new(DistributedLobRepo::new(vec![])));

// 队列服务单例（包装在 Arc 中）
static MPMC_QUEUE: Lazy<Arc<MPMCQueue>> = Lazy::new(|| {
    let queue = MPMCQueue::new();
    queue.get_or_create_channel(SpotTopic::KLine.name());
    queue.get_or_create_channel(SpotTopic::EntityChangeLog.name());
    Arc::new(queue)
});

static KAFKA_QUEUE: Lazy<Arc<KafkaQueue>> = Lazy::new(|| {
    let queue = KafkaQueue::new();
    queue.get_or_create_channel(SpotTopic::KLine.name());
    queue.get_or_create_channel(SpotTopic::EntityChangeLog.name());
    Arc::new(queue)
});

// 核心服务单例（直接包装在 Arc 中）
static SPOT_TRADE_BEHAVIOR_V2_EMBEDDED: Lazy<
    Arc<SpotTradeBehaviorV2Impl<Arc<EmbeddedLobRepo<SpotOrder>>>>,
> = Lazy::new(|| {
    Arc::new(SpotTradeBehaviorV2Impl::new(
        BALANCE_REPO.clone(),
        TRADE_REPO.clone(),
        ORDER_REPO.clone(),
        USER_DATA_REPO.clone(),
        MARKET_DATA_REPO.clone(),
        EMBEDDED_LOB_REPO.clone(),
        MPMC_QUEUE.clone(),
    ))
});

static SPOT_TRADE_BEHAVIOR_V2_DISTRIBUTED: Lazy<
    Arc<SpotTradeBehaviorV2Impl<Arc<DistributedLobRepo<SpotOrder>>>>,
> = Lazy::new(|| {
    Arc::new(SpotTradeBehaviorV2Impl::new(
        BALANCE_REPO.clone(),
        TRADE_REPO.clone(),
        ORDER_REPO.clone(),
        USER_DATA_REPO.clone(),
        MARKET_DATA_REPO.clone(),
        DISTRIBUTED_LOB_REPO.clone(),
        MPMC_QUEUE.clone(), // 使用 MPMC 队列而不是 Kafka 队列
    ))
});

static SPOT_MARKET_DATA_SERVICE: Lazy<Arc<SpotMarketDataImpl>> =
    Lazy::new(|| Arc::new(SpotMarketDataImpl::new()));
static SPOT_USER_DATA_SERVICE: Lazy<Arc<SpotUserDataImpl>> =
    Lazy::new(|| Arc::new(SpotUserDataImpl::new()));
static SPOT_USER_DATA_LISTEN_KEY_SERVICE: Lazy<Arc<SpotUserDataListenKeyImpl>> =
    Lazy::new(|| Arc::new(SpotUserDataListenKeyImpl::new()));

// WebSocket 相关服务单例
static CONNECTION_REPO: Lazy<Arc<ConnectionRepo>> =
    Lazy::new(|| Arc::new(ConnectionRepo::default()));
static PUSH_SERVICE: Lazy<Arc<PushService>> =
    Lazy::new(|| Arc::new(PushService::new(CONNECTION_REPO.clone(), MPMC_QUEUE.clone())));
static SUBSCRIPTION_SERVICE: Lazy<Arc<SubscriptionService>> =
    Lazy::new(|| Arc::new(SubscriptionService::new(CONNECTION_REPO.clone())));
static SPOT_MARKET_DATA_SSE_IMPL: Lazy<Arc<SpotMarketDataSSEImpl>> =
    Lazy::new(|| Arc::new(SpotMarketDataSSEImpl::new()));
static SPOT_USER_DATA_SSE_IMPL: Lazy<Arc<SpotUserDataSSEImpl>> =
    Lazy::new(|| Arc::new(SpotUserDataSSEImpl::new()));

pub fn get_spot_trade_behavior_v2_embedded()
-> Arc<SpotTradeBehaviorV2Impl<Arc<EmbeddedLobRepo<SpotOrder>>>> {
    SPOT_TRADE_BEHAVIOR_V2_EMBEDDED.clone()
}

pub fn get_spot_trade_behavior_v2_distributed()
-> Arc<SpotTradeBehaviorV2Impl<Arc<DistributedLobRepo<SpotOrder>>>> {
    SPOT_TRADE_BEHAVIOR_V2_DISTRIBUTED.clone()
}

pub fn get_spot_market_data_service() -> Arc<SpotMarketDataImpl> {
    SPOT_MARKET_DATA_SERVICE.clone()
}

pub fn get_spot_user_data_service() -> Arc<SpotUserDataImpl> {
    SPOT_USER_DATA_SERVICE.clone()
}

pub fn get_spot_user_data_listen_key_service() -> Arc<SpotUserDataListenKeyImpl> {
    SPOT_USER_DATA_LISTEN_KEY_SERVICE.clone()
}

// WebSocket 相关服务访问方法
pub fn get_connection_repo() -> Arc<ConnectionRepo> {
    CONNECTION_REPO.clone()
}

pub fn get_push_service() -> Arc<PushService> {
    PUSH_SERVICE.clone()
}

pub fn get_subscription_service() -> Arc<SubscriptionService> {
    SUBSCRIPTION_SERVICE.clone()
}

pub fn get_spot_market_data_sse_impl() -> Arc<SpotMarketDataSSEImpl> {
    SPOT_MARKET_DATA_SSE_IMPL.clone()
}

pub fn get_spot_user_data_sse_impl() -> Arc<SpotUserDataSSEImpl> {
    SPOT_USER_DATA_SSE_IMPL.clone()
}

// 队列服务访问方法
pub fn get_mpmc_queue() -> Arc<MPMCQueue> {
    MPMC_QUEUE.clone()
}

pub fn get_kafka_queue() -> Arc<KafkaQueue> {
    KAFKA_QUEUE.clone()
}

// MySqlDbRepo 访问方法
pub fn get_balance_repo() -> Arc<MySqlDbRepo<Balance>> {
    BALANCE_REPO.clone()
}

pub fn get_trade_repo() -> Arc<MySqlDbRepo<SpotTrade>> {
    TRADE_REPO.clone()
}

pub fn get_order_repo() -> Arc<MySqlDbRepo<SpotOrder>> {
    ORDER_REPO.clone()
}

pub fn get_user_data_repo() -> Arc<MySqlDbRepo<SpotOrder>> {
    USER_DATA_REPO.clone()
}

pub fn get_market_data_repo() -> Arc<MySqlDbRepo<SpotOrder>> {
    MARKET_DATA_REPO.clone()
}
