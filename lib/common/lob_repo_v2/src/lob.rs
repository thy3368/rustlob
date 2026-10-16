use common_entity::EntityReplayableEvent;
use example_core_entity::SpotOrderSide;

use crate::LobError;
pub trait AssetLobV2 {
    type Order;
    fn match_orders(
        &self,
        price: u64,
        quantity: u64,
        taker_side: SpotOrderSide,
    ) -> (Option<Vec<&Self::Order>>, u64);
    fn best_bid(&self) -> Option<u64>;
    fn best_ask(&self) -> Option<u64>;

    fn asset(&self) -> u32;

    fn replay_events(&mut self, events: &[EntityReplayableEvent]) -> Result<(), LobError>;
    fn replay_event(&mut self, event: &EntityReplayableEvent) -> Result<(), LobError>;
}
