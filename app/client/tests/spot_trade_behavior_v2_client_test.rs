use std::error::Error;

use base_types::cqrs::cqrs_types::CMetadata;
use sapp::client::spot::{
    spot_user_data_websocket_stream_client::SpotUserDataWebSocketStreamClient
};
use sapp::client::spot::spot_http_client::SpotHttpClient;
use spot_behavior::proc::behavior::v2::spot_trade_behavior_v2::{
    NewOrderCmd, NewOrderRespType, OrderSide, OrderType, SpotTradeBehaviorV2, SpotTradeCmdAny, TestNewOrderCmd,
    TimeInForce
};


#[tokio::test]
async fn test_send_limit_order() -> Result<(), Box<dyn Error>> {
    // SpotTradeV2HttpClient下发订单，SpotUserDataWebSocketStreamClient接收user data
    // event

    // 创建WebSocket客户端并连接
    let ws_client = SpotUserDataWebSocketStreamClient::new("ws://localhost:8084");
    ws_client.connect().await?;

    // 确保WebSocket连接已建立
    assert!(ws_client.is_connected().await);

    // 创建HTTP客户端实例
    let mut http_client = SpotHttpClient::new("http://localhost:3001");

    // 构建NewOrder命令
    let new_order_cmd = SpotTradeCmdAny::NewOrder(NewOrderCmd {
        metadata: CMetadata {
            command_id: "test_limit_order_123".to_string(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            correlation_id: None,
            causation_id: None,
            actor: Some("test_user".to_string()),
            attributes: Vec::new()
        },
        symbol: "BTCUSDT".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        time_in_force: Some(TimeInForce::GTC),
        quantity: Some(0.1),
        quote_order_qty: None,
        price: Some(45000.0),
        new_client_order_id: Some("client_order_456".to_string()),
        strategy_id: None,
        strategy_type: None,
        stop_price: None,
        trailing_delta: None,
        iceberg_qty: None,
        new_order_resp_type: Some(NewOrderRespType::ACK),
        self_trade_prevention_mode: None,
        peg_price_type: None,
        peg_offset_value: None,
        peg_offset_type: None,
        recv_window: None,
        timestamp: chrono::Utc::now().timestamp_millis()
    });

    // 发送命令
    println!("发送NewOrder命令...");
    let result = http_client.handle(new_order_cmd).await;

    // 验证结果
    match result {
        Ok(resp) => {
            println!("✅ 成功: {:?}", resp);
            // 检查响应是否包含预期字段
            // assert!(resp.metadata.command_id.len() > 0);
        }
        Err(e) => {
            println!("❌ 失败: {:?}", e);
            panic!("测试失败: {}", e);
        }
    }

    // 等待一段时间以接收用户数据事件
    println!("等待用户数据事件...");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // 发送测试订单
    println!("发送TestNewOrder命令...");
    let test_order_cmd = SpotTradeCmdAny::TestNewOrder(TestNewOrderCmd {
        new_order: NewOrderCmd {
            metadata: CMetadata {
                command_id: "test_test_order_789".to_string(),
                timestamp: chrono::Utc::now().timestamp_millis() as u64,
                correlation_id: None,
                causation_id: None,
                actor: Some("test_user".to_string()),
                attributes: Vec::new()
            },
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(1.0),
            quote_order_qty: None,
            price: Some(1800.0),
            new_client_order_id: Some("test_client_order_789".to_string()),
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: Some(NewOrderRespType::ACK),
            self_trade_prevention_mode: None,
            peg_price_type: None,
            peg_offset_value: None,
            peg_offset_type: None,
            recv_window: None,
            timestamp: chrono::Utc::now().timestamp_millis()
        },
        compute_commission_rates: Some(false)
    });

    let test_result = http_client.handle(test_order_cmd).await;

    match test_result {
        Ok(resp) => {
            println!("✅ 测试订单成功: {:?}", resp);
        }
        Err(e) => {
            println!("❌ 测试订单失败: {:?}", e);
            panic!("测试订单失败: {}", e);
        }
    }

    // 再次等待用户数据事件
    println!("再次等待用户数据事件...");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    Ok(())
}
