use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// USDS-M期货交易相关导入
use derivatives_behavior::proc::usds_m_future::behavior::trade_behavior::{
    UsdsMFutureTradeBehavior, UsdsMFutureTradeCmdAny, UsdsMFutureTradeRes,
};
use derivatives_behavior::proc::usds_m_future::usds_trade::UsdsMFutureTradeBehaviorImpl;
use derivatives_behavior::proc::usds_m_future::behavior::trade_behavior::UsdsMFutureTradeCmdError;
use spot_behavior::proc::behavior::spot_trade_behavior::CmdResp;

// ============================================================================
// 应用服务 - 封装交易处理器
// ============================================================================

/// 交易服务 - 封装交易处理器
pub struct TradeService {
    processor: Arc<Mutex<UsdsMFutureTradeBehaviorImpl>>,
}

impl TradeService {
    /// 创建新的交易服务实例
    pub fn new() -> Self {
        let processor = UsdsMFutureTradeBehaviorImpl {};
        Self { processor: Arc::new(Mutex::new(processor)) }
    }

    /// 处理交易请求 - 使用服务层
    pub async fn handle_all(&self, cmd: UsdsMFutureTradeCmdAny) -> Result<CmdResp<UsdsMFutureTradeRes>, String> {
        println!("📋 收到USDS-M期货交易请求: {:?}", cmd);

        self.processor
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?
            .handle(cmd)
            .map_err(|e| format!("{:?}", e))
    }
}

// ============================================================================
// USDS-M期货交易处理接口 - 使用应用服务层
// ============================================================================

/// 交易响应 DTO
#[derive(Debug, Serialize)]
pub struct TradeResponse {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub async fn handle(State(service): State<Arc<TradeService>>, Json(cmd): Json<UsdsMFutureTradeCmdAny>) -> impl IntoResponse {
    println!("📋 收到USDS-M期货交易请求: {:?}", cmd);

    match service.handle_all(cmd).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(&err),
    }
}

/// 创建 JSON 响应
fn create_json_response(
    response: CmdResp<UsdsMFutureTradeRes>,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

/// 创建错误响应
fn create_error_response(
    error_msg: &str,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let response = TradeResponse {
        success: false,
        message: "Request failed".to_string(),
        order_id: None,
        error: Some(error_msg.to_string()),
    };
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}
