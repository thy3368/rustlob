use base_types::cqrs::cqrs_types::CmdResp;
use base_types::handler::handler::Handler;
use reqwest::Client;
use spot_behavior::proc::behavior::v2::{
    spot_trade_behavior::{SpotTradeCmdOrQuery, SpotTradeResAny},
    spot_trade_error::{CommonError, SpotApiErrorAny},
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
    async fn post_cmd(
        &self,
        cmd: SpotTradeCmdOrQuery,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotApiErrorAny> {
        let url = format!("{}/api/spot/v2/", self.base_url);

        self.client
            .post(&url)
            .json(&cmd)
            .send()
            .await
            .map_err(|e| {
                SpotApiErrorAny::Common(CommonError::Internal {
                    message: format!("HTTP request failed: {}", e),
                })
            })?
            .json::<CmdResp<SpotTradeResAny>>()
            .await
            .map_err(|e| {
                SpotApiErrorAny::Common(CommonError::Internal {
                    message: format!("Failed to parse response: {}", e),
                })
            })
    }
}

impl Handler<SpotTradeCmdOrQuery, SpotTradeResAny, SpotApiErrorAny> for RestfulClient {
    /// 处理交易命令/查询 - 通过 HTTP POST 发送到远程服务。
    async fn handle(
        &self,
        cmd: SpotTradeCmdOrQuery,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotApiErrorAny> {
        self.post_cmd(cmd).await
    }
}

#[cfg(test)]
mod tests {
    use base_types::cqrs::cqrs_types::CMetadata;
    use base_types::exchange::spot::spot_types::OrderType;
    use base_types::{OrderSide, TradingPair};
    use spot_behavior::proc::behavior::v2::spot_trade_behavior::{
        NewOrderCmd, SpotTradeCmd, TestNewOrderCmd,
    };

    use super::*;

    #[tokio::test]
    #[ignore = "需要先启动 axum_server 服务端"]
    async fn test_restful_client_test_new_order() {
        // 注意：需要先启动服务端 (axum_server)
        let client = RestfulClient::new("http://localhost:3001");

        let test_cmd = SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::TestNewOrder(TestNewOrderCmd::new(
            NewOrderCmd::new(
                CMetadata::default(),
                TradingPair::BtcUsdt,
                OrderSide::Buy,
                OrderType::Limit,
                None, // time_in_force
                None, // quantity
                None, // quote_order_qty
                None, // price
                None, // new_client_order_id
                None, // strategy_id
                None, // strategy_type
                None, // stop_price
                None, // trailing_delta
                None, // iceberg_qty
                None, // new_order_resp_type
                None, // self_trade_prevention_mode
                None, // peg_price_type
                None, // peg_offset_value
                None, // peg_offset_type
            ),
            None, // compute_commission_rates
        )));

        let result = client.handle(test_cmd).await;
        println!("Result: {:?}", result);
    }
}
