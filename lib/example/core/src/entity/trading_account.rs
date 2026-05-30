#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradingAccount {
    pub account_id: String,
    pub available_quote: u64,
    pub frozen_quote: u64,
    pub version: u64,
}

impl TradingAccount {
    pub fn can_reserve_quote(&self, reserved_quote: u64) -> bool {
        self.available_quote >= reserved_quote
    }

    pub fn reserve_quote_after(&self, reserved_quote: u64) -> Option<(u64, u64)> {
        let next_available = self.available_quote.checked_sub(reserved_quote)?;
        let next_frozen = self.frozen_quote.checked_add(reserved_quote)?;
        Some((next_available, next_frozen))
    }
}
