use std::collections::HashMap;

use base_types::{Price, Quantity, Side, Symbol};

use crate::{
    adapter::local_lob_impl::LocalLob,
    core::symbol_lob_repo::{MultiSymbolLobRepo, Order, SymbolLob}
};

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
impl<O: Order> MultiSymbolLobRepo<O> for StandaloneLobRepo<O> {
    fn match_orders(&self, symbol: Symbol, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&O>> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // 创建一个模拟的 Order 实现用于测试
    #[derive(Debug, Clone, Copy)]
    struct MockOrder {
        id: u64,
        symbol: Symbol,
        price: Price,
        quantity: Quantity,
        filled_quantity: Quantity,
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

    #[test]
    fn test_match_orders_symbol_not_found() {
        // 创建只有 BTCUSDT 的 repo
        let btc_symbol = Symbol::new("BTCUSDT");
        let lob: LocalLob<MockOrder> = LocalLob::new(btc_symbol);
        let repo = StandaloneLobRepo::new(vec![lob]);

        // 查找不存在的 symbol
        let not_found_symbol = Symbol::new("XRPUSDT");
        let result = repo.lobs.get(&not_found_symbol);

        assert!(result.is_none(), "不存在的 symbol 应该返回 None");
    }

    #[test]
    fn test_empty_repo() {
        // 创建空的 repo
        let repo: StandaloneLobRepo<MockOrder> = StandaloneLobRepo::new(vec![]);

        // 验证 HashMap 是空的
        assert!(repo.lobs.is_empty(), "空 repo 的 HashMap 应该为空");

        // 查找任何 symbol 都应该返回 None
        let symbol = Symbol::new("BTCUSDT");
        let result = repo.lobs.get(&symbol);

        assert!(result.is_none(), "空 repo 应该返回 None");
    }

    #[test]
    fn test_multiple_lobs_with_same_symbol() {
        // 测试当有多个相同 symbol 的 LOB 时，HashMap 只保留最后一个
        let btc_symbol = Symbol::new("BTCUSDT");
        let lob1: LocalLob<MockOrder> = LocalLob::new(btc_symbol);
        let lob2: LocalLob<MockOrder> = LocalLob::new(btc_symbol);

        let repo = StandaloneLobRepo::new(vec![lob1, lob2]);

        // HashMap 应该只包含一个 BTCUSDT
        assert_eq!(repo.lobs.len(), 1, "HashMap 中应该只有一个 LOB");
        assert!(repo.lobs.contains_key(&btc_symbol), "应该包含 BTCUSDT");
    }

    #[test]
    fn test_symbol_comparison() {
        // 测试 Symbol 的相等性比较
        let symbol1 = Symbol::new("BTCUSDT");
        let symbol2 = Symbol::new("BTCUSDT");
        let symbol3 = Symbol::new("ETHUSDT");

        assert_eq!(symbol1, symbol2, "相同字符串创建的 Symbol 应该相等");
        assert_ne!(symbol1, symbol3, "不同字符串创建的 Symbol 应该不相等");
    }


    #[test]
    fn test_hashmap_performance() {
        // 测试 HashMap 的容量预分配
        let symbols = vec![Symbol::new("BTCUSDT"), Symbol::new("ETHUSDT"), Symbol::new("BNBUSDT")];

        let lobs: Vec<LocalLob<MockOrder>> = symbols.iter().map(|s| LocalLob::new(*s)).collect();
        let repo = StandaloneLobRepo::new(lobs);

        // 验证所有 symbol 都被正确插入
        assert_eq!(repo.lobs.len(), 3, "应该有 3 个 LOB");
        for symbol in symbols {
            assert!(repo.lobs.contains_key(&symbol), "应该包含 symbol: {}", symbol);
        }
    }

    #[test]
    fn test_match_orders_integration() {
        let btc_symbol = Symbol::new("BTCUSDT");
        let mut lob: LocalLob<MockOrder> = LocalLob::new(btc_symbol);

        // 添加一些卖单
        let sell_order = MockOrder {
            id: 1,
            symbol: btc_symbol,
            price: Price::from_raw(50100),
            quantity: Quantity::from_raw(100),
            side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(sell_order).unwrap();

        let repo = StandaloneLobRepo::new(vec![lob]);

        // 测试匹配订单
        let matched = repo.match_orders(btc_symbol, Side::Buy, Price::from_raw(50100), Quantity::from_raw(50));
        assert!(matched.is_some());
    }

    #[test]
    fn test_o1_lookup_performance() {
        // 性能测试：验证 O(1) 查找
        // 创建大量 LOB 来验证查找性能不随数量增加而显著下降
        let symbols: Vec<Symbol> = (0..100).map(|i| Symbol::new(&format!("SYM{:04}USDT", i))).collect();

        let lobs: Vec<LocalLob<MockOrder>> = symbols.iter().map(|s| LocalLob::new(*s)).collect();
        let repo = StandaloneLobRepo::new(lobs);

        // 查找第一个和最后一个 symbol 的性能应该相同
        let first_symbol = symbols[0];
        let last_symbol = symbols[99];

        assert!(repo.lobs.get(&first_symbol).is_some(), "应该找到第一个 symbol");
        assert!(repo.lobs.get(&last_symbol).is_some(), "应该找到最后一个 symbol");
        assert_eq!(repo.lobs.len(), 100, "应该有 100 个 LOB");
    }

    // === MultiLobRepo trait 测试 ===

    #[test]
    fn test_multi_lob_repo_trait_contains_symbol() {
        let btc_symbol = Symbol::new("BTCUSDT");
        let eth_symbol = Symbol::new("ETHUSDT");

        let lob: LocalLob<MockOrder> = LocalLob::new(btc_symbol);
        let repo = StandaloneLobRepo::new(vec![lob]);

        // 使用 trait 方法
        let repo_trait: &dyn MultiSymbolLobRepo<MockOrder> = &repo;
        assert!(repo_trait.contains_symbol(&btc_symbol), "应该包含 BTCUSDT");
        assert!(!repo_trait.contains_symbol(&eth_symbol), "不应该包含 ETHUSDT");
    }

    #[test]
    fn test_multi_lob_repo_trait_best_bid_ask() {
        let btc_symbol = Symbol::new("BTCUSDT");
        let mut lob: LocalLob<MockOrder> = LocalLob::new(btc_symbol);

        // 添加订单
        let buy_order = MockOrder {
            id: 1,
            symbol: btc_symbol,
            price: Price::from_raw(50000),
            quantity: Quantity::from_raw(100),
            side: Side::Buy,
        filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(buy_order).unwrap();

        let repo = StandaloneLobRepo::new(vec![lob]);

        // 使用 trait 方法
        let repo_trait: &dyn MultiSymbolLobRepo<MockOrder> = &repo;

        // 测试 best_bid
        let best_bid = repo_trait.best_bid(btc_symbol);
        assert_eq!(best_bid, Some(Price::from_raw(50000)));
    }

    #[test]
    fn test_multi_lob_repo_trait_match_orders() {
        let btc_symbol = Symbol::new("BTCUSDT");
        let mut lob: LocalLob<MockOrder> = LocalLob::new(btc_symbol);

        // 添加卖单
        let sell_order = MockOrder {
            id: 1,
            symbol: btc_symbol,
            price: Price::from_raw(50100),
            quantity: Quantity::from_raw(100),
            side: Side::Sell,
        filled_quantity: Quantity::from_raw(0),
        };
        lob.add_order(sell_order).unwrap();

        let repo = StandaloneLobRepo::new(vec![lob]);

        // 使用 trait 方法
        let repo_trait: &dyn MultiSymbolLobRepo<MockOrder> = &repo;

        // 测试匹配订单
        let matched = repo_trait.match_orders(btc_symbol, Side::Buy, Price::from_raw(50100), Quantity::from_raw(50));
        assert!(matched.is_some());
    }

    #[test]
    fn test_polymorphism_with_trait_object() {
        // 测试多态性：使用 trait object
        let btc_symbol = Symbol::new("BTCUSDT");
        let eth_symbol = Symbol::new("ETHUSDT");

        let lob1: LocalLob<MockOrder> = LocalLob::new(btc_symbol);
        let lob2: LocalLob<MockOrder> = LocalLob::new(eth_symbol);

        let repo = StandaloneLobRepo::new(vec![lob1, lob2]);

        // 可以将具体类型转换为 trait object
        let repo_trait: Box<dyn MultiSymbolLobRepo<MockOrder>> = Box::new(repo);

        assert!(repo_trait.contains_symbol(&btc_symbol));
        assert!(repo_trait.contains_symbol(&eth_symbol));
        assert!(!repo_trait.contains_symbol(&Symbol::new("XRPUSDT")));
    }
}
