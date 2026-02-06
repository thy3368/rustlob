//! 仓储接口定义
//!
//! 遵循 Clean Architecture，仓储接口定义在领域层

use crate::domain::entity::{Order, OrderId, Position, PositionId, PositionSide, Price, TraderId};

/// 仓储错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    /// 未找到
    NotFound,
    /// 重复
    Duplicate,
    /// 容量已满
    CapacityFull,
}

/// 订单仓储接口
pub trait OrderRepository: Send + Sync {
    /// 生成订单ID
    fn next_order_id(&mut self) -> OrderId;

    /// 保存订单
    fn save_order(&mut self, order: Order) -> Result<(), RepositoryError>;

    /// 获取订单
    fn get_order(&self, id: OrderId) -> Option<&Order>;

    /// 获取可变订单
    fn get_order_mut(&mut self, id: OrderId) -> Option<&mut Order>;

    /// 删除订单
    fn remove_order(&mut self, id: OrderId) -> Option<Order>;

    /// 获取某价位的买单（按时间优先）
    fn get_bids_at_price(&self, price: Price) -> Vec<&Order>;

    /// 获取某价位的卖单（按时间优先）
    fn get_asks_at_price(&self, price: Price) -> Vec<&Order>;

    /// 获取最优买价
    fn best_bid(&self) -> Option<Price>;

    /// 获取最优卖价
    fn best_ask(&self) -> Option<Price>;

    /// 获取买单（价格从高到低）
    fn get_bids(&self) -> Vec<&Order>;

    /// 获取卖单（价格从低到高）
    fn get_asks(&self) -> Vec<&Order>;
}

/// 仓位仓储接口
pub trait PositionRepository: Send + Sync {
    /// 生成仓位ID
    fn next_position_id(&mut self) -> PositionId;

    /// 保存仓位
    fn save_position(&mut self, position: Position) -> Result<(), RepositoryError>;

    /// 获取仓位
    fn get_position(&self, id: PositionId) -> Option<&Position>;

    /// 获取可变仓位
    fn get_position_mut(&mut self, id: PositionId) -> Option<&mut Position>;

    /// 删除仓位
    fn remove_position(&mut self, id: PositionId) -> Option<Position>;

    /// 获取用户某方向的仓位
    fn get_position_by_trader_side(
        &self,
        trader: TraderId,
        position_side: PositionSide,
    ) -> Option<&Position>;

    /// 获取用户某方向的可变仓位
    fn get_position_by_trader_side_mut(
        &mut self,
        trader: TraderId,
        position_side: PositionSide,
    ) -> Option<&mut Position>;
}
