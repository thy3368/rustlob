mod model;

#[cfg(test)]
mod tests;

pub use model::{
    AssetReservation, MarginReservation, Reservation, ReservationCloseReason, ReservationError,
    ReservationKind, ReservationMarketKind, ReservationStatus,
};

pub(super) const RESERVATION_ENTITY_TYPE: u8 = 23;
