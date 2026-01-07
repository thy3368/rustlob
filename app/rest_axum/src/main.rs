use axum::{
    routing::{get, post},
    Router,
    extract::Json,
    response::IntoResponse,
    body::Bytes,
};
use tracing_subscriber;
use serde::{Deserialize, Serialize};
use simd_json;
use sbe::trade_codec::{TradeEncoder, TradeDecoder, SBE_BLOCK_LENGTH};
use sbe::{Encoder, ReadBuf, WriteBuf};

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
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .route("/api/user", post(handle_user))
        .route("/api/trade/sbe", post(handle_trade_sbe));

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port");

    println!("🚀 Server started at http://localhost:3000");
    println!("📊 Health check: GET /health");
    println!("👤 User API: POST /api/user (JSON)");
    println!("📈 Trade SBE API: POST /api/trade/sbe (SBE)");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

// 处理 SBE 编码的交易请求和响应
async fn handle_trade_sbe(body: Bytes) -> impl IntoResponse {
    // 第一步：解码 SBE 格式的请求
    let read_buf = ReadBuf::new(&body);
    let decoder = TradeDecoder::default().wrap(read_buf, 0, SBE_BLOCK_LENGTH, 0);

    let trade_id = decoder.trade_id();
    let symbol = decoder.symbol();
    let price = decoder.price();
    let quantity = decoder.quantity();

    // 打印接收到的交易信息（用于调试）
    println!("📈 接收到 SBE 交易: ID={}, 符号={}, 价格={}, 数量={}",
             trade_id, symbol as char, price, quantity);

    // 第二步：处理交易（这里可以添加业务逻辑）
    let processed_price = price * 1.01;  // 示例：价格上涨 1%
    let processed_quantity = quantity * 2;  // 示例：数量翻倍

    // 第三步：编码 SBE 格式的响应
    let mut buffer = vec![0u8; SBE_BLOCK_LENGTH as usize];
    let write_buf = WriteBuf::new(&mut buffer);
    let mut encoder = TradeEncoder::default().wrap(write_buf, 0);

    encoder.trade_id(trade_id);
    encoder.symbol(symbol);
    encoder.price(processed_price);
    encoder.quantity(processed_quantity);

    // 返回 SBE 编码的响应
    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/octet-stream")],
        buffer,
    )
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn health_check() -> &'static str {
    "OK"
}

// 处理 JSON 请求和响应
async fn handle_user(Json(request): Json<RequestData>) -> impl IntoResponse {
    // 处理请求数据
    let is_adult = request.age >= 18;

    // 构建响应数据
    let response = ResponseData {
        message: format!("Hello, {}! Welcome to our API.", request.name),
        user: UserInfo {
            name: request.name,
            age: request.age,
            email: request.email,
            is_adult,
        },
    };

    // 使用 simd-json 序列化
    let json_response = simd_json::to_string(&response).unwrap();

    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        json_response,
    )
}