use base_types::{cqrs::cqrs_types::CmdResp, handler::handler::Handler};
use reqwest::Client;
use spot_behavior::proc::behavior::{
    spot_trade_behavior::{CommonError, SpotCmdErrorAny},
    v2::{
        spot_market_data_behavior::{SpotMarketDataBehavior, SpotMarketDataCmdAny, SpotMarketDataResAny},
        spot_trade_behavior_v2::{SpotTradeBehaviorV2, SpotTradeCmdAny, SpotTradeResAny},
        spot_user_data_behavior::{SpotUserDataBehavior, SpotUserDataCmdAny, SpotUserDataResAny},
        spot_user_data_sse_behavior::{
            SpotUserDataListenKeyBehavior, SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny
        }
    }
};
// 实现HTTP调用客户端，参考
// /Users/hongyaotang/src/rustlob/app/gw_axum/src/interfaces/spot/http_server.rs

/// 泛型Spot HTTP客户端，支持多种行为类型
pub struct SpotHttpClient {
    http_client: Client,
    base_url: String
}

impl SpotHttpClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            http_client: Client::new(),
            base_url: base_url.to_string()
        }
    }

    /// 通用发送命令方法，接受路径参数
    async fn send_generic_command<C, R>(&self, cmd: C, path: &str) -> Result<CmdResp<R>, SpotCmdErrorAny>
    where
        C: serde::Serialize + std::fmt::Debug,
        R: serde::de::DeserializeOwned + std::fmt::Debug
    {
        let url = format!("{}/api/spot/{}/", self.base_url, path);

        println!("📡 发送HTTP请求到: {}", url);
        println!("🔧 请求命令: {:?}", cmd);

        let response = self.http_client.post(&url).json(&cmd).send().await.map_err(|e| {
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("HTTP请求失败: {}", e)
            })
        })?;

        let status = response.status();
        println!("📨 服务器响应状态: {}", status);

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "无法读取错误响应".to_string());
            return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("服务器返回错误状态: {} - {}", status, error_text)
            }));
        }

        let cmd_resp: CmdResp<R> = response.json().await.map_err(|e| {
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("响应解析失败: {}", e)
            })
        })?;

        println!("✅ 响应解析成功: {:?}", cmd_resp);

        Ok(cmd_resp)
    }
}


// 实现SpotTradeBehaviorV2
impl Handler<SpotTradeCmdAny, SpotTradeResAny, SpotCmdErrorAny> for SpotHttpClient {
    async fn handle(&self, cmd: SpotTradeCmdAny) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        self.send_generic_command(cmd, "v1/v2").await
    }
}


// 实现SpotUserDataBehavior
impl Handler<SpotUserDataCmdAny, SpotUserDataResAny, SpotCmdErrorAny> for SpotHttpClient {
    async fn handle(&self, cmd: SpotUserDataCmdAny) -> Result<CmdResp<SpotUserDataResAny>, SpotCmdErrorAny> {
        self.send_generic_command(cmd, "user_data").await
    }
}


// 实现SpotMarketDataBehavior
impl Handler<SpotMarketDataCmdAny, SpotMarketDataResAny, SpotCmdErrorAny> for SpotHttpClient {
    async fn handle(&self, cmd: SpotMarketDataCmdAny) -> Result<CmdResp<SpotMarketDataResAny>, SpotCmdErrorAny> {
        self.send_generic_command(cmd, "market_data").await
    }
}


impl Handler<SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny, SpotCmdErrorAny> for SpotHttpClient {
    async fn handle(
        &self, cmd: SpotUserDataListenKeyCmdAny
    ) -> Result<CmdResp<SpotUserDataListenKeyResAny>, SpotCmdErrorAny> {
        self.send_generic_command(cmd, "listen_key").await
    }
}

impl Clone for SpotHttpClient {
    fn clone(&self) -> Self {
        Self {
            http_client: Client::new(),
            base_url: self.base_url.clone()
        }
    }
}

impl Default for SpotHttpClient {
    fn default() -> Self { Self::new("http://localhost:3001") }
}

#[cfg(test)]
mod tests {
    use base_types::cqrs::cqrs_types::CMetadata;
    use spot_behavior::proc::behavior::v2::spot_trade_behavior_v2::{
        NewOrderCmd, OrderSide, OrderType, SpotTradeCmdAny, TestNewOrderCmd
    };

    use super::*;

    #[tokio::test]
    async fn test_trade_v2_http_connection() {
        // 注意：需要先启动服务端 (gw_axum)
        println!("🧪 测试 Trade V2 HTTP 连接...");

        // 创建客户端实例（使用默认地址 http://localhost:3001）
        let client = SpotHttpClient::default();

        // 创建测试命令 - 使用 TestNewOrder 命令进行连接测试
        let test_cmd = SpotTradeCmdAny::TestNewOrder(TestNewOrderCmd {
            new_order: NewOrderCmd {
                metadata: CMetadata::default(),
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                time_in_force: None,
                quantity: Some(0.001),
                quote_order_qty: None,
                price: Some(50000.0),
                new_client_order_id: Some("test_order_123".to_string()),
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                peg_price_type: None,
                peg_offset_value: None,
                peg_offset_type: None,
                recv_window: None,
                timestamp: chrono::Utc::now().timestamp_millis()
            },
            compute_commission_rates: Some(false)
        });

        println!("📡 发送测试命令到: http://localhost:3001/api/spot/trade/v2/");

        // 发送测试命令
        match (&client).handle(test_cmd).await {
            Ok(response) => {
                println!("✅ 连接成功！响应: {:?}", response);
                // assert!(response.success, "响应成功标志应为 true");
            }
            Err(error) => {
                println!("❌ 连接失败: {:?}", error);
                // 如果服务端未启动，测试将失败 - 这是预期的行为
                // 提示用户需要先启动 gw_axum 服务端
                panic!("无法连接到 Trade V2 服务端。请确保已启动 gw_axum 服务端（监听端口 3001）。错误: {:?}", error);
            }
        }
    }
}
