//! Adaptor module
//!
//! Contains inbound (service implementations) and outbound (persistence) adaptors

pub mod inbound;
pub mod outbound;

// Re-export for convenience
pub use inbound::AccountServiceImpl;
pub use outbound::{InMemoryAccountRepository, InMemoryBalanceRepository};

