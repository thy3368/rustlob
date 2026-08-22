//! Decimal 余额批处理容器。
//!
//! 文件名保留以减少引用面；金额字段已经改为 `Decimal`，不再承诺 SIMD 友好布局。

use crate::Quantity;
use crate::account::error::BalanceError;

/// 多个余额的分列存储容器。
#[derive(Debug, Clone)]
pub struct BalanceSoa {
    /// 账户ID数组
    pub account_ids: Vec<u64>,
    /// 资产ID数组
    pub asset_ids: Vec<u32>,
    /// 可用余额数组
    pub availables: Vec<Quantity>,
    /// 冻结余额数组
    pub frozens: Vec<Quantity>,
    /// 版本号数组
    pub versions: Vec<u64>,
    /// 更新时间数组
    pub updated_ats: Vec<u64>,
}

impl BalanceSoa {
    /// 创建指定容量的容器。
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            account_ids: Vec::with_capacity(capacity),
            asset_ids: Vec::with_capacity(capacity),
            availables: Vec::with_capacity(capacity),
            frozens: Vec::with_capacity(capacity),
            versions: Vec::with_capacity(capacity),
            updated_ats: Vec::with_capacity(capacity),
        }
    }

    /// 获取余额数量。
    #[inline]
    pub fn len(&self) -> usize {
        self.account_ids.len()
    }

    /// 检查是否为空。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.account_ids.is_empty()
    }

    /// 添加余额。
    #[inline]
    pub fn push(
        &mut self,
        account_id: u64,
        asset_id: u32,
        available: Quantity,
        frozen: Quantity,
        version: u64,
        updated_at: u64,
    ) {
        self.account_ids.push(account_id);
        self.asset_ids.push(asset_id);
        self.availables.push(available);
        self.frozens.push(frozen);
        self.versions.push(version);
        self.updated_ats.push(updated_at);
    }

    /// 批量添加可用余额。
    #[inline]
    pub fn batch_add_available(&mut self, indices: &[usize], amounts: &[Quantity], now: u64) {
        assert_eq!(indices.len(), amounts.len());

        for (&idx, &amount) in indices.iter().zip(amounts.iter()) {
            self.availables[idx] += amount;
            self.versions[idx] += 1;
            self.updated_ats[idx] = now;
        }
    }

    /// 批量冻结余额。
    #[inline]
    pub fn batch_freeze(
        &mut self,
        indices: &[usize],
        amounts: &[Quantity],
        now: u64,
    ) -> Result<(), BalanceError> {
        assert_eq!(indices.len(), amounts.len());

        for (&idx, &amount) in indices.iter().zip(amounts.iter()) {
            if self.availables[idx] < amount {
                return Err(BalanceError::InsufficientAvailable {
                    required: amount,
                    available: self.availables[idx],
                });
            }
        }

        for (&idx, &amount) in indices.iter().zip(amounts.iter()) {
            self.availables[idx] -= amount;
            self.frozens[idx] += amount;
            self.versions[idx] += 1;
            self.updated_ats[idx] = now;
        }

        Ok(())
    }

    /// 批量计算总余额。
    #[inline]
    pub fn batch_total(&self, indices: &[usize]) -> Vec<Quantity> {
        indices.iter().map(|&idx| self.availables[idx] + self.frozens[idx]).collect()
    }

    /// 批量检查可用余额是否充足。
    #[inline]
    pub fn batch_check_available(&self, indices: &[usize], amounts: &[Quantity]) -> Vec<bool> {
        assert_eq!(indices.len(), amounts.len());

        indices
            .iter()
            .zip(amounts.iter())
            .map(|(&idx, &amount)| self.availables[idx] >= amount)
            .collect()
    }
}

impl Default for BalanceSoa {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}
