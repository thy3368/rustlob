use std::collections::HashSet;

use common_entity::{Entity, EntityError, LogEntry, StateMachineV2, StateMachineV2Diff};
use example_core_entity::account::coin::Coin;
use example_core_entity::common::CommonOrderTrigger;
use example_core_entity::SpotOrderV3;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::{
    OrderGroupRelation, OrderSide, OrderTif, OrderTriggerRole, OrderType, SpotOrderV2,
    SpotOrderV2BehaviorError,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceOnlySpotOrderV3OrderType {
    None,
    Trigger { is_market: bool, trigger_price: String, trigger_role: CommonOrderTrigger },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceOnlySpotOrderV3OrderCmd {
    pub coin: Coin,
    pub order_id: u64, //创建时，是没有orderId的，其实也挺聪明的，你直接在外面带给我就好
    pub is_buy: bool,
    pub price: String,
    pub size: String,
    pub order_type: PlaceOnlySpotOrderV3OrderType,
    pub reduce_only: bool,
    pub tif: OrderTif,
    pub cloid: Option<String>,
    // pub maker_fee_bps: u64,
    // pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceOnlySpotOrderV3Cmd {
    Single(PlaceOnlySpotOrderV3OrderCmd),
    NormalTpsl { parent: PlaceOnlySpotOrderV3OrderCmd, children: Vec<PlaceOnlySpotOrderV3OrderCmd> },
}

pub type PlaceOnlySpotOrderV3State = ();

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOnlySpotOrderV3AfterChanges {
    Single { created_order: SpotOrderV3 },
    NormalTpsl { created_parent_order: SpotOrderV3, created_child_orders: Vec<SpotOrderV3> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOnlySpotOrderV3Changes {
    Single { created_order: SpotOrderV3 },
    NormalTpsl { created_parent_order: SpotOrderV3, created_child_orders: Vec<SpotOrderV3> },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceOnlySpotOrderV3Error {
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
    #[error("on_command branch does not match given state branch")]
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
pub struct PlaceOnlySpotOrderV3UseCase;

impl StateMachineV2 for PlaceOnlySpotOrderV3UseCase {
    type Command = LogEntry<PlaceOnlySpotOrderV3Cmd>;
    type StateGiven = PlaceOnlySpotOrderV3State;
    type Error = PlaceOnlySpotOrderV3Error;
    type StateChanged = PlaceOnlySpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match cmd.payload() {
            PlaceOnlySpotOrderV3Cmd::Single(order) => check_order_command(order),
            PlaceOnlySpotOrderV3Cmd::NormalTpsl { parent, children } => {
                check_order_command(parent)?;
                if children.is_empty() {
                    return Err(PlaceOnlySpotOrderV3Error::ChildrenRequired);
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
        if let PlaceOnlySpotOrderV3Cmd::NormalTpsl { parent, children } = cmd.payload() {
            validate_normal_tpsl(parent, children)?;
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        _given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {

    }
}

