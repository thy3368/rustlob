use base_types::{OrderId, Price, Quantity, Side, Symbol};
use crate::core::symbol_lob_repo::{Order, RepoError, SymbolLob};

// todo 通过rpc连接远程的lob
pub struct RemoteLob<O: Order> {
    symbol: Symbol,
    _phantom: std::marker::PhantomData<O>
}

impl<O: Order> RemoteLob<O> {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            symbol,
            _phantom: std::marker::PhantomData
        }
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }
}

impl<O: Order> SymbolLob<O> for RemoteLob<O> {
    fn match_orders(&self, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&O>> {
        todo!()
    }

    fn add_order(&mut self, order: O) -> Result<(), RepoError> {
        todo!()
    }

    fn remove_order(&mut self, order_id: OrderId) -> bool {
        todo!()
    }

    fn find_order(&self, order_id: OrderId) -> Option<&O> {
        todo!()
    }

    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut O> {
        todo!()
    }

    fn best_bid(&self) -> Option<Price> {
        todo!()
    }

    fn best_ask(&self) -> Option<Price> {
        todo!()
    }
}

