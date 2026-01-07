//! Account 模块
//!
//! 负责账户和余额管理，通过统一的 `AccountCommand` 处理来自 LOB（下单/撤单）和
//! Settlement（结算）的所有余额操作。
//!
//! # 核心功能
//!
//! - **余额管理**: 管理用户各资产的可用/冻结余额
//! - **余额检查**: 为下单提供余额充足性验证
//! - **统一命令处理**: 处理来自 LOB 和 Settlement 的 AccountCommand
//! - **乐观锁**: 使用 version 字段保证并发安全
//!
//! # 架构
//!
//! 遵循 Clean Architecture：
//! - **Domain Layer**: Entity, Service trait, Repository trait
//! - **Adaptor Layer**: Inbound (Service impl), Outbound (Repository impl)
//!
//! # 使用示例
//!
//! ```ignore
//! use account::adaptor::{AccountServiceImpl, InMemoryAccountRepository, InMemoryBalanceRepository};
//! use account::domain::{AccountCommand, AccountService, Side, TradingPair, AccountId};
//!
//! // 创建 Repository
//! let account_repo = InMemoryAccountRepository::new();
//! let balance_repo = InMemoryBalanceRepository::with_default_timestamp();
//!
//! // 创建 Service
//! let mut service = AccountServiceImpl::new(
//!     account_repo,
//!     balance_repo,
//!     || std::time::SystemTime::now()
//!         .duration_since(std::time::UNIX_EPOCH)
//!         .unwrap()
//!         .as_nanos() as u64,
//! );
//!
//! // 下单时冻结资金
//! let cmd = AccountCommand::CheckAndFreeze {
//!     account_id: AccountId(1),
//!     order_id: 1001,
//!     pair: TradingPair::BTC_USDT,
//!     side: Side::Buy,
//!     price: 50000,
//!     quantity: 1,
//! };
//!
//! let result = service.execute(cmd);
//! ```

pub mod domain;

// Re-export commonly used types
// Re-export adaptor implementations
pub use domain::{
    entity::{
        Account, AccountCommand, AccountCommandResult, AccountId, AccountStatus, AccountType, AssetId, Balance,
        BalanceError, BalanceOp, OrderId,  Side, TradingPair,
        Timestamp, UserId
    },
};
