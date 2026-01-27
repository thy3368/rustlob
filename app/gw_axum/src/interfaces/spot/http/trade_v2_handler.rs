use std::sync::Arc;

use axum::{
    extract::{Json, State},
    response::IntoResponse
};
use serde::Serialize;
// Spot 交易相关导入
use spot_behavior::proc::behavior::v2::spot_trade_behavior_v2::{
    SpotTradeBehaviorV2, SpotTradeCmdAny, SpotTradeResAny
};
use spot_behavior::proc::{
    behavior::spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    trade_v2::spot_trade_v2::SpotTradeBehaviorV2Impl
};


pub async fn handle(
    State(service): State<Arc<SpotTradeBehaviorV2Impl>>,
    Json(cmd): Json<SpotTradeCmdAny>
) -> impl IntoResponse {
    tracing::info!("📋 收到交易请求: {:?}", cmd);

    match service.handle(cmd).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(err)
    }
}

/// 创建 JSON 响应
fn create_json_response(
    response: CmdResp<SpotTradeResAny>
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

/// 创建错误响应
fn create_error_response(
    error: SpotCmdErrorAny
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&error).unwrap();

    (axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}
