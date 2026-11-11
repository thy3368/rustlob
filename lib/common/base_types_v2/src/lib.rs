use std::fmt;

use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Coin {
    BTC,
    ETH,
    USDT,
    USDC,
    USD,
    JPY,
    SOL,
}

impl Coin {
    pub fn index(self) -> u32 {
        match self {
            Self::BTC => 0,
            Self::ETH => 1,
            Self::USDT => 2,
            Self::USDC => 3,
            Self::USD => 4,
            Self::JPY => 5,
            Self::SOL => 6,
        }
    }

    pub fn scale(self) -> u32 {
        match self {
            Self::BTC => 8,
            Self::ETH => 18,
            Self::USDT | Self::USDC => 6,
            Self::USD => 2,
            Self::JPY => 0,
            Self::SOL => 9,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::BTC => "BTC",
            Self::ETH => "ETH",
            Self::USDT => "USDT",
            Self::USDC => "USDC",
            Self::USD => "USD",
            Self::JPY => "JPY",
            Self::SOL => "SOL",
        }
    }

    pub fn tick(self) -> Decimal {
        Decimal::new(1, self.scale())
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Symbol {
    pub id: SymbolId,
    pub base: Coin,
    pub quote: Coin,
}

impl Symbol {
    pub fn new(id: u32, base: Coin, quote: Coin) -> Self {
        Self { id: SymbolId(id), base, quote }
    }

    pub fn id(self) -> u32 {
        self.id.0
    }

    pub fn price_scale(self) -> u32 {
        self.quote.scale()
    }

    pub fn qty_scale(self) -> u32 {
        self.base.scale()
    }

    pub fn align_price(self, mut price: Decimal) -> Decimal {
        price.rescale(self.price_scale());
        price
    }

    pub fn align_qty(self, mut qty: Decimal) -> Decimal {
        qty.rescale(self.qty_scale());
        qty
    }

    pub fn notional(self, price: Decimal, qty: Decimal) -> Decimal {
        self.align_price(price * qty)
    }

    pub fn code(self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::{Coin, Symbol};

    #[test]
    fn coin_metadata_is_stable() {
        let coins = [Coin::BTC, Coin::ETH, Coin::USDT, Coin::USDC, Coin::USD, Coin::JPY, Coin::SOL];
        let expected_scales = [8, 18, 6, 6, 2, 0, 9];

        for (index, coin) in coins.into_iter().enumerate() {
            assert_eq!(coin.index(), index as u32);
            assert_eq!(coin.scale(), expected_scales[index]);
            assert_eq!(coin.tick(), Decimal::new(1, expected_scales[index]));
            assert_eq!(coin.to_string(), coin.code());
        }
    }

    #[test]
    fn symbol_derives_precision_and_code_from_pair() {
        let symbol = Symbol::new(7, Coin::BTC, Coin::USDT);

        assert_eq!(symbol.id(), 7);
        assert_eq!(symbol.price_scale(), 6);
        assert_eq!(symbol.qty_scale(), 8);
        assert_eq!(symbol.align_price(Decimal::new(123, 2)).scale(), 6);
        assert_eq!(symbol.align_qty(Decimal::new(123, 2)).scale(), 8);
        assert_eq!(
            symbol.notional(Decimal::new(25, 1), Decimal::new(2, 0)),
            Decimal::new(5_000_000, 6)
        );
        assert_eq!(symbol.code(), "BTC/USDT");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn symbol_round_trips_through_serde() {
        let symbol = Symbol::new(7, Coin::BTC, Coin::USDT);
        let bytes = bincode::serialize(&symbol).unwrap();
        let decoded: Symbol = bincode::deserialize(&bytes).unwrap();

        assert_eq!(decoded, symbol);
    }
}
