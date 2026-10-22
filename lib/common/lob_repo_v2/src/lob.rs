use common_entity::EntityReplayableEvent;

use crate::LobError;
pub trait LobOrder: Clone + std::fmt::Debug + Send + Sync + 'static {
    type Side: Copy + Eq;

    fn entity_type() -> u8;
    fn entity_id(&self) -> u64;
    fn asset(&self) -> u32;
    fn side(&self) -> Self::Side;
    fn price(&self) -> u64;
    fn qty(&self) -> u64;
    fn filled_qty(&self) -> u64;
    fn created_at(&self) -> u64;
    fn is_active(&self) -> bool;
    fn from_replay_event(event: &EntityReplayableEvent, id: u64) -> Result<Self, crate::LobError>;
    fn apply_replay_event(self, event: &EntityReplayableEvent) -> Result<Self, crate::LobError>;
}

pub trait AssetLobV2 {
    type Order: LobOrder;
    fn match_orders(
        &self,
        price: u64,
        quantity: u64,
        taker_side: <Self::Order as LobOrder>::Side,
    ) -> (Option<Vec<&Self::Order>>, u64);
    fn best_bid(&self) -> Option<u64>;
    fn best_ask(&self) -> Option<u64>;

    fn asset(&self) -> u32;

    fn replay_events(&mut self, events: &[EntityReplayableEvent]) -> Result<(), LobError>;
    fn replay_event(&mut self, event: &EntityReplayableEvent) -> Result<(), LobError>;
}
