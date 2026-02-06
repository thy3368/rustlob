//! 撮合服务
//!
//! 实现永续合约订单撮合逻辑

use crate::domain::ErrorCode;
use crate::domain::entity::{
    Leverage, MarginMode, Order, OrderId, OrderStatus, Position, PositionSide, Price, Quantity,
    Side, TimeInForce, Timestamp, Trade, TraderId,
};
use crate::domain::repository::{OrderRepository, PositionRepository};
use crate::domain::service::command::{Command, CommandResult, PrepCommandHandler};

/// 撮合服务
pub struct MatchingService<O, P>
where
    O: OrderRepository,
    P: PositionRepository,
{
    /// 订单仓储
    order_repo: O,
    /// 仓位仓储
    position_repo: P,
    /// 成交ID计数器
    trade_id_counter: u64,
    /// 当前时间戳
    current_timestamp: Timestamp,
    /// 默认杠杆
    default_leverage: Leverage,
    /// 默认保证金模式
    default_margin_mode: MarginMode,
}

impl<O, P> MatchingService<O, P>
where
    O: OrderRepository,
    P: PositionRepository,
{
    /// 创建撮合服务
    pub fn new(order_repo: O, position_repo: P) -> Self {
        Self {
            order_repo,
            position_repo,
            trade_id_counter: 0,
            current_timestamp: 0,
            default_leverage: 10,
            default_margin_mode: MarginMode::Cross,
        }
    }

    /// 设置当前时间戳
    pub fn set_timestamp(&mut self, ts: Timestamp) {
        self.current_timestamp = ts;
    }

    /// 生成成交ID
    fn next_trade_id(&mut self) -> u64 {
        self.trade_id_counter += 1;
        self.trade_id_counter
    }

    /// 处理限价单
    pub fn handle_limit_order(
        &mut self,
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        position_side: PositionSide,
        reduce_only: bool,
        time_in_force: TimeInForce,
    ) -> CommandResult {
        // 验证参数
        if quantity == 0 {
            return CommandResult::Error {
                code: ErrorCode::InvalidQuantity,
                message: "数量不能为0".to_string(),
            };
        }
        if price == 0 {
            return CommandResult::Error {
                code: ErrorCode::InvalidPrice,
                message: "价格不能为0".to_string(),
            };
        }

        // reduce_only 验证
        if reduce_only {
            let position = self.position_repo.get_position_by_trader_side(trader, position_side);
            match position {
                None => {
                    return CommandResult::Error {
                        code: ErrorCode::ReduceOnlyRejected,
                        message: "无持仓，只减仓订单被拒绝".to_string(),
                    };
                }
                Some(pos) if pos.quantity < quantity => {
                    return CommandResult::Error {
                        code: ErrorCode::ReduceOnlyRejected,
                        message: "只减仓数量超过持仓".to_string(),
                    };
                }
                _ => {}
            }
        }

        // 创建订单
        let order_id = self.order_repo.next_order_id();
        let order = Order::new(
            order_id,
            trader,
            side,
            price,
            quantity,
            position_side,
            reduce_only,
            time_in_force,
            self.current_timestamp,
        );

        // 撮合
        let (trades, remaining) = self.match_order(order);

        // 处理 TimeInForce
        let status = match time_in_force {
            TimeInForce::FOK => {
                if remaining > 0 {
                    return CommandResult::LimitOrder {
                        order_id,
                        trades: vec![],
                        remaining_quantity: quantity,
                        status: OrderStatus::Cancelled,
                    };
                }
                OrderStatus::Filled
            }
            TimeInForce::IOC => {
                if remaining > 0 && !trades.is_empty() {
                    OrderStatus::PartiallyFilled
                } else if trades.is_empty() {
                    OrderStatus::Cancelled
                } else {
                    OrderStatus::Filled
                }
            }
            TimeInForce::PostOnly => {
                if !trades.is_empty() {
                    return CommandResult::Error {
                        code: ErrorCode::InvalidPrice,
                        message: "PostOnly 订单会立即成交".to_string(),
                    };
                }
                OrderStatus::New
            }
            _ => {
                if remaining == 0 {
                    OrderStatus::Filled
                } else if !trades.is_empty() {
                    OrderStatus::PartiallyFilled
                } else {
                    OrderStatus::New
                }
            }
        };

        // 剩余数量挂单
        if remaining > 0
            && matches!(
                time_in_force,
                TimeInForce::GTC | TimeInForce::GTD { .. } | TimeInForce::PostOnly
            )
        {
            let remaining_order = Order::new(
                order_id,
                trader,
                side,
                price,
                remaining,
                position_side,
                reduce_only,
                time_in_force,
                self.current_timestamp,
            );
            let _ = self.order_repo.save_order(remaining_order);
        }

        CommandResult::LimitOrder { order_id, trades, remaining_quantity: remaining, status }
    }

    /// 撮合订单
    fn match_order(&mut self, order: Order) -> (Vec<Trade>, Quantity) {
        let mut trades = Vec::new();
        let mut remaining = order.remaining_quantity;

        // 收集可匹配的对手方订单信息
        let matches: Vec<(OrderId, TraderId, Price, Quantity, PositionSide, bool)> = match order
            .side
        {
            Side::Buy => self
                .order_repo
                .get_asks()
                .iter()
                .filter(|o| o.is_active() && order.can_match(o.price))
                .map(|o| {
                    (o.id, o.trader, o.price, o.remaining_quantity, o.position_side, o.reduce_only)
                })
                .collect(),
            Side::Sell => self
                .order_repo
                .get_bids()
                .iter()
                .filter(|o| o.is_active() && o.can_match(order.price))
                .map(|o| {
                    (o.id, o.trader, o.price, o.remaining_quantity, o.position_side, o.reduce_only)
                })
                .collect(),
        };

        for (
            opposite_id,
            opposite_trader,
            opposite_price,
            opposite_qty,
            opposite_pos_side,
            opposite_reduce_only,
        ) in matches
        {
            if remaining == 0 {
                break;
            }

            let match_qty = remaining.min(opposite_qty);
            let match_price = opposite_price;

            // 更新对手方订单
            if let Some(opposite_order) = self.order_repo.get_order_mut(opposite_id) {
                opposite_order.fill(match_qty, self.current_timestamp);
            }

            // 计算手续费
            let fee = self.calc_fee(match_qty, match_price, false);
            let trade_id = self.next_trade_id();

            // 创建成交记录
            let trade = Trade::new(
                trade_id,
                order.id,
                match_price,
                match_qty,
                order.side,
                order.position_side,
                self.current_timestamp,
                fee,
                0,     // realized_pnl
                false, // is_maker (taker)
            );

            // 更新仓位
            self.update_positions(
                order.trader,
                opposite_trader,
                order.side,
                order.position_side,
                opposite_pos_side,
                match_qty,
                match_price,
                order.reduce_only,
                opposite_reduce_only,
            );

            trades.push(trade);
            remaining -= match_qty;

            // 移除已完成订单
            if opposite_qty == match_qty {
                self.order_repo.remove_order(opposite_id);
            }
        }

        (trades, remaining)
    }

    /// 更新仓位
    fn update_positions(
        &mut self,
        taker_trader: TraderId,
        maker_trader: TraderId,
        taker_side: Side,
        taker_position_side: PositionSide,
        maker_position_side: PositionSide,
        quantity: Quantity,
        price: Price,
        taker_reduce_only: bool,
        maker_reduce_only: bool,
    ) {
        // 更新 Taker 仓位
        self.update_single_position(
            taker_trader,
            taker_side,
            taker_position_side,
            quantity,
            price,
            taker_reduce_only,
        );

        // 更新 Maker 仓位
        let maker_side = match taker_side {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        };
        self.update_single_position(
            maker_trader,
            maker_side,
            maker_position_side,
            quantity,
            price,
            maker_reduce_only,
        );
    }

    /// 更新单个仓位
    fn update_single_position(
        &mut self,
        trader: TraderId,
        side: Side,
        position_side: PositionSide,
        quantity: Quantity,
        price: Price,
        reduce_only: bool,
    ) {
        let is_opening = match (side, position_side) {
            (Side::Buy, PositionSide::Long) | (Side::Buy, PositionSide::Both) => !reduce_only,
            (Side::Sell, PositionSide::Short) => !reduce_only,
            (Side::Sell, PositionSide::Long) | (Side::Sell, PositionSide::Both) => false,
            (Side::Buy, PositionSide::Short) => false,
        };

        // 先检查是否有仓位，获取ID
        let position_id =
            self.position_repo.get_position_by_trader_side(trader, position_side).map(|p| p.id);

        if let Some(pos_id) = position_id {
            if let Some(position) = self.position_repo.get_position_mut(pos_id) {
                if is_opening {
                    position.add(quantity, price, self.current_timestamp);
                } else {
                    position.reduce(quantity, price, self.current_timestamp);
                    if position.is_empty() {
                        self.position_repo.remove_position(pos_id);
                    }
                }
            }
        } else if is_opening {
            // 创建新仓位
            let position_id = self.position_repo.next_position_id();
            let margin = self.calc_margin(quantity, price, self.default_leverage);
            let position = Position::new(
                position_id,
                trader,
                position_side,
                quantity,
                price,
                self.default_margin_mode,
                self.default_leverage,
                margin,
                self.current_timestamp,
            );
            let _ = self.position_repo.save_position(position);
        }
    }

    /// 计算保证金
    fn calc_margin(&self, quantity: Quantity, price: Price, leverage: Leverage) -> u64 {
        (quantity * price) / leverage as u64
    }

    /// 计算手续费
    fn calc_fee(&self, quantity: Quantity, price: Price, is_maker: bool) -> u64 {
        let fee_rate = if is_maker { 2 } else { 5 }; // 0.02% maker, 0.05% taker
        quantity * price * fee_rate / 100_000
    }
}

impl<O, P> PrepCommandHandler for MatchingService<O, P>
where
    O: OrderRepository,
    P: PositionRepository,
{
    fn handle(&mut self, command: Command) -> CommandResult {
        match command {
            Command::LimitOrder {
                trader,
                side,
                price,
                quantity,
                position_side,
                reduce_only,
                time_in_force,
            } => self.handle_limit_order(
                trader,
                side,
                price,
                quantity,
                position_side,
                reduce_only,
                time_in_force,
            ),

            Command::CancelOrder { order_id } => {
                if let Some(order) = self.order_repo.get_order_mut(order_id) {
                    let cancelled_qty = order.remaining_quantity;
                    order.cancel(self.current_timestamp);
                    self.order_repo.remove_order(order_id);
                    CommandResult::CancelOrder {
                        order_id,
                        success: true,
                        cancelled_quantity: cancelled_qty,
                    }
                } else {
                    CommandResult::Error {
                        code: ErrorCode::OrderNotFound,
                        message: "订单不存在".to_string(),
                    }
                }
            }

            _ => CommandResult::Error {
                code: ErrorCode::SystemError,
                message: "命令未实现".to_string(),
            },
        }
    }

    fn handler_name(&self) -> &'static str {
        "PrepMatchingService"
    }
}
