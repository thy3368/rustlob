use crate::SpotOrderV2;

/// 按成交资产和成交价格读取当前应被触发的条件单。
///
/// 这是 core 对 outbound state source 的最小要求；具体存储实现负责加载 authoritative
/// 订单状态，价格触发规则由 [`SpotOrderV2::is_triggered_by_market_price`] 表达。
pub trait PendingTriggerSpotOrderSource: Send + Sync {
    type Error: std::error::Error;

    fn load_pending_trigger_orders(
        &self,
        asset: u32,
        market_price: u64,
    ) -> Result<Vec<SpotOrderV2>, Self::Error>;
}
