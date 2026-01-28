use std::sync::Arc;

use axum::{
    extract::{Json, State},
    response::IntoResponse
};
use base_types::handler::handler::Handler;
// Spot 市场数据相关导入
use spot_behavior::proc::behavior::v2::spot_market_data_behavior::{
    SpotMarketDataBehavior, SpotMarketDataCmdAny, SpotMarketDataResAny
};
use spot_behavior::proc::behavior::v2::spot_user_data_behavior::{
    SpotUserDataBehavior, SpotUserDataCmdAny, SpotUserDataResAny
};
use spot_behavior::proc::behavior::v2::spot_trade_behavior_v2::{
    SpotTradeBehaviorV2, SpotTradeCmdAny, SpotTradeResAny
};
use spot_behavior::proc::{
    behavior::spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    trade_v2::spot_market_data::SpotMarketDataImpl
};
use spot_behavior::proc::trade_v2::spot_user_data::SpotUserDataImpl;
use spot_behavior::proc::trade_v2::spot_trade_v2::SpotTradeBehaviorV2Impl;


// ==================== 市场数据处理 ====================

pub async fn handle_market_data(
    State(service): State<Arc<SpotMarketDataImpl>>,
    Json(cmd): Json<SpotMarketDataCmdAny>
) -> impl IntoResponse {
    println!("📊 收到市场数据请求: {:?}", cmd);

    match service.handle(cmd).await {
        Ok(response) => create_json_response_market_data(response),
        Err(err) => create_error_response(err)
    }
}

fn create_json_response_market_data(
    response: CmdResp<SpotMarketDataResAny>
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

// ==================== 用户数据处理 ====================

pub async fn handle_user_data(
    State(service): State<Arc<SpotUserDataImpl>>,
    Json(cmd): Json<SpotUserDataCmdAny>
) -> impl IntoResponse {
    println!("👤 收到用户数据请求: {:?}", cmd);

    match service.handle(cmd).await {
        Ok(response) => create_json_response_user_data(response),
        Err(err) => create_error_response(err)
    }
}

fn create_json_response_user_data(
    response: CmdResp<SpotUserDataResAny>
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

// ==================== 交易处理 ====================

pub async fn handle_trade_v2(
    State(service): State<Arc<SpotTradeBehaviorV2Impl>>,
    Json(cmd): Json<SpotTradeCmdAny>
) -> impl IntoResponse {
    println!("💹 收到交易请求: {:?}", cmd);

    match service.handle(cmd).await {
        Ok(response) => create_json_response_trade(response),
        Err(err) => create_error_response(err)
    }
}

fn create_json_response_trade(
    response: CmdResp<SpotTradeResAny>
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}


//todo 增加user data listen key



// ==================== 通用错误处理 ====================

/// 创建错误响应
fn create_error_response(
    error: SpotCmdErrorAny
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&error).unwrap();

    (axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}
