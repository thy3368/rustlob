use base_types::{OrderId, Price, Quantity, Side, TradingPair};
use crate::core::symbol_lob_repo::{Order, RepoError, SymbolLob};

// todo 通过rpc连接远程的lob
//todo 用type 代码范型
pub struct RemoteLob<O: Order> {
    symbol: TradingPair,
    _phantom: std::marker::PhantomData<O>
}

impl<O: Order> RemoteLob<O> {
    pub fn new(symbol: TradingPair) -> Self {
        Self {
            symbol,
            _phantom: std::marker::PhantomData
        }
    }

    pub fn symbol(&self) -> &TradingPair {
        &self.symbol
    }
}

impl<O: Order> SymbolLob for RemoteLob<O> {
    type Order = O;

    fn match_orders(&self, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&Self::Order>> {
        todo!()
    }

    fn add_order(&mut self, order: Self::Order) -> Result<(), RepoError> {
        todo!()
    }

    fn remove_order(&mut self, order_id: OrderId) -> bool {
        todo!()
    }

    fn find_order(&self, order_id: OrderId) -> Option<&Self::Order> {
        todo!()
    }

    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut Self::Order> {
        todo!()
    }

    fn best_bid(&self) -> Option<Price> {
        todo!()
    }

    fn best_ask(&self) -> Option<Price> {
        todo!()
    }

    fn last_price(&self) -> Option<Price> {
        todo!()
    }

    fn update_last_price(&mut self, _price: Price) {
        todo!()
    }
}

