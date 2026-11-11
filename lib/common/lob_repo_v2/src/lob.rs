use base_types_v2::Symbol;
use common_entity::EntityReplayableEvent;
use rust_decimal::Decimal;

use crate::LobError;
pub trait LobOrder: Clone + std::fmt::Debug + Send + Sync + 'static {
    type Side: Copy + Eq;

    fn entity_type() -> u8;
    fn entity_id(&self) -> u64;
    fn symbol(&self) -> Symbol;
    fn side(&self) -> Self::Side;
    fn is_buy_side(side: Self::Side) -> bool;
    fn px(&self) -> Decimal;
    fn orig_size(&self) -> Decimal;
    fn size(&self) -> Decimal;
    fn created_at(&self) -> u64;
    fn entity_ver(&self) -> u64;
    fn is_active(&self) -> bool;
    fn from_replay_event(event: &EntityReplayableEvent, id: u64) -> Result<Self, crate::LobError>;
    fn apply_replay_event(self, event: &EntityReplayableEvent) -> Result<Self, crate::LobError>;
}

pub trait AssetLobV2 {
    type Order: LobOrder;

    // price_tick_size: Decimal, // 价格最小步长
    // size_tick_size: Decimal,  // 数量最小步长

    fn match_orders(
        &self,
        price: Decimal,
        quantity: Decimal,
        taker_side: <Self::Order as LobOrder>::Side,
    ) -> (Option<Vec<&Self::Order>>, Decimal);
    fn best_bid(&self) -> Option<Decimal>;
    fn best_ask(&self) -> Option<Decimal>;

    fn symbol(&self) -> Symbol;

    fn replay_events(&mut self, events: &[EntityReplayableEvent]) -> Result<(), LobError>;
    fn replay_event(&mut self, event: &EntityReplayableEvent) -> Result<(), LobError>;
}
