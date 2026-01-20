use base_types::{OrderId, Price, Quantity, Side, TradingPair};
use lob_repo::{
    adapter::local_lob_impl::LocalLob,
    core::symbol_lob_repo::{LobOrder, SymbolLob}
};

// 创建模拟订单用于测试
#[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
struct MockOrder {
    id: u64,
    // #[replay(skip)]
    symbol: TradingPair,
    // #[replay(skip)]
    price: Price,
    // #[replay(skip)]
    quantity: Quantity,
    // #[replay(skip)]
    filled_quantity: Quantity,
    // #[replay(skip)]
    side: Side
}


impl LobOrder for MockOrder {
    fn order_id(&self) -> OrderId { self.id }

    fn price(&self) -> Price { self.price }

    fn quantity(&self) -> Quantity { self.quantity }

    fn filled_quantity(&self) -> Quantity { self.filled_quantity }

    fn side(&self) -> Side { self.side }

    fn symbol(&self) -> TradingPair { self.symbol }
}

#[test]
fn test_last_price_initially_none() {
    let symbol = TradingPair::from_symbol_str("BTCUSDT").unwrap();
    let lob: LocalLob<MockOrder> = LocalLob::new(symbol);

    // 初始状态，最后成交价应为 None
    assert_eq!(lob.last_price(), None);
}
