mod http;
mod websocket;

use http::http_proxy::HttpProxyServer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpProxyServer::start()
}
