use std::collections::HashMap;

use base_types::lob::lob::LobOrder;
use base_types::{OrderId, OrderSide, Price, Quantity, TradingPair};
use parking_lot::RwLock;

use crate::adapter::local_lob_impl::LocalLob;
use crate::core::repo_snapshot_support::LobError;
use crate::core::symbol_lob_repo::{MultiSymbolLobRepo, SymbolLob};

#[allow(dead_code)]
///规则：BDD必须用EmbeddedLobRepo
pub struct EmbeddedLobRepo<O: LobOrder> {
    lobs: RwLock<HashMap<TradingPair, LocalLob<O>>>,
}

impl<O: LobOrder> EmbeddedLobRepo<O> {
    pub fn new(lobs: Vec<LocalLob<O>>) -> Self {
        let mut map = HashMap::with_capacity(lobs.len());
        for lob in lobs {
            map.insert(*lob.symbol(), lob);
        }
        Self { lobs: RwLock::new(map) }
    }
}

impl<O: LobOrder> MultiSymbolLobRepo for EmbeddedLobRepo<O> {
    type Order = O;

    fn match_orders(
        &self,
        _symbol: TradingPair,
        _side: OrderSide,
        _price: Price,
        quantity: Quantity,
    ) -> (Option<Vec<&Self::Order>>, Quantity) {
        (None, quantity)
    }

    fn best_bid(&self, symbol: TradingPair) -> Option<Price> {
        self.lobs.read().get(&symbol)?.best_bid()
    }

    fn best_ask(&self, symbol: TradingPair) -> Option<Price> {
        self.lobs.read().get(&symbol)?.best_ask()
    }

    fn contains_symbol(&self, symbol: &TradingPair) -> bool {
        self.lobs.read().contains_key(symbol)
    }

    fn add_order(&self, symbol: TradingPair, order: Self::Order) -> Result<(), LobError> {
        let mut lobs = self.lobs.write();
        let lob = lobs.get_mut(&symbol).ok_or(LobError::SymbolMismatch {
            expected: symbol.to_string(),
            actual: order.symbol().to_string(),
        })?;
        lob.add_order(order)
    }

    fn remove_order(&self, _symbol: TradingPair, _order_id: OrderId) -> bool {
        todo!()
    }

    fn find_order(&self, _p0: TradingPair, _p1: OrderId) -> Option<&Self::Order> {
        todo!()
    }

    fn find_order_mut(&self, _p0: TradingPair, _order_id: OrderId) -> Option<&mut Self::Order> {
        todo!()
    }

    fn last_price(&self, symbol: TradingPair) -> Option<Price> {
        self.lobs.read().get(&symbol)?.last_price()
    }

    fn update_last_price(&self, symbol: TradingPair, price: Price) {
        if let Some(lob) = self.lobs.write().get_mut(&symbol) {
            lob.update_last_price(price);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, entity_derive::Entity)]
    struct MockOrder {
        id: u64,
        #[replay(skip)]
        #[created(skip)]
        symbol: TradingPair,
        #[replay(skip)]
        #[created(skip)]
        price: Price,
        #[replay(skip)]
        #[created(skip)]
        quantity: Quantity,
        #[replay(skip)]
        #[created(skip)]
        filled_quantity: Quantity,
        #[replay(skip)]
        #[created(skip)]
        side: OrderSide,
    }

    impl LobOrder for MockOrder {
        fn order_id(&self) -> base_types::OrderId {
            self.id
        }

        fn price(&self) -> Price {
            self.price
        }

        fn base_qty(&self) -> Quantity {
            self.quantity
        }

        fn filled_base_qty(&self) -> Quantity {
            self.filled_quantity
        }

        fn side(&self) -> OrderSide {
            self.side
        }

        fn symbol(&self) -> TradingPair {
            self.symbol
        }
    }

    #[test]
    fn test_contains_symbol() {
        let repo = EmbeddedLobRepo::<MockOrder>::new(Vec::new());
        assert!(!repo.contains_symbol(&TradingPair::BtcUsdt));
    }

    #[test]
    fn test_last_price_no_lob() {
        let repo = EmbeddedLobRepo::<MockOrder>::new(Vec::new());
        assert!(repo.last_price(TradingPair::BtcUsdt).is_none());
    }
}
