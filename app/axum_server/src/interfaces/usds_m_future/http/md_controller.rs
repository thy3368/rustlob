use std::sync::{Arc, Mutex};

use axum::extract::{Json, State};
use axum::response::IntoResponse;
// USDS-M期货市场数据相关导入
use derivatives_behavior::proc::usds_m_future::behavior::market_data_behavior::{
    UsdsMFutureMarketDataBehavior, UsdsMFutureMarketDataCmdAny, UsdsMFutureMarketDataRes,
};
use derivatives_behavior::proc::usds_m_future::usds_market_data::UsdsMFutureMarketDataBehaviorImpl;
use serde::Serialize;
use spot_behavior::proc::behavior::spot_trade_behavior::CmdResp;

// ============================================================================
// 应用服务 - 封装市场数据处理器
// ============================================================================

/// 市场数据服务 - 封装市场数据处理器
pub struct MarketDataService {
    processor: Arc<Mutex<UsdsMFutureMarketDataBehaviorImpl>>,
}

impl MarketDataService {
    /// 创建新的市场数据服务实例
    pub fn new() -> Self {
        let processor = UsdsMFutureMarketDataBehaviorImpl {};
        Self { processor: Arc::new(Mutex::new(processor)) }
    }

    /// 处理市场数据请求 - 使用服务层
    pub async fn handle_all(
        &self,
        cmd: UsdsMFutureMarketDataCmdAny,
    ) -> Result<CmdResp<UsdsMFutureMarketDataRes>, String> {
        println!("📊 收到USDS-M期货市场数据请求: {:?}", cmd);

        self.processor
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?
            .handle(cmd)
            .map_err(|e| format!("{:?}", e))
    }
}

// ============================================================================
// USDS-M期货市场数据处理接口 - 使用应用服务层
// ============================================================================

/// 市场数据响应 DTO
#[derive(Debug, Serialize)]
pub struct MarketDataResponse {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub async fn handle(
    State(service): State<Arc<MarketDataService>>,
    Json(cmd): Json<UsdsMFutureMarketDataCmdAny>,
) -> impl IntoResponse {
    println!("📊 收到USDS-M期货市场数据请求: {:?}", cmd);

    match service.handle_all(cmd).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(&err),
    }
}

/// 创建 JSON 响应
fn create_json_response(
    response: CmdResp<UsdsMFutureMarketDataRes>,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

/// 创建错误响应
fn create_error_response(
    error_msg: &str,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let response = MarketDataResponse {
        success: false,
        message: "Request failed".to_string(),
        error: Some(error_msg.to_string()),
    };
    let json = serde_json::to_string(&response).unwrap();
    (
        axum::http::StatusCode::BAD_REQUEST,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        json,
    )
}
