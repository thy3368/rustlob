//! 账户服务实现
//!
//! 遵循 Clean Architecture：Service 层实现业务逻辑
//! Repository 层只负责纯粹的 CRUD 操作

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
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: u64, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_mut(account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
            account_id,
            asset_id
        })?;

        if balance.available < amount {
            return Err(BalanceError::InsufficientAvailable {
                required: amount,
                available: balance.available
            });
        }

        let new_frozen = balance.frozen.checked_add(amount).ok_or(BalanceError::Overflow)?;

        balance.available -= amount;
        balance.frozen = new_frozen;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 解冻操作：冻结 → 可用
    #[inline]
    fn do_unfreeze(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: u64, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_mut(account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
            account_id,
            asset_id
        })?;

        if balance.frozen < amount {
            return Err(BalanceError::InsufficientFrozen {
                required: amount,
                frozen: balance.frozen
            });
        }

        let new_available = balance.available.checked_add(amount).ok_or(BalanceError::Overflow)?;

        balance.frozen -= amount;
        balance.available = new_available;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 入账操作：增加可用余额
    #[inline]
    fn do_credit(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: u64, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_or_create(account_id, asset_id, now);

        balance.available = balance.available.checked_add(amount).ok_or(BalanceError::Overflow)?;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 扣款操作：减少可用余额
    #[inline]
    fn do_debit(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: u64, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_mut(account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
            account_id,
            asset_id
        })?;

        if balance.available < amount {
            return Err(BalanceError::InsufficientAvailable {
                required: amount,
                available: balance.available
            });
        }

        balance.available -= amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 扣减冻结余额
    #[inline]
    fn do_debit_frozen(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, amount: u64, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_mut(account_id, asset_id).ok_or(BalanceError::BalanceNotFound {
            account_id,
            asset_id
        })?;

        if balance.frozen < amount {
            return Err(BalanceError::InsufficientFrozen {
                required: amount,
                frozen: balance.frozen
            });
        }

        balance.frozen -= amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    /// 转账操作（原子）
    #[inline]
    fn do_transfer(
        &mut self, from_account_id: crate::AccountId, to_account_id: crate::AccountId, asset_id: crate::AssetId,
        amount: u64, now: Timestamp
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
                    required: amount,
                    available: from_balance.available
                });
            }
        }

        // 2. 预检查：目标账户是否会溢出
        {
            if let Some(to_balance) = self.balance_repo.get(to_account_id, asset_id) {
                to_balance.available.checked_add(amount).ok_or(BalanceError::Overflow)?;
            }
        }

        // 3. 执行转账 - 扣减源账户
        {
            let from_balance = self.balance_repo.get_mut(from_account_id, asset_id).unwrap();
            from_balance.available -= amount;
            from_balance.version += 1;
            from_balance.updated_at = now;
        }

        // 4. 增加目标账户
        {
            let to_balance = self.balance_repo.get_or_create(to_account_id, asset_id, now);
            to_balance.available += amount;
            to_balance.version += 1;
            to_balance.updated_at = now;
        }

        Ok(())
    }

    /// 结算盈亏
    #[inline]
    fn do_settle_pnl(
        &mut self, account_id: crate::AccountId, asset_id: crate::AssetId, pnl: i64, now: Timestamp
    ) -> Result<crate::Balance, BalanceError> {
        let balance = self.balance_repo.get_or_create(account_id, asset_id, now);

        if pnl >= 0 {
            balance.available = balance.available.checked_add(pnl as u64).ok_or(BalanceError::Overflow)?;
        } else {
            let loss = (-pnl) as u64;
            if balance.available < loss {
                return Err(BalanceError::InsufficientAvailable {
                    required: loss,
                    available: balance.available
                });
            }
            balance.available -= loss;
        }
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
                        required: *amount,
                        available: balance.available
                    });
                }
                // 检查目标账户是否会溢出
                if let Some(to_balance) = self.balance_repo.get(*to_account_id, *asset_id) {
                    to_balance.available.checked_add(*amount).ok_or(BalanceError::Overflow)?;
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
                    balance.available.checked_add(*amount).ok_or(BalanceError::Overflow)?;
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
                        required: amount,
                        available: balance.available
                    });
                }

                balance.frozen.checked_add(amount).ok_or(BalanceError::Overflow)?;
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
                        required: *amount,
                        available: balance.available
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
                        required: *amount,
                        available: balance.available
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
                        required: *amount,
                        frozen: balance.frozen
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

    #[test]
    fn test_check_and_freeze_buy() {
        let mut service = create_test_service();

        let cmd = AccountCommand::CheckAndFreeze {
            account_id: AccountId(1),
            order_id: 1001,
            pair: TradingPair::BTC_USDT,
            side: Side::Buy,
            price: 100,
            quantity: 10
        };

        let result = service.execute(cmd);

        match result {
            AccountCommandResult::Frozen {
                reference_id,
                asset_id,
                amount,
                new_available,
                new_frozen
            } => {
                assert_eq!(reference_id, 1001);
                assert_eq!(asset_id, AssetId::USDT);
                assert_eq!(amount, 1000); // 100 * 10
                assert_eq!(new_available, 9000); // 10000 - 1000
                assert_eq!(new_frozen, 1000);
            }
            _ => panic!("Expected Frozen result")
        }
    }

    #[test]
    fn test_check_and_freeze_sell() {
        let mut service = create_test_service();

        let cmd = AccountCommand::CheckAndFreeze {
            account_id: AccountId(1),
            order_id: 1002,
            pair: TradingPair::BTC_USDT,
            side: Side::Sell,
            price: 100,
            quantity: 10
        };

        let result = service.execute(cmd);

        match result {
            AccountCommandResult::Frozen {
                reference_id,
                asset_id,
                amount,
                new_available,
                new_frozen
            } => {
                assert_eq!(reference_id, 1002);
                assert_eq!(asset_id, AssetId::BTC);
                assert_eq!(amount, 10);
                assert_eq!(new_available, 90); // 100 - 10
                assert_eq!(new_frozen, 10);
            }
            _ => panic!("Expected Frozen result")
        }
    }

    #[test]
    fn test_unfreeze() {
        let mut service = create_test_service();

        // 先冻结
        let freeze_cmd = AccountCommand::CheckAndFreeze {
            account_id: AccountId(1),
            order_id: 1001,
            pair: TradingPair::BTC_USDT,
            side: Side::Buy,
            price: 100,
            quantity: 10
        };
        service.execute(freeze_cmd);

        // 再解冻
        let unfreeze_cmd = AccountCommand::Unfreeze {
            account_id: AccountId(1),
            order_id: 1001,
            pair: TradingPair::BTC_USDT,
            side: Side::Buy,
            price: 100,
            quantity: 10
        };

        let result = service.execute(unfreeze_cmd);

        match result {
            AccountCommandResult::Unfrozen {
                new_available,
                new_frozen,
                ..
            } => {
                assert_eq!(new_available, 10000); // 恢复原值
                assert_eq!(new_frozen, 0);
            }
            _ => panic!("Expected Unfrozen result")
        }
    }

    #[test]
    fn test_insufficient_balance() {
        let mut service = create_test_service();

        let cmd = AccountCommand::CheckAndFreeze {
            account_id: AccountId(1),
            order_id: 1001,
            pair: TradingPair::BTC_USDT,
            side: Side::Buy,
            price: 1000,
            quantity: 100 // 需要 100000，但只有 10000
        };

        let result = service.execute(cmd);

        match result {
            AccountCommandResult::Error(BalanceError::InsufficientAvailable {
                required,
                available
            }) => {
                assert_eq!(required, 100000);
                assert_eq!(available, 10000);
            }
            _ => panic!("Expected InsufficientAvailable error")
        }
    }

    #[test]
    fn test_batch_execute() {
        let mut service = create_test_service();

        // 添加第二个账户
        let account2 = Account::new(AccountId(2), UserId(101), AccountType::Spot, 1000);
        service.account_repo_mut().save(account2);
        service.balance_repo_mut().set_balance(AccountId(2), AssetId::USDT, 5000);

        let cmds = vec![
            AccountCommand::Debit {
                account_id: AccountId(1),
                asset_id: AssetId::USDT,
                amount: 1000,
                reference_id: 1
            },
            AccountCommand::Credit {
                account_id: AccountId(2),
                asset_id: AssetId::USDT,
                amount: 1000,
                reference_id: 1
            },
        ];

        let results = service.execute_batch(cmds).unwrap();
        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
    }
}
