use crate::entity::{HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpPosition};

/// 统一根据风险快照推导强平触发原因。
pub(crate) fn derive_hyperliquid_perp_liquidation_trigger_reason(
    position: &HyperliquidPerpPosition,
    available_margin: u64,
    mark_price: u64,
    bankruptcy_price: u64,
) -> Option<HyperliquidPerpLiquidationTriggerReason> {
    if mark_price == 0 || bankruptcy_price == 0 {
        return None;
    }
    if !position.is_liquidatable() {
        return None;
    }
    if !position.liquidation_triggered_by_mark_price(mark_price, bankruptcy_price) {
        return None;
    }

    Some(if available_margin == 0 {
        HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk
    } else {
        HyperliquidPerpLiquidationTriggerReason::MaintenanceMarginBreach
    })
}
