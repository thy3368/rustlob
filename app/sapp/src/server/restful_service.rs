/// Axum HTTP 订单匹配服务
///
/// 提供RESTful API接口，遵循Clean Architecture原则
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use lob::lob::domain::service::matching_service::MatchingService as LobMatchingService;
use lob::lob::domain::::{
    in_memory::InMemoryOrderRepository,
    traits::OrderRepository,
};
use lob::lob::domain::entity::lob_types::{Side, TraderId};
use lob::lob::domain::service::handler::{Command, CommandResult, OrderCommandHandler};
// ============ DTOs ============

/// 下单请求
#[derive(Debug, Deserialize)]
pub struct PlaceOrderRequest {
    pub trader_id: String,
    pub side: String,      // "buy" 或 "sell"
    pub price: u32,
    pub quantity: u32,
}

/// 下单响应
#[derive(Debug, Serialize)]
pub struct PlaceOrderResponse {
    pub order_id: u64,
    pub status: String,
    pub trades: Vec<TradeDto>,
}

/// 交易DTO
#[derive(Debug, Serialize)]
pub struct TradeDto {
    pub buyer: String,
    pub seller: String,
    pub price: u32,
    pub quantity: u32,
}

/// 取消订单请求
#[derive(Debug, Deserialize)]
pub struct CancelOrderRequest {
    pub order_id: u64,
}

/// 取消订单响应
#[derive(Debug, Serialize)]
pub struct CancelOrderResponse {
    pub success: bool,
    pub message: String,
}

/// 错误响应
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// ============ 应用状态 ============

pub struct AppState {
    matching_service: RwLock<LobMatchingService<InMemoryOrderRepository>>,
}

impl AppState {
    pub fn new() -> Self {
        let repository = InMemoryOrderRepository::new(100_000, 1_000_000);
        let matching_service = LobMatchingService::new(repository);

        Self {
            matching_service: RwLock::new(matching_service),
        }
    }
}

// ============ HTTP Handlers ============

/// 健康检查
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "matching-service"
    }))
}

/// 下限价单
async fn place_limit_order(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PlaceOrderRequest>,
) -> Result<Json<PlaceOrderResponse>, AppError> {
    info!("收到下单请求: {:?}", req);

    // 验证和转换
    let trader = TraderId::from_str(&req.trader_id);
    let side = match req.side.to_lowercase().as_str() {
        "buy" => Side::Buy,
        "sell" => Side::Sell,
        _ => return Err(AppError::InvalidInput("invalid side, must be 'buy' or 'sell'".to_string())),
    };

    // 创建命令
    let command = Command::LimitOrder {
        trader,
        side,
        price: req.price,
        quantity: req.quantity,
    };

    // 执行命令
    let mut service = state.matching_service.write().await;
    let result = service.handle(command);

    // 转换结果
    match result {
        CommandResult::LimitOrder { order_id, trades } => {
            let status = if order_id == 0 {
                "filled".to_string()
            } else if trades.is_empty() {
                "open".to_string()
            } else {
                "partial".to_string()
            };

            let trade_dtos = trades
                .iter()
                .map(|t| TradeDto {
                    buyer: t.buyer.to_string(),
                    seller: t.seller.to_string(),
                    price: t.price,
                    quantity: t.quantity,
                })
                .collect();

            info!("订单执行成功: order_id={}, status={}", order_id, status);

            Ok(Json(PlaceOrderResponse {
                order_id,
                status,
                trades: trade_dtos,
            }))
        }
        _ => Err(AppError::Internal("unexpected result type".to_string())),
    }
}

/// 取消订单
async fn cancel_order(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CancelOrderRequest>,
) -> Result<Json<CancelOrderResponse>, AppError> {
    info!("收到取消订单请求: order_id={}", req.order_id);

    let command = Command::CancelOrder {
        order_id: req.order_id,
    };

    let mut service = state.matching_service.write().await;
    let result = service.handle(command);

    match result {
        CommandResult::CancelOrder { success } => {
            let message = if success {
                "订单已取消".to_string()
            } else {
                "订单未找到或已取消".to_string()
            };

            Ok(Json(CancelOrderResponse { success, message }))
        }
        _ => Err(AppError::Internal("unexpected result type".to_string())),
    }
}

/// 查询市场深度
async fn get_market_depth(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let service = state.matching_service.read().await;
    let repo = service.repository();

    let best_bid = repo.best_bid();
    let best_ask = repo.best_ask();

    Ok(Json(serde_json::json!({
        "best_bid": best_bid,
        "best_ask": best_ask,
        "spread": match (best_bid, best_ask) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    })))
}

// ============ 错误处理 ============

#[derive(Debug)]
enum AppError {
    InvalidInput(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}

// ============ 服务器启动 ============

pub async fn start(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("info,matching_service=debug")
        .init();

    info!("初始化订单匹配服务...");

    // 创建应用状态
    let state = Arc::new(AppState::new());

    // 配置CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建路由
    let app = Router::new()
        // 健康检查
        .route("/health", get(health_check))

        // 订单API
        .route("/api/orders", post(place_limit_order))
        .route("/api/orders/cancel", post(cancel_order))

        // 市场数据
        .route("/api/market/depth", get(get_market_depth))

        // 添加状态和中间件
        .with_state(state)
        .layer(cors);

    // 启动服务器
    let addr = format!("0.0.0.0:{}", port);
    info!("订单匹配服务启动在 http://{}", addr);
    info!("API文档:");
    info!("  POST   /api/orders          - 下限价单");
    info!("  POST   /api/orders/cancel   - 取消订单");
    info!("  GET    /api/market/depth    - 查询市场深度");
    info!("  GET    /health              - 健康检查");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
