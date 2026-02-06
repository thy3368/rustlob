use async_trait::async_trait;
use serde_json::Value;
use crate::{Config, Tool, ToolResult};
use crate::tools::ToolContext;
use spot_behavior::proc::behavior::v2::spot_trade_behavior_v2::{
    SpotTradeCmdAny, NewOrderCmd, TestNewOrderCmd, CancelOrderCmd,
};
use base_types::{OrderSide, Price, Quantity, Timestamp, TradingPair};
use base_types::cqrs::cqrs_types::CMetadata;
use base_types::exchange::spot::spot_types::{OrderType, TimeInForce};
use base_types::handler::handler::Handler;
use base_types::Decimal;
use client::client::spot::spot_http_client::SpotHttpClient;

pub struct TradeTool;

#[async_trait]
impl Tool for TradeTool {
    fn name(&self) -> &str {
        "trade_ops"
    }

    fn description(&self) -> &str {
        "Spot交易操作工具，支持下单、测试下单、取消订单等操作"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "交易命令类型: new_order(下单), test_order(测试下单), cancel_order(取消订单)",
                    "enum": ["new_order", "test_order", "cancel_order"]
                },
                "symbol": {
                    "type": "string",
                    "description": "交易对，如 BTCUSDT"
                },
                "side": {
                    "type": "string",
                    "description": "订单方向: BUY(买入) 或 SELL(卖出)",
                    "enum": ["BUY", "SELL"]
                },
                "order_type": {
                    "type": "string",
                    "description": "订单类型: LIMIT(限价单), MARKET(市价单), STOP_LOSS(止损单), STOP_LOSS_LIMIT(止损限价单), TAKE_PROFIT(止盈单), TAKE_PROFIT_LIMIT(止盈限价单), LIMIT_MAKER(限价挂单)",
                    "enum": ["LIMIT", "MARKET", "STOP_LOSS", "STOP_LOSS_LIMIT", "TAKE_PROFIT", "TAKE_PROFIT_LIMIT", "LIMIT_MAKER"]
                },
                "quantity": {
                    "type": "number",
                    "description": "订单数量"
                },
                "price": {
                    "type": "number",
                    "description": "订单价格(限价单必填)"
                },
                "time_in_force": {
                    "type": "string",
                    "description": "有效时间: GTC(一直有效), IOC(立即成交或取消), FOK(全部成交或取消)",
                    "enum": ["GTC", "IOC", "FOK"]
                },
                "order_id": {
                    "type": "integer",
                    "description": "订单ID(取消订单时必填)"
                },
                "orig_client_order_id": {
                    "type": "string",
                    "description": "原始客户端订单ID(取消订单时可选)"
                },
                "new_client_order_id": {
                    "type": "string",
                    "description": "新客户端订单ID(可选)"
                }
            },
            "required": ["command", "symbol"],
            "if": {
                "properties": { "command": { "const": "new_order" } }
            },
            "then": {
                "required": ["side", "order_type", "quantity"]
            },
            "else if": {
                "properties": { "command": { "const": "test_order" } }
            },
            "then": {
                "required": ["side", "order_type", "quantity"]
            },
            "else if": {
                "properties": { "command": { "const": "cancel_order" } }
            },
            "then": {
                "required": ["order_id"]
            }
        })
    }

    fn is_read_only(&self) -> bool {
        false
    }

    async fn execute(&self, args: Value, _ctx: &ToolContext, _config: &Config) -> crate::Result<ToolResult> {
        let command = args["command"].as_str().unwrap_or("").to_string();
        let symbol = args["symbol"].as_str().unwrap_or("").to_string();

        // 创建 SpotHttpClient 实例（默认连接到本地服务）
        let client = SpotHttpClient::default();

        match command.as_str() {
            "new_order" => self.execute_new_order(&client, args.clone(), &symbol).await,
            "test_order" => self.execute_test_order(&client, args.clone(), &symbol).await,
            "cancel_order" => self.execute_cancel_order(&client, args, &symbol).await,
            _ => Ok(ToolResult::error(format!("不支持的命令类型: {}", command))),
        }
    }

    fn validate_args(&self, args: &Value) -> crate::Result<()> {
        // 调用默认验证方法
        let schema = self.parameters();
        if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
            for field in required {
                if let Some(field_name) = field.as_str() {
                    if args.get(field_name).is_none() {
                        return Err(crate::error::ToolError::InvalidArgs(format!(
                            "缺少必填字段: {}", field_name
                        )).into());
                    }
                }
            }
        }

        // 验证命令特定的必填字段
        let command = args["command"].as_str().unwrap_or("");
        match command {
            "new_order" | "test_order" => {
                if args.get("side").is_none() || args.get("order_type").is_none() || args.get("quantity").is_none() {
                    return Err(crate::error::ToolError::InvalidArgs(
                        "下单命令需要 side、order_type 和 quantity 字段".to_string()
                    ).into());
                }
                // 限价单需要价格
                let order_type = args["order_type"].as_str().unwrap_or("");
                if order_type == "LIMIT" || order_type == "STOP_LOSS_LIMIT" || order_type == "TAKE_PROFIT_LIMIT" || order_type == "LIMIT_MAKER" {
                    if args.get("price").is_none() {
                        return Err(crate::error::ToolError::InvalidArgs(
                            "限价单需要 price 字段".to_string()
                        ).into());
                    }
                }
            },
            "cancel_order" => {
                if args.get("order_id").is_none() && args.get("orig_client_order_id").is_none() {
                    return Err(crate::error::ToolError::InvalidArgs(
                        "取消订单命令需要 order_id 或 orig_client_order_id 字段".to_string()
                    ).into());
                }
            },
            _ => return Err(crate::error::ToolError::InvalidArgs(
                format!("不支持的命令类型: {}", command)
            ).into()),
        }

        Ok(())
    }

    fn to_definition(&self) -> Value {
        serde_json::json!({
            "name": self.name(),
            "description": self.description(),
            "parameters": self.parameters(),
        })
    }
}

impl TradeTool {
    async fn execute_new_order(&self, client: &SpotHttpClient, args: Value, symbol: &str) -> crate::Result<ToolResult> {
        let side = match args["side"].as_str().unwrap_or("") {
            "BUY" => OrderSide::Buy,
            "SELL" => OrderSide::Sell,
            _ => return Ok(ToolResult::error("无效的订单方向".to_string())),
        };

        let order_type = match args["order_type"].as_str().unwrap_or("") {
            "LIMIT" => OrderType::Limit,
            "MARKET" => OrderType::Market,
            "STOP_LOSS" => OrderType::StopLoss,
            "STOP_LOSS_LIMIT" => OrderType::StopLossLimit,
            "TAKE_PROFIT" => OrderType::TakeProfit,
            "TAKE_PROFIT_LIMIT" => OrderType::TakeProfitLimit,
            "LIMIT_MAKER" => OrderType::LimitMaker,
            _ => return Ok(ToolResult::error("无效的订单类型".to_string())),
        };

        let time_in_force = args["time_in_force"].as_str().map(|s| match s {
            "GTC" => TimeInForce::GTC,
            "IOC" => TimeInForce::IOC,
            "FOK" => TimeInForce::FOK,
            _ => TimeInForce::GTC,
        });

        let quantity = Decimal::from_f64(args["quantity"].as_f64().unwrap_or(0.0));
        let price = args["price"].as_f64().map(Decimal::from_f64);
        let new_client_order_id = args["new_client_order_id"].as_str().map(|s| s.to_string());

        let trading_pair = match symbol {
            "BTCUSDT" => TradingPair::BtcUsdt,
            "ETHUSDT" => TradingPair::EthUsdt,
            "BTCETH" => TradingPair::BtcEth,
            _ => return Ok(ToolResult::error(format!("不支持的交易对: {}", symbol))),
        };

        let cmd = SpotTradeCmdAny::NewOrder(NewOrderCmd::new(
            CMetadata::default(),
            trading_pair,
            side,
            order_type,
            time_in_force,
            Some(quantity),
            None,
            price,
            new_client_order_id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Timestamp::default(),
        ));

        match client.handle(cmd).await {
            Ok(resp) => Ok(ToolResult::success(format!("下单成功: {:?}", resp))),
            Err(e) => Ok(ToolResult::error(format!("下单失败: {:?}", e))),
        }
    }

    async fn execute_test_order(&self, client: &SpotHttpClient, args: Value, symbol: &str) -> crate::Result<ToolResult> {
        let side = match args["side"].as_str().unwrap_or("") {
            "BUY" => OrderSide::Buy,
            "SELL" => OrderSide::Sell,
            _ => return Ok(ToolResult::error("无效的订单方向".to_string())),
        };

        let order_type = match args["order_type"].as_str().unwrap_or("") {
            "LIMIT" => OrderType::Limit,
            "MARKET" => OrderType::Market,
            "STOP_LOSS" => OrderType::StopLoss,
            "STOP_LOSS_LIMIT" => OrderType::StopLossLimit,
            "TAKE_PROFIT" => OrderType::TakeProfit,
            "TAKE_PROFIT_LIMIT" => OrderType::TakeProfitLimit,
            "LIMIT_MAKER" => OrderType::LimitMaker,
            _ => return Ok(ToolResult::error("无效的订单类型".to_string())),
        };

        let time_in_force = args["time_in_force"].as_str().map(|s| match s {
            "GTC" => TimeInForce::GTC,
            "IOC" => TimeInForce::IOC,
            "FOK" => TimeInForce::FOK,
            _ => TimeInForce::GTC,
        });

        let quantity = Decimal::from_f64(args["quantity"].as_f64().unwrap_or(0.0));
        let price = args["price"].as_f64().map(Decimal::from_f64);
        let new_client_order_id = args["new_client_order_id"].as_str().map(|s| s.to_string());

        let trading_pair = match symbol {
            "BTCUSDT" => TradingPair::BtcUsdt,
            "ETHUSDT" => TradingPair::EthUsdt,
            "BTCETH" => TradingPair::BtcEth,
            _ => return Ok(ToolResult::error(format!("不支持的交易对: {}", symbol))),
        };

        let new_order_cmd = NewOrderCmd::new(
            CMetadata::default(),
            trading_pair,
            side,
            order_type,
            time_in_force,
            Some(quantity),
            None,
            price,
            new_client_order_id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Timestamp::default(),
        );

        let cmd = SpotTradeCmdAny::TestNewOrder(TestNewOrderCmd::new(
            new_order_cmd,
            Some(false),
        ));

        match client.handle(cmd).await {
            Ok(resp) => Ok(ToolResult::success(format!("测试下单成功: {:?}", resp))),
            Err(e) => Ok(ToolResult::error(format!("测试下单失败: {:?}", e))),
        }
    }

    async fn execute_cancel_order(&self, client: &SpotHttpClient, args: Value, symbol: &str) -> crate::Result<ToolResult> {
        let order_id = args["order_id"].as_i64().map(|v| v as u64);
        let orig_client_order_id = args["orig_client_order_id"].as_str().map(|s| s.to_string());
        let new_client_order_id = args["new_client_order_id"].as_str().map(|s| s.to_string());

        let trading_pair = match symbol {
            "BTCUSDT" => TradingPair::BtcUsdt,
            "ETHUSDT" => TradingPair::EthUsdt,
            "BTCETH" => TradingPair::BtcEth,
            _ => return Ok(ToolResult::error(format!("不支持的交易对: {}", symbol))),
        };

        let cmd = SpotTradeCmdAny::CancelOrder(CancelOrderCmd::new(
            CMetadata::default(),
            trading_pair,
            order_id,
            orig_client_order_id,
            new_client_order_id,
            None,
            None,
            Timestamp::default(),
        ));

        match client.handle(cmd).await {
            Ok(resp) => Ok(ToolResult::success(format!("取消订单成功: {:?}", resp))),
            Err(e) => Ok(ToolResult::error(format!("取消订单失败: {:?}", e))),
        }
    }
}