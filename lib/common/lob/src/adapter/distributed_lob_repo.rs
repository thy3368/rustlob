use std::collections::HashMap;

use base_types::{Price, Quantity, Side, Symbol};
use crate::core::symbol_lob_repo::{MultiSymbolLobRepo, Order};
use crate::adapter::remote_lob_impl::RemoteLob;

#[allow(dead_code)]
pub struct DistributedLobRepo<O: Order> {
    lobs: HashMap<Symbol, RemoteLob<O>>
}

impl<O: Order> MultiSymbolLobRepo<O> for DistributedLobRepo<O> {
    fn match_orders(
        &self, symbol: Symbol, side: Side, price: Price, quantity: Quantity
    ) -> Option<Vec<&O>> {
        //todo 根据Symbol 找到对应的Lob
        todo!()
    }

    fn best_bid(&self, symbol: Symbol) -> Option<Price> {
        todo!()
    }

    fn best_ask(&self, symbol: Symbol) -> Option<Price> {
        todo!()
    }

    fn contains_symbol(&self, symbol: &Symbol) -> bool {
        todo!()
    }
}

