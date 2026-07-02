use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel,
    FinancialClassification, FourColorArchetype, MiCausalRelation, MiCausalSourceMetadata,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const RESERVATION_ENTITY_TYPE: u8 = 23;
const RESERVATION_CREATED_ENTITY_TYPE: u8 = 24;
const RESERVATION_CONSUMED_ENTITY_TYPE: u8 = 25;
const RESERVATION_RELEASED_ENTITY_TYPE: u8 = 26;

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
    PerpOpenMargin,
    PerpFlipNetNewMargin,
}

impl ReservationKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpotBuyQuote => "spot_buy_quote",
            Self::SpotSellBase => "spot_sell_base",
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
    pub reservation_id: String,
    pub owner_account_id: String,
    pub caused_by_order_id: String,
    pub market_kind: ReservationMarketKind,
    pub reservation_kind: ReservationKind,
    pub asset_id: String,
    pub original_amount: u64,
    pub consumed_amount: u64,
    pub released_amount: u64,
    pub remaining_amount: u64,
    pub status: ReservationStatus,
    pub close_reason: Option<ReservationCloseReason>,
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

    pub fn consume(
        &self,
        amount: u64,
        close_reason: Option<ReservationCloseReason>,
    ) -> Result<Self, ReservationError> {
        if amount == 0 {
            return Err(ReservationError::InvalidAmount);
        }
        if self.status.is_closed() {
            return Err(ReservationError::AlreadyClosed);
        }
        if amount > self.remaining_amount {
            return Err(ReservationError::AmountExceedsRemaining);
        }

        let mut after = self.clone();
        after.consumed_amount = after
            .consumed_amount
            .checked_add(amount)
            .ok_or(ReservationError::ArithmeticOverflow)?;
        after.remaining_amount = after
            .remaining_amount
            .checked_sub(amount)
            .ok_or(ReservationError::ArithmeticOverflow)?;
        after.version = after.version.checked_add(1).ok_or(ReservationError::ArithmeticOverflow)?;
        after.recompute_terminal_state(close_reason)?;
        Ok(after)
    }

    pub fn release(
        &self,
        amount: u64,
        close_reason: Option<ReservationCloseReason>,
    ) -> Result<Self, ReservationError> {
        if amount == 0 {
            return Err(ReservationError::InvalidAmount);
        }
        if self.status.is_closed() {
            return Err(ReservationError::AlreadyClosed);
        }
        if amount > self.remaining_amount {
            return Err(ReservationError::AmountExceedsRemaining);
        }

        let mut after = self.clone();
        after.released_amount = after
            .released_amount
            .checked_add(amount)
            .ok_or(ReservationError::ArithmeticOverflow)?;
        after.remaining_amount = after
            .remaining_amount
            .checked_sub(amount)
            .ok_or(ReservationError::ArithmeticOverflow)?;
        after.version = after.version.checked_add(1).ok_or(ReservationError::ArithmeticOverflow)?;
        after.recompute_terminal_state(close_reason)?;
        Ok(after)
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
        AggregateRole::AggregateRoot
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

/// Reservation 建立时的 append-only 事实。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReservationCreated {
    pub event_id: String,
    pub reservation_id: String,
    pub owner_account_id: String,
    pub caused_by_order_id: String,
    pub market_kind: ReservationMarketKind,
    pub reservation_kind: ReservationKind,
    pub asset_id: String,
    pub original_amount: u64,
}

impl ReservationCreated {
    pub fn from_reservation(event_id: String, reservation: &Reservation) -> Self {
        Self {
            event_id,
            reservation_id: reservation.reservation_id.clone(),
            owner_account_id: reservation.owner_account_id.clone(),
            caused_by_order_id: reservation.caused_by_order_id.clone(),
            market_kind: reservation.market_kind,
            reservation_kind: reservation.reservation_kind,
            asset_id: reservation.asset_id.clone(),
            original_amount: reservation.original_amount,
        }
    }
}

/// Reservation 消耗时的 append-only 事实。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReservationConsumed {
    pub event_id: String,
    pub reservation_id: String,
    pub owner_account_id: String,
    pub asset_id: String,
    pub amount: u64,
    pub caused_by_ref_id: String,
    pub remaining_amount_after: u64,
}

impl ReservationConsumed {
    pub fn new(
        event_id: String,
        reservation: &Reservation,
        amount: u64,
        caused_by_ref_id: String,
    ) -> Self {
        Self {
            event_id,
            reservation_id: reservation.reservation_id.clone(),
            owner_account_id: reservation.owner_account_id.clone(),
            asset_id: reservation.asset_id.clone(),
            amount,
            caused_by_ref_id,
            remaining_amount_after: reservation.remaining_amount,
        }
    }
}

/// Reservation 释放时的 append-only 事实。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReservationReleased {
    pub event_id: String,
    pub reservation_id: String,
    pub owner_account_id: String,
    pub asset_id: String,
    pub amount: u64,
    pub caused_by_ref_id: String,
    pub remaining_amount_after: u64,
    pub close_reason: ReservationCloseReason,
}

impl ReservationReleased {
    pub fn new(
        event_id: String,
        reservation: &Reservation,
        amount: u64,
        caused_by_ref_id: String,
        close_reason: ReservationCloseReason,
    ) -> Self {
        Self {
            event_id,
            reservation_id: reservation.reservation_id.clone(),
            owner_account_id: reservation.owner_account_id.clone(),
            asset_id: reservation.asset_id.clone(),
            amount,
            caused_by_ref_id,
            remaining_amount_after: reservation.remaining_amount,
            close_reason,
        }
    }
}

impl Entity for ReservationCreated {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.event_id.clone()
    }

    fn entity_type() -> u8 {
        RESERVATION_CREATED_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel
    where
        Self: Sized,
    {
        EntityMutationModel::AppendOnlyRecord
    }

    fn aggregate_role() -> AggregateRole
    where
        Self: Sized,
    {
        AggregateRole::AggregateRoot
    }

    fn financial_classification() -> FinancialClassification
    where
        Self: Sized,
    {
        FinancialClassification::BusinessVoucher
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("event_id", "", self.event_id.clone()),
            EntityFieldChange::new("reservation_id", "", self.reservation_id.clone()),
            EntityFieldChange::new("owner_account_id", "", self.owner_account_id.clone()),
            EntityFieldChange::new("caused_by_order_id", "", self.caused_by_order_id.clone()),
            EntityFieldChange::new("market_kind", "", self.market_kind.as_str()),
            EntityFieldChange::new("reservation_kind", "", self.reservation_kind.as_str()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("original_amount", "", self.original_amount.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "event_id" | "reservation_id" | "owner_account_id" | "caused_by_order_id"
            | "market_kind" | "reservation_kind" | "asset_id" => 0,
            "original_amount" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.event_id))
    }
}

impl Entity for ReservationConsumed {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.event_id.clone()
    }

    fn entity_type() -> u8 {
        RESERVATION_CONSUMED_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel
    where
        Self: Sized,
    {
        EntityMutationModel::AppendOnlyRecord
    }

    fn aggregate_role() -> AggregateRole
    where
        Self: Sized,
    {
        AggregateRole::AggregateRoot
    }

    fn financial_classification() -> FinancialClassification
    where
        Self: Sized,
    {
        FinancialClassification::BusinessVoucher
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("event_id", "", self.event_id.clone()),
            EntityFieldChange::new("reservation_id", "", self.reservation_id.clone()),
            EntityFieldChange::new("owner_account_id", "", self.owner_account_id.clone()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("amount", "", self.amount.to_string()),
            EntityFieldChange::new("caused_by_ref_id", "", self.caused_by_ref_id.clone()),
            EntityFieldChange::new(
                "remaining_amount_after",
                "",
                self.remaining_amount_after.to_string(),
            ),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "event_id" | "reservation_id" | "owner_account_id" | "asset_id"
            | "caused_by_ref_id" => 0,
            "amount" | "remaining_amount_after" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.event_id))
    }
}

impl Entity for ReservationReleased {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.event_id.clone()
    }

    fn entity_type() -> u8 {
        RESERVATION_RELEASED_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel
    where
        Self: Sized,
    {
        EntityMutationModel::AppendOnlyRecord
    }

    fn aggregate_role() -> AggregateRole
    where
        Self: Sized,
    {
        AggregateRole::AggregateRoot
    }

    fn financial_classification() -> FinancialClassification
    where
        Self: Sized,
    {
        FinancialClassification::BusinessVoucher
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("event_id", "", self.event_id.clone()),
            EntityFieldChange::new("reservation_id", "", self.reservation_id.clone()),
            EntityFieldChange::new("owner_account_id", "", self.owner_account_id.clone()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("amount", "", self.amount.to_string()),
            EntityFieldChange::new("caused_by_ref_id", "", self.caused_by_ref_id.clone()),
            EntityFieldChange::new(
                "remaining_amount_after",
                "",
                self.remaining_amount_after.to_string(),
            ),
            EntityFieldChange::new("close_reason", "", self.close_reason.as_str()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "event_id" | "reservation_id" | "owner_account_id" | "asset_id"
            | "caused_by_ref_id" | "close_reason" => 0,
            "amount" | "remaining_amount_after" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.event_id))
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

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn active_reservation() -> Reservation {
        Reservation::new(
            "reservation:1".to_string(),
            "trader-1".to_string(),
            "order-1".to_string(),
            ReservationMarketKind::Spot,
            ReservationKind::SpotBuyQuote,
            "USDT".to_string(),
            200,
        )
        .unwrap()
    }

    #[test]
    fn constructor_sets_active_amounts() {
        let reservation = active_reservation();

        assert_eq!(reservation.remaining_amount, 200);
        assert_eq!(reservation.status, ReservationStatus::Active);
        assert!(reservation.has_consistent_amounts());
    }

    #[test]
    fn partial_consume_keeps_reservation_active() {
        let after = active_reservation().consume(80, None).unwrap();

        assert_eq!(after.consumed_amount, 80);
        assert_eq!(after.remaining_amount, 120);
        assert_eq!(after.status, ReservationStatus::Active);
        assert_eq!(after.close_reason, None);
        assert!(after.has_consistent_amounts());
    }

    #[test]
    fn full_consume_closes_as_exhausted() {
        let after =
            active_reservation().consume(200, Some(ReservationCloseReason::Filled)).unwrap();

        assert_eq!(after.remaining_amount, 0);
        assert_eq!(after.status, ReservationStatus::ExhaustedByConsume);
        assert_eq!(after.close_reason, Some(ReservationCloseReason::Filled));
        assert!(after.has_consistent_amounts());
    }

    #[test]
    fn mixed_close_tracks_consumed_and_released() {
        let after_consume = active_reservation().consume(80, None).unwrap();
        let after_release =
            after_consume.release(120, Some(ReservationCloseReason::IocRemainderCanceled)).unwrap();

        assert_eq!(after_release.consumed_amount, 80);
        assert_eq!(after_release.released_amount, 120);
        assert_eq!(after_release.remaining_amount, 0);
        assert_eq!(after_release.status, ReservationStatus::ClosedMixed);
        assert_eq!(after_release.close_reason, Some(ReservationCloseReason::IocRemainderCanceled));
    }

    #[test]
    fn reservation_create_record_is_append_only() {
        let reservation = active_reservation();
        let created =
            ReservationCreated::from_reservation("reservation-created:1".to_string(), &reservation);

        assert_eq!(ReservationCreated::mutation_model(), EntityMutationModel::AppendOnlyRecord);
        assert_eq!(created.original_amount, 200);
    }

    #[test]
    fn reservation_facts_use_business_voucher_financial_classification() {
        assert_eq!(
            Reservation::financial_classification(),
            FinancialClassification::BusinessVoucher
        );
        assert_eq!(
            ReservationCreated::financial_classification(),
            FinancialClassification::BusinessVoucher
        );
        assert_eq!(
            ReservationConsumed::financial_classification(),
            FinancialClassification::BusinessVoucher
        );
        assert_eq!(
            ReservationReleased::financial_classification(),
            FinancialClassification::BusinessVoucher
        );
    }
}
