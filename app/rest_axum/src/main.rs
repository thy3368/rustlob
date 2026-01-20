pub mod trade_gw;
use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber;

// Spot 订单处理相关导入
use base_types::{
    exchange::spot::spot_types::{TimeInForce, TraderId},
    AccountId, AssetId, Price, Quantity, Side, TradingPair,
};
use spot_proc::proc::behavior::trading_spot_order_proc::{CMetadata, CancelOrder, CmdMetadata, LimitOrder, MarketOrder, SpotCmdAny};

// 请求数据结构
#[derive(Debug, Deserialize)]
struct RequestData {
    name: String,
    age: u32,
    email: String,
}

// 响应数据结构
#[derive(Debug, Serialize)]
struct ResponseData {
    message: String,
    user: UserInfo,
}

#[derive(Debug, Serialize)]
struct UserInfo {
    name: String,
    age: u32,
    email: String,
    is_adult: bool,
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建路由
    let app = Router::new()
        .route("/health", get(health_check))
        // Spot 订单处理接口
        .route("/api/spot/order/limit", post(handle_limit_order))
        .route("/api/spot/order/market", post(handle_market_order))
        .route("/api/spot/order/cancel", post(handle_cancel_order));

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind port");

    axum::serve(listener, app).await.expect("Server failed to start");
}

async fn health_check() -> &'static str {
    "OK"
}

// ============================================================================
// Spot 订单处理接口
// ============================================================================

/// 限价单请求 DTO
#[derive(Debug, Deserialize)]
struct LimitOrderRequest {
    trader_id: [u8; 8],
    account_id: u64,
    base_asset: String,    // 例如: "BTC"
    quote_asset: String,   // 例如: "USDT"
    side: String,          // "Buy" 或 "Sell"
    price: f64,            // 价格（浮点数，内部会转换为定点数）
    quantity: f64,         // 数量（浮点数，内部会转换为定点数）
    time_in_force: String, // "GTC", "IOC", "FOK", "GTX", "GTD"
    client_order_id: Option<String>,
}

/// 市价单请求 DTO
#[derive(Debug, Deserialize)]
struct MarketOrderRequest {
    trader_id: [u8; 8],
    account_id: u64,
    base_asset: String,
    quote_asset: String,
    side: String,
    quantity: f64,
    price_limit: Option<f64>, // 价格保护
    client_order_id: Option<String>,
}

/// 取消订单请求 DTO
#[derive(Debug, Deserialize)]
struct CancelOrderRequest {
    order_id: u64,
}

/// 订单响应 DTO
#[derive(Debug, Serialize)]
struct OrderResponse {
    success: bool,
    message: String,
    order_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// 处理限价单
/// todo 可以直接用 Json<LimitOrder> 么？
async fn handle_limit_order(Json(request): Json<LimitOrderRequest>) -> impl IntoResponse {
    println!("📋 收到限价单请求: {:?}", request);

    // 解析 Side
    let side = match request.side.as_str() {
        "Buy" => Side::Buy,
        "Sell" => Side::Sell,
        _ => {
            return create_error_response("Invalid side, must be 'Buy' or 'Sell'");
        }
    };

    // 解析 TimeInForce
    let time_in_force = match request.time_in_force.as_str() {
        "GTC" => TimeInForce::GTC,
        "IOC" => TimeInForce::IOC,
        "FOK" => TimeInForce::FOK,
        "GTX" => TimeInForce::GTX,
        "GTD" => TimeInForce::GTD,
        _ => {
            return create_error_response("Invalid time_in_force");
        }
    };

    // 解析资产
    let base_asset = match parse_asset(&request.base_asset) {
        Some(asset) => asset,
        None => {
            return create_error_response(&format!("Invalid base_asset: {}", request.base_asset));
        }
    };

    let quote_asset = match parse_asset(&request.quote_asset) {
        Some(asset) => asset,
        None => {
            return create_error_response(&format!("Invalid quote_asset: {}", request.quote_asset));
        }
    };

    // 创建交易对
    let trading_pair = TradingPair { base_asset, quote_asset };

    // 创建命令元数据
    let metadata = CMetadata {
        command_id: uuid::Uuid::new_v4().to_string(),
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
        correlation_id: None,
        causation_id: None,
        actor: Some("rest_api".to_string()),
        attributes: vec![],
    };

    // 创建限价单命令
    let limit_order = LimitOrder {
        metadata,
        trader: TraderId::new(request.trader_id),
        account_id: AccountId(request.account_id),
        trading_pair,
        side,
        price: Price::from_f64(request.price),
        quantity: Quantity::from_f64(request.quantity),
        time_in_force,
        client_order_id: request.client_order_id,
    };

    // 包装为 SpotCmdAny
    let _spot_cmd = SpotCmdAny::LimitOrder(limit_order);

    // TODO: 调用 SpotOrderExchProc::handle() 处理命令
    // let result = processor.handle(spot_cmd);

    // 暂时返回成功响应（实际应该根据处理结果返回）
    let response = OrderResponse {
        success: true,
        message: "Limit order received and queued for processing".to_string(),
        order_id: Some(12345), // TODO: 使用实际生成的订单ID
        error: None,
    };

    create_json_response(response)
}

/// 处理市价单
async fn handle_market_order(Json(request): Json<MarketOrderRequest>) -> impl IntoResponse {
    println!("📋 收到市价单请求: {:?}", request);

    let side = match request.side.as_str() {
        "Buy" => Side::Buy,
        "Sell" => Side::Sell,
        _ => {
            return create_error_response("Invalid side");
        }
    };

    let base_asset = match parse_asset(&request.base_asset) {
        Some(asset) => asset,
        None => return create_error_response("Invalid base_asset"),
    };

    let quote_asset = match parse_asset(&request.quote_asset) {
        Some(asset) => asset,
        None => return create_error_response("Invalid quote_asset"),
    };

    let trading_pair = TradingPair { base_asset, quote_asset };

    let metadata = CmdMetadata {
        command_id: uuid::Uuid::new_v4().to_string(),
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
        correlation_id: None,
        causation_id: None,
        actor: Some("rest_api".to_string()),
        attributes: vec![],
    };

    let market_order = MarketOrder {
        metadata,
        trader: TraderId::new(request.trader_id),
        account_id: AccountId(request.account_id),
        trading_pair,
        side,
        quantity: Quantity::from_f64(request.quantity),
        price_limit: request.price_limit.map(Price::from_f64),
        time_in_force: Some(TimeInForce::IOC),
        client_order_id: request.client_order_id,
    };

    let _spot_cmd = SpotCmdAny::MarketOrder(market_order);

    // TODO: 调用处理器
    let response = OrderResponse {
        success: true,
        message: "Market order received and queued for processing".to_string(),
        order_id: Some(12346),
        error: None,
    };

    create_json_response(response)
}

/// 处理取消订单
async fn handle_cancel_order(Json(request): Json<CancelOrderRequest>) -> impl IntoResponse {
    println!("📋 收到取消订单请求: {:?}", request);

    let metadata = CmdMetadata {
        command_id: uuid::Uuid::new_v4().to_string(),
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
        correlation_id: None,
        causation_id: None,
        actor: Some("rest_api".to_string()),
        attributes: vec![],
    };

    let cancel_order = CancelOrder { metadata, order_id: request.order_id };

    let _spot_cmd = SpotCmdAny::CancelOrder(cancel_order);

    // TODO: 调用处理器
    let response = OrderResponse {
        success: true,
        message: "Cancel order received and queued for processing".to_string(),
        order_id: Some(request.order_id),
        error: None,
    };

    create_json_response(response)
}

/// 解析资产字符串到 AssetId
fn parse_asset(asset_str: &str) -> Option<AssetId> {
    match asset_str.to_uppercase().as_str() {
        "BTC" => Some(AssetId::BTC),
        "ETH" => Some(AssetId::ETH),
        "USDT" => Some(AssetId::USDT),
        "USDC" => Some(AssetId::USDC),
        "BNB" => Some(AssetId::BNB),
        "SOL" => Some(AssetId::SOL),
        "XRP" => Some(AssetId::XRP),
        "ADA" => Some(AssetId::ADA),
        "DOGE" => Some(AssetId::DOGE),
        "TRX" => Some(AssetId::TRX),
        _ => None,
    }
}

/// 创建 JSON 响应
fn create_json_response(response: OrderResponse) -> impl IntoResponse {
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}

/// 创建错误响应
fn create_error_response(error_msg: &str) -> impl IntoResponse {
    let response = OrderResponse {
        success: false,
        message: "Request failed".to_string(),
        order_id: None,
        error: Some(error_msg.to_string()),
    };
    let json = serde_json::to_string(&response).unwrap();
    (axum::http::StatusCode::BAD_REQUEST, [(axum::http::header::CONTENT_TYPE, "application/json")], json)
}
