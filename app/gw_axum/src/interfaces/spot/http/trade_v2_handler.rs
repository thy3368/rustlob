use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Spot 交易相关导入
use spot_behavior::proc::behavior::v2::spot_trade_behavior_v2::{
    SpotTradeBehaviorV2, SpotTradeCmdAny, SpotTradeResAny,
};
use spot_behavior::proc::trade_v2::spot_trade_v2::SpotTradeBehaviorV2Impl;
use spot_behavior::proc::behavior::spot_trade_behavior::CmdResp;



// ============================================================================
// Spot 交易处理接口 - 使用应用服务层
// ============================================================================

/// 交易响应 DTO
#[derive(Debug, Serialize)]
pub struct TradeV2Response {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[hotpath::measure]
pub async fn handle(State(mut service): State<Arc<SpotTradeBehaviorV2Impl>>, Json(cmd): Json<SpotTradeCmdAny>) -> impl IntoResponse {
    println!("📋 收到交易请求: {:?}", cmd);


    //todo 调用SpotTradeBehaviorV2Impl处理

    match service.handle(cmd) {
            Ok(response) => create_json_response(response),
            Err(err) => create_error_response(&err),
    }


    // match service.handle(cmd).await {
    //     Ok(response) => create_json_response(response),
    //     Err(err) => create_error_response(&err),
    // }
}

/// 创建 JSON 响应
#[hotpath::measure]
fn create_json_response(
    response: CmdResp<SpotTradeResAny>,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

/// 创建错误响应
#[hotpath::measure]
//todo 入参改为 SpotCmdErrorAny
fn create_error_response(
    error_msg: &str,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let response = TradeV2Response {
        success: false,
        message: "Request failed".to_string(),
        error: Some(error_msg.to_string()),
    };
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}