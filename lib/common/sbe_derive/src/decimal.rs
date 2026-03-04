//! Decimal type for SBE encoding
//!
//! Represents a decimal number as mantissa × 10^exponent

/// Decimal number representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decimal {
    mantissa: i64,
    exponent: i8,
}

impl Decimal {
    /// Create a new Decimal from mantissa and exponent
    pub fn new(mantissa: i64, exponent: i8) -> Self {
        Self { mantissa, exponent }
    }

    /// Get the mantissa
    pub fn mantissa(&self) -> i64 {
        self.mantissa
    }

    /// Get the exponent
    pub fn exponent(&self) -> i8 {
        self.exponent
    }

    /// Convert to f64
    pub fn to_f64(&self) -> f64 {
        self.mantissa as f64 * 10f64.powi(self.exponent as i32)
    }

    /// Create from f64 with specified exponent
    pub fn from_f64(value: f64, exponent: i8) -> Self {
        let mantissa = (value * 10f64.powi(-exponent as i32)).round() as i64;
        Self { mantissa, exponent }
    }
}

impl From<Decimal> for f64 {
    fn from(d: Decimal) -> f64 {
        d.to_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_roundtrip() {
        let d = Decimal::new(12345, -2);
        assert_eq!(d.mantissa(), 12345);
        assert_eq!(d.exponent(), -2);
        assert_eq!(d.to_f64(), 123.45);
    }

    #[test]
    fn test_decimal_from_f64() {
        let d = Decimal::from_f64(123.45, -2);
        assert_eq!(d.mantissa(), 12345);
        assert_eq!(d.exponent(), -2);
    }
}
