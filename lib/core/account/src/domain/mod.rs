//! Domain layer module
//!
//! Contains entity, service and repo interface definitions

pub mod entity;
pub mod repo;
pub mod service;

// Re-export for convenience
pub use entity::*;
pub use repo::{AccountRepo, BalanceRepo};
pub use service::AccountService;
