use std::collections::HashMap;

use base_types::{lob::lob::LobOrder, OrderId, Price, Quantity, OrderSide, TradingPair};

use crate::{
    adapter::remote_lob_impl::RemoteLob,
    core::{repo_snapshot_support::RepoError, symbol_lob_repo::MultiSymbolLobRepo}
};

// todo 用type 代码范型

#[allow(dead_code)]
pub struct DistributedLobRepo<O: LobOrder> {
    lobs: HashMap<TradingPair, RemoteLob<O>>
}

impl<O: LobOrder> DistributedLobRepo<O> {
    /// 创建新的 SingleLobRepo
    ///
    /// # 参数
    /// - `lobs`: LOB 的向量，将被转换为 HashMap
    #[allow(dead_code)]
    pub fn new(lobs: Vec<RemoteLob<O>>) -> Self {
        let mut map = HashMap::with_capacity(lobs.len());
        for lob in lobs {
            map.insert(*lob.symbol(), lob);
        }
        Self {
            lobs: map
        }
    }
}
impl<O: LobOrder> MultiSymbolLobRepo for DistributedLobRepo<O> {
    type Order = O;


    fn match_orders(
        &self, symbol: TradingPair, side: OrderSide, price: Price, quantity: Quantity
    ) -> Option<Vec<&Self::Order>> {
        // todo 根据Symbol 找到对应的Lob
        todo!()
    }

    fn best_bid(&self, symbol: TradingPair) -> Option<Price> { todo!() }

    fn best_ask(&self, symbol: TradingPair) -> Option<Price> { todo!() }

    fn contains_symbol(&self, symbol: &TradingPair) -> bool { todo!() }

    fn add_order(&self, symbol: TradingPair, order: Self::Order) -> Result<(), RepoError> { todo!() }

    fn remove_order(&self, symbol: TradingPair, order_id: OrderId) -> bool { todo!() }

    fn find_order(&self, p0: TradingPair, p1: OrderId) -> Option<&Self::Order> { todo!() }

    fn find_order_mut(&self, p0: TradingPair, order_id: OrderId) -> Option<&mut Self::Order> { todo!() }
}
