use std::collections::HashMap;

use base_types::{OrderId, Price, Quantity, Side, TradingPair};
use base_types::lob::lob::LobOrder;
use crate::core::symbol_lob_repo::{MultiSymbolLobRepo};
use crate::adapter::remote_lob_impl::RemoteLob;
use crate::core::repo_snapshot_support::RepoError;

//todo 用type 代码范型

#[allow(dead_code)]
pub struct DistributedLobRepo<O: LobOrder> {
    lobs: HashMap<TradingPair, RemoteLob<O>>
}

impl<O: LobOrder> MultiSymbolLobRepo for DistributedLobRepo<O> {
    type Order = O;

    fn match_orders(
        &self, symbol: TradingPair, side: Side, price: Price, quantity: Quantity
    ) -> Option<Vec<&Self::Order>> {
        //todo 根据Symbol 找到对应的Lob
        todo!()
    }

    fn best_bid(&self, symbol: TradingPair) -> Option<Price> {
        todo!()
    }

    fn best_ask(&self, symbol: TradingPair) -> Option<Price> {
        todo!()
    }

    fn contains_symbol(&self, symbol: &TradingPair) -> bool {
        todo!()
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

