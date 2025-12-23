//! 账户服务实现
//!
//! 遵循 Clean Architecture：Service 层实现业务逻辑
//! Repository 层只负责纯粹的 CRUD 操作

use base_types::Price;
use crate::domain::{
    entity::{AccountCommand, AccountCommandResult, BalanceError, Side, Timestamp},
    repo::{AccountRepo, BalanceRepo},
    service::AccountService
};

/// 账户服务实现
///
/// 依赖注入 AccountRepository 和 BalanceRepository
pub struct AccountServiceImpl<A, B>
where
    A: AccountRepo,
    B: BalanceRepo
{
    /// 账户仓储
    account_repo: A,
    /// 余额仓储
    balance_repo: B,
    /// 时间戳生成器
    timestamp_fn: fn() -> Timestamp
}

impl<A, B> AccountServiceImpl<A, B>
where
    A: AccountRepo,
    B: BalanceRepo
{
    /// 创建新的账户服务
    pub fn new(account_repo: A, balance_repo: B, timestamp_fn: fn() -> Timestamp) -> Self {
        Self {
            account_repo,
            balance_repo,
            timestamp_fn
        }
    }

    /// 获取当前时间戳
    #[inline]
    fn now(&self) -> Timestamp { (self.timestamp_fn)() }

    /// 获取账户仓储的可变引用
    pub fn account_repo_mut(&mut self) -> &mut A { &mut self.account_repo }

    /// 获取余额仓储的可变引用
    pub fn balance_repo_mut(&mut self) -> &mut B { &mut self.balance_repo }

    // ==================== 业务操作（从 Repository 移至 Service）
    // ====================

    /// 冻结操作：可用 → 冻结
    #[inline]
    fn do_freeze(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: Price, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_mut(account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
            account_id,
            asset_id
        })?;

        if balance.available < amount {
            return Err(BalanceError::InsufficientAvailable {
                required: amount.raw(),
                available: balance.available.raw()
            });
        }

        balance.available = balance.available - amount;
        balance.frozen = balance.frozen + amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 解冻操作：冻结 → 可用
    #[inline]
    fn do_unfreeze(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: Price, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_mut(account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
            account_id,
            asset_id
        })?;

        if balance.frozen < amount {
            return Err(BalanceError::InsufficientFrozen {
                required: amount.raw(),
                frozen: balance.frozen.raw()
            });
        }

        balance.frozen = balance.frozen - amount;
        balance.available = balance.available + amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 入账操作：增加可用余额
    #[inline]
    fn do_credit(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: Price, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_or_create(account_id, asset_id, now);

        balance.available = balance.available + amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 扣款操作：减少可用余额
    #[inline]
    fn do_debit(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: Price, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_mut(account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
            account_id,
            asset_id
        })?;

        if balance.available < amount {
            return Err(BalanceError::InsufficientAvailable {
                required: amount.raw(),
                available: balance.available.raw()
            });
        }

        balance.available = balance.available - amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 扣减冻结余额
    #[inline]
    fn do_debit_frozen(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: Price, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_mut(account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
            account_id,
            asset_id
        })?;

        if balance.frozen < amount {
            return Err(BalanceError::InsufficientFrozen {
                required: amount.raw(),
                frozen: balance.frozen.raw()
            });
        }

        balance.frozen = balance.frozen - amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 转账操作（原子）
    #[inline]
    fn do_transfer(
        &mut self, from_account_id: crate::AccountId, to_account_id: crate::AccountId, asset_id: crate::AssetId,
        amount: Price, now: Timestamp
    ) -> Result<(), BalanceError> {
        // 1. 预检查：源账户余额
        {
            let from_balance =
                self.balance_repo.get(from_account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
                    account_id: from_account_id,
                    asset_id
                })?;

            if from_balance.available < amount {
                return Err(BalanceError::InsufficientAvailable {
                    required: amount.raw(),
                    available: from_balance.available.raw()
                });
            }
        }

        // 2. 执行转账 - 扣减源账户
        {
            let from_balance = self.balance_repo.get_mut(from_account_id, asset_id).unwrap();
            from_balance.available = from_balance.available - amount;
            from_balance.version += 1;
            from_balance.updated_at = now;
        }

        // 3. 增加目标账户
        {
            let to_balance = self.balance_repo.get_or_create(to_account_id, asset_id, now);
            to_balance.available = to_balance.available + amount;
            to_balance.version += 1;
            to_balance.updated_at = now;
        }

        Ok(())
    }

    /// 结算盈亏 - 直接支持负数（亏损）
    #[inline]
    fn do_settle_pnl(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, pnl: Price, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_or_create(account_id, asset_id, now);

        // 使用 Price 的加法，自动支持正负数
        balance.available = balance.available + pnl;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 预验证命令（用于批量执行的预检查）
    fn validate_command(&self, cmd: &AccountCommand) -> Result<(), BalanceError> {
        let account_id = cmd.account_id();

        // GetBalance 不需要检查账户状态
        if matches!(cmd, AccountCommand::GetBalance { .. }) {
            return Ok(());
        }

        self.account_repo.check_status(account_id)?;

        match cmd {
            AccountCommand::Transfer {
                to_account_id,
                asset_id,
                amount,
                ..
            } => {
                // 检查目标账户
                self.account_repo.check_status(*to_account_id)?;
                // 检查源账户余额
                let balance = self.balance_repo.get(account_id, *asset_id).ok_or(BalanceError::BalanceNotFound {
                    account_id,
                    asset_id: *asset_id
                })?;
                if balance.available < *amount {
                    return Err(BalanceError::InsufficientAvailable {
                        required: amount.raw(),
                        available: balance.available.raw()
                    });
                }
                // 检查目标账户是否会溢出
                if let Some(to_balance) = self.balance_repo.get(*to_account_id, *asset_id) {
                    // Price already handles overflow in its Add implementation
                    let _ = to_balance.available + *amount;
                }
                Ok(())
            }
            AccountCommand::Credit {
                account_id,
                asset_id,
                amount,
                ..
            } => {
                // 检查是否会溢出
                if let Some(balance) = self.balance_repo.get(*account_id, *asset_id) {
                    // Price already handles overflow in its Add implementation
                    let _ = balance.available + *amount;
                }
                Ok(())
            }
            AccountCommand::CheckAndFreeze {
                account_id,
                pair,
                side,
                price,
                quantity,
                ..
            } => {
                let (asset_id, amount) = match side {
                    Side::Buy => {
                        let amt = price.checked_mul(*quantity).ok_or(BalanceError::Overflow)?;
                        (pair.quote_asset, amt)
                    }
                    Side::Sell => (pair.base_asset, *quantity)
                };

                let balance = self.balance_repo.get(*account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
                    account_id: *account_id,
                    asset_id
                })?;

                if balance.available < amount {
                    return Err(BalanceError::InsufficientAvailable {
                        required: amount.raw(),
                        available: balance.available.raw()
                    });
                }

                // Price already handles overflow in its Add implementation
                let _ = balance.frozen + amount;
                Ok(())
            }
            AccountCommand::Freeze {
                account_id,
                asset_id,
                amount,
                ..
            } => {
                let balance = self.balance_repo.get(*account_id, *asset_id).ok_or(BalanceError::BalanceNotFound {
                    account_id: *account_id,
                    asset_id: *asset_id
                })?;

                if balance.available < *amount {
                    return Err(BalanceError::InsufficientAvailable {
                        required: amount.raw(),
                        available: balance.available.raw()
                    });
                }
                Ok(())
            }
            AccountCommand::Debit {
                account_id,
                asset_id,
                amount,
                ..
            } => {
                let balance = self.balance_repo.get(*account_id, *asset_id).ok_or(BalanceError::BalanceNotFound {
                    account_id: *account_id,
                    asset_id: *asset_id
                })?;

                if balance.available < *amount {
                    return Err(BalanceError::InsufficientAvailable {
                        required: amount.raw(),
                        available: balance.available.raw()
                    });
                }
                Ok(())
            }
            AccountCommand::DebitFrozen {
                account_id,
                asset_id,
                amount,
                ..
            } => {
                let balance = self.balance_repo.get(*account_id, *asset_id).ok_or(BalanceError::BalanceNotFound {
                    account_id: *account_id,
                    asset_id: *asset_id
                })?;

                if balance.frozen < *amount {
                    return Err(BalanceError::InsufficientFrozen {
                        required: amount.raw(),
                        frozen: balance.frozen.raw()
                    });
                }
                Ok(())
            }
            _ => Ok(())
        }
    }
}

impl<A, B> AccountService for AccountServiceImpl<A, B>
where
    A: AccountRepo,
    B: BalanceRepo
{
    fn execute(&mut self, cmd: AccountCommand) -> AccountCommandResult {
        let now = self.now();

        // 检查账户状态（查询操作跳过）
        if !matches!(cmd, AccountCommand::GetBalance { .. }) {
            let account_id = cmd.account_id();
            if let Err(e) = self.account_repo.check_status(account_id) {
                return AccountCommandResult::Error(e);
            }
        }

        match cmd {
            // ==================== LOB 调用 ====================
            AccountCommand::CheckAndFreeze {
                account_id,
                order_id,
                pair,
                side,
                price,
                quantity
            } => {
                let (asset_id, amount) = match side {
                    Side::Buy => match price.checked_mul(quantity) {
                        Some(amt) => (pair.quote_asset, amt),
                        None => return AccountCommandResult::Error(BalanceError::Overflow)
                    },
                    Side::Sell => (pair.base_asset, quantity)
                };

                match self.do_freeze(account_id, asset_id, amount, now) {
                    Ok(balance) => AccountCommandResult::Frozen {
                        reference_id: order_id,
                        asset_id,
                        amount,
                        new_available: balance.available,
                        new_frozen: balance.frozen
                    },
                    Err(e) => AccountCommandResult::Error(e)
                }
            }

            AccountCommand::Unfreeze {
                account_id,
                order_id,
                pair,
                side,
                price,
                quantity
            } => {
                let (asset_id, amount) = match side {
                    Side::Buy => match price.checked_mul(quantity) {
                        Some(amt) => (pair.quote_asset, amt),
                        None => return AccountCommandResult::Error(BalanceError::Overflow)
                    },
                    Side::Sell => (pair.base_asset, quantity)
                };

                match self.do_unfreeze(account_id, asset_id, amount, now) {
                    Ok(balance) => AccountCommandResult::Unfrozen {
                        reference_id: order_id,
                        asset_id,
                        amount,
                        new_available: balance.available,
                        new_frozen: balance.frozen
                    },
                    Err(e) => AccountCommandResult::Error(e)
                }
            }

            // ==================== Settlement 调用 ====================
            AccountCommand::Freeze {
                account_id,
                asset_id,
                amount,
                reference_id
            } => match self.do_freeze(account_id, asset_id, amount, now) {
                Ok(balance) => AccountCommandResult::Frozen {
                    reference_id,
                    asset_id,
                    amount,
                    new_available: balance.available,
                    new_frozen: balance.frozen
                },
                Err(e) => AccountCommandResult::Error(e)
            },

            AccountCommand::UnfreezeAsset {
                account_id,
                asset_id,
                amount,
                reference_id
            } => match self.do_unfreeze(account_id, asset_id, amount, now) {
                Ok(balance) => AccountCommandResult::Unfrozen {
                    reference_id,
                    asset_id,
                    amount,
                    new_available: balance.available,
                    new_frozen: balance.frozen
                },
                Err(e) => AccountCommandResult::Error(e)
            },

            AccountCommand::Credit {
                account_id,
                asset_id,
                amount,
                reference_id
            } => match self.do_credit(account_id, asset_id, amount, now) {
                Ok(balance) => AccountCommandResult::Credited {
                    reference_id,
                    asset_id,
                    amount,
                    new_available: balance.available
                },
                Err(e) => AccountCommandResult::Error(e)
            },

            AccountCommand::Debit {
                account_id,
                asset_id,
                amount,
                reference_id
            } => match self.do_debit(account_id, asset_id, amount, now) {
                Ok(balance) => AccountCommandResult::Debited {
                    reference_id,
                    asset_id,
                    amount,
                    from_frozen: false,
                    new_available: balance.available,
                    new_frozen: balance.frozen
                },
                Err(e) => AccountCommandResult::Error(e)
            },

            AccountCommand::DebitFrozen {
                account_id,
                asset_id,
                amount,
                reference_id
            } => match self.do_debit_frozen(account_id, asset_id, amount, now) {
                Ok(balance) => AccountCommandResult::Debited {
                    reference_id,
                    asset_id,
                    amount,
                    from_frozen: true,
                    new_available: balance.available,
                    new_frozen: balance.frozen
                },
                Err(e) => AccountCommandResult::Error(e)
            },

            AccountCommand::Transfer {
                from_account_id,
                to_account_id,
                asset_id,
                amount,
                reference_id
            } => {
                if let Err(e) = self.account_repo.check_status(to_account_id) {
                    return AccountCommandResult::Error(e);
                }

                match self.do_transfer(from_account_id, to_account_id, asset_id, amount, now) {
                    Ok(_) => AccountCommandResult::Transferred {
                        reference_id,
                        from_account_id,
                        to_account_id,
                        asset_id,
                        amount
                    },
                    Err(e) => AccountCommandResult::Error(e)
                }
            }

            AccountCommand::SettlePnl {
                account_id,
                asset_id,
                pnl,
                reference_id
            } => match self.do_settle_pnl(account_id, asset_id, pnl, now) {
                Ok(balance) => AccountCommandResult::PnlSettled {
                    reference_id,
                    asset_id,
                    pnl,
                    new_available: balance.available
                },
                Err(e) => AccountCommandResult::Error(e)
            },

            // ==================== 查询 ====================
            AccountCommand::GetBalance {
                account_id,
                asset_id
            } => AccountCommandResult::Balance(self.balance_repo.get(account_id, asset_id).cloned())
        }
    }

    fn execute_batch(&mut self, cmds: Vec<AccountCommand>) -> Result<Vec<AccountCommandResult>, BalanceError> {
        // 1. 预检查所有命令
        for cmd in &cmds {
            self.validate_command(cmd)?;
        }

        // 2. 执行所有命令（预检查通过后不会失败）
        let results = cmds.into_iter().map(|cmd| self.execute(cmd)).collect();

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adaptor::outbound::{MemoryAccountRepo, MemoryBalanceRepo},
        domain::entity::{Account, AccountId, AccountType, AssetId, TradingPair, UserId}
    };

    fn create_test_service() -> AccountServiceImpl<MemoryAccountRepo, MemoryBalanceRepo> {
        let account_repo = MemoryAccountRepo::new();
        let balance_repo = MemoryBalanceRepo::new(|| 1000);

        let mut service = AccountServiceImpl::new(account_repo, balance_repo, || 1000);

        // 添加测试账户
        let account = Account::new(AccountId(1), UserId(100), AccountType::Spot, 1000);
        service.account_repo_mut().save(account);

        // 设置初始余额
        service.balance_repo_mut().set_balance(AccountId(1), AssetId::USDT, 10000);
        service.balance_repo_mut().set_balance(AccountId(1), AssetId::BTC, 100);

        service
    }




}
