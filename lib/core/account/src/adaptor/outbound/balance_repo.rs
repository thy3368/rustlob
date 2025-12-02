//! 内存实现的余额仓储

use std::collections::HashMap;

use crate::domain::entity::{AccountId, AssetId, Balance, BalanceError, Timestamp};
use crate::domain::repository::BalanceRepository;

/// 内存余额仓储实现
pub struct InMemoryBalanceRepository {
    /// 余额缓存 ((account_id, asset_id) -> Balance)
    balances: HashMap<(AccountId, AssetId), Balance>,
    /// 时间戳生成器
    timestamp_fn: fn() -> Timestamp,
}

impl InMemoryBalanceRepository {
    /// 创建新的内存余额仓储
    pub fn new(timestamp_fn: fn() -> Timestamp) -> Self {
        Self {
            balances: HashMap::new(),
            timestamp_fn,
        }
    }

    /// 使用默认时间戳函数创建
    pub fn with_default_timestamp() -> Self {
        Self::new(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        })
    }

    /// 获取当前时间戳
    #[inline]
    fn now(&self) -> Timestamp {
        (self.timestamp_fn)()
    }

    /// 设置余额（测试用）
    pub fn set_balance(&mut self, account_id: AccountId, asset_id: AssetId, available: u64) {
        let now = self.now();
        self.balances.insert(
            (account_id, asset_id),
            Balance::with_available(account_id, asset_id, available, now),
        );
    }
}

impl BalanceRepository for InMemoryBalanceRepository {
    fn get(&self, account_id: AccountId, asset_id: AssetId) -> Option<&Balance> {
        self.balances.get(&(account_id, asset_id))
    }

    fn get_mut(&mut self, account_id: AccountId, asset_id: AssetId) -> Option<&mut Balance> {
        self.balances.get_mut(&(account_id, asset_id))
    }

    fn get_or_create(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        now: Timestamp,
    ) -> &mut Balance {
        self.balances
            .entry((account_id, asset_id))
            .or_insert_with(|| Balance::new(account_id, asset_id, now))
    }

    fn save(&mut self, balance: Balance) {
        self.balances
            .insert((balance.account_id, balance.asset_id), balance);
    }

    fn exists(&self, account_id: AccountId, asset_id: AssetId) -> bool {
        self.balances.contains_key(&(account_id, asset_id))
    }

    fn get_all_by_account(&self, account_id: AccountId) -> Vec<&Balance> {
        self.balances
            .iter()
            .filter(|((acc_id, _), _)| *acc_id == account_id)
            .map(|(_, balance)| balance)
            .collect()
    }

    fn freeze(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError> {
        let balance = self
            .get_mut(account_id, asset_id)
            .ok_or(BalanceError::BalanceNotFound {
                account_id,
                asset_id,
            })?;

        if balance.available < amount {
            return Err(BalanceError::InsufficientAvailable {
                required: amount,
                available: balance.available,
            });
        }

        let new_frozen = balance
            .frozen
            .checked_add(amount)
            .ok_or(BalanceError::Overflow)?;

        balance.available -= amount;
        balance.frozen = new_frozen;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    fn unfreeze(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError> {
        let balance = self
            .get_mut(account_id, asset_id)
            .ok_or(BalanceError::BalanceNotFound {
                account_id,
                asset_id,
            })?;

        if balance.frozen < amount {
            return Err(BalanceError::InsufficientFrozen {
                required: amount,
                frozen: balance.frozen,
            });
        }

        let new_available = balance
            .available
            .checked_add(amount)
            .ok_or(BalanceError::Overflow)?;

        balance.frozen -= amount;
        balance.available = new_available;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    fn credit(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError> {
        let balance = self.get_or_create(account_id, asset_id, now);

        balance.available = balance
            .available
            .checked_add(amount)
            .ok_or(BalanceError::Overflow)?;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    fn debit(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError> {
        let balance = self
            .get_mut(account_id, asset_id)
            .ok_or(BalanceError::BalanceNotFound {
                account_id,
                asset_id,
            })?;

        if balance.available < amount {
            return Err(BalanceError::InsufficientAvailable {
                required: amount,
                available: balance.available,
            });
        }

        balance.available -= amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    fn debit_frozen(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError> {
        let balance = self
            .get_mut(account_id, asset_id)
            .ok_or(BalanceError::BalanceNotFound {
                account_id,
                asset_id,
            })?;

        if balance.frozen < amount {
            return Err(BalanceError::InsufficientFrozen {
                required: amount,
                frozen: balance.frozen,
            });
        }

        balance.frozen -= amount;
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }

    fn transfer(
        &mut self,
        from_account_id: AccountId,
        to_account_id: AccountId,
        asset_id: AssetId,
        amount: u64,
        now: Timestamp,
    ) -> Result<(), BalanceError> {
        // 1. 预检查：源账户余额
        {
            let from_balance =
                self.get(from_account_id, asset_id)
                    .ok_or(BalanceError::BalanceNotFound {
                        account_id: from_account_id,
                        asset_id,
                    })?;

            if from_balance.available < amount {
                return Err(BalanceError::InsufficientAvailable {
                    required: amount,
                    available: from_balance.available,
                });
            }
        }

        // 2. 预检查：目标账户是否会溢出
        {
            if let Some(to_balance) = self.get(to_account_id, asset_id) {
                to_balance
                    .available
                    .checked_add(amount)
                    .ok_or(BalanceError::Overflow)?;
            }
        }

        // 3. 执行转账 - 扣减源账户
        {
            let from_balance = self.get_mut(from_account_id, asset_id).unwrap();
            from_balance.available -= amount;
            from_balance.version += 1;
            from_balance.updated_at = now;
        }

        // 4. 增加目标账户
        {
            let to_balance = self.get_or_create(to_account_id, asset_id, now);
            to_balance.available += amount;
            to_balance.version += 1;
            to_balance.updated_at = now;
        }

        Ok(())
    }

    fn settle_pnl(
        &mut self,
        account_id: AccountId,
        asset_id: AssetId,
        pnl: i64,
        now: Timestamp,
    ) -> Result<Balance, BalanceError> {
        let balance = self.get_or_create(account_id, asset_id, now);

        if pnl >= 0 {
            balance.available = balance
                .available
                .checked_add(pnl as u64)
                .ok_or(BalanceError::Overflow)?;
        } else {
            let loss = (-pnl) as u64;
            if balance.available < loss {
                return Err(BalanceError::InsufficientAvailable {
                    required: loss,
                    available: balance.available,
                });
            }
            balance.available -= loss;
        }
        balance.version += 1;
        balance.updated_at = now;

        Ok(balance.clone())
    }
}
