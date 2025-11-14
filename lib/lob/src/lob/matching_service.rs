/// 订单匹配服务
///
/// 实现价格-时间优先的订单匹配算法
/// 遵循Clean Architecture的领域服务模式

use super::handler::{OrderCommandHandler, Command, CommandResult};
use super::repository::{OrderRepository, RepositoryAccessor};
use super::types::{Price, Quantity, Side, Trade, TraderId};



/// 匹配服务
///
/// 负责订单匹配逻辑，不直接处理数据存储
pub struct MatchingService;

impl MatchingService {
    /// 创建新的匹配服务
    pub fn new() -> Self {
        Self
    }

    /// 执行限价订单匹配
    ///
    /// 返回 (成交列表, 剩余数量)
    pub fn match_limit_order<R>(
        &self,
        repository: &mut R,
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    ) -> (Vec<Trade>, Quantity)
    where
        R: OrderRepository + RepositoryAccessor,
    {
        let mut remaining = quantity;
        let mut trades = Vec::new();

        match side {
            Side::Buy => {
                // 从最佳（最低）卖价开始匹配
                self.match_buy_order(repository, trader, price, &mut remaining, &mut trades);
            }
            Side::Sell => {
                // 从最佳（最高）买价开始匹配
                self.match_sell_order(repository, trader, price, &mut remaining, &mut trades);
            }
        }

        (trades, remaining)
    }

    /// 匹配买单
    fn match_buy_order<R>(
        &self,
        repository: &mut R,
        trader: TraderId,
        price: Price,
        remaining: &mut Quantity,
        trades: &mut Vec<Trade>,
    ) where
        R: OrderRepository + RepositoryAccessor,
    {
        // 从最低价开始匹配卖单
        let mut current_price = 0;

        while *remaining > 0 && current_price <= price {
            // 查找下一个非空卖价
            if let Some(ask_price) = self.find_next_non_empty_price(repository, current_price, price, Side::Sell) {
                let fills = self.match_at_price(repository, trader, Side::Buy, ask_price, remaining);
                trades.extend(fills);
                current_price = ask_price + 1;
            } else {
                break;
            }
        }
    }

    /// 匹配卖单
    fn match_sell_order<R>(
        &self,
        repository: &mut R,
        trader: TraderId,
        price: Price,
        remaining: &mut Quantity,
        trades: &mut Vec<Trade>,
    ) where
        R: OrderRepository + RepositoryAccessor,
    {
        // 从最高价开始匹配买单
        let mut current_price = u32::MAX;

        while *remaining > 0 && current_price >= price {
            // 查找上一个非空买价
            if let Some(bid_price) = self.find_prev_non_empty_price(repository, current_price, price, Side::Buy) {
                let fills = self.match_at_price(repository, trader, Side::Sell, bid_price, remaining);
                trades.extend(fills);

                if bid_price == 0 {
                    break;
                }
                current_price = bid_price.saturating_sub(1);
            } else {
                break;
            }
        }
    }

    /// 在特定价格级别执行匹配
    fn match_at_price<R>(
        &self,
        repository: &mut R,
        trader: TraderId,
        side: Side,
        price: Price,
        remaining: &mut Quantity,
    ) -> Vec<Trade>
    where
        R: OrderRepository + RepositoryAccessor,
    {
        let mut trades = Vec::new();

        // 获取对手方
        let opposite_side = side.opposite();

        let mut current_idx = repository.get_first_order_at_price(price, opposite_side);
        let mut first_active_idx = None;

        while *remaining > 0 && current_idx.is_some() {
            let idx = current_idx.unwrap();

            if let Some(entry) = repository.get_entry_mut(idx) {
                if entry.is_active() {
                    // 跟踪第一个活跃订单
                    if first_active_idx.is_none() {
                        first_active_idx = Some(idx);
                    }

                    // 计算成交数量
                    let fill_qty = (*remaining).min(entry.quantity);

                    // 创建交易记录
                    let trade = match side {
                        Side::Buy => Trade::new(trader, entry.trader, price, fill_qty),
                        Side::Sell => Trade::new(entry.trader, trader, price, fill_qty),
                    };
                    trades.push(trade);

                    // 更新数量
                    *remaining -= fill_qty;
                    entry.quantity -= fill_qty;

                    // 如果订单完全成交，标记为非活跃
                    if entry.quantity == 0 {
                        let order_id = entry.order_id;
                        // 通过 cancel_order 移除索引
                        repository.cancel_order(order_id);

                        // 如果这是第一个活跃订单，重置标记
                        if first_active_idx == Some(idx) {
                            first_active_idx = None;
                        }
                    }
                }

                // 获取下一个订单索引
                current_idx = repository.get_next_order(idx);

                // 更新 first_active_idx
                if first_active_idx.is_none() && current_idx.is_some() {
                    if let Some(next_entry) = repository.get_entry(current_idx.unwrap()) {
                        if next_entry.is_active() {
                            first_active_idx = current_idx;
                        }
                    }
                }
            } else {
                break;
            }
        }

        // 更新价格点以反映第一个活跃订单
        if first_active_idx.is_none() && current_idx.is_none() {
            // 所有订单都已消费，清空价格级别
            repository.update_price_point(price, opposite_side, None, None);
        } else if first_active_idx.is_some() {
            // 更新为第一个活跃订单
            repository.update_price_point(price, opposite_side, first_active_idx, None);
        }

        trades
    }

    /// 查找下一个非空的价格级别（用于买单匹配卖单）
    fn find_next_non_empty_price<R>(
        &self,
        repository: &R,
        start_price: Price,
        max_price: Price,
        side: Side,
    ) -> Option<Price>
    where
        R: OrderRepository,
    {
        for price in start_price..=max_price {
            if !repository.is_price_empty(price, side) {
                return Some(price);
            }
        }
        None
    }

    /// 查找上一个非空的价格级别（用于卖单匹配买单）
    fn find_prev_non_empty_price<R>(
        &self,
        repository: &R,
        start_price: Price,
        min_price: Price,
        side: Side,
    ) -> Option<Price>
    where
        R: OrderRepository,
    {
        let start = start_price.min(100_000); // 限制搜索范围

        for price in (min_price..=start).rev() {
            if !repository.is_price_empty(price, side) {
                return Some(price);
            }
        }
        None
    }
}

impl Default for MatchingService {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderCommandHandler for MatchingService {
    fn handle<R>(
        &self,
        repository: &mut R,
        command: Command,
    ) -> CommandResult
    where
        R: OrderRepository + RepositoryAccessor,
    {
        match command {
            // ========== 已实现的基础订单类型 ==========
            Command::LimitOrder { trader, side, price, quantity } => {
                let (trades, _remaining) = self.handle_limit_order(
                    repository, trader, side, price, quantity
                );
                CommandResult::LimitOrder {
                    order_id: repository.allocate_order_id(),
                    trades,
                }
            }

            Command::MarketOrder { trader, side, quantity } => {
                let (trades, _remaining) = self.handle_market_order(
                    repository, trader, side, quantity
                );
                CommandResult::MarketOrder { trades }
            }

            Command::IcebergOrder { trader, side, price, total_quantity, display_quantity } => {
                let (trades, remaining_total, current_display) = self.handle_iceberg_order(
                    repository, trader, side, price, total_quantity, display_quantity
                );
                CommandResult::IcebergOrder {
                    order_id: repository.allocate_order_id(),
                    trades,
                    remaining_total,
                    current_display,
                }
            }

            Command::CancelOrder { order_id } => {
                let success = repository.cancel_order(order_id);
                CommandResult::CancelOrder { success }
            }

            // ========== 待实现的时间条件订单 ==========
            Command::FillOrKillOrder { .. } => {
                todo!("FOK订单待实现")
            }

            Command::ImmediateOrCancelOrder { .. } => {
                todo!("IOC订单待实现")
            }

            Command::AllOrNoneOrder { .. } => {
                todo!("AON订单待实现")
            }

            Command::GoodTillDateOrder { .. } => {
                todo!("GTD订单待实现")
            }

            // ========== 待实现的止损订单 ==========
            Command::StopMarketOrder { .. } => {
                todo!("止损市价单待实现")
            }

            Command::StopLimitOrder { .. } => {
                todo!("止损限价单待实现")
            }

            Command::TrailingStopOrder { .. } => {
                todo!("追踪止损单待实现")
            }

            Command::TrailingStopPercentOrder { .. } => {
                todo!("追踪止损百分比单待实现")
            }

            // ========== 待实现的订单修改命令 ==========
            Command::ModifyOrder { .. } => {
                todo!("修改订单待实现")
            }

            Command::CancelReplaceOrder { .. } => {
                todo!("取消并替换订单待实现")
            }

            Command::CancelAllOrders { .. } => {
                todo!("批量取消订单待实现")
            }

            // ========== 待实现的高级订单类型 ==========
            Command::HiddenOrder { .. } => {
                todo!("隐藏订单待实现")
            }

            Command::PeggedOrder { .. } => {
                todo!("钉住订单待实现")
            }

            Command::MinimumQuantityOrder { .. } => {
                todo!("最小成交量订单待实现")
            }

            Command::TwoWayQuote { .. } => {
                todo!("双向报价待实现")
            }

            // ========== 待实现的算法交易订单 ==========
            Command::TwapOrder { .. } => {
                todo!("TWAP订单待实现")
            }

            Command::VwapOrder { .. } => {
                todo!("VWAP订单待实现")
            }

            Command::PovOrder { .. } => {
                todo!("POV订单待实现")
            }

            Command::ImplementationShortfallOrder { .. } => {
                todo!("实施缺口订单待实现")
            }

            // ========== 待实现的条件订单 ==========
            Command::OcoOrder { .. } => {
                todo!("OCO订单待实现")
            }

            Command::BracketOrder { .. } => {
                todo!("括号订单待实现")
            }

            // ========== 待实现的交易所特定订单 ==========
            Command::AuctionOrder { .. } => {
                todo!("拍卖订单待实现")
            }

            Command::MarketMakerQuote { .. } => {
                todo!("做市商双边报价待实现")
            }
        }
    }

    fn handle_limit_order<R>(
        &self,
        repository: &mut R,
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    ) -> (Vec<Trade>, Quantity)
    where
        R: OrderRepository + RepositoryAccessor,
    {
        // 使用已有的match_limit_order方法
        self.match_limit_order(repository, trader, side, price, quantity)
    }

    fn handle_market_order<R>(
        &self,
        repository: &mut R,
        trader: TraderId,
        side: Side,
        quantity: Quantity,
    ) -> (Vec<Trade>, Quantity)
    where
        R: OrderRepository + RepositoryAccessor,
    {
        // 市价单使用极端价格来确保成交
        // 买单使用最大价格，卖单使用最小价格
        let price = match side {
            Side::Buy => u32::MAX,  // 买单愿意支付任何价格
            Side::Sell => 0,        // 卖单愿意接受任何价格
        };

        self.match_limit_order(repository, trader, side, price, quantity)
    }

    fn handle_iceberg_order<R>(
        &self,
        repository: &mut R,
        trader: TraderId,
        side: Side,
        price: Price,
        total_quantity: Quantity,
        display_quantity: Quantity,
    ) -> (Vec<Trade>, Quantity, Quantity)
    where
        R: OrderRepository + RepositoryAccessor,
    {
        // 冰山单逻辑：
        // 1. 首先尝试匹配显示数量
        // 2. 如果显示数量全部成交，从总量中补充新的显示数量
        // 3. 返回剩余总量和当前显示数量

        let mut remaining_total = total_quantity;
        let mut current_display = display_quantity.min(remaining_total);
        let mut all_trades = Vec::new();

        while remaining_total > 0 && current_display > 0 {
            // 匹配当前显示数量
            let (trades, remaining_display) = self.match_limit_order(
                repository,
                trader,
                side,
                price,
                current_display,
            );

            all_trades.extend(trades);

            // 更新剩余总量
            let matched = current_display - remaining_display;
            remaining_total -= matched;

            if remaining_display > 0 {
                // 显示数量未完全成交，说明没有更多对手方订单
                current_display = remaining_display;
                break;
            }

            // 显示数量已完全成交，补充新的显示数量
            current_display = display_quantity.min(remaining_total);

            // 如果没有新的显示数量，退出
            if current_display == 0 {
                break;
            }
        }

        (all_trades, remaining_total, current_display)
    }

    fn handler_name(&self) -> &'static str {
        "PriceTimeMatchingService"
    }
}

/// 市场数据查询服务
///
/// 提供订单簿市场数据查询功能
pub struct MarketDataService;

impl MarketDataService {
    /// 创建新的市场数据服务
    pub fn new() -> Self {
        Self
    }

    /// 查找最佳买价
    pub fn find_best_bid<R>(&self, repository: &R) -> Option<Price>
    where
        R: OrderRepository,
    {
        self.find_best_price(repository, Side::Buy, true)
    }

    /// 查找最佳卖价
    pub fn find_best_ask<R>(&self, repository: &R) -> Option<Price>
    where
        R: OrderRepository,
    {
        self.find_best_price(repository, Side::Sell, false)
    }

    /// 查找最佳价格
    fn find_best_price<R>(&self, repository: &R, side: Side, descending: bool) -> Option<Price>
    where
        R: OrderRepository,
    {
        let max_search = 100_000u32;

        if descending {
            // 从高到低搜索（买单）
            for price in (0..=max_search).rev() {
                if !repository.is_price_empty(price, side) {
                    return Some(price);
                }
            }
        } else {
            // 从低到高搜索（卖单）
            for price in 0..=max_search {
                if !repository.is_price_empty(price, side) {
                    return Some(price);
                }
            }
        }

        None
    }

    /// 计算买卖价差
    pub fn calculate_spread<R>(&self, repository: &R) -> Option<Price>
    where
        R: OrderRepository,
    {
        match (self.find_best_ask(repository), self.find_best_bid(repository)) {
            (Some(ask), Some(bid)) if ask > bid => Some(ask - bid),
            _ => None,
        }
    }

    /// 计算中间价
    pub fn calculate_mid_price<R>(&self, repository: &R) -> Option<Price>
    where
        R: OrderRepository,
    {
        match (self.find_best_ask(repository), self.find_best_bid(repository)) {
            (Some(ask), Some(bid)) => Some((ask + bid) / 2),
            _ => None,
        }
    }
}

impl Default for MarketDataService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lob::repository::InMemoryOrderRepository;
    use crate::lob::types::{TraderId, OrderEntry};

    #[test]
    fn test_simple_match() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let service = MatchingService::new();

        let seller = TraderId::from_str("SELLER");
        let buyer = TraderId::from_str("BUYER");

        // 添加卖单
        let sell_order_id = repo.allocate_order_id();
        let sell_entry = OrderEntry::new(sell_order_id, seller, 100);
        repo.add_order(sell_order_id, sell_entry, Side::Sell, 10000).unwrap();

        // 匹配买单
        let (trades, remaining) = service.match_limit_order(&mut repo, buyer, Side::Buy, 10000, 100);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, 100);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn test_partial_fill() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let service = MatchingService::new();

        let seller = TraderId::from_str("SELLER");
        let buyer = TraderId::from_str("BUYER");

        // 添加大卖单
        let sell_order_id = repo.allocate_order_id();
        let sell_entry = OrderEntry::new(sell_order_id, seller, 200);
        repo.add_order(sell_order_id, sell_entry, Side::Sell, 10000).unwrap();

        // 匹配小买单
        let (trades, remaining) = service.match_limit_order(&mut repo, buyer, Side::Buy, 10000, 50);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, 50);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn test_market_data_service() {
        let mut repo = InMemoryOrderRepository::new(100_000, 1000);
        let md_service = MarketDataService::new();

        let trader = TraderId::from_str("TRADER");

        // 添加订单
        let buy_id = repo.allocate_order_id();
        let buy_entry = OrderEntry::new(buy_id, trader, 100);
        repo.add_order(buy_id, buy_entry, Side::Buy, 9900).unwrap();

        let sell_id = repo.allocate_order_id();
        let sell_entry = OrderEntry::new(sell_id, trader, 100);
        repo.add_order(sell_id, sell_entry, Side::Sell, 10100).unwrap();

        // 验证市场数据
        assert_eq!(md_service.find_best_bid(&repo), Some(9900));
        assert_eq!(md_service.find_best_ask(&repo), Some(10100));
        assert_eq!(md_service.calculate_spread(&repo), Some(200));
        assert_eq!(md_service.calculate_mid_price(&repo), Some(10000));
    }
}
