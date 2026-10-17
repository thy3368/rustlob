use std::collections::{BTreeMap, HashMap};

use common_entity::{Entity, EntityChangeType, EntityReplayableEvent};
use example_core_entity::{SpotOrderSide, SpotOrderStatus, SpotOrderV2};
use serde::{Deserialize, Serialize};

use crate::lob::AssetLobV2;
use crate::{LobError, apply_fields, create_order};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalLobBtree {
    asset: u32,
    slots: Vec<Option<SpotOrderV2>>,
    ids: HashMap<u64, usize>,
    bids: BTreeMap<u64, Vec<usize>>,
    asks: BTreeMap<u64, Vec<usize>>,
    sequence: u64,
}
impl LocalLobBtree {
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
    pub fn order(&self, id: u64) -> Option<&SpotOrderV2> {
        self.ids.get(&id).and_then(|&i| self.slots[i].as_ref())
    }

    pub fn insert_order(&mut self, order: SpotOrderV2) -> Result<(), LobError> {
        if order.asset() != self.asset {
            return Err(LobError::Event("wrong order book".into()));
        }
        self.put(order);
        Ok(())
    }

    fn active(o: &SpotOrderV2) -> bool {
        matches!(o.status(), SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
            && o.filled_qty() < o.qty()
    }
    fn remove_idx(&mut self, o: &SpotOrderV2, i: usize) {
        let m = if o.side() == SpotOrderSide::Buy { &mut self.bids } else { &mut self.asks };
        if let Some(v) = m.get_mut(&o.limit_price) {
            v.retain(|&x| x != i);
            if v.is_empty() {
                m.remove(&o.limit_price);
            }
        }
    }
    fn add_idx(&mut self, o: &SpotOrderV2, i: usize) {
        if !Self::active(o) {
            return;
        }
        let m = if o.side() == SpotOrderSide::Buy { &mut self.bids } else { &mut self.asks };
        m.entry(o.limit_price).or_default().push(i)
    }
    fn put(&mut self, o: SpotOrderV2) {
        let id = o.order_id();
        if let Some(i) = self.ids.get(&id).copied() {
            if let Some(old) = self.slots[i].take() {
                self.remove_idx(&old, i)
            }
            self.slots[i] = Some(o.clone());
            self.add_idx(&o, i)
        } else {
            let i = self.slots.len();
            self.slots.push(Some(o.clone()));
            self.ids.insert(id, i);
            self.add_idx(&o, i)
        }
    }
    fn delete(&mut self, id: u64) {
        if let Some(i) = self.ids.remove(&id) {
            if let Some(o) = self.slots[i].take() {
                self.remove_idx(&o, i)
            }
        }
    }
}
impl AssetLobV2 for LocalLobBtree {
    type Order = SpotOrderV2;
    fn match_orders(&self, p: u64, q: u64, s: SpotOrderSide) -> (Option<Vec<&Self::Order>>, u64) {
        if q == 0 {
            return (Some(vec![]), 0);
        }
        let m = if s == SpotOrderSide::Buy { &self.asks } else { &self.bids };
        let mut r = q;
        let mut out = vec![];
        let it: Box<dyn Iterator<Item = (&u64, &Vec<usize>)>> =
            if s == SpotOrderSide::Buy { Box::new(m.iter()) } else { Box::new(m.iter().rev()) };
        for (&px, ss) in it {
            if (s == SpotOrderSide::Buy && px > p) || (s == SpotOrderSide::Sell && px < p) {
                break;
            }
            let mut x = ss.clone();
            x.sort_by_key(|&i| {
                let o = self.slots[i].as_ref().unwrap();
                (o.created_at, o.order_id())
            });
            for i in x {
                let o = self.slots[i].as_ref().unwrap();
                r = r.saturating_sub(o.qty() - o.filled_qty());
                out.push(o);
                if r == 0 {
                    return (Some(out), 0);
                }
            }
        }
        (if out.is_empty() { None } else { Some(out) }, r)
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

    fn replay_events(&mut self, es: &[EntityReplayableEvent]) -> Result<(), LobError> {
        let mut n = self.clone();
        for e in es {
            n.replay_event(e)?
        }
        *self = n;
        Ok(())
    }
    fn replay_event(&mut self, e: &EntityReplayableEvent) -> Result<(), LobError> {
        if e.entity_type != SpotOrderV2::entity_type() {
            return Err(LobError::Event("wrong entity type".into()));
        }
        let id =
            u64::try_from(e.entity_id).map_err(|_| LobError::Event("negative entity id".into()))?;
        if e.sequence != self.sequence + 1 {
            return Err(LobError::Event("nonconsecutive event sequence".into()));
        }
        match e.change_type {
            t if t == EntityChangeType::Created.as_tag() => {
                if self.ids.contains_key(&id) || e.old_version != 0 || e.new_version != 1 {
                    return Err(LobError::Event("invalid create".into()));
                }
                let o = create_order(e, id)?;
                if o.asset() != self.asset {
                    return Err(LobError::Event("wrong order book".into()));
                }
                self.put(o)
            }
            t if t == EntityChangeType::Updated.as_tag() => {
                let old = self
                    .order(id)
                    .ok_or_else(|| LobError::Event("order does not exist".into()))?
                    .clone();
                if old.version != e.old_version || e.new_version != e.old_version + 1 {
                    return Err(LobError::Event("invalid update".into()));
                }
                let o = apply_fields(old, e)?;
                if o.asset() != self.asset {
                    return Err(LobError::Event("cross-book update".into()));
                }
                self.put(o)
            }
            t if t == EntityChangeType::Deleted.as_tag() => {
                let o =
                    self.order(id).ok_or_else(|| LobError::Event("order does not exist".into()))?;
                if o.version != e.old_version || e.new_version != e.old_version + 1 {
                    return Err(LobError::Event("invalid delete".into()));
                }
                self.delete(id)
            }
            _ => return Err(LobError::Event("unknown change type".into())),
        }
        self.sequence = e.sequence;
        Ok(())
    }
}
