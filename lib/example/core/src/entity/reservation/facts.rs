use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel,
    FinancialClassification, FourColorArchetype,
};
use serde::{Deserialize, Serialize};

use super::{
    RESERVATION_CONSUMED_ENTITY_TYPE, RESERVATION_CREATED_ENTITY_TYPE,
    RESERVATION_RELEASED_ENTITY_TYPE, Reservation, ReservationCloseReason, ReservationKind,
    ReservationMarketKind, stable_entity_id,
};

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
