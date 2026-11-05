use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::perp::trade::perp_position_v3::{MarginModeV3, PerpPositionV3};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerpClearinghouseStateV3 {
    pub margin_summary: MarginSummary,
    pub cross_margin_summary: MarginSummary,
    pub cross_maintenance_margin_used: Decimal,
    pub withdrawable: Decimal,
    pub asset_positions: Vec<AssetPosition>,
    pub time: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MarginSummary {
    account_value: Decimal,     // 账户权益 = raw_usd + 全部 uPnL
    total_ntl_pos: Decimal,     // 所有仓位名义值总和
    total_raw_usd: Decimal,     // 原始 USDC（不含浮盈）
    total_margin_used: Decimal, // 所有仓位 IM 占用总和
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetPosition {
    pub position_type: PositionType,
    pub position: PerpPositionV3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PositionType {
    /// 单向持仓：一个币种一个净仓位，szi 正负表示多空
    OneWay,
    // 预留扩展：
    // Hedge,    // 同时持有多空（对冲模式）
    // Split,    // 多空分开记账
}

impl PerpClearinghouseStateV3 {
    pub fn new(cross_equity: Decimal, cross_mm: Decimal, positions: Vec<AssetPosition>, time: u64) -> Self {
        let summary = MarginSummary { account_value: cross_equity, total_ntl_pos: Decimal::ZERO, total_raw_usd: cross_equity, total_margin_used: Decimal::ZERO };
        Self { margin_summary: summary.clone(), cross_margin_summary: summary, cross_maintenance_margin_used: cross_mm, withdrawable: Decimal::ZERO, asset_positions: positions, time }
    }
    pub fn positions(&self) -> &[AssetPosition] { &self.asset_positions }
    pub fn apply_mark_price_updated(&mut self, logical_ts: u64, cross_equity: Decimal, cross_mm: Decimal) -> Result<(), &'static str> {
        if logical_ts < self.time || cross_mm.is_sign_negative() { return Err("invalid risk snapshot"); }
        self.cross_margin_summary.account_value = cross_equity;
        self.margin_summary.account_value = cross_equity;
        self.cross_maintenance_margin_used = cross_mm;
        self.time = logical_ts;
        Ok(())
    }
    /// 全账户权益（含浮盈）
    fn equity(&self) -> Decimal {
        self.margin_summary.account_value
    }

    /// cross 池权益
    pub fn cross_equity(&self) -> Decimal {
        self.cross_margin_summary.account_value
    }

    /// 已结算 USDC（不含浮盈）
    fn settled_usdc(&self) -> Decimal {
        self.margin_summary.total_raw_usd
    }

    /// 总名义仓位
    fn total_notional(&self) -> Decimal {
        self.margin_summary.total_ntl_pos
    }

    /// cross 强平红线
    pub fn cross_mm_required(&self) -> Decimal {
        self.cross_maintenance_margin_used
    }

    /// 当前 cross 保证金率（健康度指标）
    fn cross_margin_ratio(&self) -> Option<Decimal> {
        if self.cross_margin_summary.account_value.is_zero() {
            return None;
        }
        Some(self.cross_maintenance_margin_used / self.cross_margin_summary.account_value)
    }

    /// 快照时间
    pub fn snapshot_at(&self) -> u64 {
        self.time
    }
}

impl PerpClearinghouseStateV3 {
    /// cross 账户级是否可强平
    pub fn cross_liquidatable(&self) -> bool {
        self.cross_margin_summary.account_value < self.cross_maintenance_margin_used
    }

    /// 是否进入 backstop 区（equity < 2/3 MM）
    pub fn cross_backstop_zone(&self) -> bool {
        let threshold =
            self.cross_maintenance_margin_used * Decimal::new(2, 0) / Decimal::new(3, 0);
        self.cross_margin_summary.account_value < threshold
    }

    /// 挑最该先平的 cross 仓位（按 notional 大/亏损大）
    fn worst_cross_position(&self, marks: &HashMap<String, Decimal>) -> Option<&PerpPositionV3> {
        self.asset_positions
            .iter()
            .filter(|ap| ap.position.margin_mode() == MarginModeV3::Cross)
            .max_by(|a, b| {
                let na = a.position.notional(marks[a.position.coin().code()]);
                let nb = b.position.notional(marks[b.position.coin().code()]);
                na.cmp(&nb)
            })
            .map(|ap| &ap.position)
    }

    // /// 生成强平动作（不直接执行，返回事件）
    // fn liquidation_plan(&self, marks: &HashMap<String, Decimal>) -> LiquidationPlan {
    //     if !self.cross_liquidatable() {
    //         return LiquidationPlan::None;
    //     }
    //     if self.cross_backstop_zone() {
    //         return LiquidationPlan::BackstopAllCross;
    //     }
    //     match self.worst_cross_position(marks) {
    //         Some(p) => LiquidationPlan::MarketOrder { coin: p.coin.clone(), sz: p.szi },
    //         None => LiquidationPlan::None,
    //     }
    // }

    /// 可提现校验
    fn can_withdraw(&self, amount: Decimal) -> bool {
        amount <= self.withdrawable
    }
}
