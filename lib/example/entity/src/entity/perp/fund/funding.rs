use common_entity::{
    AggregateRole, Entity, EntityFieldChange, EntityMutationModel, FieldDiff,
    FinancialClassification, FourColorArchetype, Fsm, Shard,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::account::symbol::Symbol;
use crate::entity::common::FundingRate;

/// 资金费结算明细
const USER_FUNDING_ENTITY_TYPE: u8 = 14;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserFunding {
    /// 本次用户资金费事实的稳定标识
    funding_id: u64,
    /// 用户
    party_id: u64,

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
    state: UserFundingState,
    version: u64,

    /// 资金费事实创建时间，单位为 Unix 毫秒
    created_at: u64,
    /// 资金费事实最后更新时间，单位为 Unix 毫秒
    updated_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserFundingState {
    Pending,
    Settled,
}
impl UserFundingState {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Settled => "settled",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserFundingEvent {
    Settle { settled_at: u64 },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserFundingOutput {
    BalanceImpact { party_id: u64, quote_coin_idx: u64, signed_usdc_delta: Decimal },
}
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum UserFundingError {
    #[error("user funding is already settled")]
    AlreadySettled,
    #[error("settlement time must not precede creation time")]
    InvalidSettlementTime,
    #[error("user funding version overflow")]
    VersionOverflow,
}

impl UserFunding {
    /// 装配已经校验并计算完成的用户资金费事实。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        funding_id: u64,
        party_id: u64,
        coin: Symbol,
        usdc: Decimal,
        szi: Decimal,
        funding_rate: FundingRate,
        n_samples: Option<u32>,
        created_at: u64,
    ) -> Self {
        let funding_rate = funding_rate.value();
        Self {
            funding_id,
            party_id,
            coin,
            usdc,
            szi,
            funding_rate,
            n_samples,
            state: UserFundingState::Pending,
            version: 1,
            created_at,
            updated_at: created_at,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn restore_settled(
        funding_id: u64,
        party_id: u64,
        coin: Symbol,
        usdc: Decimal,
        szi: Decimal,
        funding_rate: FundingRate,
        n_samples: Option<u32>,
        created_at: u64,
        settled_at: u64,
        version: u64,
    ) -> Self {
        Self {
            funding_id,
            party_id,
            coin,
            usdc,
            szi,
            funding_rate: funding_rate.value(),
            n_samples,
            state: UserFundingState::Settled,
            version,
            created_at,
            updated_at: settled_at,
        }
    }

    pub fn funding_id(&self) -> u64 {
        self.funding_id
    }

    pub fn coin(&self) -> Symbol {
        self.coin
    }
    pub fn party_id(&self) -> u64 {
        self.party_id
    }
    pub fn state(&self) -> UserFundingState {
        self.state
    }
    pub fn version(&self) -> u64 {
        self.version
    }
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    pub fn updated_at(&self) -> u64 {
        self.updated_at
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

impl Fsm for UserFunding {
    type State = UserFundingState;
    type Event = UserFundingEvent;
    type Output = UserFundingOutput;
    type Error = UserFundingError;
    fn transition(
        &mut self,
        event: Self::Event,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        let UserFundingEvent::Settle { settled_at } = event;
        if self.state == UserFundingState::Settled {
            return Err(UserFundingError::AlreadySettled);
        }
        if settled_at < self.created_at {
            return Err(UserFundingError::InvalidSettlementTime);
        }
        let version = self.version.checked_add(1).ok_or(UserFundingError::VersionOverflow)?;
        let output = UserFundingOutput::BalanceImpact {
            party_id: self.party_id,
            quote_coin_idx: u64::from(self.coin.quote.index()),
            signed_usdc_delta: self.usdc,
        };
        self.state = UserFundingState::Settled;
        self.version = version;
        self.updated_at = settled_at;
        Ok((self.state, output))
    }
}

impl FieldDiff for UserFunding {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("funding_id", "", self.funding_id.to_string()),
            EntityFieldChange::new("party_id", "", self.party_id.to_string()),
            EntityFieldChange::new("coin", "", self.coin.code()),
            EntityFieldChange::new("usdc", "", self.usdc.to_string()),
            EntityFieldChange::new("szi", "", self.szi.to_string()),
            EntityFieldChange::new("funding_rate", "", self.funding_rate.to_string()),
            EntityFieldChange::new(
                "n_samples",
                "",
                self.n_samples.map(|value| value.to_string()).unwrap_or_default(),
            ),
            EntityFieldChange::new("state", "", self.state.as_str()),
            EntityFieldChange::new("version", "", self.version.to_string()),
            EntityFieldChange::new("created_at", "", self.created_at.to_string()),
            EntityFieldChange::new("updated_at", "", self.updated_at.to_string()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let fields = [
            ("party_id", self.party_id.to_string(), other.party_id.to_string()),
            ("state", self.state.as_str().to_owned(), other.state.as_str().to_owned()),
            ("version", self.version.to_string(), other.version.to_string()),
            ("created_at", self.created_at.to_string(), other.created_at.to_string()),
            ("updated_at", self.updated_at.to_string(), other.updated_at.to_string()),
        ];
        fields
            .into_iter()
            .filter(|(_, a, b)| a != b)
            .map(|(n, a, b)| EntityFieldChange::new(n, a, b))
            .collect()
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
        self.version
    }

    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::VersionedMutable
    }

    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }

    fn financial_classification() -> FinancialClassification {
        FinancialClassification::BusinessVoucher
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "funding_id" | "party_id" | "usdc" | "szi" | "funding_rate" | "n_samples" | "state"
            | "version" | "created_at" | "updated_at" => 1,
            _ => 0,
        }
    }
}
