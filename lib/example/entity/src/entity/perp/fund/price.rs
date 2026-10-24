//! Hyperliquid 风格的 oracle price 与 mark price 纯计算规则。
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 价格单位为交易标的的报价货币，使用定点十进制定数保存。
///
/// 该别名同时用于现货、永续合约、oracle 和 mark price。
pub type Price = Decimal;
/// 单一现货交易场所的中间价及其聚合权重。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VenueMid {
    /// 交易场所身份，例如交易所或行情供应商名称。
    pub venue: String,
    /// 该场所的有效中间价，必须为正数。
    pub price: Price,
    /// 聚合该场所报价时使用的正权重。
    pub weight: Decimal,
}
/// 验证者提交的报价及其共识权重。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorQuote {
    /// 提交报价的验证者身份。
    pub validator: String,
    /// 验证者报价，必须为正数。
    pub price: Price,
    /// 验证者权益（stake），作为正权重参与中位数聚合。
    pub stake: Decimal,
}
/// 某一交易簿时刻的盘口价格快照。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookSnapshot {
    /// 最高买价；缺失表示调用方没有可用买盘。
    pub bid: Option<Price>,
    /// 最低卖价；缺失表示调用方没有可用卖盘。
    pub ask: Option<Price>,
    /// 最近成交价；缺失表示快照没有成交记录。
    pub last: Option<Price>,
    /// 调用方计算的盘口中间价，通常为 bid 与 ask 的中点。
    pub mid: Option<Price>,
}
/// 单一外部永续市场的中间价及其聚合权重。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalPerpMid {
    /// 外部永续市场的身份。
    pub venue: String,
    /// 外部永续中间价，必须为正数。
    pub price: Price,
    /// 聚合外部市场报价时使用的正权重。
    pub weight: Decimal,
}
/// 计算 oracle price 所需的现货与验证者报价输入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OraclePriceInput {
    /// 交易标的或币种标识，用于说明本次报价属于哪个市场。
    pub coin: String,
    /// 各现货场所的中间价候选。
    pub cex_spot_mids: Vec<VenueMid>,
    /// 验证者报价；非空时其 stake 加权中位数作为共识价格。
    pub validator_quotes: Vec<ValidatorQuote>,
}
/// 计算 mark price 所需的 oracle、盘口、外部永续和 EMA 状态。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkPriceInput {
    /// 已确定的 oracle price，作为 mark 计算的基准，必须为正数。
    pub oracle: Price,
    /// 当前盘口快照；行情由调用方采集后传入。
    pub book: BookSnapshot,
    /// 外部永续市场中间价候选。
    pub external_perp_mids: Vec<ExternalPerpMid>,
    /// 上一次 basis EMA 状态；缺失表示本次 basis EMA 首次取值。
    pub previous_basis_ema: Option<Decimal>,
    /// basis EMA 的 alpha，必须位于 (0, 1]。
    pub basis_alpha: Decimal,
    /// 上一次 30 秒盘口 EMA 状态；缺失表示首次取盘口平均值。
    pub previous_book_ema_30s: Option<Decimal>,
    /// 30 秒盘口 EMA 的 alpha，必须位于 (0, 1]。
    pub book_ema_alpha_30s: Decimal,
}
/// oracle 或 mark 价格纯计算可能返回的业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PriceCalculationError {
    /// 要求至少一个候选，但调用方传入了空集合。
    #[error("{0} input is empty")]
    EmptyInput(&'static str),
    /// 某个要求为正的价格输入不满足约束。
    #[error("{0} price must be greater than zero")]
    NonPositivePrice(&'static str),
    /// 某个加权候选的权重或权益不为正。
    #[error("{0} weight must be greater than zero")]
    InvalidWeight(&'static str),
    /// EMA alpha 不在开区间左端、闭区间右端的合法范围内。
    #[error("EMA alpha must be in (0, 1]")]
    InvalidAlpha,
    /// 定点十进制加减乘除发生溢出或无效除法。
    #[error("decimal arithmetic overflow")]
    ArithmeticOverflow,
    /// 可用于 mark 的候选价格不足，且没有盘口回退值。
    #[error("no usable mark price candidate")]
    NoMarkCandidate,
}
// checked arithmetic keeps decimal overflow as a domain error instead of panicking.
fn add(a: Decimal, b: Decimal) -> Result<Decimal, PriceCalculationError> {
    a.checked_add(b).ok_or(PriceCalculationError::ArithmeticOverflow)
}
fn sub(a: Decimal, b: Decimal) -> Result<Decimal, PriceCalculationError> {
    a.checked_sub(b).ok_or(PriceCalculationError::ArithmeticOverflow)
}
fn mul(a: Decimal, b: Decimal) -> Result<Decimal, PriceCalculationError> {
    a.checked_mul(b).ok_or(PriceCalculationError::ArithmeticOverflow)
}
fn div(a: Decimal, b: Decimal) -> Result<Decimal, PriceCalculationError> {
    a.checked_div(b).ok_or(PriceCalculationError::ArithmeticOverflow)
}
fn weighted_median(
    items: &[(Decimal, Decimal)],
    label: &'static str,
) -> Result<Decimal, PriceCalculationError> {
    if items.is_empty() {
        return Err(PriceCalculationError::EmptyInput(label));
    }
    let mut sorted = items.to_vec();
    for (price, weight) in &sorted {
        if *price <= Decimal::ZERO {
            return Err(PriceCalculationError::NonPositivePrice(label));
        }
        if *weight <= Decimal::ZERO {
            return Err(PriceCalculationError::InvalidWeight(label));
        }
    }
    // Sort by price, then return the first value whose cumulative weight reaches half.
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let total = sorted.iter().try_fold(Decimal::ZERO, |s, (_, w)| add(s, *w))?;
    let half = div(total, Decimal::from(2u32))?;
    let mut cumulative = Decimal::ZERO;
    for (price, weight) in sorted {
        cumulative = add(cumulative, weight)?;
        if cumulative >= half {
            return Ok(price);
        }
    }
    Err(PriceCalculationError::ArithmeticOverflow)
}
fn median(values: &[Decimal], label: &'static str) -> Result<Decimal, PriceCalculationError> {
    // Equal unit weights make even-sized inputs choose the upper middle candidate.
    weighted_median(&values.iter().map(|p| (*p, Decimal::ONE)).collect::<Vec<_>>(), label)
}

/// 计算外部现货与验证者共识形成的 `oraclePx`。
///
/// 这是确定性的纯计算；行情快照必须由调用方传入。本函数不访问网络，也不修改
/// funding、position 或 balance。`oraclePx` 用于现货/验证者共识与资金费结算。
pub fn oracle_px(input: &OraclePriceInput) -> Result<Price, PriceCalculationError> {
    let spot = input.cex_spot_mids.iter().map(|v| (v.price, v.weight)).collect::<Vec<_>>();
    let single = weighted_median(&spot, "spot mids")?;
    if input.validator_quotes.is_empty() {
        return Ok(single);
    }
    weighted_median(
        &input.validator_quotes.iter().map(|v| (v.price, v.stake)).collect::<Vec<_>>(),
        "validator quotes",
    )
}
fn ema(
    previous: Option<Decimal>,
    value: Decimal,
    alpha: Decimal,
) -> Result<Decimal, PriceCalculationError> {
    // The first EMA value is the current value; later values use alpha in (0, 1].
    if alpha <= Decimal::ZERO || alpha > Decimal::ONE {
        return Err(PriceCalculationError::InvalidAlpha);
    }
    match previous {
        None => Ok(value),
        Some(old) => add(mul(alpha, value)?, mul(sub(Decimal::ONE, alpha)?, old)?),
    }
}

/// 计算用于未实现盈亏、保证金风险及强平的 `markPx`。
///
/// 这是确定性的纯计算；所有行情和历史 EMA 状态都由调用方通过输入快照提供。
/// 本函数不访问网络，也不修改 funding、position 或 balance。`markPx` 与 `midPx`
/// 属于风险定价边界：它用于未实现盈亏、保证金风险和强平，而不是替代盘口中间价。
pub fn mark_px(input: &MarkPriceInput) -> Result<Price, PriceCalculationError> {
    if input.oracle <= Decimal::ZERO {
        return Err(PriceCalculationError::NonPositivePrice("oracle"));
    }
    let mut candidates = Vec::with_capacity(4);
    if let Some(mid) = input.book.mid {
        if mid <= Decimal::ZERO {
            return Err(PriceCalculationError::NonPositivePrice("book mid"));
        }
        let basis = sub(mid, input.oracle)?;
        candidates
            .push(add(input.oracle, ema(input.previous_basis_ema, basis, input.basis_alpha)?)?);
    }
    let book_values =
        [input.book.bid, input.book.ask, input.book.last].into_iter().flatten().collect::<Vec<_>>();
    if book_values.len() == 3 {
        candidates.push(median(&book_values, "book prices")?);
    }
    if !input.external_perp_mids.is_empty() {
        candidates.push(weighted_median(
            &input.external_perp_mids.iter().map(|v| (v.price, v.weight)).collect::<Vec<_>>(),
            "external perp mids",
        )?);
    }
    if candidates.len() < 3 {
        if book_values.is_empty() {
            return Err(PriceCalculationError::NoMarkCandidate);
        }
        let average = div(
            book_values.iter().try_fold(Decimal::ZERO, |s, p| add(s, *p))?,
            Decimal::from(book_values.len() as u32),
        )?;
        candidates.push(ema(input.previous_book_ema_30s, average, input.book_ema_alpha_30s)?);
    }
    for candidate in &candidates {
        if *candidate <= Decimal::ZERO {
            return Err(PriceCalculationError::NonPositivePrice("mark candidate"));
        }
    }
    median(&candidates, "mark candidates")
}

#[cfg(test)]
mod tests {
    use super::*;
    fn d(v: i64) -> Decimal {
        Decimal::from(v)
    }
    fn venue(p: i64, w: i64) -> VenueMid {
        VenueMid { venue: String::new(), price: d(p), weight: d(w) }
    }
    fn mark() -> MarkPriceInput {
        MarkPriceInput {
            oracle: d(100),
            book: BookSnapshot {
                bid: Some(d(99)),
                ask: Some(d(101)),
                last: Some(d(100)),
                mid: Some(d(100)),
            },
            external_perp_mids: vec![ExternalPerpMid {
                venue: String::new(),
                price: d(102),
                weight: d(1),
            }],
            previous_basis_ema: Some(d(0)),
            basis_alpha: d(1),
            previous_book_ema_30s: None,
            book_ema_alpha_30s: d(1),
        }
    }
    #[test]
    fn spot_weighted_median() {
        let i = OraclePriceInput {
            coin: "BTC".into(),
            cex_spot_mids: vec![venue(100, 1), venue(110, 3)],
            validator_quotes: vec![],
        };
        assert_eq!(oracle_px(&i).unwrap(), d(110));
    }
    #[test]
    fn validator_median_wins() {
        let i = OraclePriceInput {
            coin: "BTC".into(),
            cex_spot_mids: vec![venue(100, 1)],
            validator_quotes: vec![
                ValidatorQuote { validator: String::new(), price: d(90), stake: d(1) },
                ValidatorQuote { validator: String::new(), price: d(110), stake: d(3) },
            ],
        };
        assert_eq!(oracle_px(&i).unwrap(), d(110));
    }
    #[test]
    fn mark_three_candidates() {
        assert_eq!(mark_px(&mark()).unwrap(), d(100));
    }
    #[test]
    fn mark_fallback() {
        let mut i = mark();
        i.external_perp_mids.clear();
        i.book.last = None;
        assert_eq!(mark_px(&i).unwrap(), d(100));
    }
    #[test]
    fn invalid_rejected() {
        let i = OraclePriceInput {
            coin: "BTC".into(),
            cex_spot_mids: vec![venue(0, 1)],
            validator_quotes: vec![],
        };
        assert!(oracle_px(&i).is_err());
    }
    #[test]
    fn ema_explicit() {
        assert_eq!(ema(None, d(10), d(1)).unwrap(), d(10));
        assert_eq!(ema(Some(d(10)), d(20), d(1) / d(2)).unwrap(), d(15));
        assert!(ema(None, d(1), d(0)).is_err());
    }
}
