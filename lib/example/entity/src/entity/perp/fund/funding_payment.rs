use super::funding_payment_factory::Size;
use crate::account::coin::Coin;
use crate::entity::account::usdc::Usdc;
use crate::entity::common::{FundingRate, UserId};

/// 用户级资金费结算事件（L0 fact）
/// 对应 userFunding 返回的一行
#[derive(Clone, Debug)]
pub struct FundingPayment {
    pub user: UserId,
    pub time_ms: u64, // 结算时间
    pub seq: u64,     // 全局有序序列（重放用）
    pub hash: TxHash, // 链上 tx hash

    // —— delta 主体 ——
    pub coin: Coin,                // 结算品种
    pub funding_rate: FundingRate, // 本次采用费率（小时）
    pub szi: Size,                 // 结算时带符号仓位（正多负空）
    pub usdc: Usdc,                // 正负：正=收钱，负=付钱（已结算到余额）

    pub n_samples: Option<u32>, // 采样数，通常为 null
}

type TxHash = [u8; 32];
