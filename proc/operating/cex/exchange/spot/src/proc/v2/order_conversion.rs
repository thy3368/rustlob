//! NewOrderCmd 到 SpotOrder 的转换
//!
//! 将命令对象转换为领域实体，支持多种订单类型

use base_types::base_types::TraderId;
use base_types::exchange::spot::spot_types::{
    ConditionalType, ExecutionMethod, OrderType, SpotOrder, TimeInForce,
};
use base_types::{Price, Quantity};

use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;
use crate::proc::v2::id_repo::order_next_id;

impl From<NewOrderCmd> for SpotOrder {
    #[inline(always)]
    fn from(cmd: NewOrderCmd) -> Self {
        let order_id = order_next_id as u64;

        let trader_id = TraderId::default(); // TODO: 从 metadata 中获取真实的 trader_id
        let trading_pair = cmd.symbol().clone();

        // todo 可以simd优化吗
        let order_type = *cmd.order_type();
        match order_type {
            OrderType::Limit => {
                // 限价单 - 直接使用命令字段创建 SpotOrder，零拷贝
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().unwrap_or_default().clone(),
                );
                order.order_type = order_type;
                order.state.status = base_types::exchange::spot::spot_types::OrderStatus::Pending;

                order
            }
            OrderType::Market => {
                // 市价单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    Price::from_f64(0.0), // 市价单价格为0
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC, // 市价单默认IOC
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().unwrap_or_default().clone(),
                );
                order.order_type = order_type;
                order.execution_method = ExecutionMethod::Market;
                order.state.status = base_types::exchange::spot::spot_types::OrderStatus::Pending;
                order.price = None; // 市价单价格为None
                order
            }
            OrderType::StopLoss => {
                // 止损单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    Price::from_f64(0.0), // 市价止损
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC,
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().unwrap_or_default().clone(),
                );
                order.order_type = order_type;
                order.conditional_type = ConditionalType::StopLoss;
                order.stop_price = *cmd.stop_price();
                order.execution_method = ExecutionMethod::Market;
                order.price = None;
                order.state.status =
                    base_types::exchange::spot::spot_types::OrderStatus::ConditionalPending;
                order
            }
            OrderType::StopLossLimit => {
                // 止损限价单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().unwrap_or_default().clone(),
                );
                order.order_type = order_type;
                order.conditional_type = ConditionalType::StopLoss;
                order.execution_method = ExecutionMethod::Limit;
                order.stop_price = *cmd.stop_price();
                order.state.status =
                    base_types::exchange::spot::spot_types::OrderStatus::ConditionalPending;
                order
            }
            OrderType::TakeProfit => {
                // 止盈单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    Price::from_f64(0.0), // 市价止盈
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC,
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().unwrap_or_default().clone(),
                );
                order.order_type = order_type;
                order.conditional_type = ConditionalType::TakeProfit;
                order.stop_price = *cmd.stop_price();
                order.execution_method = ExecutionMethod::Market;
                order.price = None;
                order.state.status =
                    base_types::exchange::spot::spot_types::OrderStatus::ConditionalPending;
                order
            }
            OrderType::TakeProfitLimit => {
                // 止盈限价单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().unwrap_or_default().clone(),
                );
                order.order_type = order_type;
                order.conditional_type = ConditionalType::TakeProfit;
                order.execution_method = ExecutionMethod::Limit;
                order.stop_price = *cmd.stop_price();
                order.state.status =
                    base_types::exchange::spot::spot_types::OrderStatus::ConditionalPending;
                order
            }
            OrderType::LimitMaker => {
                // 限价只挂单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::GTX, // GTX = PostOnly
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().unwrap_or_default().clone(),
                );
                order.order_type = order_type;
                order
            }
        }
    }
}
