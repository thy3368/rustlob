pub mod core;

pub use core::Decimal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_creation() {
        let d = Decimal::from_raw(100_000_000);
        assert_eq!(d.to_f64(), 1.0);
    }

    #[test]
    fn test_decimal_operations() {
        let a = Decimal::from_f64(10.0);
        let b = Decimal::from_f64(2.0);
        assert_eq!((a + b).to_f64(), 12.0);
        assert_eq!((a - b).to_f64(), 8.0);
    }
}
