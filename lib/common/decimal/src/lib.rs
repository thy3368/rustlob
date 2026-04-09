use num_traits::ToPrimitive;
use rust_decimal::Decimal as Rd;

#[derive(Clone, Copy, Default)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecimalWrapper(i64);

pub type Decimal = DecimalWrapper;
pub type LobDecimal = DecimalWrapper;

impl DecimalWrapper {
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
        Self((value * 100_000_000.0) as i64)
    }

    #[inline]
    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
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

    pub fn checked_mul(&self, rhs: DecimalWrapper) -> Option<DecimalWrapper> {
        let lhs = self.0 as i128;
        let rhs = rhs.0 as i128;
        let result = lhs.checked_mul(rhs)?;
        let normalized = result.checked_div(100_000_000)?;
        i64::try_from(normalized).ok().map(Self)
    }

    #[inline]
    pub fn to_rd(&self) -> Rd {
        Rd::new(self.0, 8)
    }

    #[inline]
    pub fn from_rd(rd: Rd) -> Self {
        Self(rd.to_i64().unwrap_or(0))
    }
}

impl From<i64> for DecimalWrapper {
    fn from(raw: i64) -> Self {
        Self(raw)
    }
}

impl From<DecimalWrapper> for i128 {
    fn from(d: DecimalWrapper) -> Self {
        d.0 as i128
    }
}

impl std::ops::Add for DecimalWrapper {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for DecimalWrapper {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for DecimalWrapper {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for DecimalWrapper {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul for DecimalWrapper {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let result = self.0 as i128 * rhs.0 as i128;
        let normalized = result / 100_000_000;
        Self(normalized as i64)
    }
}

impl std::ops::Div for DecimalWrapper {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            Self(0)
        } else {
            let result = self.0 as i128 * 100_000_000 as i128;
            let normalized = result / rhs.0 as i128;
            Self(normalized as i64)
        }
    }
}

impl PartialEq for DecimalWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for DecimalWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for DecimalWrapper {}

impl Ord for DecimalWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::hash::Hash for DecimalWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl std::fmt::Debug for DecimalWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_f64())
    }
}

impl std::fmt::Display for DecimalWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_f64())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for DecimalWrapper {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_rd().to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DecimalWrapper {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let rd: Rd = s.parse().unwrap_or(Rd::ZERO);
        Ok(Self(rd.to_i64().unwrap_or(0)))
    }
}

impl std::iter::Sum for DecimalWrapper {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(0), |acc, x| acc + x)
    }
}

impl<'a> std::iter::Sum<&'a DecimalWrapper> for DecimalWrapper {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().fold(Self(0), |acc, x| acc + x)
    }
}
