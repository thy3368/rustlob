use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::hyperliquid_ws::error::HyperliquidWsError;
use crate::hyperliquid_ws::wire::{
    HyperliquidWsMessageParser, WsClientMessageWire, WsServerMessageWire,
};

type HyperliquidSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
pub struct HyperliquidWsClient {
    stream: HyperliquidSocket,
}

impl HyperliquidWsClient {
    pub async fn connect(url: &str) -> Result<Self, HyperliquidWsError> {
        let (stream, _) = connect_async(url).await.map_err(HyperliquidWsError::Connect)?;
        Ok(Self { stream })
    }

    pub async fn send(&mut self, message: &WsClientMessageWire) -> Result<(), HyperliquidWsError> {
        let text = serde_json::to_string(message).map_err(HyperliquidWsError::Json)?;
        self.stream.send(Message::Text(text.into())).await.map_err(HyperliquidWsError::Frame)
    }

    pub async fn send_ping(&mut self) -> Result<(), HyperliquidWsError> {
        self.send(&WsClientMessageWire::Ping(super::wire::PingWire)).await
    }

    pub async fn recv(&mut self) -> Result<WsServerMessageWire, HyperliquidWsError> {
        loop {
            let next = self.stream.next().await.ok_or(HyperliquidWsError::ConnectionClosed)?;
            match next.map_err(HyperliquidWsError::Frame)? {
                Message::Text(text) => {
                    return HyperliquidWsMessageParser::parse_text(text.as_str());
                }
                Message::Binary(bytes) => {
                    let text = String::from_utf8(bytes.to_vec())
                        .map_err(HyperliquidWsError::BinaryFrameNotUtf8)?;
                    return HyperliquidWsMessageParser::parse_text(&text);
                }
                Message::Ping(payload) => {
                    self.stream
                        .send(Message::Pong(payload))
                        .await
                        .map_err(HyperliquidWsError::Frame)?;
                }
                Message::Pong(_) => {}
                Message::Close(_) => return Err(HyperliquidWsError::ConnectionClosed),
                _ => {}
            }
        }
    }
}
