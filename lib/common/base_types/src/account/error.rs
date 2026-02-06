//! 错误类型定义

use crate::{AccountId, AssetId};

/// 余额错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BalanceError {
    /// 可用余额不足
    InsufficientAvailable { required: i64, available: i64 },
    /// 冻结余额不足
    InsufficientFrozen { required: i64, frozen: i64 },
    /// 余额溢出（price * quantity 超出 i64）
    Overflow,
    /// 账户不存在
    AccountNotFound { account_id: AccountId },
    /// 余额记录不存在
    BalanceNotFound { account_id: AccountId, asset_id: AssetId },
    /// 账户已冻结（禁止交易）
    AccountFrozen { account_id: AccountId },
    /// 账户已注销
    AccountClosed { account_id: AccountId },
    /// 版本冲突（乐观锁）
    VersionConflict { expected: u64, actual: u64 },
}

impl std::fmt::Display for BalanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BalanceError::InsufficientAvailable { required, available } => {
                write!(
                    f,
                    "Insufficient available balance: required {}, available {}",
                    required, available
                )
            }
            BalanceError::InsufficientFrozen { required, frozen } => {
                write!(f, "Insufficient frozen balance: required {}, frozen {}", required, frozen)
            }
            BalanceError::Overflow => write!(f, "Balance overflow"),
            BalanceError::AccountNotFound { account_id } => {
                write!(f, "Account not found: {:?}", account_id)
            }
            BalanceError::BalanceNotFound { account_id, asset_id } => {
                write!(f, "Balance not found: account {:?}, asset {:?}", account_id, asset_id)
            }
            BalanceError::AccountFrozen { account_id } => {
                write!(f, "Account frozen: {:?}", account_id)
            }
            BalanceError::AccountClosed { account_id } => {
                write!(f, "Account closed: {:?}", account_id)
            }
            BalanceError::VersionConflict { expected, actual } => {
                write!(f, "Version conflict: expected {}, actual {}", expected, actual)
            }
        }
    }
}

impl std::error::Error for BalanceError {}
