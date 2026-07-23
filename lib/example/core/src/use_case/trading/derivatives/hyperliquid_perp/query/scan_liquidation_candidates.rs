use cmd_handler::command_use_case_def2::IssuedByParty;
use cmd_handler::query_use_case_def::QueryUseCase;
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode, HyperliquidPerpPosition,
};
use crate::use_case::trading::derivatives::hyperliquid_perp::liquidation_trigger_reason::derive_hyperliquid_perp_liquidation_trigger_reason;

/// 批量查询 Hyperliquid perp 强平候选的输入。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpLiquidationCandidates {
    /// 发起风险扫描的业务主体。
    pub party_id: String,
    /// 扫描批次 ID。
    pub scan_batch_id: String,
}

impl IssuedByParty for QueryHyperliquidPerpLiquidationCandidates {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 单个仓位的风险扫描快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpRiskSnapshot {
    /// 被扫描仓位。
    pub position: HyperliquidPerpPosition,
    /// 当前仓位保证金模式。
    pub margin_mode: HyperliquidPerpMarginMode,
    /// Cross 或 Isolated 可用于判定的保证金额度。
    pub available_margin: u64,
    /// 当前仓位的破产价格。
    pub bankruptcy_price: u64,
    /// 当前市场标记价格。
    pub mark_price: u64,
    /// 当前仓位是否已经进入强平流程。
    pub has_active_liquidation: bool,
}

/// 批量强平扫描时需要的已加载读模型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpLiquidationCandidatesReadModel {
    /// 当前扫描批次涉及的风险快照。
    pub snapshots: Vec<HyperliquidPerpRiskSnapshot>,
}

/// 单个强平候选的业务视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpLiquidationCandidate {
    pub position_id: String,
    pub account_id: String,
    pub asset: u32,
    pub symbol: String,
    pub margin_mode: HyperliquidPerpMarginMode,
    pub mark_price: u64,
    pub bankruptcy_price: u64,
    pub trigger_reason: HyperliquidPerpLiquidationTriggerReason,
}

/// 批量查询强平候选时的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum QueryHyperliquidPerpLiquidationCandidatesError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("scan_batch_id must not be empty")]
    InvalidScanBatchId,
    #[error("risk snapshot contains invalid mark or bankruptcy price")]
    InvalidRiskPrice,
    #[error("risk snapshot position state is inconsistent")]
    InconsistentPositionState,
}

/// 批量筛选当前应进入强平流程的 Hyperliquid perp 仓位。
///
/// 该用例只做读模型判定，不写入事件；命中的候选由上层 adapter 扇出成多条
/// `StartHyperliquidPerpLiquidationCmd`。
#[derive(Debug, Clone, Copy, Default)]
pub struct QueryHyperliquidPerpLiquidationCandidatesUseCase;

impl QueryUseCase for QueryHyperliquidPerpLiquidationCandidatesUseCase {
    type Query = QueryHyperliquidPerpLiquidationCandidates;
    type ReadModel = QueryHyperliquidPerpLiquidationCandidatesReadModel;
    type View = Vec<HyperliquidPerpLiquidationCandidate>;
    type Error = QueryHyperliquidPerpLiquidationCandidatesError;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_query(&self, query: &Self::Query) -> Result<(), Self::Error> {
        if query.party_id.is_empty() {
            return Err(QueryHyperliquidPerpLiquidationCandidatesError::InvalidPartyId);
        }
        if query.scan_batch_id.is_empty() {
            return Err(QueryHyperliquidPerpLiquidationCandidatesError::InvalidScanBatchId);
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
                    QueryHyperliquidPerpLiquidationCandidatesError::InconsistentPositionState,
                );
            }
            if snapshot.mark_price == 0 || snapshot.bankruptcy_price == 0 {
                return Err(QueryHyperliquidPerpLiquidationCandidatesError::InvalidRiskPrice);
            }
        }

        Ok(())
    }

    fn compute_view(
        &self,
        _query: &Self::Query,
        read_model: Self::ReadModel,
    ) -> Result<Self::View, Self::Error> {
        let mut candidates = Vec::with_capacity(read_model.snapshots.len());

        for snapshot in read_model.snapshots {
            if snapshot.has_active_liquidation {
                continue;
            }

            let Some(trigger_reason) = derive_hyperliquid_perp_liquidation_trigger_reason(
                &snapshot.position,
                snapshot.available_margin,
                snapshot.mark_price,
                snapshot.bankruptcy_price,
            ) else {
                continue;
            };

            candidates.push(HyperliquidPerpLiquidationCandidate {
                position_id: snapshot.position.position_key,
                account_id: snapshot.position.account_id,
                asset: snapshot.position.perp_asset_id,
                symbol: snapshot.position.coin,
                margin_mode: snapshot.margin_mode,
                mark_price: snapshot.mark_price,
                bankruptcy_price: snapshot.bankruptcy_price,
                trigger_reason,
            });
        }

        Ok(candidates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn position(position_id: &str, account_id: &str, signed_size: i64) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::new(
            position_id.to_string(),
            account_id.to_string(),
            7,
            "BTC-PERP".to_string(),
            signed_size,
            60_000,
            5,
            HyperliquidPerpMarginMode::Cross,
            0,
            3,
        )
    }

    #[test]
    fn role_returns_risk_engine() {
        assert_eq!(QueryHyperliquidPerpLiquidationCandidatesUseCase.role(), "RiskEngine");
    }

    #[test]
    fn pre_check_rejects_blank_batch_id() {
        let result = QueryHyperliquidPerpLiquidationCandidatesUseCase.pre_check_query(
            &QueryHyperliquidPerpLiquidationCandidates {
                party_id: "risk-engine".to_string(),
                scan_batch_id: String::new(),
            },
        );

        assert_eq!(result, Err(QueryHyperliquidPerpLiquidationCandidatesError::InvalidScanBatchId));
    }

    #[test]
    fn compute_filters_only_triggered_positions() {
        let query = QueryHyperliquidPerpLiquidationCandidates {
            party_id: "risk-engine".to_string(),
            scan_batch_id: "scan-1".to_string(),
        };
        let read_model = QueryHyperliquidPerpLiquidationCandidatesReadModel {
            snapshots: vec![
                HyperliquidPerpRiskSnapshot {
                    position: position("position-1", "trader-1", 2),
                    margin_mode: HyperliquidPerpMarginMode::Cross,
                    available_margin: 100,
                    bankruptcy_price: 50_000,
                    mark_price: 49_000,
                    has_active_liquidation: false,
                },
                HyperliquidPerpRiskSnapshot {
                    position: position("position-2", "trader-2", 2),
                    margin_mode: HyperliquidPerpMarginMode::Cross,
                    available_margin: 100,
                    bankruptcy_price: 50_000,
                    mark_price: 50_100,
                    has_active_liquidation: false,
                },
                HyperliquidPerpRiskSnapshot {
                    position: position("position-3", "trader-3", -2),
                    margin_mode: HyperliquidPerpMarginMode::Isolated,
                    available_margin: 0,
                    bankruptcy_price: 50_000,
                    mark_price: 50_100,
                    has_active_liquidation: false,
                },
            ],
        };

        QueryHyperliquidPerpLiquidationCandidatesUseCase
            .validate_against_read_model(&query, &read_model)
            .unwrap();
        let view = QueryHyperliquidPerpLiquidationCandidatesUseCase
            .compute_view(&query, read_model)
            .unwrap();

        assert_eq!(view.len(), 2);
        assert_eq!(view[0].position_id, "position-1");
        assert_eq!(
            view[0].trigger_reason,
            HyperliquidPerpLiquidationTriggerReason::MaintenanceMarginBreach
        );
        assert_eq!(view[1].position_id, "position-3");
        assert_eq!(view[1].trigger_reason, HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk);
    }
}
