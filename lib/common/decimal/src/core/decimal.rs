//! 定点数类型实现
//!
//! 包含 Decimal 类型及其各种操作实现

use std::fmt;

/// 定点数（内部使用 i64 存储，8 位小数精度）
///
/// 统一的数值类型，用于：
/// - 价格：50000.12345678 USDT
/// - 数量：1.00000000 BTC
/// - 金额：1000.50000000
///
/// 特点：
/// - 8 位小数精度（与 Bitcoin Satoshi 一致）
/// - 最小单位：0.00000001
/// - 避免浮点数精度问题
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Decimal(i64);

impl Decimal {
    const DECIMALS: i64 = 100_000_000; // 8 位小数

    #[inline]
    pub fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    #[inline]
    pub fn raw(&self) -> i64 {
        self.0
    }

    #[inline]
    pub fn from_f64(value: f64) -> Self {
        Self((value * Self::DECIMALS as f64) as i64)
    }

    #[inline]
    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / Self::DECIMALS as f64
    }

    #[inline]
    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// 定点数乘法：Decimal * Decimal -> Decimal
    /// 用于计算名义价值 (notional) 等场景
    #[inline]
    pub fn checked_mul(&self, rhs: impl Into<i128>) -> Option<Decimal> {
        let lhs = self.0 as i128;
        let rhs = rhs.into();
        let result = lhs.checked_mul(rhs)?;
        // 需要除以 DECIMALS 因为两个都有 8 位小数
        let normalized = result.checked_div(Self::DECIMALS as i128)?;
        i64::try_from(normalized).ok().map(Decimal)
    }

    /// 与另一个 Decimal 相乘
    #[inline]
    pub fn mul(&self, other: Decimal) -> Option<Decimal> {
        self.checked_mul(other.raw())
    }
}

impl std::ops::Add for Decimal {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Decimal {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::AddAssign for Decimal {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::SubAssign for Decimal {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul for Decimal {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let result = self.0 as i128 * rhs.0 as i128;
        let normalized = result / Self::DECIMALS as i128;
        Decimal(normalized as i64)
    }
}

impl std::ops::Div for Decimal {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0 {
            Decimal(0)
        } else {
            let result = self.0 as i128 * Self::DECIMALS as i128;
            let normalized = result / rhs.0 as i128;
            Decimal(normalized as i64)
        }
    }
}

impl Default for Decimal {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

impl From<Decimal> for i128 {
    #[inline]
    fn from(d: Decimal) -> Self {
        d.0 as i128
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_f64())
    }
}

impl std::iter::Sum for Decimal {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Decimal(0), |acc, x| acc + x)
    }
}

impl<'a> std::iter::Sum<&'a Decimal> for Decimal {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().fold(Decimal(0), |acc, x| acc + x)
    }
}
