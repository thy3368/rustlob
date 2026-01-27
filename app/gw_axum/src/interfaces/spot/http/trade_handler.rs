use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};

// Spot 订单处理相关导入
use spot_behavior::proc::behavior::spot_trade_behavior::{CmdResp, SpotTradeCmdAny, SpotTradeResAny, SpotTradeBehavior};
use spot_behavior::proc::trade::spot_trade::SpotTradeBehaviorImpl;

// 基础设施依赖
use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use db_repo::MySqlDbRepo;
use id_generator::generator::IdGenerator;
use lob_repo::adapter::standalone_lob_repo::StandaloneLobRepo;

// ============================================================================
// 应用服务 - 封装订单处理器
// ============================================================================

/// 应用服务 - 封装订单处理器
pub struct TradeService {
    //todo SpotTradeBehaviorImpl是无状态的，是不是可以不用mutex
    processor: Arc<Mutex<SpotTradeBehaviorImpl>>,
}

impl TradeService {
    /// 创建新的订单服务实例（使用 Mock 仓储）
    #[hotpath::measure]
    pub fn new() -> Self {
        // 1. 初始化各个仓储（使用 Mock 版本）
        let balance_repo = MySqlDbRepo::<Balance>::new_mock();
        let trade_repo = MySqlDbRepo::<SpotTrade>::new_mock();
        let order_repo = MySqlDbRepo::<SpotOrder>::new_mock();

        // 2. 初始化 LOB 仓储（内存版本，空的 LOB 列表）
        let lob_repo = StandaloneLobRepo::<SpotOrder>::new(vec![]);

        // 3. 初始化 ID 生成器（节点ID为0）
        let id_generator = IdGenerator::new(0);

        // 4. 创建处理器实例
        let processor = SpotTradeBehaviorImpl::new(balance_repo, trade_repo, order_repo, lob_repo, id_generator);

        Self { processor: Arc::new(Mutex::new(processor)) }
    }

    /// 处理限价单 - 使用服务层
    #[hotpath::measure]
    pub async fn handle_all(&self, cmd: SpotTradeCmdAny) -> Result<CmdResp<SpotTradeResAny>, String> {
        println!("📋 收到限价单请求: {:?}", cmd);

        // 调用真实的处理器，直接返回领域层结果
        self.processor
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?
            .handle(cmd)
            .map_err(|e| format!("{:?}", e))
    }
}

// ============================================================================
// Spot 订单处理接口 - 使用应用服务层
// ============================================================================

/// 订单响应 DTO
#[derive(Debug, Serialize)]
pub struct OrderResponse {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[hotpath::measure]
pub async fn handle(State(service): State<Arc<TradeService>>, Json(cmd): Json<SpotTradeCmdAny>) -> impl IntoResponse {
    println!("📋 收到限价单请求: {:?}", cmd);

    match service.handle_all(cmd).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(&err),
    }
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
fn create_error_response(
    error_msg: &str,
) -> (axum::http::StatusCode, [(axum::http::header::HeaderName, &'static str); 1], String) {
    let response = OrderResponse {
        success: false,
        message: "Request failed".to_string(),
        order_id: None,
        error: Some(error_msg.to_string()),
    };
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}
