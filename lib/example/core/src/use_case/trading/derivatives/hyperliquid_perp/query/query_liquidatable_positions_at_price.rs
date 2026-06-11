use cmd_handler::command_use_case_def2::IssuedByParty;
use cmd_handler::query_use_case_def::QueryUseCase;
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode, HyperliquidPerpPosition,
};

/// 查询指定价格下哪些 Hyperliquid perp 仓位会触发爆仓。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpLiquidatablePositionsAtPrice {
    /// 发起风险扫描的业务主体。
    pub party_id: String,
    /// 用于判定爆仓的指定标记价格。
    pub mark_price: u64,
}

impl IssuedByParty for QueryHyperliquidPerpLiquidatablePositionsAtPrice {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 单个仓位在指定价格下的风险判定快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
    /// 被扫描的仓位。
    pub position: HyperliquidPerpPosition,
    /// 当前仓位的保证金模式。
    pub margin_mode: HyperliquidPerpMarginMode,
    /// 当前仓位的破产价格。
    pub bankruptcy_price: u64,
    /// 当前仓位是否已经进入强平流程。
    pub has_active_liquidation: bool,
}

/// 查询指定价格下可爆仓位时需要的已加载读模型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel {
    /// 当前扫描批次涉及的仓位风险快照。
    pub snapshots: Vec<HyperliquidPerpLiquidatablePositionAtPriceSnapshot>,
}

/// 单个会在指定价格下爆仓的仓位业务视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpLiquidatablePositionAtPriceView {
    pub position_id: String,
    pub account_id: String,
    pub asset: u32,
    pub symbol: String,
    pub side: crate::entity::HyperliquidPerpPositionSide,
    pub qty: u64,
    pub entry_price: u64,
    pub margin: u64,
    pub margin_mode: HyperliquidPerpMarginMode,
    pub mark_price: u64,
    pub bankruptcy_price: u64,
    pub trigger_reason: HyperliquidPerpLiquidationTriggerReason,
    pub liquidation_notional: u64,
}

/// 指定价格下爆仓扫描的汇总业务视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpLiquidatablePositionsAtPriceView {
    pub mark_price: u64,
    pub positions: Vec<HyperliquidPerpLiquidatablePositionAtPriceView>,
    pub total_liquidation_notional: u64,
    pub triggered_position_count: usize,
}

/// 查询指定价格下哪些仓位会爆时的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum QueryHyperliquidPerpLiquidatablePositionsAtPriceError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("mark_price must be greater than zero")]
    InvalidMarkPrice,
    #[error("risk snapshot contains invalid bankruptcy price")]
    InvalidRiskPrice,
    #[error("risk snapshot position state is inconsistent")]
    InconsistentPositionState,
    #[error("arithmetic overflow while deriving liquidation notional")]
    ArithmeticOverflow,
}

/// 查询在指定标记价格下会触发爆仓的 Hyperliquid perp 仓位及总名义金额。
///
/// 该用例只做读模型判定，不写入事件；命中的仓位由上层 adapter 再决定是否推进到
/// `StartHyperliquidPerpLiquidationUseCase`。
#[derive(Debug, Clone, Copy, Default)]
pub struct QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase;

impl QueryUseCase for QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase {
    type Query = QueryHyperliquidPerpLiquidatablePositionsAtPrice;
    type ReadModel = QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel;
    type View = QueryHyperliquidPerpLiquidatablePositionsAtPriceView;
    type Error = QueryHyperliquidPerpLiquidatablePositionsAtPriceError;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_query(&self, query: &Self::Query) -> Result<(), Self::Error> {
        if query.party_id.is_empty() {
            return Err(QueryHyperliquidPerpLiquidatablePositionsAtPriceError::InvalidPartyId);
        }
        if query.mark_price == 0 {
            return Err(QueryHyperliquidPerpLiquidatablePositionsAtPriceError::InvalidMarkPrice);
        }
        Ok(())
    }

    fn validate_against_read_model(
        &self,
        _query: &Self::Query,
        read_model: &Self::ReadModel,
    ) -> Result<(), Self::Error> {
        for snapshot in &read_model.snapshots {
            if !snapshot.position.has_consistent_state() {
                return Err(
                    QueryHyperliquidPerpLiquidatablePositionsAtPriceError::InconsistentPositionState,
                );
            }
            if snapshot.bankruptcy_price == 0 {
                return Err(
                    QueryHyperliquidPerpLiquidatablePositionsAtPriceError::InvalidRiskPrice,
                );
            }
        }

        Ok(())
    }

    fn compute_view(
        &self,
        query: &Self::Query,
        read_model: Self::ReadModel,
    ) -> Result<Self::View, Self::Error> {
        let mut positions = Vec::with_capacity(read_model.snapshots.len());
        let mut total_liquidation_notional = 0_u64;

        for snapshot in read_model.snapshots {
            if snapshot.has_active_liquidation || !snapshot.position.is_liquidatable() {
                continue;
            }
            if !snapshot
                .position
                .liquidation_triggered_by_mark_price(query.mark_price, snapshot.bankruptcy_price)
            {
                continue;
            }

            let liquidation_notional =
                snapshot.position.qty.checked_mul(query.mark_price).ok_or(
                    QueryHyperliquidPerpLiquidatablePositionsAtPriceError::ArithmeticOverflow,
                )?;
            total_liquidation_notional = total_liquidation_notional
                .checked_add(liquidation_notional)
                .ok_or(QueryHyperliquidPerpLiquidatablePositionsAtPriceError::ArithmeticOverflow)?;

            let trigger_reason = trigger_reason(&snapshot.position);

            positions.push(HyperliquidPerpLiquidatablePositionAtPriceView {
                position_id: snapshot.position.position_id,
                account_id: snapshot.position.account_id,
                asset: snapshot.position.asset,
                symbol: snapshot.position.symbol,
                side: snapshot.position.side,
                qty: snapshot.position.qty,
                entry_price: snapshot.position.entry_price,
                margin: snapshot.position.margin,
                margin_mode: snapshot.margin_mode,
                mark_price: query.mark_price,
                bankruptcy_price: snapshot.bankruptcy_price,
                trigger_reason,
                liquidation_notional,
            });
        }

        Ok(QueryHyperliquidPerpLiquidatablePositionsAtPriceView {
            mark_price: query.mark_price,
            triggered_position_count: positions.len(),
            positions,
            total_liquidation_notional,
        })
    }
}

fn trigger_reason(position: &HyperliquidPerpPosition) -> HyperliquidPerpLiquidationTriggerReason {
    if position.margin == 0 {
        HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk
    } else {
        HyperliquidPerpLiquidationTriggerReason::MaintenanceMarginBreach
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::HyperliquidPerpPositionSide;

    fn position(
        position_id: &str,
        account_id: &str,
        side: HyperliquidPerpPositionSide,
        qty: u64,
        margin: u64,
    ) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::new(
            position_id.to_string(),
            account_id.to_string(),
            7,
            "BTC-PERP".to_string(),
            side,
            qty,
            60_000,
            5,
            margin,
            0,
            3,
        )
    }

    fn query(mark_price: u64) -> QueryHyperliquidPerpLiquidatablePositionsAtPrice {
        QueryHyperliquidPerpLiquidatablePositionsAtPrice {
            party_id: "risk-engine".to_string(),
            mark_price,
        }
    }

    #[test]
    fn role_returns_risk_engine() {
        assert_eq!(QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase.role(), "RiskEngine");
    }

    #[test]
    fn pre_check_rejects_blank_party_id() {
        let result = QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase.pre_check_query(
            &QueryHyperliquidPerpLiquidatablePositionsAtPrice {
                party_id: String::new(),
                mark_price: 49_000,
            },
        );

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpLiquidatablePositionsAtPriceError::InvalidPartyId)
        );
    }

    #[test]
    fn pre_check_rejects_zero_mark_price() {
        let result =
            QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase.pre_check_query(&query(0));

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpLiquidatablePositionsAtPriceError::InvalidMarkPrice)
        );
    }

    #[test]
    fn validate_rejects_inconsistent_position_state() {
        let read_model = QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel {
            snapshots: vec![HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                position: HyperliquidPerpPosition::new(
                    "position-1".to_string(),
                    "trader-1".to_string(),
                    7,
                    "BTC-PERP".to_string(),
                    HyperliquidPerpPositionSide::Long,
                    0,
                    60_000,
                    5,
                    24_000,
                    0,
                    3,
                ),
                margin_mode: HyperliquidPerpMarginMode::Cross,
                bankruptcy_price: 50_000,
                has_active_liquidation: false,
            }],
        };

        let result = QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase
            .validate_against_read_model(&query(49_000), &read_model);

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpLiquidatablePositionsAtPriceError::InconsistentPositionState)
        );
    }

    #[test]
    fn validate_rejects_zero_bankruptcy_price() {
        let read_model = QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel {
            snapshots: vec![HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                position: position(
                    "position-1",
                    "trader-1",
                    HyperliquidPerpPositionSide::Long,
                    2,
                    24_000,
                ),
                margin_mode: HyperliquidPerpMarginMode::Cross,
                bankruptcy_price: 0,
                has_active_liquidation: false,
            }],
        };

        let result = QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase
            .validate_against_read_model(&query(49_000), &read_model);

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpLiquidatablePositionsAtPriceError::InvalidRiskPrice)
        );
    }

    #[test]
    fn compute_filters_only_triggered_positions() {
        let read_model = QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel {
            snapshots: vec![
                HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                    position: position(
                        "position-1",
                        "trader-1",
                        HyperliquidPerpPositionSide::Long,
                        2,
                        24_000,
                    ),
                    margin_mode: HyperliquidPerpMarginMode::Cross,
                    bankruptcy_price: 50_000,
                    has_active_liquidation: false,
                },
                HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                    position: position(
                        "position-2",
                        "trader-2",
                        HyperliquidPerpPositionSide::Long,
                        3,
                        36_000,
                    ),
                    margin_mode: HyperliquidPerpMarginMode::Cross,
                    bankruptcy_price: 48_000,
                    has_active_liquidation: false,
                },
                HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                    position: position(
                        "position-3",
                        "trader-3",
                        HyperliquidPerpPositionSide::Short,
                        4,
                        0,
                    ),
                    margin_mode: HyperliquidPerpMarginMode::Isolated,
                    bankruptcy_price: 49_000,
                    has_active_liquidation: false,
                },
            ],
        };

        let view = QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase
            .compute_view(&query(49_000), read_model)
            .unwrap();

        assert_eq!(view.triggered_position_count, 2);
        assert_eq!(view.positions.len(), 2);
        assert_eq!(view.positions[0].position_id, "position-1");
        assert_eq!(
            view.positions[0].trigger_reason,
            HyperliquidPerpLiquidationTriggerReason::MaintenanceMarginBreach
        );
        assert_eq!(view.positions[1].position_id, "position-3");
        assert_eq!(
            view.positions[1].trigger_reason,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk
        );
    }

    #[test]
    fn compute_skips_positions_already_in_liquidation() {
        let read_model = QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel {
            snapshots: vec![HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                position: position(
                    "position-1",
                    "trader-1",
                    HyperliquidPerpPositionSide::Long,
                    2,
                    24_000,
                ),
                margin_mode: HyperliquidPerpMarginMode::Cross,
                bankruptcy_price: 50_000,
                has_active_liquidation: true,
            }],
        };

        let view = QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase
            .compute_view(&query(49_000), read_model)
            .unwrap();

        assert_eq!(view.triggered_position_count, 0);
        assert_eq!(view.total_liquidation_notional, 0);
    }

    #[test]
    fn compute_skips_flat_or_non_liquidatable_positions() {
        let read_model = QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel {
            snapshots: vec![
                HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                    position: HyperliquidPerpPosition::empty_slot(
                        "position-1".to_string(),
                        "trader-1".to_string(),
                        7,
                        "BTC-PERP".to_string(),
                        5,
                    ),
                    margin_mode: HyperliquidPerpMarginMode::Cross,
                    bankruptcy_price: 50_000,
                    has_active_liquidation: false,
                },
                HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                    position: position(
                        "position-2",
                        "trader-2",
                        HyperliquidPerpPositionSide::Long,
                        2,
                        24_000,
                    ),
                    margin_mode: HyperliquidPerpMarginMode::Cross,
                    bankruptcy_price: 48_000,
                    has_active_liquidation: false,
                },
            ],
        };

        let view = QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase
            .compute_view(&query(49_000), read_model)
            .unwrap();

        assert_eq!(view.triggered_position_count, 0);
        assert!(view.positions.is_empty());
    }

    #[test]
    fn compute_returns_item_notional_and_total_notional() {
        let read_model = QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel {
            snapshots: vec![
                HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                    position: position(
                        "position-1",
                        "trader-1",
                        HyperliquidPerpPositionSide::Long,
                        2,
                        24_000,
                    ),
                    margin_mode: HyperliquidPerpMarginMode::Cross,
                    bankruptcy_price: 50_000,
                    has_active_liquidation: false,
                },
                HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                    position: position(
                        "position-2",
                        "trader-2",
                        HyperliquidPerpPositionSide::Short,
                        3,
                        12_000,
                    ),
                    margin_mode: HyperliquidPerpMarginMode::Isolated,
                    bankruptcy_price: 49_000,
                    has_active_liquidation: false,
                },
            ],
        };

        let view = QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase
            .compute_view(&query(49_000), read_model)
            .unwrap();

        assert_eq!(view.positions[0].liquidation_notional, 98_000);
        assert_eq!(view.positions[1].liquidation_notional, 147_000);
        assert_eq!(view.total_liquidation_notional, 245_000);
        assert_eq!(view.triggered_position_count, 2);
    }

    #[test]
    fn compute_returns_arithmetic_overflow_when_item_notional_overflows() {
        let read_model = QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel {
            snapshots: vec![HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                position: position(
                    "position-1",
                    "trader-1",
                    HyperliquidPerpPositionSide::Long,
                    u64::MAX,
                    24_000,
                ),
                margin_mode: HyperliquidPerpMarginMode::Cross,
                bankruptcy_price: u64::MAX,
                has_active_liquidation: false,
            }],
        };

        let result = QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase
            .compute_view(&query(2), read_model);

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpLiquidatablePositionsAtPriceError::ArithmeticOverflow)
        );
    }

    #[test]
    fn compute_returns_arithmetic_overflow_when_total_sum_overflows() {
        let read_model = QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel {
            snapshots: vec![
                HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                    position: position(
                        "position-1",
                        "trader-1",
                        HyperliquidPerpPositionSide::Long,
                        u64::MAX / 2 + 1,
                        24_000,
                    ),
                    margin_mode: HyperliquidPerpMarginMode::Cross,
                    bankruptcy_price: u64::MAX,
                    has_active_liquidation: false,
                },
                HyperliquidPerpLiquidatablePositionAtPriceSnapshot {
                    position: position(
                        "position-2",
                        "trader-2",
                        HyperliquidPerpPositionSide::Short,
                        u64::MAX / 2 + 1,
                        24_000,
                    ),
                    margin_mode: HyperliquidPerpMarginMode::Cross,
                    bankruptcy_price: 1,
                    has_active_liquidation: false,
                },
            ],
        };

        let result = QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase
            .compute_view(&query(1), read_model);

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpLiquidatablePositionsAtPriceError::ArithmeticOverflow)
        );
    }
}
