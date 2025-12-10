//! Repository interfaces
//!
//! Defines repo traits for persistence abstraction

pub mod account_repo;
pub mod balance_repo;

pub use account_repo::AccountRepo;
pub use balance_repo::BalanceRepo;
