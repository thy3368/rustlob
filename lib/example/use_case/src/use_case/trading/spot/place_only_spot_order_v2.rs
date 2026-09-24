use std::collections::HashSet;

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, ExecutionContext, IssuedByParty, ReplayableChanges,
    StateMachineOwnedV2Diff, StateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::{
    SpotOrderGroupRelation, SpotOrderSide, SpotOrderTif, SpotOrderTriggerRole, SpotOrderType,
    SpotOrderV2, SpotOrderV2BehaviorError,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceOnlySpotOrderV2OrderType {
    Limit { tif: String },
    Trigger { is_market: bool, trigger_price: String, trigger_role: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceOnlySpotOrderV2OrderCmd {
    pub party_id: String,
    pub asset: u32,
    pub order_id: u64,
    pub symbol: String,
    pub is_buy: bool,
    pub price: String,
    pub size: String,
    pub order_type: PlaceOnlySpotOrderV2OrderType,
    pub reduce_only: bool,
    pub cloid: Option<String>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceOnlySpotOrderV2Cmd {
    Single(PlaceOnlySpotOrderV2OrderCmd),
    NormalTpsl { parent: PlaceOnlySpotOrderV2OrderCmd, children: Vec<PlaceOnlySpotOrderV2OrderCmd> },
}

impl IssuedByParty for PlaceOnlySpotOrderV2Cmd {
    fn party_id(&self) -> Option<&str> {
        match self {
            Self::Single(order) => Some(order.party_id.as_str()),
            Self::NormalTpsl { parent, .. } => Some(parent.party_id.as_str()),
        }
    }
}

pub type PlaceOnlySpotOrderV2State = ();

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOnlySpotOrderV2AfterChanges {
    Single { created_order: SpotOrderV2 },
    NormalTpsl { created_parent_order: SpotOrderV2, created_child_orders: Vec<SpotOrderV2> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOnlySpotOrderV2Changes {
    Single { created_order: SpotOrderV2 },
    NormalTpsl { created_parent_order: SpotOrderV2, created_child_orders: Vec<SpotOrderV2> },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceOnlySpotOrderV2Error {
    #[error("party id must not be empty")]
    EmptyPartyId,
    #[error("order oid must be greater than zero")]
    InvalidOrderId,
    #[error("symbol must not be empty")]
    EmptySymbol,
    #[error("asset id must not be empty")]
    EmptyAssetId,
    #[error("price must be a positive integer string")]
    InvalidPrice,
    #[error("size must be a positive integer string")]
    InvalidSize,
    #[error("time in force must be gtc, ioc, or alo")]
    InvalidTimeInForce,
    #[error("trigger price must be a positive integer string")]
    InvalidTriggerPrice,
    #[error("trigger role must be tp/take_profit or sl/stop_loss")]
    InvalidTriggerRole,
    #[error("command branch does not match given state branch")]
    BranchMismatch,
    #[error("normalTpsl must contain at least one child order")]
    ChildrenRequired,
    #[error("normalTpsl parent must be a limit order")]
    ParentMustBeLimit,
    #[error("normalTpsl parent must not be reduce-only")]
    ParentMustNotBeReduceOnly,
    #[error("normalTpsl child must be a trigger order")]
    ChildMustBeTrigger,
    #[error("normalTpsl child must be reduce-only")]
    ChildMustBeReduceOnly,
    #[error("normalTpsl child must have the opposite side from its parent")]
    ChildSideMustOpposeParent,
    #[error("normalTpsl child account differs from its parent")]
    ChildAccountMismatch,
    #[error("normalTpsl child asset differs from its parent")]
    ChildAssetMismatch,
    #[error("normalTpsl child symbol differs from its parent")]
    ChildSymbolMismatch,
    #[error("normalTpsl child quantity exceeds its parent quantity")]
    ChildQuantityExceedsParent,
    #[error("normalTpsl order ids must be unique")]
    DuplicateOrderId,
    #[error("arithmetic overflow while computing place-only spot order v2")]
    ArithmeticOverflow,
    #[error(transparent)]
    OrderBehavior(#[from] SpotOrderV2BehaviorError),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceOnlySpotOrderV2UseCase;

impl ReplayableChanges for PlaceOnlySpotOrderV2Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        match self {
            Self::Single { created_order } => Ok(vec![created_order.track_create_event()?]),
            Self::NormalTpsl { created_parent_order, created_child_orders } => {
                let mut events = Vec::with_capacity(created_child_orders.len().saturating_add(1));
                events.push(created_parent_order.track_create_event()?);
                for child in created_child_orders {
                    events.push(child.track_create_event()?);
                }
                Ok(events)
            }
        }
    }
}

impl StateMachineV2Unchecked for PlaceOnlySpotOrderV2UseCase {
    type Command = PlaceOnlySpotOrderV2Cmd;
    type StateGiven = PlaceOnlySpotOrderV2State;
    type Error = PlaceOnlySpotOrderV2Error;
    type StateChanged = PlaceOnlySpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match cmd {
            PlaceOnlySpotOrderV2Cmd::Single(order) => check_order_command(order),
            PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, children } => {
                check_order_command(parent)?;
                if children.is_empty() {
                    return Err(PlaceOnlySpotOrderV2Error::ChildrenRequired);
                }
                for child in children {
                    check_order_command(child)?;
                }
                Ok(())
            }
        }
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        _given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if let PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, children } = cmd {
            validate_normal_tpsl(parent, children)?;
        }
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        _given_state: &Self::StateGiven,
        context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        match cmd {
            PlaceOnlySpotOrderV2Cmd::Single(order) => {
                Ok(PlaceOnlySpotOrderV2AfterChanges::Single {
                    created_order: build_order(order, context.execution_time_ns)?,
                })
            }
            PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, children } => {
                let parent_order_id = parent.order_id.clone();
                let mut created_parent_order = build_order(parent, context.execution_time_ns)?;
                created_parent_order.group_relation = SpotOrderGroupRelation::NormalTpslParent;

                let mut created_child_orders = Vec::with_capacity(children.len());
                for child in children {
                    let mut child_order = build_order(child, context.execution_time_ns)?;
                    child_order.group_relation = SpotOrderGroupRelation::NormalTpslChild {
                        parent_order_id: parent_order_id.clone(),
                    };
                    created_child_orders.push(child_order);
                }

                Ok(PlaceOnlySpotOrderV2AfterChanges::NormalTpsl {
                    created_parent_order,
                    created_child_orders,
                })
            }
        }
    }
}

impl StateMachineOwnedV2Diff for PlaceOnlySpotOrderV2UseCase {
    type StateDiff = PlaceOnlySpotOrderV2Changes;

    fn do_compute_state_diff(
        _given_state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(match after {
            PlaceOnlySpotOrderV2AfterChanges::Single { created_order } => {
                PlaceOnlySpotOrderV2Changes::Single { created_order }
            }
            PlaceOnlySpotOrderV2AfterChanges::NormalTpsl {
                created_parent_order,
                created_child_orders,
            } => PlaceOnlySpotOrderV2Changes::NormalTpsl {
                created_parent_order,
                created_child_orders,
            },
        })
    }
}

fn check_order_command(
    order: &PlaceOnlySpotOrderV2OrderCmd,
) -> Result<(), PlaceOnlySpotOrderV2Error> {
    if order.party_id.is_empty() {
        return Err(PlaceOnlySpotOrderV2Error::EmptyPartyId);
    }
    if order.order_id == 0 {
        return Err(PlaceOnlySpotOrderV2Error::InvalidOrderId);
    }
    if order.symbol.is_empty() {
        return Err(PlaceOnlySpotOrderV2Error::EmptySymbol);
    }
    if order.base_asset_id.is_empty() || order.quote_asset_id.is_empty() {
        return Err(PlaceOnlySpotOrderV2Error::EmptyAssetId);
    }
    parse_positive_u64(&order.price, PlaceOnlySpotOrderV2Error::InvalidPrice)?;
    parse_positive_u64(&order.size, PlaceOnlySpotOrderV2Error::InvalidSize)?;
    match &order.order_type {
        PlaceOnlySpotOrderV2OrderType::Limit { tif } => {
            parse_tif(tif)?;
        }
        PlaceOnlySpotOrderV2OrderType::Trigger { trigger_price, trigger_role, .. } => {
            parse_positive_u64(trigger_price, PlaceOnlySpotOrderV2Error::InvalidTriggerPrice)?;
            parse_trigger_role(trigger_role)?;
        }
    }
    Ok(())
}

fn validate_normal_tpsl(
    parent: &PlaceOnlySpotOrderV2OrderCmd,
    children: &[PlaceOnlySpotOrderV2OrderCmd],
) -> Result<(), PlaceOnlySpotOrderV2Error> {
    if !matches!(parent.order_type, PlaceOnlySpotOrderV2OrderType::Limit { .. }) {
        return Err(PlaceOnlySpotOrderV2Error::ParentMustBeLimit);
    }
    if parent.reduce_only {
        return Err(PlaceOnlySpotOrderV2Error::ParentMustNotBeReduceOnly);
    }
    if children.is_empty() {
        return Err(PlaceOnlySpotOrderV2Error::ChildrenRequired);
    }

    let parent_qty = parse_positive_u64(&parent.size, PlaceOnlySpotOrderV2Error::InvalidSize)?;
    let mut order_ids = HashSet::with_capacity(children.len().saturating_add(1));
    order_ids.insert(parent.order_id);

    for child in children {
        if !matches!(child.order_type, PlaceOnlySpotOrderV2OrderType::Trigger { .. }) {
            return Err(PlaceOnlySpotOrderV2Error::ChildMustBeTrigger);
        }
        if !child.reduce_only {
            return Err(PlaceOnlySpotOrderV2Error::ChildMustBeReduceOnly);
        }
        if child.is_buy == parent.is_buy {
            return Err(PlaceOnlySpotOrderV2Error::ChildSideMustOpposeParent);
        }
        if child.party_id != parent.party_id {
            return Err(PlaceOnlySpotOrderV2Error::ChildAccountMismatch);
        }
        if child.asset != parent.asset {
            return Err(PlaceOnlySpotOrderV2Error::ChildAssetMismatch);
        }
        if child.symbol != parent.symbol {
            return Err(PlaceOnlySpotOrderV2Error::ChildSymbolMismatch);
        }
        if parse_positive_u64(&child.size, PlaceOnlySpotOrderV2Error::InvalidSize)? > parent_qty {
            return Err(PlaceOnlySpotOrderV2Error::ChildQuantityExceedsParent);
        }
        if !order_ids.insert(child.order_id) {
            return Err(PlaceOnlySpotOrderV2Error::DuplicateOrderId);
        }
    }
    Ok(())
}

fn build_order(
    order: &PlaceOnlySpotOrderV2OrderCmd,
    created_at: u64,
) -> Result<SpotOrderV2, PlaceOnlySpotOrderV2Error> {
    let side = if order.is_buy { SpotOrderSide::Buy } else { SpotOrderSide::Sell };
    let qty = parse_positive_u64(&order.size, PlaceOnlySpotOrderV2Error::InvalidSize)?;
    let price = parse_positive_u64(&order.price, PlaceOnlySpotOrderV2Error::InvalidPrice)?;
    match &order.order_type {
        PlaceOnlySpotOrderV2OrderType::Limit { tif } => {
            let tif = parse_tif(tif)?;
            let mut created = SpotOrderV2::new_pending_limit(
                order.order_id,
                order.asset,
                order.party_id.clone(),
                order.symbol.clone(),
                side,
                qty,
                price,
                SpotOrderType::Limit { tif },
                order.cloid.clone(),
                1,
                created_at,
            );
            created.reduce_only = order.reduce_only;
            Ok(created)
        }
        PlaceOnlySpotOrderV2OrderType::Trigger { is_market, trigger_price, trigger_role } => {
            let mut created = SpotOrderV2::new_pending_trigger(
                order.order_id,
                order.asset,
                order.party_id.clone(),
                order.symbol.clone(),
                side,
                qty,
                price,
                SpotOrderType::Trigger {
                    is_market: *is_market,
                    trigger_price: parse_positive_u64(
                        trigger_price,
                        PlaceOnlySpotOrderV2Error::InvalidTriggerPrice,
                    )?,
                    tpsl: parse_trigger_role(trigger_role)?,
                },
                order.cloid.clone(),
                1,
                created_at,
            );
            created.reduce_only = order.reduce_only;
            Ok(created)
        }
    }
}

fn parse_positive_u64(
    raw: &str,
    error: PlaceOnlySpotOrderV2Error,
) -> Result<u64, PlaceOnlySpotOrderV2Error> {
    match raw.parse::<u64>() {
        Ok(value) if value > 0 => Ok(value),
        _ => Err(error),
    }
}

fn parse_tif(raw: &str) -> Result<SpotOrderTif, PlaceOnlySpotOrderV2Error> {
    match raw {
        "gtc" | "Gtc" => Ok(SpotOrderTif::Gtc),
        "ioc" | "Ioc" => Ok(SpotOrderTif::Ioc),
        "alo" | "Alo" => Ok(SpotOrderTif::Alo),
        _ => Err(PlaceOnlySpotOrderV2Error::InvalidTimeInForce),
    }
}

fn parse_trigger_role(raw: &str) -> Result<SpotOrderTriggerRole, PlaceOnlySpotOrderV2Error> {
    match raw {
        "tp" | "take_profit" | "TakeProfit" => Ok(SpotOrderTriggerRole::TakeProfit),
        "sl" | "stop_loss" | "StopLoss" => Ok(SpotOrderTriggerRole::StopLoss),
        _ => Err(PlaceOnlySpotOrderV2Error::InvalidTriggerRole),
    }
}
