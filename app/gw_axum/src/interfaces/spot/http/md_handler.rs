use std::sync::Arc;

use axum::{
    extract::{Json, State},
    response::IntoResponse
};
// Spot 市场数据相关导入
use spot_behavior::proc::behavior::v2::spot_market_data_behavior::{
    SpotMarketDataBehavior, SpotMarketDataCmdAny, SpotMarketDataRes
};
use spot_behavior::proc::{
    behavior::spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    trade_v2::spot_market_data::SpotMarketDataImpl
};


pub async fn handle(
    State(service): State<Arc<SpotMarketDataImpl>>, Json(cmd): Json<SpotMarketDataCmdAny>
) -> impl IntoResponse {
    println!("📊 收到市场数据请求: {:?}", cmd);

    match service.handle(cmd) {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(err)
    }
}

/// 创建 JSON 响应
fn create_json_response(
    response: CmdResp<SpotMarketDataRes>
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
