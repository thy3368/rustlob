use std::collections::HashMap;

use account::{Price, Quantity, Side, Symbol};

use crate::proc::{
    prep_types::InternalOrder,
    repo::{symbol_lob_repo::MultiSymbolLobRepo, local_lob_repo_impl::LocalLob}
};
use crate::proc::repo::remote_lob_repo_impl::RemoteLob;

#[allow(dead_code)]
pub struct DistributedLobRepo {
    lobs: HashMap<Symbol, RemoteLob>
}


impl MultiSymbolLobRepo for DistributedLobRepo {
    fn match_orders(
        &self, symbol: Symbol, side: Side, price: Price, quantity: Quantity
    ) -> Option<Vec<&InternalOrder>> {
        //todo 根据Symbol 找到对应的Lob
        todo!()
    }

    fn best_bid(&self, symbol: Symbol) -> Option<Price> { todo!() }

    fn best_ask(&self, symbol: Symbol) -> Option<Price> { todo!() }

    fn contains_symbol(&self, symbol: &Symbol) -> bool { todo!() }
}
