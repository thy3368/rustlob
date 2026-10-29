// ============ SymbolId ============

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::account::coin::Coin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolId(u32);

// ============ Symbol（KISS：只有三个字段） ============

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Symbol {
    pub id: SymbolId,
    /// 真正被买卖的币，也就是订单数量对应的币。
    ///
    /// 例如 `BTC/USDT` 中，买卖的是 BTC。
    pub base: Coin,
    /// 用来给 `base` 定价、付款和结算的币。
    ///
    /// 例如 `BTC/USDT` 中，价格用 USDT 表示。
    pub quote: Coin,
}

impl Symbol {
    pub fn new(id: u32, base: Coin, quote: Coin) -> Self {
        Symbol { id: SymbolId(id), base, quote }
    }

    // ---- 精度从币种派生 ----

    /// 价格精度跟 `quote` 走，因为价格是用报价币表示的。
    pub fn price_scale(&self) -> u32 {
        self.quote.scale()
    }

    /// 数量精度跟 `base` 走，因为数量表示要买卖多少基础币。
    pub fn qty_scale(&self) -> u32 {
        self.base.scale()
    }

    pub fn align_price(&self, price: Decimal) -> Decimal {
        let mut p = price;
        p.rescale(self.quote.scale());
        p
    }

    pub fn align_qty(&self, qty: Decimal) -> Decimal {
        let mut q = qty;
        q.rescale(self.base.scale());
        q
    }

    /// 成交金额 = 价格 × 数量，结果用 `quote` 表示并对齐到 `quote` 精度。
    pub fn notional(&self, price: Decimal, qty: Decimal) -> Decimal {
        let mut n = price * qty;
        n.rescale(self.quote.scale());
        n
    }

    pub fn code(&self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}
