//! Outbound adaptors
//!
//! Provides persistence and external service implementations
//! TODO: PostgresBalanceStore implementation

pub mod account_repo;
pub mod balance_repo;

pub use account_repo::InMemoryAccountRepository;
pub use balance_repo::InMemoryBalanceRepository;