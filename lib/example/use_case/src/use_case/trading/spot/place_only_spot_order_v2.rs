use std::collections::HashSet;

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, IssuedByParty, ReplayableChanges,
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
    pub order_id: String,
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
    #[error("order id must not be empty")]
    EmptyOrderId,
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
    ) -> Result<Self::StateChanged, Self::Error> {
        match cmd {
            PlaceOnlySpotOrderV2Cmd::Single(order) => {
                Ok(PlaceOnlySpotOrderV2AfterChanges::Single { created_order: build_order(order)? })
            }
            PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, children } => {
                let parent_order_id = parent.order_id.clone();
                let mut created_parent_order = build_order(parent)?;
                created_parent_order.group_relation = SpotOrderGroupRelation::NormalTpslParent;

                let mut created_child_orders = Vec::with_capacity(children.len());
                for child in children {
                    let mut child_order = build_order(child)?;
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
    if order.order_id.is_empty() {
        return Err(PlaceOnlySpotOrderV2Error::EmptyOrderId);
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
    order_ids.insert(parent.order_id.as_str());

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
        if !order_ids.insert(child.order_id.as_str()) {
            return Err(PlaceOnlySpotOrderV2Error::DuplicateOrderId);
        }
    }
    Ok(())
}

fn build_order(
    order: &PlaceOnlySpotOrderV2OrderCmd,
) -> Result<SpotOrderV2, PlaceOnlySpotOrderV2Error> {
    let side = if order.is_buy { SpotOrderSide::Buy } else { SpotOrderSide::Sell };
    let qty = parse_positive_u64(&order.size, PlaceOnlySpotOrderV2Error::InvalidSize)?;
    let price = parse_positive_u64(&order.price, PlaceOnlySpotOrderV2Error::InvalidPrice)?;
    match &order.order_type {
        PlaceOnlySpotOrderV2OrderType::Limit { tif } => {
            let tif = parse_tif(tif)?;
            let mut created = SpotOrderV2::new_active(
                order.order_id.clone(),
                order.asset,
                None,
                order.party_id.clone(),
                order.symbol.clone(),
                side,
                price,
                SpotOrderType::Limit { tif },
                qty,
                order.base_asset_id.as_str(),
                order.quote_asset_id.as_str(),
                order.maker_fee_bps,
                order.taker_fee_bps,
                order.cloid.clone(),
            )?;
            created.reduce_only = order.reduce_only;
            created.order_type = SpotOrderType::Limit { tif };
            Ok(created)
        }
        PlaceOnlySpotOrderV2OrderType::Trigger { is_market, trigger_price, trigger_role } => {
            let mut created = SpotOrderV2::new_trigger_pending(
                order.order_id.clone(),
                order.asset,
                None,
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

#[cfg(test)]
mod tests {
    use common_entity::StateMachineOwnedV2Diff;

    use super::*;
    use crate::entity::{ReservationStatus, SpotOrderStatus};

    fn limit_cmd(tif: &str) -> PlaceOnlySpotOrderV2OrderCmd {
        PlaceOnlySpotOrderV2OrderCmd {
            party_id: "trader-1".to_string(),
            asset: 10_001,
            order_id: "order-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            is_buy: true,
            price: "100".to_string(),
            size: "2".to_string(),
            order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: tif.to_string() },
            reduce_only: false,
            cloid: Some("cloid-1".to_string()),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 1,
            taker_fee_bps: 5,
        }
    }

    fn trigger_cmd(order_id: &str, is_market: bool) -> PlaceOnlySpotOrderV2OrderCmd {
        PlaceOnlySpotOrderV2OrderCmd {
            order_id: order_id.to_string(),
            is_buy: false,
            price: "95".to_string(),
            size: "1".to_string(),
            order_type: PlaceOnlySpotOrderV2OrderType::Trigger {
                is_market,
                trigger_price: "90".to_string(),
                trigger_role: "sl".to_string(),
            },
            reduce_only: true,
            cloid: None,
            ..limit_cmd("gtc")
        }
    }

    fn single_order(
        order: PlaceOnlySpotOrderV2OrderCmd,
    ) -> Result<SpotOrderV2, PlaceOnlySpotOrderV2Error> {
        match PlaceOnlySpotOrderV2UseCase
            .compute_state_diff(&PlaceOnlySpotOrderV2Cmd::Single(order), ())?
        {
            PlaceOnlySpotOrderV2Changes::Single { created_order } => Ok(created_order),
            PlaceOnlySpotOrderV2Changes::NormalTpsl { .. } => {
                Err(PlaceOnlySpotOrderV2Error::BranchMismatch)
            }
        }
    }

    #[test]
    fn rejects_command_only_invalid_values() {
        let use_case = PlaceOnlySpotOrderV2UseCase;
        let mut invalid_price = limit_cmd("gtc");
        invalid_price.price = "0".to_string();
        assert_eq!(
            use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(invalid_price)),
            Err(PlaceOnlySpotOrderV2Error::InvalidPrice)
        );

        let mut invalid_size = limit_cmd("gtc");
        invalid_size.size = "abc".to_string();
        assert_eq!(
            use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(invalid_size)),
            Err(PlaceOnlySpotOrderV2Error::InvalidSize)
        );

        assert_eq!(
            use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(limit_cmd("day"))),
            Err(PlaceOnlySpotOrderV2Error::InvalidTimeInForce)
        );

        let mut invalid_trigger = trigger_cmd("trigger-1", false);
        invalid_trigger.order_type = PlaceOnlySpotOrderV2OrderType::Trigger {
            is_market: false,
            trigger_price: "0".to_string(),
            trigger_role: "bad".to_string(),
        };
        assert_eq!(
            use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(invalid_trigger)),
            Err(PlaceOnlySpotOrderV2Error::InvalidTriggerPrice)
        );

        let mut invalid_trigger_role = trigger_cmd("trigger-2", false);
        invalid_trigger_role.order_type = PlaceOnlySpotOrderV2OrderType::Trigger {
            is_market: false,
            trigger_price: "90".to_string(),
            trigger_role: "bad".to_string(),
        };
        assert_eq!(
            use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(invalid_trigger_role)),
            Err(PlaceOnlySpotOrderV2Error::InvalidTriggerRole)
        );

        let mut empty_party = limit_cmd("gtc");
        empty_party.party_id.clear();
        assert_eq!(
            use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(empty_party)),
            Err(PlaceOnlySpotOrderV2Error::EmptyPartyId)
        );
    }

    #[test]
    fn creates_limit_and_trigger_orders_without_side_effect_facts()
    -> Result<(), PlaceOnlySpotOrderV2Error> {
        for (tif, expected_tif) in
            [("gtc", SpotOrderTif::Gtc), ("Alo", SpotOrderTif::Alo), ("Ioc", SpotOrderTif::Ioc)]
        {
            let order = single_order(limit_cmd(tif))?;
            assert_eq!(order.status, SpotOrderStatus::Open);
            assert!(order.active_reservation().is_some());
            assert_eq!(order.time_in_force(), expected_tif);
            assert!(order.reservation.is_active());
            assert!(order.fee_reservation.is_active());
        }

        for (is_market, expected_tif) in [(false, SpotOrderTif::Gtc), (true, SpotOrderTif::Ioc)] {
            let order = single_order(trigger_cmd("trigger-1", is_market))?;
            assert!(order.is_pending());
            assert_eq!(order.status, SpotOrderStatus::Pending);
            assert_eq!(order.time_in_force(), expected_tif);
            assert_eq!(order.reservation.status, ReservationStatus::ClosedByRelease);
            assert_eq!(order.fee_reservation.status, ReservationStatus::ClosedByRelease);
            assert_eq!(order.reservation.remaining_amount, 0);
            assert_eq!(order.fee_reservation.remaining_amount, 0);
        }

        let changes = PlaceOnlySpotOrderV2UseCase
            .compute_state_diff(&PlaceOnlySpotOrderV2Cmd::Single(limit_cmd("gtc")), ())?;
        let events = changes.to_replayable_events()?;
        assert_eq!(events.len(), 1);
        assert!(events[0].is_created());
        assert_eq!(events[0].entity_type, SpotOrderV2::entity_type());
        Ok(())
    }

    #[test]
    fn creates_normal_tpsl_and_replays_parent_then_children()
    -> Result<(), PlaceOnlySpotOrderV2Error> {
        let cmd = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
            parent: limit_cmd("gtc"),
            children: vec![trigger_cmd("child-tp", false), trigger_cmd("child-sl", true)],
        };
        let changes = PlaceOnlySpotOrderV2UseCase.compute_state_diff(&cmd, ())?;
        let PlaceOnlySpotOrderV2Changes::NormalTpsl { created_parent_order, created_child_orders } =
            &changes
        else {
            return Err(PlaceOnlySpotOrderV2Error::BranchMismatch);
        };

        assert_eq!(created_parent_order.group_relation, SpotOrderGroupRelation::NormalTpslParent);
        assert_eq!(created_child_orders.len(), 2);
        for child in created_child_orders {
            assert!(child.is_pending());
            assert_eq!(child.status, SpotOrderStatus::Pending);
            assert!(child.reduce_only);
            assert_ne!(child.side, created_parent_order.side);
            assert_eq!(child.account_id, created_parent_order.account_id);
            assert_eq!(child.asset, created_parent_order.asset);
            assert_eq!(
                child.group_relation,
                SpotOrderGroupRelation::NormalTpslChild {
                    parent_order_id: created_parent_order.order_id.clone()
                }
            );
        }

        let events = changes.to_replayable_events()?;
        assert_eq!(events.len(), 3);
        assert!(events.iter().all(EntityReplayableEvent::is_created));
        assert!(events.iter().all(|event| event.entity_type == SpotOrderV2::entity_type()));
        assert_eq!(events[0].entity_id, created_parent_order.track_create_event()?.entity_id);
        Ok(())
    }

    #[test]
    fn rejects_invalid_normal_tpsl_relations_and_validates_before_compute() {
        let use_case = PlaceOnlySpotOrderV2UseCase;
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent: limit_cmd("gtc"), children: vec![] },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ChildrenRequired)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: trigger_cmd("parent", false),
                    children: vec![trigger_cmd("child", false)],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ParentMustBeLimit)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: PlaceOnlySpotOrderV2OrderCmd { reduce_only: true, ..limit_cmd("gtc") },
                    children: vec![trigger_cmd("child", false)],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ParentMustNotBeReduceOnly)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: limit_cmd("gtc"),
                    children: vec![PlaceOnlySpotOrderV2OrderCmd {
                        order_id: "child".to_string(),
                        is_buy: false,
                        reduce_only: true,
                        ..limit_cmd("gtc")
                    }],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ChildMustBeTrigger)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: limit_cmd("gtc"),
                    children: vec![PlaceOnlySpotOrderV2OrderCmd {
                        reduce_only: false,
                        ..trigger_cmd("child", false)
                    }],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ChildMustBeReduceOnly)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: limit_cmd("gtc"),
                    children: vec![PlaceOnlySpotOrderV2OrderCmd {
                        is_buy: true,
                        ..trigger_cmd("child", false)
                    }],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ChildSideMustOpposeParent)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: limit_cmd("gtc"),
                    children: vec![PlaceOnlySpotOrderV2OrderCmd {
                        party_id: "trader-2".to_string(),
                        ..trigger_cmd("child", false)
                    }],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ChildAccountMismatch)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: limit_cmd("gtc"),
                    children: vec![PlaceOnlySpotOrderV2OrderCmd {
                        asset: 10_002,
                        ..trigger_cmd("child", false)
                    }],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ChildAssetMismatch)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: limit_cmd("gtc"),
                    children: vec![PlaceOnlySpotOrderV2OrderCmd {
                        symbol: "ETHUSDT".to_string(),
                        ..trigger_cmd("child", false)
                    }],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ChildSymbolMismatch)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: limit_cmd("gtc"),
                    children: vec![PlaceOnlySpotOrderV2OrderCmd {
                        size: "3".to_string(),
                        ..trigger_cmd("child", false)
                    }],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::ChildQuantityExceedsParent)
        );
        assert_eq!(
            use_case.compute_state_diff(
                &PlaceOnlySpotOrderV2Cmd::NormalTpsl {
                    parent: limit_cmd("gtc"),
                    children: vec![PlaceOnlySpotOrderV2OrderCmd {
                        order_id: "order-1".to_string(),
                        ..trigger_cmd("order-1", false)
                    }],
                },
                ()
            ),
            Err(PlaceOnlySpotOrderV2Error::DuplicateOrderId)
        );
        let mut invalid = limit_cmd("gtc");
        invalid.price = "0".to_string();
        assert_eq!(
            use_case.compute_state_diff(&PlaceOnlySpotOrderV2Cmd::Single(invalid), ()),
            Err(PlaceOnlySpotOrderV2Error::InvalidPrice)
        );
    }
}
