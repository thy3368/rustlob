use std::collections::HashMap;

use base_types::{OrderId, Price, Quantity, Side, Symbol};
use crate::core::symbol_lob_repo::{MultiSymbolLobRepo, Order};
use crate::adapter::remote_lob_impl::RemoteLob;
use crate::core::repo_snapshot_support::RepoError;

//todo 用type 代码范型

#[allow(dead_code)]
pub struct DistributedLobRepo<O: Order> {
    lobs: HashMap<Symbol, RemoteLob<O>>
}

impl<O: Order> MultiSymbolLobRepo for DistributedLobRepo<O> {
    type Order = O;

    fn match_orders(
        &self, symbol: Symbol, side: Side, price: Price, quantity: Quantity
    ) -> Option<Vec<&Self::Order>> {
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

    fn add_order(&self, symbol: Symbol, order: Self::Order) -> Result<(), RepoError> {
        todo!()
    }

    fn remove_order(&self, symbol: Symbol, order_id: OrderId) -> bool {
        todo!()
    }
}

