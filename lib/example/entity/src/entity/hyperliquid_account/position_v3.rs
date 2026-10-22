use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::hyperliquid_account::usdc::Usdc;

struct Leverage {
    leverage_type: MarginMode, // "cross" | "isolated"
    value: u8,
    raw_usd: Option<Decimal>, // isolated 才有
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarginMode {
    /// 全仓：所有 cross 仓位共用一个保证金池
    Cross,
    /// 逐仓：只用一个独立桶，亏光不影响别的
    Isolated,
    // 扩展位（HIP-3 / strict isolated / noCross 市场）：
    // StrictIsolated,  // 不能抽保证金，只能加
    // NoCross,         // HIP-3 市场禁止 cross
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FundingAccum {
    /// 该币种全历史累计资金费（跨多次开平仓）
    all_time: Decimal,
    /// 当前未平仓位自开仓以来累计
    since_open: Decimal,
    /// 自上次改变仓位结构（改杠杆/加减仓/调整保证金）以来累计
    since_change: Decimal,
}

pub struct PositionV3 {
    coin: String,       // "BTC"
    szi: Decimal,       // 带符号仓位，正多负空
    entry_px: Decimal,  // 加权开仓价
    leverage: Leverage, // { type: Cross|Isolated, value, raw_usd? }

    // ↓ 全部派生
    position_value: Decimal, // szi * mark
    unrealized_pnl: Decimal, // (mark - entry) * szi
    return_on_equity: Decimal,
    liquidation_px: Option<Decimal>, // 展示用，实时算
    margin_used: Decimal,            // IM = |ntl| / leverage
    max_leverage: u8,

    cum_funding: FundingAccum, // { all_time, since_open, since_change }
}

impl PositionV3 {
    fn notional(&self, mark: Decimal) -> Decimal {
        self.szi.abs() * mark
    }
    fn mm_required(&self, mark: Decimal) -> Decimal {
        tiered_mm(self.notional(mark))
    }

    /// isolated: 隔离桶权益；cross: 全账户 equity（需外层传入）
    fn backing_equity(&self, cross_equity: Decimal) -> Decimal {
        match self.leverage.leverage_type {
            MarginMode::Isolated => self.leverage.raw_usd.unwrap_or_default() + self.unrealized_pnl,
            MarginMode::Cross => cross_equity + self.unrealized_pnl,
        }
    }

    fn health(&self, cross_equity: Decimal, mark: Decimal) -> HealthStatus {
        let eq = self.backing_equity(cross_equity);
        let mm = self.mm_required(mark);
        if eq < mm {
            HealthStatus::Liquidate { deficit: Usdc::from_decimal(mm - eq) }
        } else if eq < mm * Decimal::new(2, 0) {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        }
    }
}

fn tiered_mm(notional: Decimal) -> Decimal {
    // V2 snapshot currently has no risk-tier table; use the base 0.5% rate.
    notional * Decimal::new(5, 3)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum HealthStatus {
    /// equity >= mm → 安全
    Healthy,

    /// equity 在 mm ~ 2×mm 之间 → 危险但还没到强平线
    /// 前端弹警告、限制开仓、禁止提现
    Warning,

    /// equity < mm → 触发强平
    Liquidate {
        deficit: Usdc, // 差多少到 mm
    },

    /// equity < 2/3 mm → 进入 backstop 区
    /// 订单簿平不掉就转 HLP
    BackstopZone,

    /// equity <= 0 → 资不抵债，需要 ADL
    Insolvent,
}

impl HealthStatus {
    /// 从 equity + mm 判定健康度
    fn from_equity_mm(equity: Usdc, mm: Usdc) -> Self {
        if equity.0 <= 0 {
            return HealthStatus::Insolvent;
        }

        let ratio =
            Decimal::from_i128_with_scale(equity.0, 6) / Decimal::from_i128_with_scale(mm.0, 6);

        if ratio < Decimal::new(667, 3) {
            HealthStatus::BackstopZone
        } else if equity < mm {
            HealthStatus::Liquidate { deficit: Usdc(mm.0 - equity.0) }
        } else if ratio < Decimal::new(2, 0) {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        }
    }
}
