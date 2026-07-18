mod model;

#[cfg(test)]
mod reservation_bdd_happy_path;
#[cfg(test)]
mod tests;

pub use model::{
    AssetReservation, MarginReservation, Reservation, ReservationCloseReason, ReservationError,
    ReservationKind, ReservationMarketKind, ReservationStatus,
};

pub(super) const RESERVATION_ENTITY_TYPE: u8 = 23;

pub(super) fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}
