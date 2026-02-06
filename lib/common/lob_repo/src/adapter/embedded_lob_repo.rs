use std::collections::HashMap;

use base_types::lob::lob::LobOrder;
use base_types::{OrderId, OrderSide, Price, Quantity, TradingPair};

use crate::adapter::local_lob_impl::LocalLob;
use crate::core::repo_snapshot_support::RepoError;
use crate::core::symbol_lob_repo::{MultiSymbolLobRepo, SymbolLob};

/// 单一 LOB 仓储
///
/// 使用 HashMap 存储多个交易对的 LOB，实现 O(1) 查找性能
#[allow(dead_code)]
pub struct EmbeddedLobRepo<O: LobOrder> {
    lobs: HashMap<TradingPair, LocalLob<O>>,
}

impl<O: LobOrder> EmbeddedLobRepo<O> {}

impl<O: LobOrder> EmbeddedLobRepo<O> {
    /// 创建新的 SingleLobRepo
    ///
    /// # 参数
    /// - `lobs`: LOB 的向量，将被转换为 HashMap
    #[allow(dead_code)]
    pub fn new(lobs: Vec<LocalLob<O>>) -> Self {
        let mut map = HashMap::with_capacity(lobs.len());
        for lob in lobs {
            map.insert(*lob.symbol(), lob);
        }
        Self { lobs: map }
    }

    /// 匹配订单
    ///
    /// 时间复杂度: O(1) 查找 + O(k) 匹配，其中 k 是匹配的订单数量
    ///
    /// # 参数
    /// - `symbol`: 交易对符号
    /// - `side`: 订单方向
    /// - `price`: 价格
    /// - `quantity`: 数量
    ///
    /// # 返回
    /// - `Some(Vec<&O>)`: 匹配到的订单列表
    /// - `None`: 找不到对应的 LOB 或无法匹配
    #[allow(dead_code)]
    pub fn match_orders(
        &self,
        symbol: TradingPair,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> Option<Vec<&O>> {
        // 使用 trait 方法
        MultiSymbolLobRepo::match_orders(self, symbol, side, price, quantity)
    }
}

/// 实现 MultiLobRepo trait
impl<O: LobOrder> MultiSymbolLobRepo for EmbeddedLobRepo<O> {
    type Order = O;

    fn match_orders(
        &self,
        symbol: TradingPair,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> Option<Vec<&Self::Order>> {
        // O(1) 查找对应的 LOB
        let lob = self.lobs.get(&symbol)?;

        // 在找到的 LOB 中进行订单匹配
        lob.match_orders(side, price, quantity)
    }

    fn best_bid(&self, symbol: TradingPair) -> Option<Price> {
        let lob = self.lobs.get(&symbol)?;
        lob.best_bid()
    }

    fn best_ask(&self, symbol: TradingPair) -> Option<Price> {
        let lob = self.lobs.get(&symbol)?;
        lob.best_ask()
    }

    fn contains_symbol(&self, symbol: &TradingPair) -> bool {
        self.lobs.contains_key(symbol)
    }

    fn add_order(&self, symbol: TradingPair, order: Self::Order) -> Result<(), RepoError> {
        todo!()
    }

    fn remove_order(&self, symbol: TradingPair, order_id: OrderId) -> bool {
        todo!()
    }

    fn find_order(&self, p0: TradingPair, p1: OrderId) -> Option<&Self::Order> {
        todo!()
    }

    fn find_order_mut(&self, p0: TradingPair, order_id: OrderId) -> Option<&mut Self::Order> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 创建一个模拟的 Order 实现用于测试
    #[derive(Debug, Clone, entity_derive::Entity)]
    struct MockOrder {
        id: u64,
        #[replay(skip)]
        #[created(skip)]
        symbol: TradingPair,
        #[replay(skip)]
        #[created(skip)]
        price: Price,
        #[replay(skip)]
        #[created(skip)]
        quantity: Quantity,
        #[replay(skip)]
        #[created(skip)]
        filled_quantity: Quantity,
        #[replay(skip)]
        #[created(skip)]
        side: OrderSide,
    }

    impl LobOrder for MockOrder {
        fn order_id(&self) -> base_types::OrderId {
            self.id
        }

        fn price(&self) -> Price {
            self.price
        }

        fn quantity(&self) -> Quantity {
            self.quantity
        }

        fn filled_quantity(&self) -> Quantity {
            self.filled_quantity
        }

        fn side(&self) -> OrderSide {
            self.side
        }

        fn symbol(&self) -> TradingPair {
            self.symbol
        }
    }
}
