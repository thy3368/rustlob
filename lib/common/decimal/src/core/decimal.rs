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

    /// 将 Decimal 转换为格式化的字符串
    pub fn to_string(&self) -> String {
        let integral = self.0 / Self::DECIMALS;
        let fractional = self.0 % Self::DECIMALS;

        let fractional_str = format!("{:08}", fractional.abs());
        let trimmed = fractional_str.trim_end_matches('0').trim_end_matches('.');
        if fractional < 0 {
            format!("-{}.{}", integral.abs(), if trimmed.is_empty() { "0" } else { trimmed })
        } else {
            format!("{}.{}", integral, if trimmed.is_empty() { "0" } else { trimmed })
        }
    }

    /// 从字符串解析 Decimal
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        let parts: Vec<_> = s.split('.').collect();
        match parts.len() {
            1 => {
                // 没有小数部分
                let integral = match parts[0].parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => return Err("Invalid integer part"),
                };
                Ok(Decimal(integral * Self::DECIMALS))
            }
            2 => {
                // 有小数部分
                let integral = match parts[0].parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => return Err("Invalid integer part"),
                };
                let mut fractional_str = parts[1].to_string();

                // 补零或截断到 8 位小数
                if fractional_str.len() < 8 {
                    fractional_str = format!("{:0<8}", fractional_str);
                } else if fractional_str.len() > 8 {
                    fractional_str = fractional_str[..8].to_string();
                }

                let fractional = match fractional_str.parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => return Err("Invalid fractional part"),
                };

                let mut total = integral.abs() * Self::DECIMALS + fractional;
                if integral < 0 {
                    total = -total;
                }

                Ok(Decimal(total))
            }
            _ => Err("Invalid decimal format"),
        }
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

#[cfg(feature = "serde")]
impl serde::Serialize for Decimal {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Decimal {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match Decimal::from_string(&s) {
            Ok(d) => Ok(d),
            Err(_) => Err(serde::de::Error::custom(format!("Invalid decimal format: {}", s))),
        }
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
