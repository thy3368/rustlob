//! 高性能WebSocket订单匹配服务
//!
//! 遵循Clean Architecture原则，实现低延迟的实时订单推送
//! 目标延迟：WebSocket消息处理 < 100μs

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use dashmap::DashMap;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, error, info, warn};

use lob::lob::{
    handler::{Command, CommandResult, OrderCommandHandler},
    matching_service::MatchingService as LobMatchingService,
    repository::{in_memory::InMemoryOrderRepository, traits::OrderRepository},
    types::lob_types::{Side, TraderId},
};

// ============ 类型定义 ============

/// 客户端ID
type ClientId = u64;

/// 无锁广播通道（零拷贝设计）
type BroadcastSender = UnboundedSender<Arc<ServerMessage>>;

// ============ WebSocket消息协议 ============

/// 客户端请求消息
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// 订阅市场数据
    Subscribe {
        channels: Vec<String>,
    },
    /// 取消订阅
    Unsubscribe {
        channels: Vec<String>,
    },
    /// 下限价单
    LimitOrder {
        trader_id: String,
        side: String,
        price: u32,
        quantity: u32,
    },
    /// 下市价单
    MarketOrder {
        trader_id: String,
        side: String,
        quantity: u32,
    },
    /// 取消订单
    CancelOrder {
        order_id: u64,
    },
    /// Ping（心跳）
    Ping,
}

/// 服务器响应消息
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Pong（心跳响应）
    Pong {
        timestamp: u64,
    },
    /// 订阅确认
    Subscribed {
        channels: Vec<String>,
    },
    /// 订单确认
    OrderAck {
        order_id: u64,
        status: String,
        latency_us: u128,
    },
    /// 成交通知（实时推送）
    Trade {
        trade_id: u64,
        buyer: String,
        seller: String,
        price: u32,
        quantity: u32,
        timestamp: u64,
    },
    /// 订单簿更新
    BookUpdate {
        best_bid: Option<u32>,
        best_ask: Option<u32>,
        spread: Option<u32>,
    },
    /// 错误消息
    Error {
        code: String,
        message: String,
    },
}

// ============ 应用状态（Clean Architecture） ============

/// WebSocket应用状态
pub struct WsAppState {
    /// 撮合服务（领域层）
    matching_service: RwLock<LobMatchingService<InMemoryOrderRepository>>,
    /// 客户端连接管理
    clients: Arc<DashMap<ClientId, ClientConnection>>,
    /// 客户端ID生成器
    next_client_id: AtomicU64,
    /// 交易ID生成器
    next_trade_id: AtomicU64,
}

/// 客户端连接信息
struct ClientConnection {
    /// 广播发送器
    sender: BroadcastSender,
    /// 订阅的频道
    subscriptions: Vec<String>,
}

impl WsAppState {
    pub fn new() -> Self {
        let repository = InMemoryOrderRepository::new(100_000, 1_000_000);
        let matching_service = LobMatchingService::new(repository);

        Self {
            matching_service: RwLock::new(matching_service),
            clients: Arc::new(DashMap::new()),
            next_client_id: AtomicU64::new(1),
            next_trade_id: AtomicU64::new(1),
        }
    }

    /// 生成客户端ID
    fn generate_client_id(&self) -> ClientId {
        self.next_client_id.fetch_add(1, Ordering::Relaxed)
    }

    /// 生成交易ID
    fn generate_trade_id(&self) -> u64 {
        self.next_trade_id.fetch_add(1, Ordering::Relaxed)
    }

    /// 注册客户端连接
    fn register_client(&self, client_id: ClientId, sender: BroadcastSender) {
        info!("客户端 {} 已连接", client_id);
        self.clients.insert(
            client_id,
            ClientConnection {
                sender,
                subscriptions: vec!["trades".to_string()], // 默认订阅成交数据
            },
        );
    }

    /// 注销客户端连接
    fn unregister_client(&self, client_id: ClientId) {
        if self.clients.remove(&client_id).is_some() {
            info!("客户端 {} 已断开", client_id);
        }
    }

    /// 广播消息到所有客户端（零拷贝）
    fn broadcast(&self, message: Arc<ServerMessage>) {
        let mut disconnected = Vec::new();

        for entry in self.clients.iter() {
            let client_id = *entry.key();
            let connection = entry.value();

            // 无锁发送（失败表示客户端断开）
            if connection.sender.unbounded_send(message.clone()).is_err() {
                disconnected.push(client_id);
            }
        }

        // 清理断开的客户端
        for client_id in disconnected {
            self.unregister_client(client_id);
        }
    }

    /// 发送市场深度更新
    async fn broadcast_book_update(&self) {
        let service = self.matching_service.read().await;
        let repo = service.repository();

        let best_bid = repo.best_bid();
        let best_ask = repo.best_ask();
        let spread = match (best_bid, best_ask) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        };

        let message = Arc::new(ServerMessage::BookUpdate {
            best_bid,
            best_ask,
            spread,
        });

        self.broadcast(message);
    }
}

// ============ WebSocket处理器 ============

/// WebSocket升级处理
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsAppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// 处理WebSocket连接（核心低延迟路径）
async fn handle_socket(socket: WebSocket, state: Arc<WsAppState>) {
    let client_id = state.generate_client_id();

    // 创建无界通道（避免背压延迟）
    let (tx, mut rx) = unbounded::<Arc<ServerMessage>>();

    // 注册客户端
    state.register_client(client_id, tx);

    // 分离读写流（全双工通信）
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // 发送任务（从rx接收，发送到WebSocket）
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.next().await {
            // 零拷贝序列化（仅一次JSON序列化）
            match serde_json::to_string(&*msg) {
                Ok(json) => {
                    if ws_sender.send(Message::Text(json)).await.is_err() {
                        break; // 客户端断开
                    }
                }
                Err(e) => {
                    error!("序列化错误: {}", e);
                }
            }
        }
    });

    // 接收任务（从WebSocket接收，处理命令）
    let recv_state = state.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Err(e) = handle_client_message(client_id, &text, &recv_state).await {
                        error!("处理消息失败: {}", e);
                    }
                }
                Ok(Message::Close(_)) => {
                    debug!("客户端 {} 关闭连接", client_id);
                    break;
                }
                Err(e) => {
                    warn!("WebSocket错误: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // 等待任一任务完成
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // 清理客户端
    state.unregister_client(client_id);
}

/// 处理客户端消息（关键路径优化）
async fn handle_client_message(
    client_id: ClientId,
    text: &str,
    state: &Arc<WsAppState>,
) -> Result<(), String> {
    let start = Instant::now();

    // 反序列化消息
    let msg: ClientMessage = serde_json::from_str(text).map_err(|e| e.to_string())?;

    match msg {
        ClientMessage::Ping => {
            // 快速路径：心跳响应
            let pong = Arc::new(ServerMessage::Pong {
                timestamp: chrono::Utc::now().timestamp_millis() as u64,
            });
            if let Some(client) = state.clients.get(&client_id) {
                let _ = client.sender.unbounded_send(pong);
            }
        }

        ClientMessage::Subscribe { channels } => {
            // 订阅频道
            if let Some(mut client) = state.clients.get_mut(&client_id) {
                client.subscriptions.extend(channels.clone());
            }
            let response = Arc::new(ServerMessage::Subscribed { channels });
            if let Some(client) = state.clients.get(&client_id) {
                let _ = client.sender.unbounded_send(response);
            }
        }

        ClientMessage::Unsubscribe { channels } => {
            // 取消订阅
            if let Some(mut client) = state.clients.get_mut(&client_id) {
                client
                    .subscriptions
                    .retain(|ch| !channels.contains(ch));
            }
        }

        ClientMessage::LimitOrder {
            trader_id,
            side,
            price,
            quantity,
        } => {
            // 限价单处理（热路径）
            let trader = TraderId::from_str(&trader_id);
            let order_side = match side.to_lowercase().as_str() {
                "buy" => Side::Buy,
                "sell" => Side::Sel,
                _ => return Err("invalid side".to_string()),
            };

            let command = Command::LimitOrder {
                trader,
                side: order_side,
                price,
                quantity,
            };

            // 执行撮合
            let mut service = state.matching_service.write().await;
            let result = service.handle(command);
            drop(service); // 尽早释放锁

            // 处理结果
            if let CommandResult::LimitOrder { order_id, trades } = result {
                let latency_us = start.elapsed().as_micros();

                // 发送订单确认
                let status = if order_id == 0 {
                    "filled"
                } else if trades.is_empty() {
                    "open"
                } else {
                    "partial"
                };

                let ack = Arc::new(ServerMessage::OrderAck {
                    order_id,
                    status: status.to_string(),
                    latency_us,
                });

                if let Some(client) = state.clients.get(&client_id) {
                    let _ = client.sender.unbounded_send(ack);
                }

                // 广播成交信息（所有订阅者）
                for trade in trades {
                    let trade_msg = Arc::new(ServerMessage::Trade {
                        trade_id: state.generate_trade_id(),
                        buyer: trade.buyer.to_string(),
                        seller: trade.seller.to_string(),
                        price: trade.price,
                        quantity: trade.quantity,
                        timestamp: chrono::Utc::now().timestamp_millis() as u64,
                    });
                    state.broadcast(trade_msg);
                }

                // 广播订单簿更新
                state.broadcast_book_update().await;

                debug!(
                    "订单处理完成: order_id={}, 延迟={}μs",
                    order_id, latency_us
                );
            }
        }

        ClientMessage::MarketOrder {
            trader_id,
            side,
            quantity,
        } => {
            // 市价单处理（类似限价单）
            let trader = TraderId::from_str(&trader_id);
            let order_side = match side.to_lowercase().as_str() {
                "buy" => Side::Buy,
                "sell" => Side::Sel,
                _ => return Err("invalid side".to_string()),
            };

            let command = Command::MarketOrder {
                trader,
                side: order_side,
                quantity,
            };

            let mut service = state.matching_service.write().await;
            let result = service.handle(command);
            drop(service);

            if let CommandResult::MarketOrder { trades } = result {
                let latency_us = start.elapsed().as_micros();

                let ack = Arc::new(ServerMessage::OrderAck {
                    order_id: 0,
                    status: "filled".to_string(),
                    latency_us,
                });

                if let Some(client) = state.clients.get(&client_id) {
                    let _ = client.sender.unbounded_send(ack);
                }

                // 广播成交
                for trade in trades {
                    let trade_msg = Arc::new(ServerMessage::Trade {
                        trade_id: state.generate_trade_id(),
                        buyer: trade.buyer.to_string(),
                        seller: trade.seller.to_string(),
                        price: trade.price,
                        quantity: trade.quantity,
                        timestamp: chrono::Utc::now().timestamp_millis() as u64,
                    });
                    state.broadcast(trade_msg);
                }

                state.broadcast_book_update().await;
            }
        }

        ClientMessage::CancelOrder { order_id } => {
            // 取消订单
            let command = Command::CancelOrder { order_id };
            let mut service = state.matching_service.write().await;
            let result = service.handle(command);
            drop(service);

            if let CommandResult::CancelOrder { success } = result {
                let latency_us = start.elapsed().as_micros();
                let status = if success { "cancelled" } else { "not_found" };

                let ack = Arc::new(ServerMessage::OrderAck {
                    order_id,
                    status: status.to_string(),
                    latency_us,
                });

                if let Some(client) = state.clients.get(&client_id) {
                    let _ = client.sender.unbounded_send(ack);
                }

                if success {
                    state.broadcast_book_update().await;
                }
            }
        }
    }

    Ok(())
}

// ============ HTTP健康检查 ============

async fn ws_health_check() -> impl IntoResponse {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "websocket-matching-service",
        "protocol": "ws"
    }))
}

// ============ 服务器启动 ============

pub async fn start(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("info,websocket_service=debug")
        .init();

    info!("初始化WebSocket订单匹配服务...");

    // 创建应用状态
    let state = Arc::new(WsAppState::new());

    // 配置CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建路由
    let app = Router::new()
        // WebSocket端点
        .route("/ws", get(ws_handler))
        // 健康检查
        .route("/health", get(ws_health_check))
        // 添加状态和中间件
        .with_state(state)
        .layer(cors);

    // 启动服务器
    let addr = format!("0.0.0.0:{}", port);
    info!("WebSocket服务启动在 ws://{}/ws", addr);
    info!("健康检查: http://{}/health", addr);
    info!("");
    info!("消息协议:");
    info!("  客户端 -> 服务器:");
    info!("    - limit_order   : 下限价单");
    info!("    - market_order  : 下市价单");
    info!("    - cancel_order  : 取消订单");
    info!("    - subscribe     : 订阅数据");
    info!("    - ping          : 心跳");
    info!("");
    info!("  服务器 -> 客户端:");
    info!("    - order_ack     : 订单确认");
    info!("    - trade         : 成交推送");
    info!("    - book_update   : 订单簿更新");
    info!("    - pong          : 心跳响应");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
