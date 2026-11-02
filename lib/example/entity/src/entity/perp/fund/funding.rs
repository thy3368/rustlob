use common_entity::{
    AggregateRole, Entity, EntityFieldChange, EntityMutationModel, FieldDiff,
    FinancialClassification, FourColorArchetype, Shard,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::account::symbol::Symbol;

/// 资金费结算明细
const USER_FUNDING_ENTITY_TYPE: u8 = 14;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserFunding {
    /// 本次用户资金费事实的稳定标识。
    funding_id: u64,

    /// 交易对（BTC/ETH…）
    coin: Symbol,

    /// 账户视角资金费（原始值，正=收，负=付）
    /// Hyperliquid 原生口径：负值=你付钱
    usdc: Decimal,

    /// 结算时仓位大小（带符号，正多负空）
    szi: Decimal,

    /// 本次小时费率（小数，如 0.0000125 = 0.00125%）
    funding_rate: Decimal,

    /// 该周期采样点数（部分 SDK 可能不返回）
    n_samples: Option<u32>,

    /// 资金费事实创建时间，单位为 Unix 毫秒。
    created_at: u64,
    /// 资金费事实最后更新时间，单位为 Unix 毫秒。
    updated_at: u64,
}

impl UserFunding {
    /// 装配已经校验并计算完成的用户资金费事实。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        funding_id: u64,
        coin: Symbol,
        usdc: Decimal,
        szi: Decimal,
        funding_rate: Decimal,
        n_samples: Option<u32>,
        created_at: u64,
        updated_at: u64,
    ) -> Self {
        Self { funding_id, coin, usdc, szi, funding_rate, n_samples, created_at, updated_at }
    }

    pub fn funding_id(&self) -> u64 {
        self.funding_id
    }

    pub fn coin(&self) -> Symbol {
        self.coin
    }

    pub fn usdc(&self) -> Decimal {
        self.usdc
    }

    pub fn signed_size(&self) -> Decimal {
        self.szi
    }

    pub fn funding_rate(&self) -> Decimal {
        self.funding_rate
    }
}

impl FieldDiff for UserFunding {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("funding_id", "", self.funding_id.to_string()),
            EntityFieldChange::new("coin", "", self.coin.code()),
            EntityFieldChange::new("usdc", "", self.usdc.to_string()),
            EntityFieldChange::new("szi", "", self.szi.to_string()),
            EntityFieldChange::new("funding_rate", "", self.funding_rate.to_string()),
            EntityFieldChange::new(
                "n_samples",
                "",
                self.n_samples.map(|value| value.to_string()).unwrap_or_default(),
            ),
            EntityFieldChange::new("created_at", "", self.created_at.to_string()),
            EntityFieldChange::new("updated_at", "", self.updated_at.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl Shard for UserFunding {
    fn partition(&self) -> u8 {
        (self.funding_id % 256) as u8
    }
}

impl Entity for UserFunding {
    fn entity_id(&self) -> u64 {
        self.funding_id
    }

    fn is_derived() -> bool {
        false
    }

    fn entity_type() -> u8
    where
        Self: Sized,
    {
        USER_FUNDING_ENTITY_TYPE
    }

    fn lsn(&self) -> u64 {
        1
    }

    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::AppendOnlyRecord
    }

    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }

    fn financial_classification() -> FinancialClassification {
        FinancialClassification::BusinessVoucher
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "funding_id" | "usdc" | "szi" | "funding_rate" | "n_samples" | "created_at"
            | "updated_at" => 1,
            _ => 0,
        }
    }
}

/// userFunding API 返回的单条记录
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct FundingRow {
    /// 整点结算时间（ms）
    pub time: i64,

    /// 链上交易哈希
    pub hash: String,

    pub delta: UserFunding,
}

impl FundingRow {
    /// 统一符号：返回正值=你收了，负值=你付了
    /// 如果 Hyperliquid 原生返回是反的，在这里翻转
    pub fn net_received(&self) -> Decimal {
        // 当前口径：usdc 正=收，负=付，直接返回
        self.delta.usdc
    }

    /// 按 coin 过滤 + 求和的辅助方法
    // pub fn sum_by_coin(rows: &[FundingRow], coin: &str) -> Decimal {
    //     rows.iter().filter(|r| r.delta.coin == coin).map(|r| r.delta.usdc).sum()
    // }

    /// 按时间窗过滤
    pub fn filter_by_time_window(
        rows: &[FundingRow],
        start_ms: i64,
        end_ms: i64,
    ) -> Vec<&FundingRow> {
        rows.iter().filter(|r| r.time >= start_ms && r.time <= end_ms).collect()
    }
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;
    use crate::account::coin::Coin;

    #[test]
    fn user_funding_projects_stable_create_event() {
        let funding = UserFunding::new(
            42,
            Symbol::new(1, Coin::BTC, Coin::USDC),
            Decimal::from(-10),
            Decimal::from(2),
            Decimal::new(1, 4),
            None,
            100,
            100,
        );

        let event = funding.track_create_event().unwrap();

        assert_eq!(funding.entity_id(), 42);
        assert_eq!(funding.lsn(), 1);
        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("usdc") && change.new_value_bytes() == b"-10"
        }));
    }
}
