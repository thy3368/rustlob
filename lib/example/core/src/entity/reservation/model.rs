use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, FieldDiff, FinancialClassification,
    FourColorArchetype, MiCausalRelation, MiCausalSourceMetadata,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{RESERVATION_ENTITY_TYPE, stable_entity_id};

/// 统一的冻结归属市场。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReservationMarketKind {
    Spot,
    Perp,
}

impl ReservationMarketKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Spot => "spot",
            Self::Perp => "perp",
        }
    }
}

/// 统一的冻结业务语义。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReservationKind {
    SpotBuyQuote,
    SpotSellBase,
    SpotBuyFeeQuote,
    SpotSellFeeQuote,
    PerpOpenMargin,
    PerpFlipNetNewMargin,
}

impl ReservationKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpotBuyQuote => "spot_buy_quote",
            Self::SpotSellBase => "spot_sell_base",
            Self::SpotBuyFeeQuote => "spot_buy_fee_quote",
            Self::SpotSellFeeQuote => "spot_sell_fee_quote",
            Self::PerpOpenMargin => "perp_open_margin",
            Self::PerpFlipNetNewMargin => "perp_flip_net_new_margin",
        }
    }
}

/// Reservation 当前生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReservationStatus {
    Active,
    ExhaustedByConsume,
    ClosedByRelease,
    ClosedMixed,
}

impl ReservationStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExhaustedByConsume => "exhausted_by_consume",
            Self::ClosedByRelease => "closed_by_release",
            Self::ClosedMixed => "closed_mixed",
        }
    }

    pub const fn is_closed(self) -> bool {
        !matches!(self, Self::Active)
    }
}

/// Reservation 闭合原因。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReservationCloseReason {
    Filled,
    Canceled,
    Rejected,
    IocRemainderCanceled,
    Expired,
}

impl ReservationCloseReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Canceled => "canceled",
            Self::Rejected => "rejected",
            Self::IocRemainderCanceled => "ioc_remainder_canceled",
            Self::Expired => "expired",
        }
    }
}

/// 通用冻结 MI。
///
/// `Reservation` 统一表达 spot / perp 的资金占用，不把顶层抽象命名成 `Margin`。
/// 其中 `reserved_*` 仍可留在订单内作为初始快照或兼容字段，但剩余量 authoritative 来源应是这里。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reservation {
    /// 本系统生成的稳定冻结凭证 ID。
    pub reservation_id: String,
    /// 冻结所属账户 ID。
    pub owner_account_id: String,
    /// 触发本次冻结的订单 ID，是 MI 因果链的前驱事实。
    pub caused_by_order_id: String,
    /// 冻结归属市场，用于区分 spot / perp。
    pub market_kind: ReservationMarketKind,
    /// 冻结业务角色，例如买单 quote、卖单 base 或 perp 开仓保证金。
    pub reservation_kind: ReservationKind,
    /// 被冻结的资产或保证金资产 ID。
    pub asset_id: String,
    /// 创建冻结时的原始冻结数量，必须大于 0。
    pub original_amount: u64,
    /// 已因成交、开仓或结算等业务消耗的冻结数量。
    pub consumed_amount: u64,
    /// 已因撤单、拒单、过期或剩余清理释放的冻结数量。
    pub released_amount: u64,
    /// 当前 authoritative 剩余冻结数量。
    pub remaining_amount: u64,
    /// 当前冻结生命周期状态。
    pub status: ReservationStatus,
    /// 冻结闭合原因；仍 active 时为 `None`，终态时必须存在。
    pub close_reason: Option<ReservationCloseReason>,
    /// 当前冻结实体版本，用于生成可回放更新事件。
    pub version: u64,
}

/// spot 资金冻结的语义别名。
pub type AssetReservation = Reservation;

/// perp 保证金冻结的语义别名。
pub type MarginReservation = Reservation;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReservationError {
    #[error("reservation original amount must be greater than zero")]
    InvalidOriginalAmount,
    #[error("reservation amount must be greater than zero")]
    InvalidAmount,
    #[error("reservation is already closed")]
    AlreadyClosed,
    #[error("reservation amount exceeds remaining amount")]
    AmountExceedsRemaining,
    #[error("reservation close reason is required when remaining amount becomes zero")]
    MissingCloseReason,
    #[error("arithmetic overflow while evolving reservation")]
    ArithmeticOverflow,
}

impl Reservation {
    /// 可 BDD 规格化的聚合根行为：创建冻结凭证。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        reservation_id: String,
        owner_account_id: String,
        caused_by_order_id: String,
        market_kind: ReservationMarketKind,
        reservation_kind: ReservationKind,
        asset_id: String,
        original_amount: u64,
    ) -> Result<Self, ReservationError> {
        if original_amount == 0 {
            return Err(ReservationError::InvalidOriginalAmount);
        }
        Ok(Self {
            reservation_id,
            owner_account_id,
            caused_by_order_id,
            market_kind,
            reservation_kind,
            asset_id,
            original_amount,
            consumed_amount: 0,
            released_amount: 0,
            remaining_amount: original_amount,
            status: ReservationStatus::Active,
            close_reason: None,
            version: 1,
        })
    }

    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.owner_account_id == account_id
    }

    pub fn is_asset(&self, asset_id: &str) -> bool {
        self.asset_id == asset_id
    }

    pub fn is_for_order(&self, order_id: &str) -> bool {
        self.caused_by_order_id == order_id
    }

    pub fn is_active(&self) -> bool {
        self.status == ReservationStatus::Active
    }

    pub fn can_consume(&self, amount: u64) -> bool {
        self.is_active() && amount > 0 && amount <= self.remaining_amount
    }

    pub fn can_release(&self, amount: u64) -> bool {
        self.is_active() && amount > 0 && amount <= self.remaining_amount
    }

    pub fn has_consistent_amounts(&self) -> bool {
        self.original_amount == self.consumed_amount + self.released_amount + self.remaining_amount
    }

    /// 可 BDD 规格化的聚合根行为：消耗冻结量。
    pub fn consume(
        &mut self,
        amount: u64,
        close_reason: Option<ReservationCloseReason>,
    ) -> Result<(), ReservationError> {
        if amount == 0 {
            return Err(ReservationError::InvalidAmount);
        }
        if self.status.is_closed() {
            return Err(ReservationError::AlreadyClosed);
        }
        if amount > self.remaining_amount {
            return Err(ReservationError::AmountExceedsRemaining);
        }

        let consumed_amount =
            self.consumed_amount.checked_add(amount).ok_or(ReservationError::ArithmeticOverflow)?;
        let remaining_amount = self
            .remaining_amount
            .checked_sub(amount)
            .ok_or(ReservationError::ArithmeticOverflow)?;
        if remaining_amount == 0 && close_reason.is_none() {
            return Err(ReservationError::MissingCloseReason);
        }
        let version = self.version.checked_add(1).ok_or(ReservationError::ArithmeticOverflow)?;

        self.consumed_amount = consumed_amount;
        self.remaining_amount = remaining_amount;
        self.version = version;
        self.recompute_terminal_state(close_reason)
    }

    /// 可 BDD 规格化的聚合根行为：释放冻结量。
    pub fn release(
        &mut self,
        amount: u64,
        close_reason: Option<ReservationCloseReason>,
    ) -> Result<(), ReservationError> {
        if amount == 0 {
            return Err(ReservationError::InvalidAmount);
        }
        if self.status.is_closed() {
            return Err(ReservationError::AlreadyClosed);
        }
        if amount > self.remaining_amount {
            return Err(ReservationError::AmountExceedsRemaining);
        }

        let released_amount =
            self.released_amount.checked_add(amount).ok_or(ReservationError::ArithmeticOverflow)?;
        let remaining_amount = self
            .remaining_amount
            .checked_sub(amount)
            .ok_or(ReservationError::ArithmeticOverflow)?;
        if remaining_amount == 0 && close_reason.is_none() {
            return Err(ReservationError::MissingCloseReason);
        }
        let version = self.version.checked_add(1).ok_or(ReservationError::ArithmeticOverflow)?;

        self.released_amount = released_amount;
        self.remaining_amount = remaining_amount;
        self.version = version;
        self.recompute_terminal_state(close_reason)
    }

    fn recompute_terminal_state(
        &mut self,
        close_reason: Option<ReservationCloseReason>,
    ) -> Result<(), ReservationError> {
        if self.remaining_amount > 0 {
            self.status = ReservationStatus::Active;
            self.close_reason = None;
            return Ok(());
        }

        self.close_reason = Some(close_reason.ok_or(ReservationError::MissingCloseReason)?);
        self.status = match (self.consumed_amount > 0, self.released_amount > 0) {
            (true, false) => ReservationStatus::ExhaustedByConsume,
            (false, true) => ReservationStatus::ClosedByRelease,
            (true, true) => ReservationStatus::ClosedMixed,
            (false, false) => return Err(ReservationError::InvalidOriginalAmount),
        };
        Ok(())
    }
}

impl FieldDiff for Reservation {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("reservation_id", "", self.reservation_id.clone()),
            EntityFieldChange::new("owner_account_id", "", self.owner_account_id.clone()),
            EntityFieldChange::new("caused_by_order_id", "", self.caused_by_order_id.clone()),
            EntityFieldChange::new("market_kind", "", self.market_kind.as_str()),
            EntityFieldChange::new("reservation_kind", "", self.reservation_kind.as_str()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("original_amount", "", self.original_amount.to_string()),
            EntityFieldChange::new("consumed_amount", "", self.consumed_amount.to_string()),
            EntityFieldChange::new("released_amount", "", self.released_amount.to_string()),
            EntityFieldChange::new("remaining_amount", "", self.remaining_amount.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
            EntityFieldChange::new(
                "close_reason",
                "",
                option_close_reason_value(self.close_reason),
            ),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = vec![
            EntityFieldChange::new(
                "reservation_id",
                self.reservation_id.clone(),
                other.reservation_id.clone(),
            ),
            EntityFieldChange::new(
                "owner_account_id",
                self.owner_account_id.clone(),
                other.owner_account_id.clone(),
            ),
            EntityFieldChange::new(
                "caused_by_order_id",
                self.caused_by_order_id.clone(),
                other.caused_by_order_id.clone(),
            ),
            EntityFieldChange::new(
                "market_kind",
                self.market_kind.as_str(),
                other.market_kind.as_str(),
            ),
            EntityFieldChange::new(
                "reservation_kind",
                self.reservation_kind.as_str(),
                other.reservation_kind.as_str(),
            ),
            EntityFieldChange::new("asset_id", self.asset_id.clone(), other.asset_id.clone()),
        ];
        push_u64_change(
            &mut changes,
            "original_amount",
            self.original_amount,
            other.original_amount,
        );
        push_u64_change(
            &mut changes,
            "consumed_amount",
            self.consumed_amount,
            other.consumed_amount,
        );
        push_u64_change(
            &mut changes,
            "released_amount",
            self.released_amount,
            other.released_amount,
        );
        push_u64_change(
            &mut changes,
            "remaining_amount",
            self.remaining_amount,
            other.remaining_amount,
        );
        if self.status != other.status {
            changes.push(EntityFieldChange::new(
                "status",
                self.status.as_str(),
                other.status.as_str(),
            ));
        }
        if self.close_reason != other.close_reason {
            changes.push(EntityFieldChange::new(
                "close_reason",
                option_close_reason_value(self.close_reason),
                option_close_reason_value(other.close_reason),
            ));
        }
        changes
    }
}

impl Entity for Reservation {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.reservation_id.clone()
    }

    fn entity_type() -> u8 {
        RESERVATION_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::MomentInterval
    }

    fn aggregate_role() -> AggregateRole
    where
        Self: Sized,
    {
        AggregateRole::AggregateMember
    }

    fn financial_classification() -> FinancialClassification
    where
        Self: Sized,
    {
        FinancialClassification::BusinessVoucher
    }

    fn is_mi_chain_root() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn mi_causal_sources() -> &'static [MiCausalSourceMetadata]
    where
        Self: Sized,
    {
        &[MiCausalSourceMetadata {
            source_fact_type: "order",
            relation: MiCausalRelation::CausedBy,
            source_role: "caused_by_order",
        }]
    }

    fn entity_version(&self) -> u64 {
        self.version
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "reservation_id" | "owner_account_id" | "caused_by_order_id" | "market_kind"
            | "reservation_kind" | "asset_id" | "status" | "close_reason" => 0,
            "original_amount" | "consumed_amount" | "released_amount" | "remaining_amount" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.reservation_id))
    }
}

fn option_close_reason_value(reason: Option<ReservationCloseReason>) -> String {
    reason.map(ReservationCloseReason::as_str).unwrap_or_default().to_string()
}

fn push_u64_change(
    changes: &mut Vec<EntityFieldChange>,
    field: &'static str,
    before: u64,
    after: u64,
) {
    if before != after {
        changes.push(EntityFieldChange::new(field, before.to_string(), after.to_string()));
    }
}
