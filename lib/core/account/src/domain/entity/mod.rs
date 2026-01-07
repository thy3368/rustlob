//! Entity module
//!
//! Contains all entity definitions for the account module

pub mod account;
pub mod balance;
pub mod command;
pub mod error;
pub mod user;

// Re-export commonly used types
pub use account::{Account, AccountStatus, AccountType};
pub use balance::{Balance, BalanceOp};
// 从 base_types 重导出共享类型
pub use base_types::{
    AccountId, AssetId, OrderId, PositionId, PositionSide, PrepPosition, Price, Quantity, Side, Timestamp, TradingPair,
    UserId
};
pub use command::{AccountCommand, AccountCommandResult};
pub use error::BalanceError;
