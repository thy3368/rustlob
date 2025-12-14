use account::{OrderId, Price, Quantity, Side, Symbol};

use crate::proc::{
    prep_types::InternalOrder,
    repo::symbol_lob_repo::{RepoError, SymbolLobRepo}
};

pub struct LocalLob {
    symbol: Symbol
}

impl LocalLob {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            symbol
        }
    }

    pub fn symbol(&self) -> &Symbol { &self.symbol }
}
impl SymbolLobRepo for LocalLob {
    fn match_orders(&self, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&InternalOrder>> { todo!() }

    fn add_order(
        &mut self, order_id: OrderId, entry: InternalOrder, side: Side, price: Price
    ) -> Result<(), RepoError> {
        todo!()
    }

    fn remove_order(&mut self, order_id: OrderId) -> bool { todo!() }

    fn find_order(&self, order_id: OrderId) -> Option<&InternalOrder> { todo!() }

    fn find_order_mut(&mut self, order_id: OrderId) -> Option<&mut InternalOrder> { todo!() }

    fn best_bid(&self) -> Option<Price> { todo!() }

    fn best_ask(&self) -> Option<Price> { todo!() }
}
