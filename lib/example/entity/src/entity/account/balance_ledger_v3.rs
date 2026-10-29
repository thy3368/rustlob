use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::account::coin::Coin;

/// 一条 create-only 的余额流水审计事实。
///
/// 该实体先记录某条 `Balance` 的身份、业务操作与金额，成功应用到余额后再补齐 before/after 快照。
/// 在聚合边界上，它自身作为一条独立的审计事实聚合根存在。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceLedgerV3 {
    /// 流水记录 ID。
    pub entry_id: u64,
    /// 用户 ID 或业务主体 ID。
    pub party_id: u64,
    /// 账户 ID。
    pub account_id: u64,
    /// 资产 ID。
    pub coin: Coin,
    /// 对应余额快照实体 ID。好象可以不要
    /// 本条流水表达的余额操作。
    pub operation: LedgerOperation,
    /// 本条流水表达的金额。
    pub amount: Decimal,
    /// 变更前可用余额。
    pub before_available: Option<Decimal>,
    /// 变更前冻结余额。
    pub before_frozen: Option<Decimal>,
    /// 变更后可用余额。
    pub after_available: Option<Decimal>,
    /// 变更后冻结余额。
    pub after_frozen: Option<Decimal>,
    /// 余额变更原因。
    /// 业务成交时间，单位为 Unix epoch 毫秒。
    pub created_at: u64,
}

/// 余额流水的权威业务操作。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerOperation {
    Freeze,
    UnfreezePay,
}

impl LedgerOperation {
    /// 返回稳定操作编码，供 replay event / 审计查询使用。
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Freeze => "freeze",
            Self::UnfreezePay => "unfreeze_pay",
        }
    }
}
