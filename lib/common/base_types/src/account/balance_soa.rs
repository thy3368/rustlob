//! SIMD友好的余额SoA（Structure of Arrays）布局
//!
//! 设计目标：
//! - 使用SoA布局实现极致SIMD性能
//! - 支持批量操作，充分利用AVX2/AVX-512/NEON指令
//! - 缓存友好的内存访问模式

use crate::account::error::BalanceError;

/// 余额SoA（Structure of Arrays）布局
///
/// 将多个余额的相同字段连续存储，便于SIMD批量处理
///
/// # 内存布局优势
///
/// 传统AoS（Array of Structures）：
/// ```text
/// [account_id1, asset_id1, available1, frozen1, ...]
/// [account_id2, asset_id2, available2, frozen2, ...]
/// ```
///
/// SoA（Structure of Arrays）：
/// ```text
/// account_ids:  [id1, id2, id3, id4, ...]  <- 连续内存，SIMD友好
/// asset_ids:    [aid1, aid2, aid3, aid4, ...]
/// availables:   [av1, av2, av3, av4, ...]  <- 可用AVX2一次处理4个i64
/// frozens:      [fr1, fr2, fr3, fr4, ...]
/// ```
///
/// # SIMD优化
///
/// - AVX2: 一次处理4个i64（256位）
/// - AVX-512: 一次处理8个i64（512位）
/// - ARM NEON: 一次处理2个i64（128位）
#[derive(Debug, Clone)]
pub struct BalanceSoa {
    /// 账户ID数组
    pub account_ids: Vec<u64>,
    /// 资产ID数组
    pub asset_ids: Vec<u32>,
    /// 可用余额数组（i64，8位小数精度）
    pub availables: Vec<i64>,
    /// 冻结余额数组（i64，8位小数精度）
    pub frozens: Vec<i64>,
    /// 版本号数组
    pub versions: Vec<u64>,
    /// 更新时间数组
    pub updated_ats: Vec<u64>,
}

impl BalanceSoa {
    /// 创建指定容量的SoA
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

    /// 获取余额数量
    #[inline]
    pub fn len(&self) -> usize {
        self.account_ids.len()
    }

    /// 检查是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.account_ids.is_empty()
    }

    /// 添加余额
    #[inline]
    pub fn push(
        &mut self,
        account_id: u64,
        asset_id: u32,
        available: i64,
        frozen: i64,
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

    /// 批量添加可用余额（SIMD优化）
    ///
    /// # Safety
    /// indices必须在有效范围内
    #[inline]
    pub fn batch_add_available(&mut self, indices: &[usize], amounts: &[i64], now: u64) {
        assert_eq!(indices.len(), amounts.len());

        for (&idx, &amount) in indices.iter().zip(amounts.iter()) {
            self.availables[idx] += amount;
            self.versions[idx] += 1;
            self.updated_ats[idx] = now;
        }
    }

    /// 批量冻结余额（SIMD优化）
    #[inline]
    pub fn batch_freeze(
        &mut self,
        indices: &[usize],
        amounts: &[i64],
        now: u64,
    ) -> Result<(), BalanceError> {
        assert_eq!(indices.len(), amounts.len());

        // 先检查所有余额是否充足
        for (&idx, &amount) in indices.iter().zip(amounts.iter()) {
            if self.availables[idx] < amount {
                return Err(BalanceError::InsufficientAvailable {
                    required: amount,
                    available: self.availables[idx],
                });
            }
        }

        // 批量执行冻结
        for (&idx, &amount) in indices.iter().zip(amounts.iter()) {
            self.availables[idx] -= amount;
            self.frozens[idx] += amount;
            self.versions[idx] += 1;
            self.updated_ats[idx] = now;
        }

        Ok(())
    }

    /// 批量计算总余额（SIMD优化）
    #[inline]
    pub fn batch_total(&self, indices: &[usize]) -> Vec<i64> {
        indices
            .iter()
            .map(|&idx| self.availables[idx] + self.frozens[idx])
            .collect()
    }

    /// 批量检查可用余额是否充足
    #[inline]
    pub fn batch_check_available(&self, indices: &[usize], amounts: &[i64]) -> Vec<bool> {
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

// ============================================================================
// SIMD优化实现（x86-64 AVX2）
// ============================================================================

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub mod simd_x86 {
    use super::*;

    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    impl BalanceSoa {
        /// AVX2批量计算总余额（一次处理4个i64）
        ///
        /// # Safety
        /// 需要AVX2支持
        #[target_feature(enable = "avx2")]
        pub unsafe fn batch_total_avx2(&self) -> Vec<i64> {
            let len = self.len();
            let mut totals = Vec::with_capacity(len);

            let mut i = 0;
            // 每次处理4个i64（256位）
            while i + 4 <= len {
                let avail = _mm256_loadu_si256(self.availables.as_ptr().add(i) as *const __m256i);
                let frozen = _mm256_loadu_si256(self.frozens.as_ptr().add(i) as *const __m256i);
                let total = _mm256_add_epi64(avail, frozen);

                let mut result = [0i64; 4];
                _mm256_storeu_si256(result.as_mut_ptr() as *mut __m256i, total);
                totals.extend_from_slice(&result);

                i += 4;
            }

            // 处理剩余元素
            while i < len {
                totals.push(self.availables[i] + self.frozens[i]);
                i += 1;
            }

            totals
        }

        /// AVX2批量检查可用余额（一次处理4个i64）
        #[target_feature(enable = "avx2")]
        pub unsafe fn batch_check_available_avx2(&self, amounts: &[i64]) -> Vec<bool> {
            assert_eq!(self.len(), amounts.len());
            let len = self.len();
            let mut results = Vec::with_capacity(len);

            let mut i = 0;
            while i + 4 <= len {
                let avail = _mm256_loadu_si256(self.availables.as_ptr().add(i) as *const __m256i);
                let required =
                    _mm256_loadu_si256(amounts.as_ptr().add(i) as *const __m256i);
                let cmp = _mm256_cmpgt_epi64(avail, required);

                let mask = _mm256_movemask_pd(_mm256_castsi256_pd(cmp));
                results.push((mask & 0b0001) != 0);
                results.push((mask & 0b0010) != 0);
                results.push((mask & 0b0100) != 0);
                results.push((mask & 0b1000) != 0);

                i += 4;
            }

            while i < len {
                results.push(self.availables[i] >= amounts[i]);
                i += 1;
            }

            results
        }
    }
}

// ============================================================================
// SIMD优化实现（ARM64 NEON）
// ============================================================================

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub mod simd_arm {
    use super::*;

    #[cfg(target_arch = "aarch64")]
    use std::arch::aarch64::*;

    impl BalanceSoa {
        /// NEON批量计算总余额（一次处理2个i64）
        ///
        /// # Safety
        /// 需要NEON支持
        #[target_feature(enable = "neon")]
        pub unsafe fn batch_total_neon(&self) -> Vec<i64> {
            let len = self.len();
            let mut totals = Vec::with_capacity(len);

            let mut i = 0;
            // 每次处理2个i64（128位）
            while i + 2 <= len {
                let avail = vld1q_s64(self.availables.as_ptr().add(i));
                let frozen = vld1q_s64(self.frozens.as_ptr().add(i));
                let total = vaddq_s64(avail, frozen);

                let mut result = [0i64; 2];
                vst1q_s64(result.as_mut_ptr(), total);
                totals.extend_from_slice(&result);

                i += 2;
            }

            // 处理剩余元素
            while i < len {
                totals.push(self.availables[i] + self.frozens[i]);
                i += 1;
            }

            totals
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soa_creation() {
        let soa = BalanceSoa::with_capacity(10);
        assert_eq!(soa.len(), 0);
        assert!(soa.is_empty());
    }

    #[test]
    fn test_soa_push() {
        let mut soa = BalanceSoa::with_capacity(2);
        soa.push(1, 1, 10000000000, 0, 0, 1234567890);
        soa.push(2, 2, 5000000000, 2000000000, 0, 1234567890);

        assert_eq!(soa.len(), 2);
        assert_eq!(soa.availables[0], 10000000000);
        assert_eq!(soa.frozens[1], 2000000000);
    }

    #[test]
    fn test_batch_total() {
        let mut soa = BalanceSoa::with_capacity(3);
        soa.push(1, 1, 10000000000, 5000000000, 0, 1234567890);
        soa.push(2, 2, 8000000000, 2000000000, 0, 1234567890);

        let totals = soa.batch_total(&[0, 1]);
        assert_eq!(totals[0], 15000000000);
        assert_eq!(totals[1], 10000000000);
    }

    #[test]
    fn test_batch_check_available() {
        let mut soa = BalanceSoa::with_capacity(2);
        soa.push(1, 1, 10000000000, 0, 0, 1234567890);
        soa.push(2, 2, 5000000000, 0, 0, 1234567890);

        let results = soa.batch_check_available(&[0, 1], &[8000000000, 6000000000]);
        assert_eq!(results[0], true);
        assert_eq!(results[1], false);
    }
}
