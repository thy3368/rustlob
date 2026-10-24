/// 市场级资金费率观测（fact，不可篡改）
/// 对应 fundingHistory 返回的一行
#[derive(Clone, Debug)]
pub struct FundingRateTick {
    pub coin: AssetId,      // "BTC" 等，内部映射为 AssetId
    pub time_ms: u64,       // 结算时间戳（ms）
    pub funding_rate: Rate, // 小时费率，scale 10，如 0.0000125 -> 12500
    pub premium: Rate,      // 溢价分量，scale 10
}
