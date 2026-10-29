use std::fmt;

use rust_decimal::Decimal;
use thiserror::Error;

// ============ Currency ============

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Coin {
    BTC,  // 8
    ETH,  // 18
    USDT, // 6
    USDC, // 6
    USD,  // 2
    JPY,  // 0
    SOL,  // 9
}

impl Coin {
    /// 返回币种在当前币种表中的稳定索引。
    pub fn index(&self) -> u32 {
        match self {
            Coin::BTC => 0,
            Coin::ETH => 1,
            Coin::USDT => 2,
            Coin::USDC => 3,
            Coin::USD => 4,
            Coin::JPY => 5,
            Coin::SOL => 6,
        }
    }

    pub fn scale(&self) -> u32 {
        match self {
            Coin::BTC => 8,
            Coin::ETH => 18,
            Coin::USDT => 6,
            Coin::USDC => 6,
            Coin::USD => 2,
            Coin::JPY => 0,
            Coin::SOL => 9,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Coin::BTC => "BTC",
            Coin::ETH => "ETH",
            Coin::USDT => "USDT",
            Coin::USDC => "USDC",
            Coin::USD => "USD",
            Coin::JPY => "JPY",
            Coin::SOL => "SOL",
        }
    }

    /// 最小单位（1 个 tick）
    pub fn tick(&self) -> Decimal {
        Decimal::new(1, self.scale())
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// 币种金额的构造错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum CoinAmountError {
    /// 金额的小数位数超过币种允许的精度。
    #[error("{currency} amount supports {currency_scale} decimal places, got {amount_scale}")]
    ExcessPrecision { currency: Coin, currency_scale: u32, amount_scale: u32 },
}

/// 带币种语义并符合该币种精度的金额值。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoinAmount {
    currency: Coin,
    amount: Decimal,
}

impl CoinAmount {
    /// 校验金额精度并构造币种金额。
    ///
    /// 多余的尾随零会先被规范化；有效的非零超精度部分不会被舍入或截断。
    pub fn new(currency: Coin, amount: Decimal) -> Result<Self, CoinAmountError> {
        let amount = amount.normalize();
        let currency_scale = currency.scale();
        let amount_scale = amount.scale();

        if amount_scale > currency_scale {
            return Err(CoinAmountError::ExcessPrecision {
                currency,
                currency_scale,
                amount_scale,
            });
        }

        Ok(Self { currency, amount })
    }

    /// 返回金额所属的币种。
    pub fn currency(&self) -> Coin {
        self.currency
    }

    /// 返回规范化后的金额。
    pub fn amount(&self) -> Decimal {
        self.amount
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::{Coin, CoinAmount, CoinAmountError};

    #[test]
    fn coin_indices_are_stable() {
        assert_eq!(Coin::BTC.index(), 0);
        assert_eq!(Coin::ETH.index(), 1);
        assert_eq!(Coin::USDT.index(), 2);
        assert_eq!(Coin::USDC.index(), 3);
        assert_eq!(Coin::USD.index(), 4);
        assert_eq!(Coin::JPY.index(), 5);
        assert_eq!(Coin::SOL.index(), 6);
    }

    #[test]
    fn constructs_amounts_with_currency_precision() {
        let btc = CoinAmount::new(Coin::BTC, Decimal::new(1, 1)).unwrap();
        assert_eq!(btc.currency(), Coin::BTC);
        assert_eq!(btc.amount(), Decimal::new(1, 1));

        let usdt = CoinAmount::new(Coin::USDT, Decimal::new(33, 2)).unwrap();
        assert_eq!(usdt.currency(), Coin::USDT);
        assert_eq!(usdt.amount(), Decimal::new(33, 2));
    }

    #[test]
    fn rejects_nonzero_fractional_digits_beyond_currency_precision() {
        assert_eq!(
            CoinAmount::new(Coin::BTC, Decimal::new(1, 9)),
            Err(CoinAmountError::ExcessPrecision {
                currency: Coin::BTC,
                currency_scale: 8,
                amount_scale: 9,
            })
        );
        assert_eq!(
            CoinAmount::new(Coin::USDT, Decimal::new(1, 7)),
            Err(CoinAmountError::ExcessPrecision {
                currency: Coin::USDT,
                currency_scale: 6,
                amount_scale: 7,
            })
        );
    }

    #[test]
    fn normalizes_trailing_zeroes_before_checking_precision() {
        let usdt = CoinAmount::new(Coin::USDT, Decimal::new(3_300_000, 7)).unwrap();

        assert_eq!(usdt.amount(), Decimal::new(33, 2));
        assert_eq!(usdt.amount().scale(), 2);
    }
}
