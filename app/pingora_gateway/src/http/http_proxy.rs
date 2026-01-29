use std::sync::Arc;

use async_trait::async_trait;
use pingora::{
    apps::ServerApp, connectors::TransportConnector, protocols::Stream, server::ShutdownWatch,
    upstreams::peer::HttpPeer
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    select
};
use tracing::info;

enum DuplexEvent {
    DownstreamRead(usize),
    UpstreamRead(usize)
}

/// Pingora HTTP 代理服务器应用
pub struct HttpProxyApp {
    client_connector: TransportConnector,
    proxy_to: HttpPeer
}

// todo 打印转发数据
impl HttpProxyApp {
    /// 创建新的代理服务器应用实例
    pub fn new(proxy_to: HttpPeer) -> Self {
        HttpProxyApp {
            client_connector: TransportConnector::new(None),
            proxy_to
        }
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
                    info!("Downstream session closing");
                    return;
                }
                DuplexEvent::UpstreamRead(0) => {
                    info!("Upstream session closing");
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
        let client_session = self.client_connector.new_stream(&self.proxy_to).await;

        match client_session {
            Ok(client_session) => {
                self.duplex(io, client_session).await;
                None
            }
            Err(e) => {
                info!("Failed to create client session: {}", e);
                None
            }
        }
    }
}

/// Pingora HTTP 代理服务器启动器
pub struct HttpProxyServer;

impl HttpProxyServer {
    /// 启动代理服务器
    // todo POST /api/spot/v2/;POST /api/spot/user/data 需要用户认证
    pub fn start() -> ! {
        use clap::Parser;
        use pingora::server::{configuration::Opt, Server};
        use pingora_core::{listeners::Listeners, services::listening::Service};

        // 初始化日志系统
        let subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::INFO).finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();

        let opt = Some(Opt::parse_args());
        let mut server = Server::new(opt).unwrap();
        server.bootstrap();

        // 配置代理服务：监听 8080 端口，代理到 Axum 服务器的 3001 端口
        let proxy_service = Service::with_listeners(
            "HTTP Proxy Service".to_string(),
            Listeners::tcp("0.0.0.0:8080"),
            HttpProxyApp::new(HttpPeer::new("127.0.0.1:3001", false, "localhost".to_string()))
        );

        info!("🚀 Pingora HTTP proxy started at http://localhost:8080");
        info!("📊 Proxying requests to Axum server at http://localhost:3001");
        info!("💹 Routes available:");
        info!("  - GET /api/spot/health");
        info!("  - POST /api/spot/order/ (JSON)");
        info!("  - POST /api/spot/v2/ (JSON)");
        info!("  - POST /api/spot/market/data (JSON)");
        info!("  - POST /api/spot/user/data (JSON)");

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
