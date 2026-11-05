use common_entity::{LogEntry, UseCase};
use example_core_entity::hyperliquid_account::perp_clearinghouse_state_v3::PerpClearinghouseStateV3;
use serde::{Deserialize, Serialize};

/// 创建委托所需的完整事实；价格和数量按 Decimal 精确解析，不舍入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiquidationCmd {
    pub liquidation_id: String,
    pub mark_prices: std::collections::BTreeMap<String, rust_decimal::Decimal>,
    pub maintenance_margin_rate: rust_decimal::Decimal,
}

/// 有多种结果，1，不需要强平， 2，强平
/// 强平又分 1，下强平单，其它动作

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOnlySpotOrderV3AfterChanges {
    NoAction,
    Backstop { liquidation_id: String },
    Adl { liquidation_id: String },
    Orders { liquidation_id: String, position_count: usize },
}

/// 现货改单所需的 authoritative 状态快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiquidationState {
    pub clearinghouse_state: PerpClearinghouseStateV3,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PerpLiquidation;

impl UseCase for PerpLiquidation {
    type Command = LogEntry<LiquidationCmd>;
    type StateGiven = LiquidationState;
    type Error = LiquidationError;
    type StateChanged = PlaceOnlySpotOrderV3AfterChanges;

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let clearinghouse_state = &given_state.clearinghouse_state;
        if cmd.liquidation_id.is_empty()
            || cmd.maintenance_margin_rate.is_sign_negative()
            || cmd.mark_prices.values().any(|p| !p.is_positive())
        {
            return Err(LiquidationError::InvalidCommand);
        }
        if cmd.logical_ts() < clearinghouse_state.snapshot_at() {
            return Err(LiquidationError::StaleSnapshot);
        }
        let equity = clearinghouse_state.cross_equity();
        let mm = clearinghouse_state.cross_mm_required();
        if equity <= rust_decimal::Decimal::ZERO {
            return Ok(PlaceOnlySpotOrderV3AfterChanges::Adl {
                liquidation_id: cmd.liquidation_id.clone(),
            });
        }
        if equity * rust_decimal::Decimal::from(3) < mm * rust_decimal::Decimal::from(2) {
            return Ok(PlaceOnlySpotOrderV3AfterChanges::Backstop {
                liquidation_id: cmd.liquidation_id.clone(),
            });
        }
        if equity <= mm {
            return Ok(PlaceOnlySpotOrderV3AfterChanges::Orders {
                liquidation_id: cmd.liquidation_id.clone(),
                position_count: clearinghouse_state.positions().len(),
            });
        }
        Ok(PlaceOnlySpotOrderV3AfterChanges::NoAction)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidationError {
    InvalidCommand,
    StaleSnapshot,
}
