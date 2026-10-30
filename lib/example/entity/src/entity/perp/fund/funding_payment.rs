use super::funding_payment_factory::Size;
use crate::account::coin::Coin;
use crate::entity::common::UserId;
use crate::entity::hyperliquid_account::usdc::Usdc;

/// 用户级资金费结算事件（L0 fact）
/// 对应 userFunding 返回的一行
#[derive(Clone, Debug)]
pub struct FundingPayment {
    pub user: UserId,
    pub time_ms: u64, // 结算时间
    pub seq: u64,     // 全局有序序列（重放用）
    pub hash: TxHash, // 链上 tx hash

    // —— delta 主体 ——
    pub coin: Coin,         // 结算品种
    pub funding_rate: Rate, // 本次采用费率（小时）
    pub szi: Size,          // 结算时带符号仓位（正多负空）
    pub usdc: Usdc,         // 正负：正=收钱，负=付钱（已结算到余额）

    pub n_samples: Option<u32>, // 采样数，通常为 null
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateParseError {
    Empty,
    InvalidInt,
    PrecisionOverflow,
    InvalidFrac,
}

type TxHash = [u8; 32];

/// 费率/比例，定点 scale 10
/// 0.0000125 → 12500
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rate(i128);

impl Rate {
    pub const SCALE: i128 = 10_i128.pow(10); // 10_000_000_000

    // ── 构造 ──

    pub fn new(raw: i128) -> Self {
        Self(raw)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub const fn raw(self) -> i128 {
        self.0
    }

    /// 从字符串解析："0.0000125" → Rate(12500)
    pub fn parse_str(s: &str) -> Result<Self, RateParseError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(RateParseError::Empty);
        }

        let (int_part, frac_part) = if let Some(dot_pos) = s.find('.') {
            (&s[..dot_pos], &s[dot_pos + 1..])
        } else {
            (s, "")
        };

        let int_val: i128 = int_part.parse().map_err(|_| RateParseError::InvalidInt)?;
        let frac_val: i128 = if frac_part.is_empty() {
            0
        } else {
            let frac = frac_part.trim_end_matches('0');
            if frac.len() > 10 {
                return Err(RateParseError::PrecisionOverflow);
            }
            let padded = format!("{frac:<0width$}", width = 10);
            padded[..10].parse().map_err(|_| RateParseError::InvalidFrac)?
        };

        let raw = int_val * Self::SCALE + frac_val;
        Ok(Self(raw))
    }

    /// 从 basis points 构造：5 bps → Rate(500_000)
    pub fn from_bps(bps: i128) -> Self {
        Self(bps * 1_000_000) // 1 bp = 0.0001 = 1_000_000 in scale 10
    }

    /// 从百分比构造：5% → Rate(500_000_000)
    pub fn from_percent(pct: i128) -> Self {
        Self(pct * 100_000_000)
    }
}
