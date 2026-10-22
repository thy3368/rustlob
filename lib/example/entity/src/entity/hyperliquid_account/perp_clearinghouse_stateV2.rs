use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

struct PerpClearinghouseState {
    margin_summary: MarginSummary,       // 全账户（cross + isolated 合计）
    cross_margin_summary: MarginSummary, // 仅 cross 部分
    cross_maintenance_margin_used: Decimal, // 所有 cross 仓位 MM 之和
    withdrawable: Decimal,               // 还能提多少 USDC
    asset_positions: Vec<AssetPosition>, // 逐仓位列表
    time: u64,                           // ms 时间戳
}

struct MarginSummary {
    account_value: Decimal,     // 账户权益 = raw_usd + 全部 uPnL
    total_ntl_pos: Decimal,     // 所有仓位名义值总和
    total_raw_usd: Decimal,     // 原始 USDC（不含浮盈）
    total_margin_used: Decimal, // 所有仓位 IM 占用总和
}

struct AssetPosition {
    position_type: PositionType, // OneWay（目前只有单向）
    position: Position,
}

struct Position {
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

struct Leverage {
    leverage_type: MarginMode, // "cross" | "isolated"
    value: u8,
    raw_usd: Option<Decimal>, // isolated 才有
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum PositionType {
    /// 单向持仓：一个币种一个净仓位，szi 正负表示多空
    OneWay,
    // 预留扩展：
    // Hedge,    // 同时持有多空（对冲模式）
    // Split,    // 多空分开记账
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum MarginMode {
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
