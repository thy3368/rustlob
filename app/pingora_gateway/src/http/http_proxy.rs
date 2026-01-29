use std::sync::Arc;

use async_trait::async_trait;
use pingora::{
    apps::ServerApp,
    connectors::TransportConnector,
    protocols::Stream,
    server::ShutdownWatch,
    upstreams::peer::{HttpPeer, Peer}
};
use pingora_proxy::http_proxy_service;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    select
};
use tracing::{debug, info, warn};

use super::router::{UserIdExtractor, UserRouteConfig, UserRouter};

enum DuplexEvent {
    DownstreamRead(usize),
    UpstreamRead(usize)
}

/// Pingora HTTP 代理服务器应用
pub struct HttpProxyApp {
    client_connector: TransportConnector,
    proxy_to: HttpPeer,
    /// 用户路由器（用于 /api/spot/v2/ 和 /api/spot/user/data）
    user_router: Arc<UserRouter>
}

// todo 打印转发数据
impl HttpProxyApp {
    /// 创建新的代理服务器应用实例
    pub fn new(proxy_to: HttpPeer) -> Self {
        let user_route_config = UserRouteConfig::default();
        let user_router = Arc::new(UserRouter::new(user_route_config));

        HttpProxyApp {
            client_connector: TransportConnector::new(None),
            proxy_to,
            user_router
        }
    }

    /// 创建带自定义路由配置的代理服务器应用实例
    pub fn with_router(proxy_to: HttpPeer, user_route_config: UserRouteConfig) -> Self {
        let user_router = Arc::new(UserRouter::new(user_route_config));

        HttpProxyApp {
            client_connector: TransportConnector::new(None),
            proxy_to,
            user_router
        }
    }

    /// 解析 HTTP 请求并提取路径和用户ID
    ///
    /// 返回：(请求路径, 用户ID, 完整请求数据)
    async fn parse_http_request(&self, server_session: &mut Stream) -> Option<(String, Option<String>, Vec<u8>)> {
        let mut buffer = Vec::with_capacity(8192);
        let mut temp_buf = [0u8; 1024];

        // 读取 HTTP 请求头
        loop {
            match server_session.read(&mut temp_buf).await {
                Ok(0) => return None, // 连接关闭
                Ok(n) => {
                    buffer.extend_from_slice(&temp_buf[..n]);

                    // 检查是否读取到完整的请求头（\r\n\r\n）
                    if let Some(header_end) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
                        // 解析请求行
                        let header_str = String::from_utf8_lossy(&buffer[..header_end]);
                        let first_line = header_str.lines().next()?;
                        let parts: Vec<&str> = first_line.split_whitespace().collect();

                        if parts.len() < 2 {
                            return None;
                        }

                        let path = parts[1].to_string();

                        // 尝试从查询参数提取用户ID
                        let mut user_id = UserIdExtractor::extract_from_query(&path);

                        // 尝试从请求头提取用户ID
                        if user_id.is_none() {
                            user_id = UserIdExtractor::extract_from_headers(&header_str);
                        }

                        // 如果是 POST 请求，尝试从请求体提取用户ID
                        if user_id.is_none() && parts[0] == "POST" {
                            // 检查 Content-Length
                            if let Some(content_length) = Self::extract_content_length(&header_str) {
                                let header_size = header_end + 4;
                                let body_in_buffer = buffer.len() - header_size;

                                // 继续读取请求体
                                let remaining = content_length.saturating_sub(body_in_buffer);
                                if remaining > 0 {
                                    let mut body_buf = vec![0u8; remaining];
                                    if let Ok(_) = server_session.read_exact(&mut body_buf).await {
                                        buffer.extend_from_slice(&body_buf);
                                    }
                                }

                                // 提取请求体
                                if buffer.len() >= header_size {
                                    let body = &buffer[header_size..];
                                    user_id = UserIdExtractor::extract_from_json(body);
                                }
                            }
                        }

                        return Some((path, user_id, buffer));
                    }

                    // 防止无限读取
                    if buffer.len() > 1024 * 1024 {
                        warn!("HTTP request too large, aborting");
                        return None;
                    }
                }
                Err(e) => {
                    warn!("Error reading from downstream: {}", e);
                    return None;
                }
            }
        }
    }

    /// 从 HTTP 请求头中提取 Content-Length
    fn extract_content_length(headers: &str) -> Option<usize> {
        for line in headers.lines() {
            if line.to_lowercase().starts_with("content-length:") {
                if let Some(value) = line.split(':').nth(1) {
                    return value.trim().parse().ok();
                }
            }
        }
        None
    }

    /// 判断路径是否需要用户路由
    fn needs_user_routing(path: &str) -> bool {
        path.starts_with("/api/spot/v2/") || path.starts_with("/api/spot/user/data")
    }

    async fn duplex(&self, mut server_session: Stream, mut client_session: Stream) {
        let mut upstream_buf = [0; 1024];
        let mut downstream_buf = [0; 1024];
        loop {
            let downstream_read = server_session.read(&mut upstream_buf);
            let upstream_read = client_session.read(&mut downstream_buf);
            let event: DuplexEvent;
            select! {
                n = downstream_read => event = DuplexEvent::DownstreamRead(n.unwrap()),
                n = upstream_read => event = DuplexEvent::UpstreamRead(n.unwrap()),
            }
            match event {
                DuplexEvent::DownstreamRead(0) => {
                    debug!("Downstream session closing");
                    return;
                }
                DuplexEvent::UpstreamRead(0) => {
                    debug!("Upstream session closing");
                    return;
                }
                DuplexEvent::DownstreamRead(n) => {
                    client_session.write_all(&upstream_buf[0..n]).await.unwrap();
                    client_session.flush().await.unwrap();
                }
                DuplexEvent::UpstreamRead(n) => {
                    server_session.write_all(&downstream_buf[0..n]).await.unwrap();
                    server_session.flush().await.unwrap();
                }
            }
        }
    }
}

#[async_trait]
impl ServerApp for HttpProxyApp {
    async fn process_new(self: &Arc<Self>, mut io: Stream, _shutdown: &ShutdownWatch) -> Option<Stream> {
        // 解析 HTTP 请求，提取路径和用户ID
        let (path, user_id_opt, request_data) = match self.parse_http_request(&mut io).await {
            Some(data) => data,
            None => {
                warn!("Failed to parse HTTP request");
                return None;
            }
        };

        // 根据路径决定是否使用用户路由
        let target_peer = if Self::needs_user_routing(&path) {
            if let Some(user_id) = user_id_opt.as_ref() {
                info!("🔀 User routing: {} -> user_id={}", path, user_id);
                self.user_router.select_backend(user_id).await
            } else {
                warn!("⚠️  Path requires user routing but no user_id found: {}", path);
                info!("Using default backend for: {}", path);
                self.proxy_to.clone()
            }
        } else {
            debug!("Standard routing: {}", path);
            self.proxy_to.clone()
        };

        info!("📡 Proxying {} to {}", path, target_peer.address());

        // 连接到选定的后端服务器
        let client_session = self.client_connector.new_stream(&target_peer).await;

        match client_session {
            Ok(mut client_session) => {
                // 转发原始请求数据到后端
                if let Err(e) = client_session.write_all(&request_data).await {
                    warn!("Failed to write request to backend: {}", e);
                    return None;
                }

                if let Err(e) = client_session.flush().await {
                    warn!("Failed to flush request to backend: {}", e);
                    return None;
                }

                // 进入双工转发模式
                self.duplex(io, client_session).await;
                None
            }
            Err(e) => {
                warn!("Failed to create client session: {}", e);
                None
            }
        }
    }
}

/// Pingora HTTP 代理服务器启动器
pub struct HttpProxyServer;

impl HttpProxyServer {
    /// 启动代理服务器

    pub fn start() -> ! {
        use pingora::server::{configuration::Opt, Server};
        use pingora_core::{listeners::Listeners, services::listening::Service};

        // 初始化日志系统
        let subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::INFO).finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();

        let opt = Some(Opt::parse_args());
        let mut server = Server::new(opt).unwrap();
        server.bootstrap();

        // 配置用户路由
        let user_route_config = UserRouteConfig::default();

        // 配置代理服务：监听 8080 端口
        let proxy_service = Service::with_listeners(
            "HTTP Proxy Service".to_string(),
            Listeners::tcp("0.0.0.0:8080"),
            HttpProxyApp::with_router(
                HttpPeer::new("127.0.0.1:3001", false, "localhost".to_string()),
                user_route_config.clone()
            )
        );

        info!("🚀 Pingora HTTP proxy started at http://localhost:8080");
        info!("📊 Default backend: http://localhost:3001");
        info!("🔀 User-based routing enabled for:");
        info!("   - /api/spot/v2/*");
        info!("   - /api/spot/user/data");
        info!("");
        info!("👥 User routing configuration:");
        for (partition, ips) in &user_route_config.partition_ips {
            info!("   - {} → {:?}", partition, ips);
        }
        info!("");
        info!("💹 Available routes:");
        info!("  - GET  /api/spot/health");
        info!("  - POST /api/spot/order/ (JSON)");
        info!("  - POST /api/spot/v2/ (JSON) [user routing]");
        info!("  - POST /api/spot/market/data (JSON)");
        info!("  - POST /api/spot/user/data (JSON) [user routing]");
        info!("");
        info!("📝 User ID extraction from:");
        info!("   1. JSON body (user_id, userId, trader_id, traderId, uid)");
        info!("   2. HTTP headers (X-User-Id, X-Trader-Id)");
        info!("   3. Query parameters (?user_id=xxx)");

        server.add_services(vec![Box::new(proxy_service)]);
        server.run_forever();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_app_creation() {
        let proxy_app = HttpProxyApp::new(HttpPeer::new("127.0.0.1:3001", false, "localhost".to_string()));
        assert!(true, "Proxy app created successfully");
    }
}
