mod facts;
mod model;

#[cfg(test)]
mod tests;

pub use facts::{ReservationConsumed, ReservationCreated, ReservationReleased};
pub use model::{
    AssetReservation, MarginReservation, Reservation, ReservationCloseReason, ReservationError,
    ReservationKind, ReservationMarketKind, ReservationStatus,
};

pub(super) const RESERVATION_ENTITY_TYPE: u8 = 23;
pub(super) const RESERVATION_CREATED_ENTITY_TYPE: u8 = 24;
pub(super) const RESERVATION_CONSUMED_ENTITY_TYPE: u8 = 25;
pub(super) const RESERVATION_RELEASED_ENTITY_TYPE: u8 = 26;

pub(super) fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}
