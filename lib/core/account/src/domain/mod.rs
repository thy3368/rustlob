//! Domain layer module
//!
//! Contains entity, service and repository interface definitions

pub mod entity;
pub mod repository;
pub mod service;

// Re-export for convenience
pub use entity::*;
pub use repository::{AccountRepository, BalanceRepository};
pub use service::AccountService;
