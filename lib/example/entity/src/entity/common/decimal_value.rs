use std::fmt;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DecimalValueError {
    #[error("decimal value exceeds scale {scale}")]
    ExcessPrecision { scale: u32 },
}

macro_rules! decimal_newtype {
    ($name:ident, $scale:literal) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name(Decimal);
        impl $name {
            pub fn new(value: Decimal) -> Result<Self, DecimalValueError> {
                let value = value.normalize();
                if value.scale() > $scale {
                    return Err(DecimalValueError::ExcessPrecision { scale: $scale });
                }
                Ok(Self(value))
            }
            pub fn from_scaled_i128(raw: i128) -> Self {
                Self(Decimal::from_i128_with_scale(raw, $scale))
            }
            pub const fn value(self) -> Decimal {
                self.0
            }
            pub fn raw(self) -> i128 {
                self.0.mantissa()
            }
            pub fn tick() -> Decimal {
                Decimal::new(1, $scale)
            }
            pub fn zero() -> Self {
                Self(Decimal::ZERO)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.normalize().fmt(f)
            }
        }
    };
}
decimal_newtype!(FundingRate, 10);
decimal_newtype!(FundingRate2, 9);

// impl FundingRate {
//     pub fn from_bps(bps: i128) -> Self {
//         Self::from_scaled_i128(bps * 1_000_000)
//     }
//     pub fn from_percent(pct: i128) -> Self {
//         Self::from_scaled_i128(pct * 100_000_000)
//     }
//     pub fn parse_str(s: &str) -> Result<Self, rust_decimal::Error> {
//         let value = s.parse::<Decimal>()?;
//         Self::new(value).map_err(|_| rust_decimal::Error::ExceedsMaximumPossibleValue)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn funding_rate_validates_precision() {
        let value = FundingRate::new(Decimal::new(216_606, 10)).unwrap();
        assert_eq!(value.to_string(), "0.0000216606");
        assert_eq!(FundingRate::from_scaled_i128(216_606), value);
        assert!(FundingRate::new(Decimal::new(1, 11)).is_err());
        assert_eq!(std::mem::size_of::<FundingRate>(), std::mem::size_of::<Decimal>());
    }
    #[test]
    fn funding_rate2_uses_scale_nine() {
        assert_eq!(FundingRate2::tick(), Decimal::new(1, 9));
        assert_eq!(FundingRate2::from_scaled_i128(21_660_600).to_string(), "0.0216606");
        assert!(FundingRate2::new(Decimal::new(1, 10)).is_err());
    }
}
