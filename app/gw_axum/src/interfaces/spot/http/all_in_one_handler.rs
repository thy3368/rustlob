use std::sync::Arc;

use axum::{
    extract::{Json, State},
    response::IntoResponse
};
use base_types::handler::handler::Handler;
use serde::Serialize;
// Spot 市场数据相关导入
use spot_behavior::proc::behavior::v2::spot_market_data_behavior::{
    SpotMarketDataCmdAny, SpotMarketDataResAny
};
use spot_behavior::proc::{
    behavior::{
        spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
        v2::{
            spot_trade_behavior_v2::{SpotTradeCmdAny, SpotTradeResAny},
            spot_user_data_behavior::{SpotUserDataCmdAny, SpotUserDataResAny}
        }
    },
    trade_v2::{
        spot_market_data::SpotMarketDataImpl, spot_trade_v2::SpotTradeBehaviorV2Impl, spot_user_data::SpotUserDataImpl
    }
};


// ==================== 通用 JSON 响应创建 ====================

/// 泛型函数统一处理成功响应序列化
#[inline]
fn create_json_response<T: Serialize>(
    response: CmdResp<T>
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

//todo handle_market_data/handle_user_data/handle_trade_v2 可统一处理吗？
// ==================== 市场数据处理 ====================

pub async fn handle_market_data(
    State(service): State<Arc<SpotMarketDataImpl>>, Json(cmd): Json<SpotMarketDataCmdAny>
) -> impl IntoResponse {
    println!("📊 收到市场数据请求: {:?}", cmd);

    match service.handle(cmd).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(err)
    }
}

// ==================== 用户数据处理 ====================

pub async fn handle_user_data(
    State(service): State<Arc<SpotUserDataImpl>>, Json(cmd): Json<SpotUserDataCmdAny>
) -> impl IntoResponse {
    println!("👤 收到用户数据请求: {:?}", cmd);

    match service.handle(cmd).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(err)
    }
}

// ==================== 交易处理 ====================

pub async fn handle_trade_v2(
    State(service): State<Arc<SpotTradeBehaviorV2Impl>>, Json(cmd): Json<SpotTradeCmdAny>
) -> impl IntoResponse {
    println!("💹 收到交易请求: {:?}", cmd);

    match service.handle(cmd).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(err)
    }
}


// todo 增加user data listen key


// ==================== 通用错误处理 ====================

/// 创建错误响应
fn create_error_response(
    error: SpotCmdErrorAny
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&error).unwrap();

    (axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}
