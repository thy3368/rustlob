use crate::account::symbol::Symbol;
use crate::entity::common::FundingRate;

/// 市场级资金费率观测（fact，不可篡改）
/// 对应 fundingHistory 返回的一行
#[derive(Clone, Debug)]
pub struct FundingRateTick {
    pub coin: Symbol, // "BTC" 等，内部映射为 AssetId
    pub time_ms: u64, // 结算时间戳（ms）
    pub funding_rate: FundingRate,
    pub premium: FundingRate,
}
