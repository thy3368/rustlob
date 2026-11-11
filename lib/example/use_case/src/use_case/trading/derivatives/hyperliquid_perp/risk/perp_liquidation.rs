use std::collections::BTreeMap;

use common_entity::{EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, UseCase};
use example_core_entity::clearinghouse::perp_clearinghouse_state_v3::PerpClearinghouseStateV3;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    PlaceHyperliquidPerpOrderChanges, PlaceHyperliquidPerpOrderCmd, PlaceHyperliquidPerpOrderError,
    PlaceHyperliquidPerpOrderExecution, PlaceHyperliquidPerpOrderUseCase,
};

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
    Orders { liquidation_id: String, orders: Vec<PlaceHyperliquidPerpOrderChanges> },
}

/// 现货改单所需的 authoritative 状态快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiquidationState {
    pub clearinghouse_state: PerpClearinghouseStateV3,
    /// 每个活动仓位对应的普通 perp 下单 authoritative state。
    pub order_states: BTreeMap<String, crate::PlaceHyperliquidPerpOrderState>,
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
            || cmd.mark_prices.values().any(|p| !p.is_sign_positive())
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
            let mut orders = Vec::new();
            for (symbol, state) in &given_state.order_states {
                let position = &state.position;
                let mark = cmd.mark_prices.get(symbol).ok_or(LiquidationError::MissingMarkPrice)?;
                let aggressive_price = *mark;
                let size = position.signed_size().abs();
                if size.is_zero() {
                    continue;
                }
                let child = PlaceHyperliquidPerpOrderCmd {
                    user_id: position.user_id(),
                    symbol: state.market_symbol,
                    side: if position.is_short() {
                        crate::entity::OrderSide::Buy
                    } else {
                        crate::entity::OrderSide::Sell
                    },
                    size,
                    reduce_only: true,
                    execution: PlaceHyperliquidPerpOrderExecution::Market { aggressive_price },
                    cloid: None,
                    liquidation_id: Some(cmd.liquidation_id.clone()),
                };
                let child_log = common_entity::LogEntrySequencer::derive(cmd, child);
                let changes = PlaceHyperliquidPerpOrderUseCase
                    .compute_state_changed(&child_log, state)
                    .map_err(|source| LiquidationError::Order { symbol: symbol.clone(), source })?;
                orders.push(changes);
            }
            return Ok(PlaceOnlySpotOrderV3AfterChanges::Orders {
                liquidation_id: cmd.liquidation_id.clone(),
                orders,
            });
        }
        Ok(PlaceOnlySpotOrderV3AfterChanges::NoAction)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LiquidationError {
    #[error("invalid liquidation command")]
    InvalidCommand,
    #[error("liquidation snapshot is stale")]
    StaleSnapshot,
    #[error("mark price is missing")]
    MissingMarkPrice,
    #[error("mark price cannot be represented as an order price")]
    InvalidMarkPrice,
    #[error("liquidation order for {symbol} failed: {source}")]
    Order { symbol: String, source: PlaceHyperliquidPerpOrderError },
}

impl ReplayableChanges for PlaceOnlySpotOrderV3AfterChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        match self {
            Self::Orders { orders, .. } => orders.iter().try_fold(Vec::new(), |mut all, order| {
                all.extend(order.to_replayable_events()?);
                Ok(all)
            }),
            _ => Ok(Vec::new()),
        }
    }
}
