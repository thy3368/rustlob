pub mod md_gw;
pub mod trade_gw;
pub mod ud_gw;

use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing_subscriber;

// Spot 订单处理相关导入
use spot_behavior::proc::behavior::spot_trade_behavior::{
    CancelOrder, CmdResp, LimitOrder, MarketOrder, SpotCmdAny, SpotCmdRes, SpotTradeBehavior,
};
use spot_behavior::proc::trade::spot_trade::SpotTradeBehaviorImpl;

// 基础设施依赖
use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use db_repo::{CmdRepo, MySqlDbRepo};
use id_generator::generator::IdGenerator;
use lob_repo::adapter::standalone_lob_repo::StandaloneLobRepo;



/// 应用服务 - 封装订单处理器
pub struct OrderService {
    //todo SpotTradeBehaviorImpl是无状态的，是不是可以不用mutex
    processor: Arc<Mutex<SpotTradeBehaviorImpl>>,
}

impl OrderService {
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

    /// 处理限价单命令
    #[hotpath::measure]
    pub async fn handle_limit_order(&self, limit_order: LimitOrder) -> Result<CmdResp<SpotCmdRes>, String> {
        println!("🔑 命令ID: {}", limit_order.metadata.command_id);
        println!("⏰ 时间戳: {}", limit_order.metadata.timestamp);

        let spot_cmd = SpotCmdAny::LimitOrder(limit_order);

        // 调用真实的处理器，直接返回领域层结果
        self.processor
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?
            .handle(spot_cmd)
            .map_err(|e| format!("{:?}", e))
    }

    /// 处理市价单命令
    #[hotpath::measure]

    pub async fn handle_market_order(&self, market_order: MarketOrder) -> Result<CmdResp<SpotCmdRes>, String> {
        println!("🔑 命令ID: {}", market_order.metadata.command_id);

        let spot_cmd = SpotCmdAny::MarketOrder(market_order);

        self.processor
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?
            .handle(spot_cmd)
            .map_err(|e| format!("{:?}", e))
    }

    /// 处理取消订单命令
    pub async fn handle_cancel_order(&self, cancel_order: CancelOrder) -> Result<CmdResp<SpotCmdRes>, String> {
        println!("🔑 命令ID: {}", cancel_order.metadata.command_id);

        let spot_cmd = SpotCmdAny::CancelOrder(cancel_order);

        self.processor
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?
            .handle(spot_cmd)
            .map_err(|e| format!("{:?}", e))
    }
}

#[tokio::main]
#[hotpath::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("🚀 Starting REST API server...");
    println!("⚠️  Running in MOCK mode (no database connection)");

    // 从环境变量读取数据库配置
    // let db_url =
    //     std::env::var("DATABASE_URL").unwrap_or_else(|_| "mysql://root:password@localhost:3306/trading_db".to_string());
    //
    // println!("📊 Connecting to database: {}", db_url);

    // 创建应用服务（单例，全局共享）
    let order_service = Arc::new(OrderService::new());

    // 创建路由，注入服务依赖
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/spot/order/limit", post(handle_limit_order))
        .route("/api/spot/order/market", post(handle_market_order))
        .route("/api/spot/order/cancel", post(handle_cancel_order))
        .with_state(order_service);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind port");

    println!("🚀 Server started at http://localhost:3000");
    println!("📊 Health check: GET /health");
    println!("💹 Spot Limit Order: POST /api/spot/order/limit (JSON)");
    println!("💹 Spot Market Order: POST /api/spot/order/market (JSON)");
    println!("💹 Spot Cancel Order: POST /api/spot/order/cancel (JSON)");

    axum::serve(listener, app).await.expect("Server failed to start");
}

async fn health_check() -> &'static str {
    "OK"
}

// ============================================================================
// Spot 订单处理接口 - 使用应用服务层
// ============================================================================

/// 订单响应 DTO
#[derive(Debug, Serialize)]
struct OrderResponse {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// 处理限价单 - 使用服务层
#[hotpath::measure]
async fn handle_limit_order(
    State(service): State<Arc<OrderService>>, Json(limit_order): Json<LimitOrder>,
) -> impl IntoResponse {
    println!("📋 收到限价单请求: {:?}", limit_order);

    match service.handle_limit_order(limit_order).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(&err),
    }
}

/// 处理市价单 - 使用服务层
#[hotpath::measure]

async fn handle_market_order(
    State(service): State<Arc<OrderService>>, Json(market_order): Json<MarketOrder>,
) -> impl IntoResponse {
    println!("📋 收到市价单请求: {:?}", market_order);

    match service.handle_market_order(market_order).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(&err),
    }
}

/// 处理取消订单 - 使用服务层
#[hotpath::measure]

async fn handle_cancel_order(
    State(service): State<Arc<OrderService>>, Json(cancel_order): Json<CancelOrder>,
) -> impl IntoResponse {
    println!("📋 收到取消订单请求: {:?}", cancel_order);

    match service.handle_cancel_order(cancel_order).await {
        Ok(response) => create_json_response(response),
        Err(err) => create_error_response(&err),
    }
}

/// 创建 JSON 响应
#[hotpath::measure]

fn create_json_response(
    response: CmdResp<SpotCmdRes>,
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
