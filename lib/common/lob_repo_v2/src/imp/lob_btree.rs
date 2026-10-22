use std::collections::{BTreeMap, HashMap};

use common_entity::{EntityChangeType, EntityReplayableEvent};
use serde::{Deserialize, Serialize};

use crate::LobError;
use crate::lob::{AssetLobV2, LobOrder};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(serialize = "O: Serialize", deserialize = "O: Deserialize<'de>"))]
pub struct LocalLobBtree<O: LobOrder> {
    asset: u32,
    slots: Vec<Option<O>>,
    ids: HashMap<u64, usize>,
    bids: BTreeMap<u64, Vec<usize>>,
    asks: BTreeMap<u64, Vec<usize>>,
    sequence: u64,
}
impl<O: LobOrder> LocalLobBtree<O> {
    pub fn new(asset: u32) -> Self {
        Self {
            asset,
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
        self.ids.get(&id).and_then(|&i| self.slots[i].as_ref())
    }
    pub fn insert_order(&mut self, order: O) -> Result<(), LobError> {
        if order.asset() != self.asset {
            return Err(LobError::Event("wrong order book".into()));
        }
        self.put(order);
        Ok(())
    }
    fn remove_idx(&mut self, o: &O, i: usize) {
        let map = if O::is_buy_side(o.side()) { &mut self.bids } else { &mut self.asks };
        if let Some(v) = map.get_mut(&o.price()) {
            v.retain(|&x| x != i);
            if v.is_empty() {
                map.remove(&o.price());
            }
        }
    }
    fn add_idx(&mut self, o: &O, i: usize) {
        if !o.is_active() {
            return;
        }
        let map = if O::is_buy_side(o.side()) { &mut self.bids } else { &mut self.asks };
        map.entry(o.price()).or_default().push(i);
    }
    fn put(&mut self, o: O) {
        let id = o.entity_id();
        if let Some(i) = self.ids.get(&id).copied() {
            if let Some(old) = self.slots[i].take() {
                self.remove_idx(&old, i);
            }
            self.slots[i] = Some(o.clone());
            self.add_idx(&o, i);
        } else {
            let i = self.slots.len();
            self.slots.push(Some(o.clone()));
            self.ids.insert(id, i);
            self.add_idx(&o, i);
        }
    }
    fn delete(&mut self, id: u64) {
        if let Some(i) = self.ids.remove(&id) {
            if let Some(o) = self.slots[i].take() {
                self.remove_idx(&o, i);
            }
        }
    }
}
impl<O: LobOrder> AssetLobV2 for LocalLobBtree<O> {
    type Order = O;
    fn match_orders(&self, p: u64, q: u64, s: O::Side) -> (Option<Vec<&Self::Order>>, u64) {
        if q == 0 {
            return (Some(vec![]), 0);
        }
        let buy = O::is_buy_side(s);
        let map = if buy { &self.asks } else { &self.bids };
        let mut remaining = q;
        let mut out = vec![];
        let iter: Box<dyn Iterator<Item = (&u64, &Vec<usize>)>> =
            if buy { Box::new(map.iter()) } else { Box::new(map.iter().rev()) };
        for (&px, indices) in iter {
            if (buy && px > p) || (!buy && px < p) {
                break;
            }
            let mut sorted = indices.clone();
            sorted.sort_by_key(|&i| {
                self.slots[i]
                    .as_ref()
                    .map(|o| (o.created_at(), o.entity_id()))
                    .unwrap_or((u64::MAX, u64::MAX))
            });
            for i in sorted {
                if let Some(o) = self.slots[i].as_ref() {
                    remaining = remaining.saturating_sub(o.qty().saturating_sub(o.filled_qty()));
                    out.push(o);
                }
                if remaining == 0 {
                    return (Some(out), 0);
                }
            }
        }
        (if out.is_empty() { None } else { Some(out) }, remaining)
    }
    fn best_bid(&self) -> Option<u64> {
        self.bids.keys().next_back().copied()
    }
    fn best_ask(&self) -> Option<u64> {
        self.asks.keys().next().copied()
    }
    fn asset(&self) -> u32 {
        self.asset
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
                if order.asset() != self.asset {
                    return Err(LobError::Event("wrong order book".into()));
                }
                self.put(order);
            }
            t if t == EntityChangeType::Updated.as_tag() => {
                let old = self
                    .order(id)
                    .ok_or_else(|| LobError::Event("order does not exist".into()))?
                    .clone();
                if old.entity_version() != event.previous_lsn {
                    return Err(LobError::Event("invalid update".into()));
                }
                let order = old.apply_replay_event(event)?;
                if order.asset() != self.asset {
                    return Err(LobError::Event("cross-book update".into()));
                }
                self.put(order);
            }
            t if t == EntityChangeType::Deleted.as_tag() => {
                let old =
                    self.order(id).ok_or_else(|| LobError::Event("order does not exist".into()))?;
                if old.entity_version() != event.previous_lsn {
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
