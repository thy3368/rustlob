use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Usdc(pub(crate) i128);
// 内部 i128 定点，scale = 6（即 1_000_000 = 1.0 USDC）
// 永远非负语义，但用 i128 方便减法出负数做比较

impl Usdc {
    pub const fn new(raw: i128) -> Self {
        Self(raw)
    }
    pub const fn raw(self) -> i128 {
        self.0
    }
    pub(crate) const SCALE: i128 = 1_000_000;

    // ---- 从 Decimal 构造（业务层入口） ----

    pub(crate) fn from_decimal(d: Decimal) -> Self {
        // 向下截断，对用户保守
        let scaled = (d * Decimal::from(Self::SCALE)).trunc();
        Usdc(
            scaled
                .to_i128()
                .unwrap_or_else(|| if scaled.is_sign_negative() { i128::MIN } else { i128::MAX }),
        )
    }

    pub(crate) fn truncate(d: Decimal) -> Self {
        Self::from_decimal(d)
    }

    // ---- 从 notional 计算构造 ----

    fn from_notional(raw: i128) -> Self {
        // raw = szi.0 * mark.0 / Price::SCALE / Size::SCALE
        Usdc(raw)
    }

    // ---- 转回 Decimal（展示/派生用） ----

    pub(crate) fn to_decimal(self) -> Decimal {
        Decimal::from_i128_with_scale(self.0, 6)
    }

    // ---- 算术 ----

    fn saturating_sub(self, rhs: Usdc) -> Usdc {
        Usdc(self.0.saturating_sub(rhs.0))
    }

    fn saturating_add(self, rhs: Usdc) -> Usdc {
        Usdc(self.0.saturating_add(rhs.0))
    }

    fn abs(self) -> Usdc {
        Usdc(self.0.abs())
    }

    // ---- 比较辅助 ----

    pub(crate) fn is_negative(self) -> bool {
        self.0 < 0
    }

    pub(crate) fn is_zero(self) -> bool {
        self.0 == 0
    }
}
