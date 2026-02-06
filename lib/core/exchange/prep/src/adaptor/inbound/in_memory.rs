//! 内存仓储实现

use std::collections::HashMap;

use crate::domain::entity::{Order, OrderId, Position, PositionId, PositionSide, Price, TraderId};
use crate::domain::repository::{OrderRepository, PositionRepository, RepositoryError};

/// 内存订单仓储
pub struct InMemoryOrderRepository {
    orders: HashMap<OrderId, Order>,
    next_id: OrderId,
}

impl InMemoryOrderRepository {
    pub fn new() -> Self {
        Self { orders: HashMap::new(), next_id: 1 }
    }
}

impl Default for InMemoryOrderRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderRepository for InMemoryOrderRepository {
    fn next_order_id(&mut self) -> OrderId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn save_order(&mut self, order: Order) -> Result<(), RepositoryError> {
        self.orders.insert(order.id, order);
        Ok(())
    }

    fn get_order(&self, id: OrderId) -> Option<&Order> {
        self.orders.get(&id)
    }

    fn get_order_mut(&mut self, id: OrderId) -> Option<&mut Order> {
        self.orders.get_mut(&id)
    }

    fn remove_order(&mut self, id: OrderId) -> Option<Order> {
        self.orders.remove(&id)
    }

    fn get_bids_at_price(&self, price: Price) -> Vec<&Order> {
        self.orders.values().filter(|o| o.is_active() && o.is_buy() && o.price == price).collect()
    }

    fn get_asks_at_price(&self, price: Price) -> Vec<&Order> {
        self.orders.values().filter(|o| o.is_active() && !o.is_buy() && o.price == price).collect()
    }

    fn best_bid(&self) -> Option<Price> {
        self.orders.values().filter(|o| o.is_active() && o.is_buy()).map(|o| o.price).max()
    }

    fn best_ask(&self) -> Option<Price> {
        self.orders.values().filter(|o| o.is_active() && !o.is_buy()).map(|o| o.price).min()
    }

    fn get_bids(&self) -> Vec<&Order> {
        let mut bids: Vec<_> =
            self.orders.values().filter(|o| o.is_active() && o.is_buy()).collect();
        bids.sort_by(|a, b| b.price.cmp(&a.price).then(a.created_at.cmp(&b.created_at)));
        bids
    }

    fn get_asks(&self) -> Vec<&Order> {
        let mut asks: Vec<_> =
            self.orders.values().filter(|o| o.is_active() && !o.is_buy()).collect();
        asks.sort_by(|a, b| a.price.cmp(&b.price).then(a.created_at.cmp(&b.created_at)));
        asks
    }
}

/// 内存仓位仓储
pub struct InMemoryPositionRepository {
    positions: HashMap<PositionId, Position>,
    next_id: PositionId,
}

impl InMemoryPositionRepository {
    pub fn new() -> Self {
        Self { positions: HashMap::new(), next_id: 1 }
    }
}

impl Default for InMemoryPositionRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl PositionRepository for InMemoryPositionRepository {
    fn next_position_id(&mut self) -> PositionId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn save_position(&mut self, position: Position) -> Result<(), RepositoryError> {
        self.positions.insert(position.id, position);
        Ok(())
    }

    fn get_position(&self, id: PositionId) -> Option<&Position> {
        self.positions.get(&id)
    }

    fn get_position_mut(&mut self, id: PositionId) -> Option<&mut Position> {
        self.positions.get_mut(&id)
    }

    fn remove_position(&mut self, id: PositionId) -> Option<Position> {
        self.positions.remove(&id)
    }

    fn get_position_by_trader_side(
        &self,
        trader: TraderId,
        position_side: PositionSide,
    ) -> Option<&Position> {
        self.positions.values().find(|p| p.trader == trader && p.position_side == position_side)
    }

    fn get_position_by_trader_side_mut(
        &mut self,
        trader: TraderId,
        position_side: PositionSide,
    ) -> Option<&mut Position> {
        self.positions.values_mut().find(|p| p.trader == trader && p.position_side == position_side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::{OrderStatus, Side, TimeInForce};
    use crate::domain::service::{Command, CommandResult, MatchingService, PrepCommandHandler};

    fn create_service() -> MatchingService<InMemoryOrderRepository, InMemoryPositionRepository> {
        let order_repo = InMemoryOrderRepository::new();
        let position_repo = InMemoryPositionRepository::new();
        MatchingService::new(order_repo, position_repo)
    }

    #[test]
    fn test_limit_order_no_match() {
        let mut service = create_service();
        service.set_timestamp(1000);

        // 下买单，无对手方
        let result = service.handle(Command::LimitOrder {
            trader: 1,
            side: Side::Buy,
            price: 50000,
            quantity: 100,
            position_side: PositionSide::Long,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        });

        match result {
            CommandResult::LimitOrder { order_id, trades, remaining_quantity, status } => {
                assert_eq!(order_id, 1);
                assert!(trades.is_empty());
                assert_eq!(remaining_quantity, 100);
                assert_eq!(status, OrderStatus::New);
            }
            _ => panic!("Expected LimitOrder result"),
        }
    }

    #[test]
    fn test_limit_order_full_match() {
        let mut service = create_service();
        service.set_timestamp(1000);

        // Maker 下卖单
        service.handle(Command::LimitOrder {
            trader: 1,
            side: Side::Sell,
            price: 50000,
            quantity: 100,
            position_side: PositionSide::Short,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        });

        service.set_timestamp(2000);

        // Taker 下买单，完全匹配
        let result = service.handle(Command::LimitOrder {
            trader: 2,
            side: Side::Buy,
            price: 50000,
            quantity: 100,
            position_side: PositionSide::Long,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        });

        match result {
            CommandResult::LimitOrder { trades, remaining_quantity, status, .. } => {
                assert_eq!(trades.len(), 1);
                assert_eq!(trades[0].quantity(), 100);
                assert_eq!(trades[0].price(), 50000);
                assert_eq!(remaining_quantity, 0);
                assert_eq!(status, OrderStatus::Filled);
            }
            _ => panic!("Expected LimitOrder result"),
        }
    }

    #[test]
    fn test_limit_order_partial_match() {
        let mut service = create_service();
        service.set_timestamp(1000);

        // Maker 下卖单 50
        service.handle(Command::LimitOrder {
            trader: 1,
            side: Side::Sell,
            price: 50000,
            quantity: 50,
            position_side: PositionSide::Short,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        });

        service.set_timestamp(2000);

        // Taker 下买单 100，部分匹配
        let result = service.handle(Command::LimitOrder {
            trader: 2,
            side: Side::Buy,
            price: 50000,
            quantity: 100,
            position_side: PositionSide::Long,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        });

        match result {
            CommandResult::LimitOrder { trades, remaining_quantity, status, .. } => {
                assert_eq!(trades.len(), 1);
                assert_eq!(trades[0].quantity(), 50);
                assert_eq!(remaining_quantity, 50);
                assert_eq!(status, OrderStatus::PartiallyFilled);
            }
            _ => panic!("Expected LimitOrder result"),
        }
    }

    #[test]
    fn test_fok_order_cancelled() {
        let mut service = create_service();
        service.set_timestamp(1000);

        // Maker 下卖单 50
        service.handle(Command::LimitOrder {
            trader: 1,
            side: Side::Sell,
            price: 50000,
            quantity: 50,
            position_side: PositionSide::Short,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        });

        service.set_timestamp(2000);

        // FOK 买单 100，无法全部成交，取消
        let result = service.handle(Command::LimitOrder {
            trader: 2,
            side: Side::Buy,
            price: 50000,
            quantity: 100,
            position_side: PositionSide::Long,
            reduce_only: false,
            time_in_force: TimeInForce::FOK,
        });

        match result {
            CommandResult::LimitOrder { trades, remaining_quantity, status, .. } => {
                assert!(trades.is_empty());
                assert_eq!(remaining_quantity, 100);
                assert_eq!(status, OrderStatus::Cancelled);
            }
            _ => panic!("Expected LimitOrder result"),
        }
    }

    #[test]
    fn test_ioc_order() {
        let mut service = create_service();
        service.set_timestamp(1000);

        // Maker 下卖单 50
        service.handle(Command::LimitOrder {
            trader: 1,
            side: Side::Sell,
            price: 50000,
            quantity: 50,
            position_side: PositionSide::Short,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        });

        service.set_timestamp(2000);

        // IOC 买单 100，部分成交
        let result = service.handle(Command::LimitOrder {
            trader: 2,
            side: Side::Buy,
            price: 50000,
            quantity: 100,
            position_side: PositionSide::Long,
            reduce_only: false,
            time_in_force: TimeInForce::IOC,
        });

        match result {
            CommandResult::LimitOrder { trades, remaining_quantity, status, .. } => {
                assert_eq!(trades.len(), 1);
                assert_eq!(trades[0].quantity(), 50);
                assert_eq!(remaining_quantity, 50);
                assert_eq!(status, OrderStatus::PartiallyFilled);
            }
            _ => panic!("Expected LimitOrder result"),
        }
    }

    #[test]
    fn test_cancel_order() {
        let mut service = create_service();
        service.set_timestamp(1000);

        // 下单
        let result = service.handle(Command::LimitOrder {
            trader: 1,
            side: Side::Buy,
            price: 50000,
            quantity: 100,
            position_side: PositionSide::Long,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        });

        let order_id = match result {
            CommandResult::LimitOrder { order_id, .. } => order_id,
            _ => panic!("Expected order_id"),
        };

        // 取消
        let cancel_result = service.handle(Command::CancelOrder { order_id });

        match cancel_result {
            CommandResult::CancelOrder { success, cancelled_quantity, .. } => {
                assert!(success);
                assert_eq!(cancelled_quantity, 100);
            }
            _ => panic!("Expected CancelOrder result"),
        }
    }
}
