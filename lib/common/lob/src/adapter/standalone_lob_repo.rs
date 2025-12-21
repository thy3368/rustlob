use std::collections::HashMap;

use base_types::{OrderId, Price, Quantity, Side, Symbol};

use crate::{
    adapter::local_lob_impl::LocalLob,
    core::symbol_lob_repo::{MultiSymbolLobRepo, Order, SymbolLob}
};
use crate::core::repo_snapshot_support::RepoError;


//todo 用type 代码范型
/// 单一 LOB 仓储
///
/// 使用 HashMap 存储多个交易对的 LOB，实现 O(1) 查找性能
#[allow(dead_code)]
pub struct StandaloneLobRepo<O: Order> {
    lobs: HashMap<Symbol, LocalLob<O>>
}

impl<O: Order> StandaloneLobRepo<O> {
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
        Self {
            lobs: map
        }
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
    pub fn match_orders(&self, symbol: Symbol, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&O>> {
        // 使用 trait 方法
        MultiSymbolLobRepo::match_orders(self, symbol, side, price, quantity)
    }
}

/// 实现 MultiLobRepo trait
impl<O: Order> MultiSymbolLobRepo for StandaloneLobRepo<O> {
    type Order = O;

    fn match_orders(&self, symbol: Symbol, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&Self::Order>> {
        // O(1) 查找对应的 LOB
        let lob = self.lobs.get(&symbol)?;

        // 在找到的 LOB 中进行订单匹配
        lob.match_orders(side, price, quantity)
    }

    fn best_bid(&self, symbol: Symbol) -> Option<Price> {
        let lob = self.lobs.get(&symbol)?;
        lob.best_bid()
    }

    fn best_ask(&self, symbol: Symbol) -> Option<Price> {
        let lob = self.lobs.get(&symbol)?;
        lob.best_ask()
    }

    fn contains_symbol(&self, symbol: &Symbol) -> bool { self.lobs.contains_key(symbol) }

    fn add_order(&self, symbol: Symbol, order: Self::Order) -> Result<(), RepoError> {
        todo!()
    }

    fn remove_order(&self, symbol: Symbol, order_id: OrderId) -> bool {
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
        symbol: Symbol,
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
        side: Side
    }

    impl Order for MockOrder {
        fn order_id(&self) -> base_types::OrderId { self.id }

        fn price(&self) -> Price { self.price }

        fn quantity(&self) -> Quantity { self.quantity }

        fn filled_quantity(&self) -> Quantity { self.filled_quantity }

        fn side(&self) -> Side { self.side }

        fn symbol(&self) -> Symbol { self.symbol }
    }

    #[test]
    fn test_match_orders_symbol_found() {
        // 创建测试用的 LOB
        let btc_symbol = Symbol::new("BTCUSDT");
        let eth_symbol = Symbol::new("ETHUSDT");

        let lob1: LocalLob<MockOrder> = LocalLob::new(btc_symbol);
        let lob2: LocalLob<MockOrder> = LocalLob::new(eth_symbol);

        let repo = StandaloneLobRepo::new(vec![lob1, lob2]);

        // 验证可以找到 BTCUSDT 的 LOB
        let btc_lob = repo.lobs.get(&btc_symbol);
        assert!(btc_lob.is_some(), "应该能找到 BTCUSDT 的 LOB");

        // 验证可以找到 ETHUSDT 的 LOB
        let eth_lob = repo.lobs.get(&eth_symbol);
        assert!(eth_lob.is_some(), "应该能找到 ETHUSDT 的 LOB");
    }




}
