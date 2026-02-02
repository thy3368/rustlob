use std::sync::Arc;

use base_types::{
    account::balance::Balance,
    exchange::spot::spot_types::{SpotOrder, SpotTrade}
};
use db_repo::{adapter::change_log_queue_repo::ChangeLogChannelQueueRepo, MySqlDbRepo};
use lob_repo::adapter::{distributed_lob_repo::DistributedLobRepo, embedded_lob_repo::EmbeddedLobRepo};
use once_cell::sync::Lazy;
use push::push::{
    connection_types::ConnectionRepo, push_service::PushService, subscription_service::SubscriptionService
};
use spot_behavior::proc::v2::{
    spot_market_data::SpotMarketDataImpl, spot_trade_v2::SpotTradeBehaviorV2Impl, spot_user_data::SpotUserDataImpl,
    spot_user_data_key::SpotUserDataListenKeyImpl
};
use crate::interfaces::spot::websocket::{
    md_sse_controller::SpotMarketDataSSEImpl, ud_sse_controller::SpotUserDataSSEImpl
};

// todo service/adapter/repo的实例化都可以在这，他们都是单例的


// 数据库仓储单例（mock版本）
static BALANCE_REPO: Lazy<MySqlDbRepo<Balance>> = Lazy::new(MySqlDbRepo::new_mock);
static TRADE_REPO: Lazy<MySqlDbRepo<SpotTrade>> = Lazy::new(MySqlDbRepo::new_mock);
static ORDER_REPO: Lazy<MySqlDbRepo<SpotOrder>> = Lazy::new(MySqlDbRepo::new_mock);
static USER_DATA_REPO: Lazy<MySqlDbRepo<SpotOrder>> = Lazy::new(MySqlDbRepo::new_mock);
static MARKET_DATA_REPO: Lazy<MySqlDbRepo<SpotOrder>> = Lazy::new(MySqlDbRepo::new_mock);

// LOB Repo 单例
static EMBEDDED_LOB_REPO: Lazy<EmbeddedLobRepo<SpotOrder>> = Lazy::new(|| EmbeddedLobRepo::new(vec![]));
static DISTRIBUTED_LOB_REPO: Lazy<DistributedLobRepo<SpotOrder>> = Lazy::new(|| DistributedLobRepo::new(vec![]));

// 核心服务单例
static SPOT_TRADE_BEHAVIOR_V2_EMBEDDED: Lazy<SpotTradeBehaviorV2Impl<EmbeddedLobRepo<SpotOrder>>> = Lazy::new(|| {
    SpotTradeBehaviorV2Impl::new(
        BALANCE_REPO.clone(),
        TRADE_REPO.clone(),
        ORDER_REPO.clone(),
        USER_DATA_REPO.clone(),
        MARKET_DATA_REPO.clone(),
        EMBEDDED_LOB_REPO.clone()
    )
});

static SPOT_TRADE_BEHAVIOR_V2_DISTRIBUTED: Lazy<SpotTradeBehaviorV2Impl<DistributedLobRepo<SpotOrder>>> =
    Lazy::new(|| {
        SpotTradeBehaviorV2Impl::new(
            BALANCE_REPO.clone(),
            TRADE_REPO.clone(),
            ORDER_REPO.clone(),
            USER_DATA_REPO.clone(),
            MARKET_DATA_REPO.clone(),
            DISTRIBUTED_LOB_REPO.clone()
        )
    });

static SPOT_MARKET_DATA_SERVICE: Lazy<SpotMarketDataImpl> = Lazy::new(SpotMarketDataImpl::new);
static SPOT_USER_DATA_SERVICE: Lazy<SpotUserDataImpl> = Lazy::new(SpotUserDataImpl::new);
static SPOT_USER_DATA_LISTEN_KEY_SERVICE: Lazy<SpotUserDataListenKeyImpl> = Lazy::new(SpotUserDataListenKeyImpl::new);

// WebSocket 相关服务单例
static CONNECTION_REPO: Lazy<Arc<ConnectionRepo>> = Lazy::new(|| Arc::new(ConnectionRepo::default()));
static CHANGE_LOG_REPO: Lazy<Arc<ChangeLogChannelQueueRepo>> = Lazy::new(|| Arc::new(ChangeLogChannelQueueRepo::new()));
static PUSH_SERVICE: Lazy<Arc<PushService>> =
    Lazy::new(|| Arc::new(PushService::new(CONNECTION_REPO.clone(), CHANGE_LOG_REPO.clone())));
static SUBSCRIPTION_SERVICE: Lazy<Arc<SubscriptionService>> =
    Lazy::new(|| Arc::new(SubscriptionService::new(CONNECTION_REPO.clone())));
static SPOT_MARKET_DATA_SSE_IMPL: Lazy<Arc<SpotMarketDataSSEImpl>> =
    Lazy::new(|| Arc::new(SpotMarketDataSSEImpl::new()));
static SPOT_USER_DATA_SSE_IMPL: Lazy<Arc<SpotUserDataSSEImpl>> =
    Lazy::new(|| Arc::new(SpotUserDataSSEImpl::new()));

// Arc 包装的单例
static SPOT_TRADE_BEHAVIOR_V2_EMBEDDED_ARC: Lazy<Arc<SpotTradeBehaviorV2Impl<EmbeddedLobRepo<SpotOrder>>>> =
    Lazy::new(|| Arc::new(SPOT_TRADE_BEHAVIOR_V2_EMBEDDED.clone()));
static SPOT_TRADE_BEHAVIOR_V2_DISTRIBUTED_ARC: Lazy<Arc<SpotTradeBehaviorV2Impl<DistributedLobRepo<SpotOrder>>>> =
    Lazy::new(|| Arc::new(SPOT_TRADE_BEHAVIOR_V2_DISTRIBUTED.clone()));
static SPOT_MARKET_DATA_SERVICE_ARC: Lazy<Arc<SpotMarketDataImpl>> =
    Lazy::new(|| Arc::new(SPOT_MARKET_DATA_SERVICE.clone()));
static SPOT_USER_DATA_SERVICE_ARC: Lazy<Arc<SpotUserDataImpl>> = Lazy::new(|| Arc::new(SPOT_USER_DATA_SERVICE.clone()));
static SPOT_USER_DATA_LISTEN_KEY_SERVICE_ARC: Lazy<Arc<SpotUserDataListenKeyImpl>> =
    Lazy::new(|| Arc::new(SPOT_USER_DATA_LISTEN_KEY_SERVICE.clone()));


pub fn get_spot_trade_behavior_v2_embedded() -> Arc<SpotTradeBehaviorV2Impl<EmbeddedLobRepo<SpotOrder>>> {
    SPOT_TRADE_BEHAVIOR_V2_EMBEDDED_ARC.clone()
}

pub fn get_spot_trade_behavior_v2_distributed() -> Arc<SpotTradeBehaviorV2Impl<DistributedLobRepo<SpotOrder>>> {
    SPOT_TRADE_BEHAVIOR_V2_DISTRIBUTED_ARC.clone()
}

pub fn get_spot_market_data_service() -> Arc<SpotMarketDataImpl> { SPOT_MARKET_DATA_SERVICE_ARC.clone() }

pub fn get_spot_user_data_service() -> Arc<SpotUserDataImpl> { SPOT_USER_DATA_SERVICE_ARC.clone() }

pub fn get_spot_user_data_listen_key_service() -> Arc<SpotUserDataListenKeyImpl> {
    SPOT_USER_DATA_LISTEN_KEY_SERVICE_ARC.clone()
}

// WebSocket 相关服务访问方法
pub fn get_connection_repo() -> Arc<ConnectionRepo> { CONNECTION_REPO.clone() }

pub fn get_change_log_repo() -> Arc<ChangeLogChannelQueueRepo> { CHANGE_LOG_REPO.clone() }

pub fn get_push_service() -> Arc<PushService> { PUSH_SERVICE.clone() }

pub fn get_subscription_service() -> Arc<SubscriptionService> { SUBSCRIPTION_SERVICE.clone() }

pub fn get_spot_market_data_sse_impl() -> Arc<SpotMarketDataSSEImpl> { SPOT_MARKET_DATA_SSE_IMPL.clone() }

pub fn get_spot_user_data_sse_impl() -> Arc<SpotUserDataSSEImpl> { SPOT_USER_DATA_SSE_IMPL.clone() }

#[test]
fn abc() {
    println!("✅ unique IDs");
}
