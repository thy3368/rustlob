use reqwest::Client;
use spot_behavior::proc::behavior::spot_trade_behavior::{
    CmdResp, CommonError, IdemSpotResult, LimitOrder, SpotCmdAny, SpotCmdError, SpotCmdRes, SpotTradeBehavior,
};

/// RESTful HTTP 客户端 - 调用远程订单处理服务
///
/// # 性能优化
/// - 复用 HTTP 连接池（reqwest::Client）
/// - 异步非阻塞调用
/// - 零拷贝 JSON 序列化
#[derive(Clone)]
pub struct RestfulClient {
    /// 服务端基础 URL
    base_url: String,
    /// HTTP 客户端（内置连接池）
    client: Client,
}

impl RestfulClient {
    /// 创建新的 RESTful 客户端实例
    ///
    /// # Arguments
    /// * `base_url` - 服务端地址，例如 "http://localhost:3000"
    ///
    /// # Example
    /// ```
    /// let client = RestfulClient::new("http://localhost:3000");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::builder()
                .tcp_nodelay(true) // 禁用 Nagle 算法，降低延迟
                .pool_max_idle_per_host(10) // 连接池优化
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    /// 发送限价单请求
    #[inline]
    async fn post_cmd(&self, cmd: SpotCmdAny) -> Result<CmdResp<SpotCmdRes>, SpotCmdError> {
        let url = format!("{}/api/spot/order/", self.base_url);

        self.client
            .post(&url)
            .json(&cmd)
            .send()
            .await
            .map_err(|e| {
                SpotCmdError::Common(CommonError::Internal { message: format!("HTTP request failed: {}", e) })
            })?
            .json::<CmdResp<SpotCmdRes>>()
            .await
            .map_err(|e| {
                SpotCmdError::Common(CommonError::Internal { message: format!("Failed to parse response: {}", e) })
            })
    }
}

impl SpotTradeBehavior for RestfulClient {
    /// 处理订单命令 - 通过 HTTP POST 发送到远程服务
    ///
    /// # 注意
    /// 这是同步接口，内部使用 tokio::runtime 进行异步调用
    /// 在高性能场景下，建议使用异步版本的 trait
    fn handle(&mut self, cmd: SpotCmdAny) -> IdemSpotResult {
        // 使用 tokio runtime 执行异步调用
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            SpotCmdError::Common(CommonError::Internal { message: format!("Failed to create runtime: {}", e) })
        })?;

        rt.block_on(async { self.post_cmd(cmd).await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_types::exchange::spot::spot_types::{TimeInForce, TraderId};
    use base_types::{AccountId, AssetId, Decimal, Side, TradingPair};
    use spot_behavior::proc::behavior::spot_trade_behavior::CMetadata;

    #[test]
    fn test_restful_client_limit_order() {
        // 注意：需要先启动服务端 (rest_axum)
        let mut client = RestfulClient::new("http://localhost:3000");

        let limit_order = LimitOrder {
            metadata: CMetadata {
                command_id: "test_order_001".to_string(),
                timestamp: 1234567890,
                correlation_id: None,
                causation_id: None,
                actor: Some("test_user".to_string()),
                attributes: vec![],
            },
            trader: TraderId::new([0, 0, 0, 0, 0, 0, 0, 100]),
            account_id: AccountId(1),
            trading_pair: TradingPair::new(AssetId::BTC, AssetId::USDT),
            side: Side::Buy,
            price: Decimal::from_f64(50000.0),
            quantity: Decimal::from_f64(1.0),
            time_in_force: TimeInForce::GTC,
            client_order_id: None,
        };

        // 调用1000次
        for i in 0..10000000 {
            let result = client.handle(SpotCmdAny::LimitOrder(limit_order.clone()));
            println!("[{}] Result: {:?}", i, result);
        }
    }
}
