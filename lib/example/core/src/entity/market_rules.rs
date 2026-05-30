#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketRules {
    pub symbol: String,
    pub min_qty: u64,
}

impl MarketRules {
    pub fn supports_symbol(&self, symbol: &str) -> bool {
        self.symbol == symbol
    }

    pub fn validate_qty(&self, qty: u64) -> bool {
        qty >= self.min_qty
    }

    pub fn required_quote(&self, qty: u64, price: u64) -> Option<u64> {
        qty.checked_mul(price)
    }
}
