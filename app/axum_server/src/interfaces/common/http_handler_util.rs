use std::fmt::Debug;
use std::sync::Arc;

use axum::extract::{Json, State};
use axum::response::IntoResponse;
use base_types::handler::handler::Handler;
use serde::Serialize;
use serde::de::DeserializeOwned;
use spot_behavior::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdErrorAny};

// ==================== 通用 JSON 响应创建 ====================

/// 泛型函数统一处理成功响应序列化
#[inline]
fn create_json_response<T: Serialize>(
    response: CmdResp<T>,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

// ==================== 通用 Handler 处理模板 ====================

/// 泛型 handler 模板函数 - 统一处理所有类型的请求
///
/// 类型参数:
/// - `S`: Service 类型,必须实现 `Handler<C, R, SpotCmdErrorAny>`
/// - `C`: Command 类型,必须可序列化和调试
/// - `R`: Response 类型,必须可序列化
#[inline]
pub async fn handle_generic<S, C, R>(
    State(service): State<Arc<S>>,
    Json(cmd): Json<C>,
) -> impl IntoResponse
where
    S: Handler<C, R, SpotCmdErrorAny>,
    C: Debug + DeserializeOwned,
    R: Serialize,
{
    println!("收到请求: {:?}", cmd);

    match service.handle(cmd).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(err),
    }
}

// ==================== 通用错误处理 ====================

/// 创建错误响应
fn create_error_response(
    error: SpotCmdErrorAny,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let json = serde_json::to_string(&error).unwrap();

    (
        axum::http::StatusCode::BAD_REQUEST,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        json,
    )
}
