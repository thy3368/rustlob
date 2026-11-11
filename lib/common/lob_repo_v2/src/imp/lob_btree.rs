use std::collections::{BTreeMap, HashMap};

use base_types_v2::Symbol;
use common_entity::{EntityChangeType, EntityReplayableEvent};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::LobError;
use crate::lob::{AssetLobV2, LobOrder};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
struct Px(Decimal);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
struct SlotIndex(usize);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(serialize = "O: Serialize", deserialize = "O: Deserialize<'de>"))]
pub struct LocalLobBtree<O: LobOrder> {
    /// 当前本地订单簿对应的交易标的，用于拒绝跨订单簿订单。
    symbol: Symbol,
    /// 订单主存储；索引保持稳定，删除订单后留下 `None` 空洞。
    slots: Vec<Option<O>>,
    /// 订单 ID 到 `SlotIndex` 的映射，用于快速定位仍存在的订单。
    ids: HashMap<u64, SlotIndex>,
    /// 活跃买单的价格索引；`PriceKey` 映射到该价位订单的 `SlotIndex` 列表。
    bids: BTreeMap<Px, Vec<SlotIndex>>,
    /// 活跃卖单的价格索引；`PriceKey` 映射到该价位订单的 `SlotIndex` 列表。
    asks: BTreeMap<Px, Vec<SlotIndex>>,
    /// 最后成功回放的事件 LSN，用于拒绝旧事件或重复事件。
    sequence: u64,
}
impl<O: LobOrder> LocalLobBtree<O> {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            symbol,
            slots: vec![],
            ids: HashMap::new(),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            sequence: 0,
        }
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
    pub fn order(&self, id: u64) -> Option<&O> {
        self.ids.get(&id).and_then(|&i| self.slots[i.0].as_ref())
    }
    pub fn insert_order(&mut self, order: O) -> Result<(), LobError> {
        if order.symbol() != self.symbol {
            return Err(LobError::Event("wrong order book".into()));
        }
        self.put(order);
        Ok(())
    }
    fn remove_idx(&mut self, o: &O, i: SlotIndex) {
        let map = if O::is_buy_side(o.side()) { &mut self.bids } else { &mut self.asks };
        let price = Px(o.px());
        if let Some(v) = map.get_mut(&price) {
            v.retain(|&x| x != i);
            if v.is_empty() {
                map.remove(&price);
            }
        }
    }
    fn add_idx(&mut self, o: &O, i: SlotIndex) {
        if !o.is_active() {
            return;
        }
        let map = if O::is_buy_side(o.side()) { &mut self.bids } else { &mut self.asks };
        map.entry(Px(o.px())).or_default().push(i);
    }
    fn put(&mut self, o: O) {
        let id = o.entity_id();
        if let Some(i) = self.ids.get(&id).copied() {
            if let Some(old) = self.slots[i.0].take() {
                self.remove_idx(&old, i);
            }
            self.slots[i.0] = Some(o.clone());
            self.add_idx(&o, i);
        } else {
            let i = SlotIndex(self.slots.len());
            self.slots.push(Some(o.clone()));
            self.ids.insert(id, i);
            self.add_idx(&o, i);
        }
    }
    fn delete(&mut self, id: u64) {
        if let Some(i) = self.ids.remove(&id) {
            if let Some(o) = self.slots[i.0].take() {
                self.remove_idx(&o, i);
            }
        }
    }
}
impl<O: LobOrder> AssetLobV2 for LocalLobBtree<O> {
    type Order = O;
    fn match_orders(
        &self,
        price: Decimal,
        qty: Decimal,
        side: O::Side,
    ) -> (Option<Vec<&Self::Order>>, Decimal) {
        if qty <= Decimal::ZERO {
            return (Some(vec![]), Decimal::ZERO);
        }
        let buy = O::is_buy_side(side);
        let map = if buy { &self.asks } else { &self.bids };
        let mut remaining = qty;
        let mut out = vec![];
        let iter: Box<dyn Iterator<Item = (&Px, &Vec<SlotIndex>)>> =
            if buy { Box::new(map.iter()) } else { Box::new(map.iter().rev()) };
        for (&Px(px), indices) in iter {
            if (buy && px > price) || (!buy && px < price) {
                break;
            }
            let mut sorted = indices.clone();
            sorted.sort_by_key(|&i| {
                self.slots[i.0]
                    .as_ref()
                    .map(|o| (o.created_at(), o.entity_id()))
                    .unwrap_or((u64::MAX, u64::MAX))
            });
            for i in sorted {
                if let Some(o) = self.slots[i.0].as_ref() {
                    let available = if o.qty() > o.filled_qty() {
                        o.qty() - o.filled_qty()
                    } else {
                        Decimal::ZERO
                    };
                    remaining =
                        if remaining > available { remaining - available } else { Decimal::ZERO };
                    out.push(o);
                }
                if remaining == Decimal::ZERO {
                    return (Some(out), Decimal::ZERO);
                }
            }
        }
        (if out.is_empty() { None } else { Some(out) }, remaining)
    }
    fn best_bid(&self) -> Option<Decimal> {
        self.bids.keys().next_back().map(|price| price.0)
    }
    fn best_ask(&self) -> Option<Decimal> {
        self.asks.keys().next().map(|price| price.0)
    }
    fn symbol(&self) -> Symbol {
        self.symbol
    }
    fn replay_events(&mut self, events: &[EntityReplayableEvent]) -> Result<(), LobError> {
        let mut next = self.clone();
        for event in events {
            next.replay_event(event)?;
        }
        *self = next;
        Ok(())
    }
    fn replay_event(&mut self, event: &EntityReplayableEvent) -> Result<(), LobError> {
        if event.entity_type != O::entity_type() {
            return Err(LobError::Event("wrong entity type".into()));
        }
        let id = event.entity_id;
        if event.lsn <= self.sequence {
            return Err(LobError::Event("nonconsecutive event sequence".into()));
        }
        match event.change_type {
            t if t == EntityChangeType::Created.as_tag() => {
                if self.ids.contains_key(&id) || event.previous_lsn != 0 {
                    return Err(LobError::Event("invalid create".into()));
                }
                let order = O::from_replay_event(event, id)?;
                if order.symbol() != self.symbol {
                    return Err(LobError::Event("wrong order book".into()));
                }
                self.put(order);
            }
            t if t == EntityChangeType::Updated.as_tag() => {
                let old = self
                    .order(id)
                    .ok_or_else(|| LobError::Event("order does not exist".into()))?
                    .clone();
                if old.entity_ver() != event.previous_lsn {
                    return Err(LobError::Event("invalid update".into()));
                }
                let order = old.apply_replay_event(event)?;
                if order.symbol() != self.symbol {
                    return Err(LobError::Event("cross-book update".into()));
                }
                self.put(order);
            }
            t if t == EntityChangeType::Deleted.as_tag() => {
                let old =
                    self.order(id).ok_or_else(|| LobError::Event("order does not exist".into()))?;
                if old.entity_ver() != event.previous_lsn {
                    return Err(LobError::Event("invalid delete".into()));
                }
                self.delete(id);
            }
            _ => return Err(LobError::Event("unknown change type".into())),
        }
        self.sequence = event.lsn;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use base_types_v2::{Coin, Symbol};
    use common_entity::EntityReplayableEvent;
    use rust_decimal::Decimal;
    use serde::{Deserialize, Serialize};

    use super::{LocalLobBtree, Px, SlotIndex};
    use crate::LobError;
    use crate::lob::{AssetLobV2, LobOrder};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    enum Side {
        Buy,
        Sell,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestOrder {
        id: u64,
        price: Decimal,
        qty: Decimal,
        filled_qty: Decimal,
        side: Side,
        created_at: u64,
        symbol: Symbol,
    }

    impl TestOrder {
        fn new(id: u64, price: Decimal, qty: Decimal, side: Side, created_at: u64) -> Self {
            Self {
                id,
                price,
                qty,
                filled_qty: Decimal::ZERO,
                side,
                created_at,
                symbol: test_symbol(),
            }
        }

        fn with_symbol(mut self, symbol: Symbol) -> Self {
            self.symbol = symbol;
            self
        }

        fn with_filled_qty(mut self, filled_qty: Decimal) -> Self {
            self.filled_qty = filled_qty;
            self
        }
    }

    impl LobOrder for TestOrder {
        type Side = Side;

        fn entity_type() -> u8 {
            1
        }
        fn entity_id(&self) -> u64 {
            self.id
        }
        fn symbol(&self) -> Symbol {
            self.symbol
        }
        fn side(&self) -> Self::Side {
            self.side
        }
        fn is_buy_side(side: Self::Side) -> bool {
            side == Side::Buy
        }
        fn px(&self) -> Decimal {
            self.price
        }
        fn qty(&self) -> Decimal {
            self.qty
        }
        fn filled_qty(&self) -> Decimal {
            self.filled_qty
        }
        fn created_at(&self) -> u64 {
            self.created_at
        }
        fn entity_ver(&self) -> u64 {
            0
        }
        fn is_active(&self) -> bool {
            true
        }
        fn from_replay_event(_: &EntityReplayableEvent, _: u64) -> Result<Self, LobError> {
            unreachable!("test orders are inserted directly")
        }
        fn apply_replay_event(self, _: &EntityReplayableEvent) -> Result<Self, LobError> {
            unreachable!("test orders are inserted directly")
        }
    }

    fn test_symbol() -> Symbol {
        Symbol::new(7, Coin::BTC, Coin::USDT)
    }

    #[test]
    fn price_key_bincode_matches_decimal() {
        let value = Decimal::new(4_212_345, 5);

        assert_eq!(bincode::serialize(&Px(value)).unwrap(), bincode::serialize(&value).unwrap());
    }

    #[test]
    fn decimal_prices_determine_best_bid_and_ask() {
        let mut lob = LocalLobBtree::new(test_symbol());
        for order in [
            TestOrder::new(1, Decimal::new(10001, 2), Decimal::ONE, Side::Buy, 1),
            TestOrder::new(2, Decimal::new(100001, 3), Decimal::ONE, Side::Buy, 2),
            TestOrder::new(3, Decimal::new(10002, 2), Decimal::ONE, Side::Buy, 3),
            TestOrder::new(4, Decimal::new(10003, 2), Decimal::ONE, Side::Sell, 4),
            TestOrder::new(5, Decimal::new(100025, 3), Decimal::ONE, Side::Sell, 5),
            TestOrder::new(6, Decimal::new(101, 0), Decimal::ONE, Side::Sell, 6),
        ] {
            lob.insert_order(order).unwrap();
        }

        assert_eq!(lob.best_bid(), Some(Decimal::new(10002, 2)));
        assert_eq!(lob.best_ask(), Some(Decimal::new(100025, 3)));
    }

    #[test]
    fn decimal_limit_respects_price_then_time_priority() {
        let mut lob = LocalLobBtree::new(test_symbol());
        for order in [
            TestOrder::new(3, Decimal::new(10002, 2), Decimal::TWO, Side::Sell, 30),
            TestOrder::new(2, Decimal::new(10001, 2), Decimal::TWO, Side::Sell, 20),
            TestOrder::new(1, Decimal::new(10001, 2), Decimal::TWO, Side::Sell, 10),
            TestOrder::new(4, Decimal::new(10003, 2), Decimal::TWO, Side::Sell, 40),
        ] {
            lob.insert_order(order).unwrap();
        }

        let (orders, remaining) =
            lob.match_orders(Decimal::new(10002, 2), Decimal::new(5, 0), Side::Buy);
        let ids: Vec<_> = orders.unwrap().into_iter().map(|order| order.id).collect();
        assert_eq!(ids, vec![1, 2, 3]);
        assert_eq!(remaining, Decimal::ZERO);

        let (orders, remaining) =
            lob.match_orders(Decimal::new(100005, 3), Decimal::ONE, Side::Buy);
        assert!(orders.is_none());
        assert_eq!(remaining, Decimal::ONE);

        let mut bids = LocalLobBtree::new(test_symbol());
        for order in [
            TestOrder::new(5, Decimal::new(9999, 2), Decimal::ONE, Side::Buy, 50),
            TestOrder::new(6, Decimal::new(9998, 2), Decimal::ONE, Side::Buy, 60),
        ] {
            bids.insert_order(order).unwrap();
        }

        let (orders, remaining) =
            bids.match_orders(Decimal::new(99985, 3), Decimal::ONE, Side::Sell);
        let ids: Vec<_> = orders.unwrap().into_iter().map(|order| order.id).collect();
        assert_eq!(ids, vec![5]);
        assert_eq!(remaining, Decimal::ZERO);

        let (orders, remaining) =
            bids.match_orders(Decimal::new(99995, 3), Decimal::ONE, Side::Sell);
        assert!(orders.is_none());
        assert_eq!(remaining, Decimal::ONE);
    }

    #[test]
    fn decimal_price_order_book_round_trips_through_bincode() {
        let mut lob = LocalLobBtree::new(test_symbol());
        lob.insert_order(TestOrder::new(
            1,
            Decimal::new(1234567, 4),
            Decimal::new(9, 0),
            Side::Buy,
            1,
        ))
        .unwrap();

        let encoded = bincode::serialize(&lob).unwrap();
        let restored: LocalLobBtree<TestOrder> = bincode::deserialize(&encoded).unwrap();

        assert_eq!(restored, lob);
        assert_eq!(restored.best_bid(), Some(Decimal::new(1234567, 4)));
    }

    #[test]
    fn slot_index_bincode_matches_usize() {
        let value = 42_usize;

        assert_eq!(
            bincode::serialize(&SlotIndex(value)).unwrap(),
            bincode::serialize(&value).unwrap()
        );
    }

    #[test]
    fn rejects_orders_from_a_different_symbol() {
        let mut lob = LocalLobBtree::new(test_symbol());
        let order = TestOrder::new(1, Decimal::ONE, Decimal::ONE, Side::Buy, 1);

        lob.insert_order(order.clone()).unwrap();
        for symbol in [
            Symbol::new(8, Coin::BTC, Coin::USDT),
            Symbol::new(7, Coin::ETH, Coin::USDT),
            Symbol::new(7, Coin::BTC, Coin::USDC),
        ] {
            assert!(matches!(
                lob.insert_order(order.clone().with_symbol(symbol)),
                Err(LobError::Event(message)) if message == "wrong order book"
            ));
        }
    }

    #[test]
    fn decimal_quantities_are_matched_without_truncation() {
        let mut lob = LocalLobBtree::new(test_symbol());
        lob.insert_order(
            TestOrder::new(1, Decimal::ONE, Decimal::new(175, 2), Side::Sell, 1)
                .with_filled_qty(Decimal::new(25, 2)),
        )
        .unwrap();
        lob.insert_order(TestOrder::new(2, Decimal::ONE, Decimal::new(225, 2), Side::Sell, 2))
            .unwrap();

        let (orders, remaining) = lob.match_orders(Decimal::ONE, Decimal::new(325, 2), Side::Buy);
        assert_eq!(orders.unwrap().len(), 2);
        assert_eq!(remaining, Decimal::ZERO);

        let (_, remaining) = lob.match_orders(Decimal::ONE, Decimal::new(4, 0), Side::Buy);
        assert_eq!(remaining, Decimal::new(25, 2));
    }

    #[test]
    fn non_positive_and_overfilled_quantities_preserve_saturation_semantics() {
        let mut lob = LocalLobBtree::new(test_symbol());
        lob.insert_order(
            TestOrder::new(1, Decimal::ONE, Decimal::ONE, Side::Sell, 1)
                .with_filled_qty(Decimal::TWO),
        )
        .unwrap();

        for quantity in [Decimal::ZERO, Decimal::NEGATIVE_ONE] {
            let (orders, remaining) = lob.match_orders(Decimal::ONE, quantity, Side::Buy);
            assert_eq!(orders, Some(vec![]));
            assert_eq!(remaining, Decimal::ZERO);
        }

        let (orders, remaining) = lob.match_orders(Decimal::ONE, Decimal::ONE, Side::Buy);
        assert_eq!(orders.unwrap().len(), 1);
        assert_eq!(remaining, Decimal::ONE);
    }
}
