use crate::lob::domain::entity::lob_types::Price;
/// 市场数据查询服务，提供行情数据服务
///
/// 提供订单簿市场数据查询功能
use crate::lob::domain::repo::OrderRepo;


/// 市场数据查询服务
///
/// 提供订单簿市场数据查询功能
pub struct MarketDataService<R>
where
    R: OrderRepo
{
    repository: R
}

impl<R> MarketDataService<R>
where
    R: OrderRepo
{
    /// 创建新的市场数据服务
    pub fn new(repository: R) -> Self {
        Self {
            repository
        }
    }

    /// 获取repository的不可变引用
    pub fn repository(&self) -> &R { &self.repository }

    /// 查找最佳买价（O(1) 缓存访问）
    pub fn find_best_bid(&self) -> Option<Price> { self.repository.best_bid() }

    /// 查找最佳卖价（O(1) 缓存访问）
    pub fn find_best_ask(&self) -> Option<Price> { self.repository.best_ask() }

    /// 计算买卖价差
    pub fn calculate_spread(&self) -> Option<Price> {
        match (self.find_best_ask(), self.find_best_bid()) {
            (Some(ask), Some(bid)) if ask > bid => Some(ask - bid),
            _ => None
        }
    }

    /// 计算中间价
    pub fn calculate_mid_price(&self) -> Option<Price> {
        match (self.find_best_ask(), self.find_best_bid()) {
            (Some(ask), Some(bid)) => Some((ask + bid) / 2),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lob::domain::{
        entity::lob_types::{OrderEntry, Side, TraderId},
        repo::MemoryOrderRepo
    };

    #[test]
    fn test_market_data_service() {
        let mut repo = MemoryOrderRepo::new(100_000, 1000);

        let trader = TraderId::from_str("TRADER");

        // 添加订单
        let buy_id = repo.allocate_order_id();
        let buy_entry = OrderEntry::new(buy_id, trader, 100);
        repo.add_order(buy_id, buy_entry, Side::Buy, 9900).unwrap();

        let sell_id = repo.allocate_order_id();
        let sell_entry = OrderEntry::new(sell_id, trader, 100);
        repo.add_order(sell_id, sell_entry, Side::Sell, 10100).unwrap();

        // 创建市场数据服务
        let md_service = MarketDataService::new(repo);

        // 验证市场数据
        assert_eq!(md_service.find_best_bid(), Some(9900));
        assert_eq!(md_service.find_best_ask(), Some(10100));
        assert_eq!(md_service.calculate_spread(), Some(200));
        assert_eq!(md_service.calculate_mid_price(), Some(10000));
    }

    #[test]
    fn test_empty_book() {
        let repo = MemoryOrderRepo::new(100_000, 1000);
        let md_service = MarketDataService::new(repo);

        assert_eq!(md_service.find_best_bid(), None);
        assert_eq!(md_service.find_best_ask(), None);
        assert_eq!(md_service.calculate_spread(), None);
        assert_eq!(md_service.calculate_mid_price(), None);
    }

    #[test]
    fn test_only_bids() {
        let mut repo = MemoryOrderRepo::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER");

        let buy_id = repo.allocate_order_id();
        let buy_entry = OrderEntry::new(buy_id, trader, 100);
        repo.add_order(buy_id, buy_entry, Side::Buy, 9900).unwrap();

        let md_service = MarketDataService::new(repo);

        assert_eq!(md_service.find_best_bid(), Some(9900));
        assert_eq!(md_service.find_best_ask(), None);
        assert_eq!(md_service.calculate_spread(), None);
        assert_eq!(md_service.calculate_mid_price(), None);
    }

    #[test]
    fn test_only_asks() {
        let mut repo = MemoryOrderRepo::new(100_000, 1000);
        let trader = TraderId::from_str("TRADER");

        let sell_id = repo.allocate_order_id();
        let sell_entry = OrderEntry::new(sell_id, trader, 100);
        repo.add_order(sell_id, sell_entry, Side::Sell, 10100).unwrap();

        let md_service = MarketDataService::new(repo);

        assert_eq!(md_service.find_best_bid(), None);
        assert_eq!(md_service.find_best_ask(), Some(10100));
        assert_eq!(md_service.calculate_spread(), None);
        assert_eq!(md_service.calculate_mid_price(), None);
    }
}
