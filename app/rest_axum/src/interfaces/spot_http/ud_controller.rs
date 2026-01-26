use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Spot 用户数据相关导入
use spot_behavior::proc::behavior::v2::spot_user_data_behavior::{
    SpotUserDataBehavior, SpotUserDataCmdAny, SpotUserDataRes,
};
use spot_behavior::proc::trade_v2::spot_user_data::SpotUserDataImpl;
use spot_behavior::proc::behavior::spot_trade_behavior::CmdResp;

// ============================================================================
// 应用服务 - 封装用户数据处理器
// ============================================================================

/// 用户数据服务 - 封装用户数据处理器
pub struct UserDataService {
    processor: Arc<Mutex<SpotUserDataImpl>>,
}

impl UserDataService {
    /// 创建新的用户数据服务实例
    #[hotpath::measure]
    pub fn new() -> Self {
        let processor = SpotUserDataImpl {};
        Self { processor: Arc::new(Mutex::new(processor)) }
    }

    /// 处理用户数据请求 - 使用服务层
    #[hotpath::measure]
    pub async fn handle_all(&self, cmd: SpotUserDataCmdAny) -> Result<CmdResp<SpotUserDataRes>, String> {
        println!("👤 收到用户数据请求: {:?}", cmd);

        self.processor
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?
            .handle(cmd)
            .map_err(|e| format!("{:?}", e))
    }
}

// ============================================================================
// Spot 用户数据处理接口 - 使用应用服务层
// ============================================================================

/// 用户数据响应 DTO
#[derive(Debug, Serialize)]
pub struct UserDataResponse {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[hotpath::measure]
pub async fn handle(State(service): State<Arc<UserDataService>>, Json(cmd): Json<SpotUserDataCmdAny>) -> impl IntoResponse {
    println!("👤 收到用户数据请求: {:?}", cmd);

    match service.handle_all(cmd).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(&err),
    }
}

/// 创建 JSON 响应
#[hotpath::measure]
fn create_json_response(
    response: CmdResp<SpotUserDataRes>,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

/// 创建错误响应
#[hotpath::measure]
fn create_error_response(
    error_msg: &str,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let response = UserDataResponse {
        success: false,
        message: "Request failed".to_string(),
        error: Some(error_msg.to_string()),
    };
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}